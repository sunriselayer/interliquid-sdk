use std::collections::BTreeMap;

use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    merkle::{OctRadPatriciaTriePath, OctRadSparseTreePath},
    state::CompressedDiffs,
};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputTx {
    pub tx_hash: [u8; 32],
    pub accum_diffs_hash_prev: [u8; 32],
    pub accum_diffs_hash_next: [u8; 32],
    pub entire_state_root: [u8; 32],
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PrivateInputTx {
    pub tx: Vec<u8>,
    pub state_sparse_tree_root: [u8; 32],
    pub keys_patricia_trie_root: [u8; 32],
    pub state_for_access: BTreeMap<Vec<u8>, Vec<u8>>,
    pub accum_diffs_prev: CompressedDiffs,
    pub read_proof_path: OctRadSparseTreePath,
    pub iter_proof_path: OctRadPatriciaTriePath,
}
