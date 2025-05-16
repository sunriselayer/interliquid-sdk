use borsh_derive::{BorshDeserialize, BorshSerialize};
use std::collections::{BTreeMap, BTreeSet};

use super::{
    key::leaf_parent_key, nibbles_from_bytes, search_near_leaf_parent_key, Nibble,
    NibblePatriciaTrieError, NibblePatriciaTrieNode, NibblePatriciaTrieNodeBranch,
    NibblePatriciaTrieNodeLeaf,
};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct NibblePatriciaTrieRootPath {
    pub nodes_branch: BTreeMap<Vec<Nibble>, NibblePatriciaTrieNodeBranch>,
    pub nodes_hashed: BTreeMap<Vec<Nibble>, [u8; 32]>,
}

impl NibblePatriciaTrieRootPath {
    /// Creates a new NibblePatriciaTrieRootPath instance.
    ///
    /// # Arguments
    ///
    /// * `nodes_branch` - A map of branch nodes in the trie, where the key is the path to the node
    /// * `nodes_hashed` - A map of hashed nodes in the trie, where the key is the path to the node
    pub fn new(
        nodes_branch: BTreeMap<Vec<Nibble>, NibblePatriciaTrieNodeBranch>,
        nodes_hashed: BTreeMap<Vec<Nibble>, [u8; 32]>,
    ) -> Self {
        Self {
            nodes_branch,
            nodes_hashed,
        }
    }

    /// Constructs an inclusion proof or non-inclusion proof from the designated leafs.
    ///
    /// This function builds a proof by:
    /// 1. Marking the leaf nodes and their parent nodes that need to be included in the proof
    /// 2. For each marked branch node, adding its non-marked child nodes to the proof
    /// 3. Recursively processing parent nodes until reaching the root
    ///
    /// # Arguments
    ///
    /// * `leaf_keys` - Set of leaf keys to include in the proof
    /// * `get_node` - Function to retrieve a node from the trie
    /// * `get_child_node_fragment_and_hash` - Function to get a child node's fragment and hash
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - A proof containing the necessary nodes and hashes
    /// * `Err(NibblePatriciaTrieError)` - If the proof construction fails
    pub fn from_leafs(
        leaf_keys: BTreeSet<Vec<Nibble>>,
        get_node: impl Fn(&[Nibble]) -> Result<NibblePatriciaTrieNode, NibblePatriciaTrieError>,
        get_child_node_fragment_and_hash: impl Fn(
            &[Nibble],
            Nibble,
        ) -> Result<
            (Vec<Nibble>, [u8; 32]),
            NibblePatriciaTrieError,
        >,
    ) -> Result<Self, NibblePatriciaTrieError> {
        // marked nodes means the nodes which are needed to be re-constructed in the inclusion proof
        let mut marked_nodes = BTreeMap::<Vec<Nibble>, NibblePatriciaTrieNode>::new();
        let mut marked_nodes_for_depth = BTreeMap::<usize, BTreeSet<Vec<Nibble>>>::new();

        for leaf_key in leaf_keys.iter() {
            let (parent_key, leaf_node) = leaf_parent_key(leaf_key, &get_node)?;
            let parent_node = get_node(&parent_key)?;

            if let NibblePatriciaTrieNode::Branch(parent_branch) = parent_node {
                marked_nodes.insert(
                    parent_key.clone(),
                    NibblePatriciaTrieNode::Branch(parent_branch),
                );
                marked_nodes_for_depth
                    .entry(parent_key.len())
                    .or_insert_with(BTreeSet::new)
                    .insert(parent_key);

                marked_nodes.insert(leaf_key.clone(), NibblePatriciaTrieNode::Leaf(leaf_node));
                marked_nodes_for_depth
                    .entry(leaf_key.len())
                    .or_insert_with(BTreeSet::new)
                    .insert(leaf_key.clone());
            } else {
                return Err(NibblePatriciaTrieError::InvalidNode);
            }
        }

        if marked_nodes.is_empty() {
            return Err(NibblePatriciaTrieError::EmptyProof);
        }

        // get the depth of the deepest marked node
        let mut depth = *marked_nodes_for_depth.keys().last().unwrap();

        let mut slf = Self::new(BTreeMap::new(), BTreeMap::new());

        loop {
            let marked_nodes_at_depth = marked_nodes_for_depth.get(&depth);
            let mut new_marked_nodes = BTreeMap::<Vec<Nibble>, NibblePatriciaTrieNode>::new();
            let mut new_marked_nodes_for_depth = BTreeMap::<usize, BTreeSet<Vec<Nibble>>>::new();

            if let Some(marked_nodes_at_depth) = marked_nodes_at_depth {
                // get all the marked nodes at the current depth
                for key in marked_nodes_at_depth.iter() {
                    let branch = marked_nodes
                        .get(key)
                        .ok_or(NibblePatriciaTrieError::NotFound)?;

                    if let NibblePatriciaTrieNode::Branch(branch) = branch {
                        // add non-marked child nodes of the marked branch node to the proof
                        for index in branch.child_key_indices.iter() {
                            let marked_child_node = marked_nodes
                                .range(key.iter().copied().chain([*index]).collect::<Vec<_>>()..)
                                .next();

                            // only the non-marked child nodes are added to the proof
                            if marked_child_node.is_some() {
                                continue;
                            }

                            let (child_key_fragment, child_node_hash) =
                                get_child_node_fragment_and_hash(&key, *index)?;

                            let child_key = key
                                .iter()
                                .copied()
                                .chain(child_key_fragment)
                                .collect::<Vec<_>>();

                            slf.nodes_hashed.insert(child_key, child_node_hash);
                        }

                        // add the parent node to the new marked nodes
                        let parent_key = &key[..key.len() - branch.key_fragment.len()];
                        let parent_node = get_node(parent_key)?;

                        if let NibblePatriciaTrieNode::Branch(parent_branch) = parent_node {
                            new_marked_nodes.insert(
                                parent_key.to_owned(),
                                NibblePatriciaTrieNode::Branch(parent_branch),
                            );
                            new_marked_nodes_for_depth
                                .entry(parent_key.len())
                                .or_insert_with(BTreeSet::new)
                                .insert(parent_key.to_owned());
                        } else {
                            return Err(NibblePatriciaTrieError::InvalidNode);
                        }

                        // add the marked node to the proof
                        slf.nodes_branch.insert(key.clone(), branch.clone());
                    }
                }
            }
            marked_nodes.extend(new_marked_nodes);
            marked_nodes_for_depth.extend(new_marked_nodes_for_depth);

            if depth == 0 {
                break;
            }

            depth -= 1;
        }

        Ok(slf)
    }

