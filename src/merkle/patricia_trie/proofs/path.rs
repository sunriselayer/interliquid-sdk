use crate::merkle::{
    bitmap::OctRadBitmap,
    consts::HASH_BYTES,
    patricia_trie::{OctRadPatriciaNodeBranch, OctRadPatriciaTrieError},
};
use borsh_derive::{BorshDeserialize, BorshSerialize};
use std::{collections::BTreeMap, iter::once};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadPatriciaPath {
    pub key_fragment: Vec<u8>,
    pub child_hashes: BTreeMap<u8, [u8; 32]>,
}

impl OctRadPatriciaPath {
    pub fn hash(
        &self,
        calculated_child_index_among_siblings: u8,
        calculated_child_hash: &[u8; HASH_BYTES],
    ) -> Result<[u8; HASH_BYTES], OctRadPatriciaTrieError> {
        if self.key_fragment.len() == 0 {
            return Err(OctRadPatriciaTrieError::InvalidProof(anyhow::anyhow!(
                "key fragment must not be empty"
            )));
        }

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

        let hash = OctRadPatriciaNodeBranch::hash_from_child_hashes(
            &self.key_fragment,
            &child_bitmap,
            child_hashes,
        );

        Ok(hash)
    }
}

pub(crate) fn key_fragment_diff(
    key: &[u8],
    path: &[OctRadPatriciaPath],
) -> Result<Vec<u8>, OctRadPatriciaTrieError> {
    let key_fragment_concatenated: Vec<u8> = path
        .iter()
        .flat_map(|path| path.key_fragment.iter())
        .copied()
        .collect();
    let concatenated_len = key_fragment_concatenated.len();

    if !key.starts_with(&key_fragment_concatenated) {
        return Err(OctRadPatriciaTrieError::InvalidProof(anyhow::anyhow!(
            "key is not consistent with the path"
        )));
    }

    let key_fragment = key[concatenated_len..].to_vec();

    Ok(key_fragment)
}
