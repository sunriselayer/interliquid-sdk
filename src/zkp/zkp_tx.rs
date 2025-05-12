use std::collections::BTreeMap;

use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    merkle::{OctRadPatriciaTriePath, OctRadSparseTreePath},
    state::{CompressedDiffs, StateLog, StateManager},
};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputTx {
    pub tx_hash: [u8; 32],
    pub accum_diffs_hash_prev: [u8; 32],
    pub accum_diffs_hash_next: [u8; 32],
    pub entire_state_root: [u8; 32],
}

impl PublicInputTx {
    pub fn new(
        tx_hash: [u8; 32],
        accum_diffs_hash_prev: [u8; 32],
        accum_diffs_hash_next: [u8; 32],
        entire_state_root: [u8; 32],
    ) -> Self {
        Self {
            tx_hash,
            accum_diffs_hash_prev,
            accum_diffs_hash_next,
            entire_state_root,
        }
    }
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

impl PrivateInputTx {
    pub fn new(
        tx: Vec<u8>,
        state_sparse_tree_root: [u8; 32],
        keys_patricia_trie_root: [u8; 32],
        state_for_access: BTreeMap<Vec<u8>, Vec<u8>>,
        accum_diffs_prev: CompressedDiffs,
        read_proof_path: OctRadSparseTreePath,
        iter_proof_path: OctRadPatriciaTriePath,
    ) -> Self {
        Self {
            tx,
            state_sparse_tree_root,
            keys_patricia_trie_root,
            state_for_access,
            accum_diffs_prev,
            read_proof_path,
            iter_proof_path,
        }
    }

    pub fn from<S: StateManager>(
        tx: Vec<u8>,
        state_sparse_tree_root: [u8; 32],
        keys_patricia_trie_root: [u8; 32],
        logs: &[StateLog],
        accum_diffs_prev: CompressedDiffs,
        state_manager: &S,
    ) -> Self {
        todo!()
    }
}