    /// Verifies the non-inclusion of a leaf key in the trie.
    ///
    /// This function verifies that a key is not present in the trie by:
    /// 1. Traversing the path from leaf to the root
    /// 2. Checking if any branch node along the path doesn't have the next nibble in its child indices
    /// 3. If such a branch node is found, the non-inclusion is proven
    ///
    /// # Arguments
    ///
    /// * `leaf_key` - The key to verify non-inclusion for
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If non-inclusion is successfully proven
    /// * `Err(NibblePatriciaTrieError)` - If the proof is invalid or non-inclusion cannot be proven
    pub fn verify_non_inclusion(&self, leaf_key: &[Nibble]) -> Result<(), NibblePatriciaTrieError> {
        let key_len = leaf_key.len();

        for i in (0..key_len).rev() {
            let key_path = &leaf_key[..i];

            let node = self.nodes_branch.get(key_path);

            if let Some(node) = node {
                // to prove the non-inclusion, branch node must not have the child key index for the leaf to prove non-inclusion
                if !node.child_key_indices.contains(&Nibble::from(leaf_key[i])) {
                    return Ok(());
                }
                return Err(NibblePatriciaTrieError::InvalidProof);
            }
        }

        Err(NibblePatriciaTrieError::InvalidProof)
    }

    pub fn leaf_key_fragment_from_path(
        &self,
        leaf_key: &[Nibble],
    ) -> Result<Vec<Nibble>, NibblePatriciaTrieError> {
        let parent_key =
            search_near_leaf_parent_key(leaf_key, |key| Ok(self.nodes_branch.get(key).cloned()))?;

        let key_fragment = leaf_key[parent_key.len()..].to_vec();

        Ok(key_fragment)
    }

