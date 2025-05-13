use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::state::{CompressedDiffs, StateLog};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct TxExecutionSnapshot {
    pub logs: Vec<StateLog>,
    pub accum_diffs: CompressedDiffs,
}

impl TxExecutionSnapshot {
    pub fn new(logs: Vec<StateLog>, accum_diffs: CompressedDiffs) -> Self {
        Self { logs, accum_diffs }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct SaveData {
    pub chain_id: String,
    pub block_height: u64,
    pub block_time_unix_secs: u64,
    pub state_sparse_tree_root: [u8; 32],
    pub keys_patricia_trie_root: [u8; 32],
    pub tx_snapshots: Vec<TxExecutionSnapshot>,
}

impl SaveData {
    pub fn new(
        chain_id: String,
        block_height: u64,
        block_time_unix_secs: u64,
        state_sparse_tree_root: [u8; 32],
        keys_patricia_trie_root: [u8; 32],
        tx_snapshots: Vec<TxExecutionSnapshot>,
    ) -> Self {
        Self {
            chain_id,
            block_height,
            block_time_unix_secs,
            state_sparse_tree_root,
            keys_patricia_trie_root,
            tx_snapshots,
        }
    }
}
