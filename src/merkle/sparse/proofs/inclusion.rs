use crate::merkle::{consts::HASH_BYTES, OctRadSparseTreeError, OctRadSparseTreeNodeLeaf};
use borsh_derive::{BorshDeserialize, BorshSerialize};

use super::_OctRadSparseTreePath;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadSparseTreeInclusionProof {
    pub node_hash: [u8; HASH_BYTES],
    pub path: Vec<_OctRadSparseTreePath>,
}

impl OctRadSparseTreeInclusionProof {
    pub fn new(node_hash: [u8; HASH_BYTES], path: Vec<_OctRadSparseTreePath>) -> Self {
        Self { node_hash, path }
    }

    pub fn from_leaf(
        key_hash_fragment: u8,
        value: Vec<u8>,
        path: Vec<_OctRadSparseTreePath>,
    ) -> Result<Self, OctRadSparseTreeError> {
        let leaf = OctRadSparseTreeNodeLeaf::new(vec![key_hash_fragment]);

        Ok(Self::new(leaf.hash(), path))
    }

    pub fn validate(&self, root: &[u8; HASH_BYTES]) -> Result<(), OctRadSparseTreeError> {
        const DEPTH: usize = HASH_BYTES;
        if self.path.len() != DEPTH - 1 {
            return Err(OctRadSparseTreeError::InvalidProof(anyhow::anyhow!(
                "path length must be equal to {} (depth - 1)",
                DEPTH - 1
            )));
        }

        let mut hash: [u8; HASH_BYTES] = self.node_hash;
        for path in self.path.iter() {
            let index_among_siblings = path.key_hash_fragment;
            hash = path.hash(index_among_siblings, &hash)?;
        }

        if !hash.eq(root) {
            return Err(OctRadSparseTreeError::InvalidProof(anyhow::anyhow!(
                "invalid proof"
            )));
        }

        Ok(())
    }
}
