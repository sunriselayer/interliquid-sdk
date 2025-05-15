use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use std::collections::{BTreeMap, BTreeSet};

use super::{
    Nibble, NibblePatriciaTrieDb, NibblePatriciaTrieError, NibblePatriciaTrieNode,
    NibblePatriciaTrieNodeBranch, NibblePatriciaTrieNodeLeaf,
};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub enum NibblePatriciaTrieProofNode {
    Branch(NibblePatriciaTrieNodeBranch),
    Hash([u8; 32]),
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct NibblePatriciaTrieProof {
    pub nodes: BTreeMap<Vec<Nibble>, NibblePatriciaTrieProofNode>,
}

impl NibblePatriciaTrieProof {
    pub fn new(nodes: BTreeMap<Vec<Nibble>, NibblePatriciaTrieProofNode>) -> Self {
        Self { nodes }
    }

    fn get_node<Db: NibblePatriciaTrieDb>(
        key: &[Nibble],
        node_db: &Db,
    ) -> Result<NibblePatriciaTrieNode, NibblePatriciaTrieError> {
        let node_bytes = node_db.get(key).ok_or(NibblePatriciaTrieError::NotFound)?;
        let node = NibblePatriciaTrieNode::try_from_slice(&node_bytes)?;
        Ok(node)
    }

    fn get_node_hash<Db: NibblePatriciaTrieDb>(
        key: &[Nibble],
        hash_db: &Db,
    ) -> Result<[u8; 32], NibblePatriciaTrieError> {
        let hash_bytes = hash_db.get(key).ok_or(NibblePatriciaTrieError::NotFound)?;
        let hash: [u8; 32] = hash_bytes
            .try_into()
            .map_err(|_| NibblePatriciaTrieError::InvalidHash)?;
        Ok(hash)
    }

    /// Before calling `from_leafs`, you need to call this function to get the parent key of the leaf
    pub fn leaf_parent_key<Db: NibblePatriciaTrieDb>(
        leaf_key: &[Nibble],
        node_db: &Db,
    ) -> Result<Vec<Nibble>, NibblePatriciaTrieError> {
        let leaf_node_bytes = node_db.get(leaf_key);

        match leaf_node_bytes {
            Some(leaf_node_bytes) => {
                let leaf_node = NibblePatriciaTrieNode::try_from_slice(&leaf_node_bytes)?;
                if let NibblePatriciaTrieNode::Leaf(leaf) = leaf_node {
                    Ok(leaf_key[..leaf_key.len() - leaf.key_fragment.len()].to_vec())
                } else {
                    Err(NibblePatriciaTrieError::InvalidNode)
                }
            }
            None => {
                let key_len = leaf_key.len();
                for i in (0..key_len).rev() {
                    let key_path = &leaf_key[..i];

                    let node = node_db.get(key_path);

                    if let Some(node_bytes) = node {
                        let node = NibblePatriciaTrieNode::try_from_slice(&node_bytes)?;
                        if let NibblePatriciaTrieNode::Branch(branch) = node {
                            // to prove the non-inclusion, branch node must not have the child key index for the leaf to prove non-inclusion
                            if !branch
                                .child_key_fragments
                                .contains_key(&Nibble::from(leaf_key[i]))
                            {
                                return Ok(leaf_key[..i].to_vec());
                            }
                        }
                        return Err(NibblePatriciaTrieError::InvalidProof);
                    }
                }

                Err(NibblePatriciaTrieError::InvalidProof)
            }
        }
    }

    /// Construct inclusion proof / non inclusion proof from the designated leafs
    /// Use `leaf_parent_key` to get the parent key of the leaf
    pub fn from_leafs<Db: NibblePatriciaTrieDb>(
        leaf_parent_keys: BTreeSet<Vec<Nibble>>,
        node_db: &Db,
        node_hash_db: &Db,
    ) -> Result<Self, NibblePatriciaTrieError> {
        // marked nodes means the nodes which are needed to be re-constructed in the inclusion proof
        let mut marked_nodes =
            BTreeMap::<usize, BTreeMap<Vec<Nibble>, NibblePatriciaTrieNodeBranch>>::new();

        for leaf_parent_key in leaf_parent_keys.iter() {
            let parent_key = leaf_parent_key;
            let parent_node = Self::get_node(parent_key, node_db)?;

            if let NibblePatriciaTrieNode::Branch(parent_branch) = parent_node {
                marked_nodes
                    .entry(parent_key.len())
                    .or_insert_with(BTreeMap::new)
                    .insert(parent_key.to_owned(), parent_branch);
            } else {
                return Err(NibblePatriciaTrieError::InvalidNode);
            }
        }

        if marked_nodes.is_empty() {
            return Err(NibblePatriciaTrieError::EmptyProof);
        }

        // get the depth of the deepest marked node
        let mut depth = *marked_nodes.keys().last().unwrap();

        let mut slf = Self::new(BTreeMap::new());

        while depth > 0 {
            let marked_nodes_in_depth = marked_nodes.get(&depth);
            let mut new_marked_nodes = BTreeMap::new();

            if let Some(marked_nodes_in_depth) = marked_nodes_in_depth {
                // get all the marked nodes at the current depth
                for (key, branch) in marked_nodes_in_depth.iter() {
                    for (_index, child_key_fragment) in branch.child_key_fragments.iter() {
                        let child_key = key
                            .iter()
                            .chain(child_key_fragment)
                            .copied()
                            .collect::<Vec<_>>();

                        // check if the child key is already marked
                        if marked_nodes_in_depth.contains_key(&child_key) {
                            continue;
                        }

                        let child_node_hash = Self::get_node_hash(&child_key, node_hash_db)?;

                        slf.nodes.insert(
                            child_key,
                            NibblePatriciaTrieProofNode::Hash(child_node_hash),
                        );
                    }

                    // add the parent node to the new marked nodes
                    let parent_key = &key[..key.len() - branch.key_fragment.len()];
                    let parent_node = Self::get_node(parent_key, node_db)?;

                    if let NibblePatriciaTrieNode::Branch(parent_branch) = parent_node {
                        new_marked_nodes
                            .entry(parent_key.len())
                            .or_insert_with(BTreeMap::new)
                            .insert(parent_key.to_owned(), parent_branch);
                    } else {
                        return Err(NibblePatriciaTrieError::InvalidNode);
                    }

                    // add the marked node to the proof
                    slf.nodes.insert(
                        key.clone(),
                        NibblePatriciaTrieProofNode::Branch(branch.clone()),
                    );
                }
            }
            marked_nodes.extend(new_marked_nodes);

            depth -= 1;
        }

        Ok(slf)
    }

    /// Verify the non inclusion of the leaf key
    pub fn verify_non_inclusion(&self, leaf_key: &[Nibble]) -> Result<(), NibblePatriciaTrieError> {
        let key_len = leaf_key.len();

        for i in (0..key_len).rev() {
            let key_path = &leaf_key[..i];

            let node = self.nodes.get(key_path);

            if let Some(node) = node {
                if let NibblePatriciaTrieProofNode::Branch(branch) = node {
                    // to prove the non-inclusion, branch node must not have the child key index for the leaf to prove non-inclusion
                    if !branch
                        .child_key_fragments
                        .contains_key(&Nibble::from(leaf_key[i]))
                    {
                        return Ok(());
                    }
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
        let mut branches =
            BTreeMap::<usize, BTreeMap<Vec<Nibble>, NibblePatriciaTrieNodeBranch>>::new();
        let mut hashes = BTreeMap::<usize, BTreeMap<Vec<Nibble>, [u8; 32]>>::new();

        for (key, node) in self.nodes {
            match node {
                NibblePatriciaTrieProofNode::Branch(branch) => {
                    branches
                        .entry(key.len())
                        .or_insert_with(BTreeMap::new)
                        .insert(key, branch);
                }
                NibblePatriciaTrieProofNode::Hash(hash) => {
                    hashes
                        .entry(key.len())
                        .or_insert_with(BTreeMap::new)
                        .insert(key, hash);
                }
            }
        }

        for (key, node) in nodes_for_inclusion_proof {
            hashes
                .entry(key.len())
                .or_insert_with(BTreeMap::new)
                .insert(key, node.hash());
        }

        if branches.is_empty() {
            return Err(NibblePatriciaTrieError::EmptyProof);
        }

        while let Some((depth, branches_at_depth)) = branches.pop_last() {
            for (key, branch) in branches_at_depth {
                let hash = branch
                    .hash(|index| {
                        let child_key_fragment = branch.child_key_fragments.get(&index).unwrap();

                        let child_key = key
                            .iter()
                            .chain(child_key_fragment)
                            .copied()
                            .collect::<Vec<_>>();

                        let child_hash = hashes.get(&child_key.len())?.get(&child_key);

                        child_hash.copied()
                    })
                    .ok_or(NibblePatriciaTrieError::InvalidProof)?;

                if let Some(ref branch_hash_callback) = branch_hash_callback {
                    branch_hash_callback(&key, &hash);
                }

                hashes
                    .entry(key.len())
                    .or_insert_with(BTreeMap::new)
                    .insert(key, hash);
            }
            // remove unnecessary hashes of longer keys
            hashes.retain(|&k, _| k <= depth);
        }

        let root_hash = hashes
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
    use super::*;
    use crate::trie::db::NibblePatriciaTrieMemoryDb;
    use crate::trie::nibble::Nibble;
    use crate::trie::node::{
        NibblePatriciaTrieNode, NibblePatriciaTrieNodeBranch, NibblePatriciaTrieNodeLeaf,
    };
    use std::collections::BTreeMap;

    fn setup_trie_and_db() -> (
        BTreeMap<Vec<Nibble>, Vec<u8>>,
        NibblePatriciaTrieMemoryDb,
        NibblePatriciaTrieMemoryDb,
        [u8; 32],
        Vec<Nibble>,
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
        NibblePatriciaTrieNode::Branch(root)
            .serialize(&mut buf)
            .unwrap();
        node_db.set(&vec![], &buf);

        (entries, node_db, hash_db, root_hash, vec![])
    }

    #[test]
    fn test_leaf_parent_key() {}

    #[test]
    fn test_inclusion_proof_and_root() {}

    #[test]
    fn test_non_inclusion_proof_and_verify() {}
}
