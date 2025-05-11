use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    merkle::{OctRadPatriciaTriePath, OctRadSparseTreePath},
    state::CompressedDiffs,
};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputBlock {
    pub tx_root: [u8; 32],
    pub entire_root_prev: [u8; 32],
    pub entire_root_next: [u8; 32],
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PrivateInputBlock {
    pub tx_hashes: Vec<[u8; 32]>,
    pub state_sparse_tree_root_prev: [u8; 32],
    pub keys_patricia_trie_root_prev: [u8; 32],
    pub accum_diffs_hashes: Vec<[u8; 32]>,
    pub accum_diffs_final: CompressedDiffs,
    pub tx_proofs: Vec<Vec<u8>>,
    pub state_next_commit_path: OctRadSparseTreePath,
    pub keys_next_commit_path: OctRadPatriciaTriePath,
}
