use borsh::BorshDeserialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use std::collections::{BTreeMap, HashSet};

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
    pub node_hashes: BTreeMap<Vec<Nibble>, NibblePatriciaTrieProofNode>,
}

impl NibblePatriciaTrieProof {
    pub fn new(node_hashes: BTreeMap<Vec<Nibble>, NibblePatriciaTrieProofNode>) -> Self {
        Self { node_hashes }
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

    pub fn from_leafs<Db: NibblePatriciaTrieDb>(
        leaf_parent_keys: HashSet<Vec<Nibble>>,
        node_db: &Db,
        hash_db: &Db,
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

                        let child_node_hash = Self::get_node_hash(&child_key, hash_db)?;

                        slf.node_hashes.insert(
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
                    slf.node_hashes.insert(
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

    pub fn assert_non_inclusion(&self, key: &[Nibble]) -> bool {
        todo!()
    }

    pub fn root(
        &self,
        nodes_for_inclusion_proof: &BTreeMap<Vec<Nibble>, NibblePatriciaTrieNodeLeaf>,
    ) -> [u8; 32] {
        todo!()
    }
}
