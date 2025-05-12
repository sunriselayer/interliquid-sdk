use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{merkle::OctRadPatriciaTriePath, state::CompressedDiffs};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputPatriciaTrie {
    pub keys_patricia_trie_root_prev: [u8; 32],
    pub keys_patricia_trie_root_next: [u8; 32],
    pub accum_diffs_final_hash: [u8; 32],
}

impl PublicInputPatriciaTrie {
    pub fn new(
        keys_patricia_trie_root_prev: [u8; 32],
        keys_patricia_trie_root_next: [u8; 32],
        accum_diffs_final_hash: [u8; 32],
    ) -> Self {
        Self {
            keys_patricia_trie_root_prev,
            keys_patricia_trie_root_next,
            accum_diffs_final_hash,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PrivateInputPatriciaTrie {
    pub accum_diffs_final: CompressedDiffs,
    pub keys_commit_path: OctRadPatriciaTriePath,
}

impl PrivateInputPatriciaTrie {
    pub fn new(
        accum_diffs_final: CompressedDiffs,
        keys_commit_path: OctRadPatriciaTriePath,
    ) -> Self {
        Self {
            accum_diffs_final,
            keys_commit_path,
        }
    }
}
