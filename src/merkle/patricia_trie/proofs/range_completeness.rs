use std::collections::BTreeSet;

use crate::merkle::{
    consts::HASH_BYTES,
    patricia_trie::{OctRadPatriciaTrieError, OctRadPatriciaTrieNode},
};
use borsh_derive::{BorshDeserialize, BorshSerialize};

use super::{inclusion::OctRadPatriciaInclusionProof, key_fragment_diff, OctRadPatriciaPath};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadPatriciaRangeCompletenessProof {
    pub node: OctRadPatriciaTrieNode,
    pub path: Vec<OctRadPatriciaPath>,
}

impl OctRadPatriciaRangeCompletenessProof {
    pub fn new(node: OctRadPatriciaTrieNode, path: Vec<OctRadPatriciaPath>) -> Self {
        Self { node, path }
    }

    pub fn from_key_prefix<'a>(
        key_prefix: &[u8],
        key_suffixes: &BTreeSet<Vec<u8>>,
        path: Vec<OctRadPatriciaPath>,
    ) -> Result<Self, OctRadPatriciaTrieError> {
        let key_fragment = key_fragment_diff(key_prefix, &path)?;

        let node = OctRadPatriciaTrieNode::from_child_suffixes(&key_fragment, key_suffixes)?;

        Ok(Self::new(node, path))
    }

    pub fn validate(self, root: &[u8; HASH_BYTES]) -> Result<(), OctRadPatriciaTrieError> {
        let node_hash = self.node.hash();

        let inclusion_proof = OctRadPatriciaInclusionProof::new(node_hash, self.path);

        inclusion_proof.validate(root)?;

        Ok(())
    }
}
