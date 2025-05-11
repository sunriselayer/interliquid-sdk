use std::collections::BTreeMap;

use borsh_derive::{BorshDeserialize, BorshSerialize};

use super::OctRadPatriciaTrieError;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadPatriciaTriePath(BTreeMap<Vec<u8>, [u8; 32]>);

impl OctRadPatriciaTriePath {
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
    ) -> Result<(), OctRadPatriciaTrieError> {
        todo!()
    }

    pub fn range_completeness_proof(
        &self,
        key_suffixes_for_prefix: &BTreeMap<Vec<u8>, Vec<u8>>,
        root: &[u8; 32],
    ) -> Result<(), OctRadPatriciaTrieError> {
        // TODO: construct node for each prefix with child nodes constructed from suffixes

        // TODO: calculate hash for each prefix node
        let prefix_node_hashes = BTreeMap::new();

        self.inclusion_proof(&prefix_node_hashes, root)
    }
}
