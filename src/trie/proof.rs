use borsh_derive::{BorshDeserialize, BorshSerialize};
use std::collections::{BTreeMap, BTreeSet};

use super::{
    key::leaf_parent_key, Nibble, NibblePatriciaTrieError, NibblePatriciaTrieNode,
    NibblePatriciaTrieNodeBranch, NibblePatriciaTrieNodeLeaf,
};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct NibblePatriciaTrieRootPath {
    pub nodes_branch: BTreeMap<Vec<Nibble>, NibblePatriciaTrieNodeBranch>,
    pub nodes_hashed: BTreeMap<Vec<Nibble>, [u8; 32]>,
}

impl NibblePatriciaTrieRootPath {
    pub fn new(
        nodes_branch: BTreeMap<Vec<Nibble>, NibblePatriciaTrieNodeBranch>,
        nodes_hashed: BTreeMap<Vec<Nibble>, [u8; 32]>,
    ) -> Self {
        Self {
            nodes_branch,
            nodes_hashed,
        }
    }

    /// Construct inclusion proof / non inclusion proof from the designated leafs
    pub fn from_leafs(
        leaf_keys: BTreeSet<Vec<Nibble>>,
        get_node: impl Fn(&[Nibble]) -> Result<NibblePatriciaTrieNode, NibblePatriciaTrieError>,
        get_node_hash: impl Fn(&[Nibble]) -> Result<[u8; 32], NibblePatriciaTrieError>,
    ) -> Result<Self, NibblePatriciaTrieError> {
        // marked nodes means the nodes which are needed to be re-constructed in the inclusion proof
        let mut marked_nodes =
            BTreeMap::<usize, BTreeMap<Vec<Nibble>, NibblePatriciaTrieNode>>::new();

        for leaf_key in leaf_keys.iter() {
            let (parent_key, leaf_node) = leaf_parent_key(leaf_key, &get_node)?;
            let parent_node = get_node(&parent_key)?;

            if let NibblePatriciaTrieNode::Branch(parent_branch) = parent_node {
                marked_nodes
                    .entry(parent_key.len())
                    .or_insert_with(BTreeMap::new)
                    .insert(
                        parent_key.to_owned(),
                        NibblePatriciaTrieNode::Branch(parent_branch),
                    );

                marked_nodes
                    .entry(leaf_key.len())
                    .or_insert_with(BTreeMap::new)
                    .insert(leaf_key.to_owned(), NibblePatriciaTrieNode::Leaf(leaf_node));
            } else {
                return Err(NibblePatriciaTrieError::InvalidNode);
            }
        }

        if marked_nodes.is_empty() {
            return Err(NibblePatriciaTrieError::EmptyProof);
        }

        // get the depth of the deepest marked node
        let mut depth = *marked_nodes.keys().last().unwrap();

        let mut slf = Self::new(BTreeMap::new(), BTreeMap::new());

        loop {
            let marked_nodes_in_depth = marked_nodes.get(&depth);
            let mut new_marked_nodes = BTreeMap::new();

            if let Some(marked_nodes_in_depth) = marked_nodes_in_depth {
                // get all the marked nodes at the current depth
                for (key, branch) in marked_nodes_in_depth.iter() {
                    if let NibblePatriciaTrieNode::Branch(branch) = branch {
                        // add non-marked child nodes of the marked branch node to the proof
                        for (_index, child_key_fragment) in branch.child_key_fragments.iter() {
                            let child_key = key
                                .iter()
                                .chain(child_key_fragment)
                                .copied()
                                .collect::<Vec<_>>();

                            // check if the child key is already marked
                            if let Some(marked_nodes_in_child_depth) =
                                marked_nodes.get(&child_key.len())
                            {
                                if marked_nodes_in_child_depth.contains_key(&child_key) {
                                    continue;
                                }
                            }

                            // only the non-marked child nodes are added to the proof
                            let child_node_hash = get_node_hash(&child_key)?;

                            slf.nodes_hashed.insert(child_key, child_node_hash);
                        }

                        // add the parent node to the new marked nodes
                        let parent_key = &key[..key.len() - branch.key_fragment.len()];
                        let parent_node = get_node(parent_key)?;

                        if let NibblePatriciaTrieNode::Branch(parent_branch) = parent_node {
                            new_marked_nodes
                                .entry(parent_key.len())
                                .or_insert_with(BTreeMap::new)
                                .insert(
                                    parent_key.to_owned(),
                                    NibblePatriciaTrieNode::Branch(parent_branch),
                                );
                        } else {
                            return Err(NibblePatriciaTrieError::InvalidNode);
                        }

                        // add the marked node to the proof
                        slf.nodes_branch.insert(key.clone(), branch.clone());
                    }
                }
            }
            marked_nodes.extend(new_marked_nodes);

            if depth == 0 {
                break;
            }

            depth -= 1;
        }

        Ok(slf)
    }

