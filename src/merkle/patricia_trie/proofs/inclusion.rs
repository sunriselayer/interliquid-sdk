use crate::merkle::{
    consts::HASH_BYTES,
    patricia_trie::{OctRadPatriciaTrieError, OctRadPatriciaTrieNodeLeaf},
};
use borsh_derive::{BorshDeserialize, BorshSerialize};

use super::{key_fragment_diff, OctRadPatriciaPath};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadPatriciaInclusionProof {
    pub node_hash: [u8; HASH_BYTES],
    pub path: Vec<OctRadPatriciaPath>,
}

impl OctRadPatriciaInclusionProof {
    pub fn new(node_hash: [u8; HASH_BYTES], path: Vec<OctRadPatriciaPath>) -> Self {
        Self { node_hash, path }
    }

    pub fn from_leaf(
        key: &[u8],
        path: Vec<OctRadPatriciaPath>,
    ) -> Result<Self, OctRadPatriciaTrieError> {
        if path.is_empty() {
            let leaf = OctRadPatriciaTrieNodeLeaf::new(key.to_vec());
            return Ok(Self::new(leaf.hash(), path));
        }

        let key_fragment = key_fragment_diff(key, &path)?;
        let leaf = OctRadPatriciaTrieNodeLeaf::new(key_fragment);

        Ok(Self::new(leaf.hash(), path))
    }

    pub fn validate(&self, root: &[u8; HASH_BYTES]) -> Result<(), OctRadPatriciaTrieError> {
        let mut hash: [u8; HASH_BYTES] = self.node_hash;
        for path in self.path.iter() {
            let index_among_siblings = path.key_fragment[0];
            hash = path.hash(index_among_siblings, &hash)?;
        }

        if !hash.eq(root) {
            return Err(OctRadPatriciaTrieError::InvalidProof(anyhow::anyhow!(
                "invalid proof"
            )));
        }

        Ok(())
    }
}
