use crate::patricia_trie::{
    bitmap::OctRadPatriciaBitmap, consts::HASH_BYTES, OctRadPatriciaNodeBranch,
    OctRadPatriciaTrieError,
};
use borsh_derive::{BorshDeserialize, BorshSerialize};
use std::iter::once;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadPatriciaPath {
    pub key_fragment: Vec<u8>,
    // it should be 0 flag for index_among_siblings
    pub child_bitmap: OctRadPatriciaBitmap,
    pub child_hashes_left: Vec<[u8; 32]>,
    pub child_hashes_right: Vec<[u8; 32]>,
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

        if self.child_bitmap.get(calculated_child_index_among_siblings) {
            return Err(OctRadPatriciaTrieError::InvalidProof(anyhow::anyhow!(
                "self index must not be in the sibling bitmap"
            )));
        }

        let child_hashes = self
            .child_hashes_left
            .iter()
            .chain(once(calculated_child_hash))
            .chain(self.child_hashes_right.iter());

        let hash = OctRadPatriciaNodeBranch::hash_from_child_hashes(
            &self.key_fragment,
            &self.child_bitmap,
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

    if key.len() < concatenated_len {
        return Err(OctRadPatriciaTrieError::InvalidProof(anyhow::anyhow!(
            "key is too short"
        )));
    }

    if !key[..concatenated_len].eq(&key_fragment_concatenated) {
        return Err(OctRadPatriciaTrieError::InvalidProof(anyhow::anyhow!(
            "key is not consistent with the path"
        )));
    }

    let key_fragment = key[concatenated_len..].to_vec();

    Ok(key_fragment)
}