    /// Verify the non inclusion of the leaf key
    pub fn verify_non_inclusion(&self, leaf_key: &[Nibble]) -> Result<(), NibblePatriciaTrieError> {
        let key_len = leaf_key.len();

        for i in (0..key_len).rev() {
            let key_path = &leaf_key[..i];

            let node = self.nodes_branch.get(key_path);

            if let Some(node) = node {
                // to prove the non-inclusion, branch node must not have the child key index for the leaf to prove non-inclusion
                if !node
                    .child_key_fragments
                    .contains_key(&Nibble::from(leaf_key[i]))
                {
                    return Ok(());
                }
                return Err(NibblePatriciaTrieError::InvalidProof);
            }
        }

        Err(NibblePatriciaTrieError::InvalidProof)
    }

    pub fn root(
        self,
        nodes_for_inclusion_proof: BTreeMap<Vec<Nibble>, NibblePatriciaTrieNodeLeaf>,
        branch_hash_callback: Option<Box<dyn Fn(&Vec<Nibble>, &[u8; 32])>>,
    ) -> Result<[u8; 32], NibblePatriciaTrieError> {
        let mut nodes_branch =
            BTreeMap::<usize, BTreeMap<Vec<Nibble>, NibblePatriciaTrieNodeBranch>>::new();
        let mut nodes_hashed = BTreeMap::<usize, BTreeMap<Vec<Nibble>, [u8; 32]>>::new();

        for (key, node) in self.nodes_branch {
            nodes_branch
                .entry(key.len())
                .or_insert_with(BTreeMap::new)
                .insert(key, node);
        }

        for (key, node) in self.nodes_hashed {
            nodes_hashed
                .entry(key.len())
                .or_insert_with(BTreeMap::new)
                .insert(key, node);
        }

        for (key, node) in nodes_for_inclusion_proof {
            nodes_hashed
                .entry(key.len())
                .or_insert_with(BTreeMap::new)
                .insert(key, node.hash());
        }

        if nodes_branch.is_empty() {
            return Err(NibblePatriciaTrieError::EmptyProof);
        }

        while let Some((_depth, nodes_branch_at_depth)) = nodes_branch.pop_last() {
            for (key, branch) in nodes_branch_at_depth {
                let hash = branch.hash(|index| {
                    let child_key_fragment = branch.child_key_fragments.get(&index).unwrap();

                    let child_key = key
                        .iter()
                        .chain(child_key_fragment)
                        .copied()
                        .collect::<Vec<_>>();

                    let nodes_hashed_at_depth = nodes_hashed.get(&child_key.len())?;
                    let child_hash = nodes_hashed_at_depth.get(&child_key);

                    child_hash.copied()
                });

                if let Some(hash) = hash {
                    if let Some(ref branch_hash_callback) = branch_hash_callback {
                        branch_hash_callback(&key, &hash);
                    }

                    nodes_hashed
                        .entry(key.len())
                        .or_insert_with(BTreeMap::new)
                        .insert(key, hash);
                }
            }
        }

        let root_hash = nodes_hashed
            .pop_first()
            .ok_or(NibblePatriciaTrieError::InvalidProof)?
            .1
            .pop_first()
            .ok_or(NibblePatriciaTrieError::InvalidProof)?
            .1;

        Ok(root_hash)
    }
}

#[cfg(test)]
mod tests {
    use borsh::BorshSerialize;

    use super::*;
    use crate::trie::db::NibblePatriciaTrieMemoryDb;
    use crate::trie::nibble::Nibble;
    use crate::trie::node::{
        NibblePatriciaTrieNode, NibblePatriciaTrieNodeBranch, NibblePatriciaTrieNodeLeaf,
    };
    use crate::trie::{get_node_from_db, get_node_hash_from_db, NibblePatriciaTrieDb};
    use std::collections::BTreeMap;

