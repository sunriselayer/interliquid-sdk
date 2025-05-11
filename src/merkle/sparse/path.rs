use std::{collections::BTreeMap, iter::empty};

use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::merkle::{bitmap::OctRadBitmap, OctRadSparseTreeNodeBranch};

use super::OctRadSparseTreeError;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadSparseTreePath(BTreeMap<Vec<u8>, [u8; 32]>);

impl OctRadSparseTreePath {
    pub fn new(path: BTreeMap<Vec<u8>, [u8; 32]>) -> Self {
        Self(path)
    }

    /// If there is a duplicated path, `remainder_node_hashes` is prioritized.
    pub fn root(&self, remainder_node_hashes: &BTreeMap<Vec<u8>, [u8; 32]>) -> [u8; 32] {
        todo!()
    }

    pub fn inclusion_proof(
        &self,
        node_hashes_to_prove: &BTreeMap<Vec<u8>, [u8; 32]>,
        root: &[u8; 32],
    ) -> Result<(), OctRadSparseTreeError> {
        todo!()
    }

    pub fn non_inclusion_proof(
        &self,
        key_hash_dead_end_lens: &BTreeMap<Vec<u8>, u8>,
        root: &[u8; 32],
    ) -> Result<(), OctRadSparseTreeError> {
        // TODO: construct dead end node hashes
        let dead_end_node_hashes = BTreeMap::new();

        self.inclusion_proof(&dead_end_node_hashes, root)
    }
}