    pub fn node_for_inclusion_proof(
        &self,
        leaf_key: &[Nibble],
        leaf_value: Vec<u8>,
    ) -> Result<NibblePatriciaTrieNodeLeaf, NibblePatriciaTrieError> {
        let leaf_key_fragment = self.leaf_key_fragment_from_path(leaf_key)?;

        Ok(NibblePatriciaTrieNodeLeaf::new(
            leaf_key_fragment,
            leaf_value,
        ))
    }

    /// Computes the root hash of the trie using the proof and inclusion nodes.
    ///
    /// This function:
    /// 1. Combines the proof's branch nodes and hashed nodes with the provided inclusion nodes
    /// 2. Processes nodes from deepest to shallowest depth
    /// 3. Computes hashes for each branch node using its children's hashes
    /// 4. Returns the final root hash
    ///
    /// # Arguments
    ///
    /// * `nodes_for_inclusion_proof` - Map of leaf values to include in the proof
    /// * `branch_hash_callback` - Optional callback function to be called when a branch node's hash is computed
    ///
    /// # Returns
    ///
    /// * `Ok([u8; 32])` - The computed root hash
    /// * `Err(NibblePatriciaTrieError)` - If the root hash computation fails
    pub fn root(
        self,
        nodes_for_inclusion_proof: BTreeMap<Vec<Nibble>, NibblePatriciaTrieNodeLeaf>,
        branch_hash_callback: Option<Box<dyn Fn(&Vec<Nibble>, &[u8; 32])>>,
    ) -> Result<[u8; 32], NibblePatriciaTrieError> {
        let mut nodes_branch = BTreeMap::<Vec<Nibble>, NibblePatriciaTrieNodeBranch>::new();
        let mut nodes_branch_for_depth = BTreeMap::<usize, BTreeSet<Vec<Nibble>>>::new();

        let mut nodes_hashed = BTreeMap::<Vec<Nibble>, [u8; 32]>::new();
        let mut nodes_hashed_for_depth = BTreeMap::<usize, BTreeSet<Vec<Nibble>>>::new();

        for (key, node) in self.nodes_branch {
            nodes_branch.insert(key.clone(), node);

            nodes_branch_for_depth
                .entry(key.len())
                .or_insert_with(BTreeSet::new)
                .insert(key);
        }

        for (key, node) in self.nodes_hashed {
            nodes_hashed.insert(key.clone(), node);

            nodes_hashed_for_depth
                .entry(key.len())
                .or_insert_with(BTreeSet::new)
                .insert(key);
        }

        for (key, node) in nodes_for_inclusion_proof {
            nodes_hashed.insert(key.clone(), node.hash());

            nodes_hashed_for_depth
                .entry(key.len())
                .or_insert_with(BTreeSet::new)
                .insert(key);
        }

        if nodes_branch.is_empty() {
            return Err(NibblePatriciaTrieError::EmptyProof);
        }

        while let Some((_depth, nodes_branch_at_depth)) = nodes_branch_for_depth.pop_last() {
            let nodes_branch_at_depth = nodes_branch_at_depth;
            for key in nodes_branch_at_depth {
                let branch = nodes_branch
                    .get(&key)
                    .ok_or(NibblePatriciaTrieError::NotFound)?;
                let hash = branch.hash(|index| {
                    let hashed_child_node = nodes_hashed
                        .range(key.iter().copied().chain([*index]).collect::<Vec<_>>()..)
                        .next()?;

                    let (_key, hash) = hashed_child_node;

                    Some(*hash)
                });

                if let Some(hash) = hash {
                    if let Some(ref branch_hash_callback) = branch_hash_callback {
                        branch_hash_callback(&key, &hash);
                    }

                    nodes_hashed.insert(key.clone(), hash);
                    nodes_hashed_for_depth
                        .entry(key.len())
                        .or_insert_with(BTreeSet::new)
                        .insert(key.clone());
                }
            }
        }

        let root_hash = nodes_hashed
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
    use crate::trie::{
        get_child_node_fragment_and_hash_from_db, get_node_from_db, NibblePatriciaTrieDb,
    };
    use std::collections::BTreeMap;

    fn setup_trie_and_db() -> (
        BTreeMap<Vec<Nibble>, Vec<u8>>,
        NibblePatriciaTrieMemoryDb,
        NibblePatriciaTrieMemoryDb,
        NibblePatriciaTrieNode,
    ) {
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
        let mut branch_1_children = BTreeSet::new();
        branch_1_children.insert(Nibble::from(2));
        branch_1_children.insert(Nibble::from(3));
        let branch_1 = NibblePatriciaTrieNodeBranch::new(vec![Nibble::from(1)], branch_1_children);

        // root
        let mut root_children = BTreeSet::new();
        root_children.insert(Nibble::from(1)); // [1] branch
        root_children.insert(Nibble::from(2)); // [2,2] leaf
        let root = NibblePatriciaTrieNodeBranch::new(vec![], root_children);

        // Prepare node_db and hash_db
        let mut node_db = NibblePatriciaTrieMemoryDb::new();
        let mut hash_db = NibblePatriciaTrieMemoryDb::new();

        // Serialize and store nodes
        let mut buf = Vec::new();
        // leaf [1,2]
        buf.clear();
        hash_db.set(&[Nibble::from(1), Nibble::from(2)], &leaf_12.hash()[..]);
        NibblePatriciaTrieNode::Leaf(leaf_12)
            .serialize(&mut buf)
            .unwrap();
        node_db.set(&[Nibble::from(1), Nibble::from(2)], &buf);
        // leaf [1,3]
        buf.clear();
        hash_db.set(&[Nibble::from(1), Nibble::from(3)], &leaf_13.hash()[..]);
        NibblePatriciaTrieNode::Leaf(leaf_13)
            .serialize(&mut buf)
            .unwrap();
        node_db.set(&[Nibble::from(1), Nibble::from(3)], &buf);
        // leaf [2,2]
        buf.clear();
        hash_db.set(&[Nibble::from(2), Nibble::from(2)], &leaf_22.hash()[..]);
        NibblePatriciaTrieNode::Leaf(leaf_22)
            .serialize(&mut buf)
            .unwrap();
        node_db.set(&[Nibble::from(2), Nibble::from(2)], &buf);
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
        hash_db.set(&[Nibble::from(1)], &branch_1_hash[..]);
        NibblePatriciaTrieNode::Branch(branch_1)
            .serialize(&mut buf)
            .unwrap();
        node_db.set(&[Nibble::from(1)], &buf);
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
        hash_db.set(&[], &root_hash[..]);
        let root_node = NibblePatriciaTrieNode::Branch(root);
        root_node.serialize(&mut buf).unwrap();
        node_db.set(&[], &buf);

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
        let get_child_node_fragment_and_hash = |key: &[Nibble], index: Nibble| {
            get_child_node_fragment_and_hash_from_db(key, index, &hash_db)
        };

        let leaf_keys = BTreeSet::from([vec![Nibble::from(1), Nibble::from(2)]]);
        let proof = NibblePatriciaTrieRootPath::from_leafs(
            leaf_keys,
            &get_node,
            &get_child_node_fragment_and_hash,
        )
        .unwrap();
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
        let get_child_node_fragment_and_hash = |key: &[Nibble], index: Nibble| {
            get_child_node_fragment_and_hash_from_db(key, index, &hash_db)
        };

        let leaf_keys = BTreeSet::from([vec![Nibble::from(1), Nibble::from(3)]]);
        let proof = NibblePatriciaTrieRootPath::from_leafs(
            leaf_keys,
            &get_node,
            &get_child_node_fragment_and_hash,
        )
        .unwrap();
        println!("proof: {:?}", proof);

        assert!(proof
            .verify_non_inclusion(&vec![Nibble::from(1), Nibble::from(4)])
            .is_ok());
    }

    #[test]
    fn test_inclusion_proof_and_root() {
        let (entries, node_db, hash_db, _root_node) = setup_trie_and_db();

        let get_node = |key: &[Nibble]| get_node_from_db(key, &node_db);
        let get_child_node_fragment_and_hash = |key: &[Nibble], index: Nibble| {
            get_child_node_fragment_and_hash_from_db(key, index, &hash_db)
        };

        let leaf_key = vec![Nibble::from(1), Nibble::from(2)];
        let leaf_keys = BTreeSet::from([leaf_key.clone()]);
        let proof = NibblePatriciaTrieRootPath::from_leafs(
            leaf_keys,
            &get_node,
            &get_child_node_fragment_and_hash,
        )
        .unwrap();
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