    fn setup_trie_and_db() -> (
        BTreeMap<Vec<Nibble>, Vec<u8>>,
        NibblePatriciaTrieMemoryDb,
        NibblePatriciaTrieMemoryDb,
        NibblePatriciaTrieNode,
    ) {
        use crate::trie::nibble::Nibble;
        // Prepare simple key-value pairs
        let mut entries = BTreeMap::new();
        entries.insert(vec![Nibble::from(1), Nibble::from(2)], b"a".to_vec());
        entries.insert(vec![Nibble::from(1), Nibble::from(3)], b"b".to_vec());
        entries.insert(vec![Nibble::from(2), Nibble::from(2)], b"c".to_vec());

        // Manually construct nodes
        // leaf [1,2]
        let leaf_12 = NibblePatriciaTrieNodeLeaf::new(vec![Nibble::from(2)], b"a".to_vec());
        let leaf_13 = NibblePatriciaTrieNodeLeaf::new(vec![Nibble::from(3)], b"b".to_vec());
        let leaf_22 =
            NibblePatriciaTrieNodeLeaf::new(vec![Nibble::from(2), Nibble::from(2)], b"c".to_vec());

        // branch [1]
        let mut branch_1_children = BTreeMap::new();
        branch_1_children.insert(Nibble::from(2), vec![Nibble::from(2)]);
        branch_1_children.insert(Nibble::from(3), vec![Nibble::from(3)]);
        let branch_1 = NibblePatriciaTrieNodeBranch::new(vec![Nibble::from(1)], branch_1_children);

        // root
        let mut root_children = BTreeMap::new();
        root_children.insert(Nibble::from(1), vec![Nibble::from(1)]); // [1] branch
        root_children.insert(Nibble::from(2), vec![Nibble::from(2), Nibble::from(2)]); // [2,2] leaf
        let root = NibblePatriciaTrieNodeBranch::new(vec![], root_children);

        // Prepare node_db and hash_db
        let mut node_db = NibblePatriciaTrieMemoryDb::new();
        let mut hash_db = NibblePatriciaTrieMemoryDb::new();

        // Serialize and store nodes
        let mut buf = Vec::new();
        // leaf [1,2]
        buf.clear();
        hash_db.set(&vec![Nibble::from(1), Nibble::from(2)], &leaf_12.hash()[..]);
        NibblePatriciaTrieNode::Leaf(leaf_12)
            .serialize(&mut buf)
            .unwrap();
        node_db.set(&vec![Nibble::from(1), Nibble::from(2)], &buf);
        // leaf [1,3]
        buf.clear();
        hash_db.set(&vec![Nibble::from(1), Nibble::from(3)], &leaf_13.hash()[..]);
        NibblePatriciaTrieNode::Leaf(leaf_13)
            .serialize(&mut buf)
            .unwrap();
        node_db.set(&vec![Nibble::from(1), Nibble::from(3)], &buf);
        // leaf [2,2]
        buf.clear();
        hash_db.set(&vec![Nibble::from(2), Nibble::from(2)], &leaf_22.hash()[..]);
        NibblePatriciaTrieNode::Leaf(leaf_22)
            .serialize(&mut buf)
            .unwrap();
        node_db.set(&vec![Nibble::from(2), Nibble::from(2)], &buf);
        // branch [1]
        buf.clear();
        // branch [1] hash
        let branch_1_hash = branch_1
            .hash(|nib| {
                let child_key = match nib.as_u8() {
                    2 => vec![Nibble::from(1), Nibble::from(2)],
                    3 => vec![Nibble::from(1), Nibble::from(3)],
                    _ => return None,
                };
                Some(<[u8; 32]>::try_from(hash_db.get(&child_key).unwrap().as_slice()).unwrap())
            })
            .unwrap();
        hash_db.set(&vec![Nibble::from(1)], &branch_1_hash[..]);
        NibblePatriciaTrieNode::Branch(branch_1)
            .serialize(&mut buf)
            .unwrap();
        node_db.set(&vec![Nibble::from(1)], &buf);
        // root
        buf.clear();
        // root hash
        let root_hash = root
            .hash(|nib| {
                let child_key = match nib.as_u8() {
                    1 => vec![Nibble::from(1)],
                    2 => vec![Nibble::from(2), Nibble::from(2)],
                    _ => return None,
                };
                Some(<[u8; 32]>::try_from(hash_db.get(&child_key).unwrap().as_slice()).unwrap())
            })
            .unwrap();
        hash_db.set(&vec![], &root_hash[..]);
        let root_node = NibblePatriciaTrieNode::Branch(root);
        root_node.serialize(&mut buf).unwrap();
        node_db.set(&vec![], &buf);

        (entries, node_db, hash_db, root_node)
    }

