use crate::merkle::{
    bitmap::OctRadBitmap, consts::HASH_BYTES, OctRadSparseTreeError, OctRadSparseTreeNodeBranch,
};
use borsh_derive::{BorshDeserialize, BorshSerialize};
use std::{collections::BTreeMap, iter::once};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct _OctRadSparseTreePath {
    pub key_hash_fragment: u8,
    pub child_hashes: BTreeMap<u8, [u8; 32]>,
}

impl _OctRadSparseTreePath {
    pub fn hash(
        &self,
        calculated_child_index_among_siblings: u8,
        calculated_child_hash: &[u8; HASH_BYTES],
    ) -> Result<[u8; HASH_BYTES], OctRadSparseTreeError> {
        let mut child_bitmap = OctRadBitmap::from_index_set(self.child_hashes.keys().copied());
        child_bitmap.set(calculated_child_index_among_siblings, true);

        let child_hashes = self
            .child_hashes
            .range(..calculated_child_index_among_siblings)
            .map(|(_, hash)| hash)
            .chain(once(calculated_child_hash))
            .chain(
                self.child_hashes
                    .range(calculated_child_index_among_siblings + 1..)
                    .map(|(_, hash)| hash),
            );

        let hash = OctRadSparseTreeNodeBranch::hash_from_child_hashes(
            self.key_hash_fragment,
            &child_bitmap,
            child_hashes,
        );

        Ok(hash)
    }
}