    #[test]
    fn test_leaf_parent_key() {
        let (_entries, node_db, _hash_db, _root_node) = setup_trie_and_db();

        let get_node = |key: &[Nibble]| get_node_from_db(key, &node_db);

        let (parent_key, _leaf_node) =
            leaf_parent_key(&vec![Nibble::from(1), Nibble::from(2)], &get_node).unwrap();
        assert_eq!(parent_key, vec![Nibble::from(1)]);

        let (parent_key, _leaf_node) =
            leaf_parent_key(&vec![Nibble::from(1), Nibble::from(3)], &get_node).unwrap();
        assert_eq!(parent_key, vec![Nibble::from(1)]);

        let (parent_key, _leaf_node) =
            leaf_parent_key(&vec![Nibble::from(2), Nibble::from(2)], &get_node).unwrap();
        assert_eq!(parent_key, vec![]);

        let err = leaf_parent_key(&vec![Nibble::from(2), Nibble::from(3)], &get_node);
        assert!(err.is_err());
    }

    #[test]
    fn test_from_leafs() {
        let (_entries, node_db, hash_db, _root_node) = setup_trie_and_db();

        let get_node = |key: &[Nibble]| get_node_from_db(key, &node_db);
        let get_node_hash = |key: &[Nibble]| get_node_hash_from_db(key, &hash_db);

        let leaf_keys = BTreeSet::from([vec![Nibble::from(1), Nibble::from(2)]]);
        let proof =
            NibblePatriciaTrieRootPath::from_leafs(leaf_keys, &get_node, &get_node_hash).unwrap();
        println!("proof: {:?}", proof);

        assert!(proof.nodes_branch.contains_key(&vec![Nibble::from(1)]));
        assert!(!proof
            .nodes_hashed
            .contains_key(&vec![Nibble::from(1), Nibble::from(2)]));
        assert!(proof
            .nodes_hashed
            .contains_key(&vec![Nibble::from(1), Nibble::from(3)]));
        assert!(proof
            .nodes_hashed
            .contains_key(&vec![Nibble::from(2), Nibble::from(2)]));
    }

    #[test]
    fn test_verify_non_inclusion() {
        let (_entries, node_db, hash_db, _root_node) = setup_trie_and_db();

        let get_node = |key: &[Nibble]| get_node_from_db(key, &node_db);
        let get_node_hash = |key: &[Nibble]| get_node_hash_from_db(key, &hash_db);

        let leaf_keys = BTreeSet::from([vec![Nibble::from(1), Nibble::from(3)]]);
        let proof =
            NibblePatriciaTrieRootPath::from_leafs(leaf_keys, &get_node, &get_node_hash).unwrap();
        println!("proof: {:?}", proof);

        assert!(proof
            .verify_non_inclusion(&vec![Nibble::from(1), Nibble::from(4)])
            .is_ok());
    }

    #[test]
    fn test_inclusion_proof_and_root() {
        let (entries, node_db, hash_db, _root_node) = setup_trie_and_db();

        let get_node = |key: &[Nibble]| get_node_from_db(key, &node_db);
        let get_node_hash = |key: &[Nibble]| get_node_hash_from_db(key, &hash_db);

        let leaf_key = vec![Nibble::from(1), Nibble::from(2)];
        let leaf_keys = BTreeSet::from([leaf_key.clone()]);
        let proof =
            NibblePatriciaTrieRootPath::from_leafs(leaf_keys, &get_node, &get_node_hash).unwrap();
        println!("proof: {:?}", proof);

        let leaf_value = entries.get(&leaf_key).unwrap();
        let root = proof
            .root(
                [(
                    leaf_key.clone(),
                    NibblePatriciaTrieNodeLeaf::new(vec![Nibble::from(2)], leaf_value.clone()),
                )]
                .into_iter()
                .collect(),
                None,
            )
            .unwrap();
        println!("root: {:?}", root);

        let root_hash: [u8; 32] = hash_db.get(&vec![]).unwrap().try_into().unwrap();

        assert_eq!(root, root_hash);
    }
}
