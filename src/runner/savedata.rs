use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::state::{CompressedDiffs, StateLog};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct TxExecutionSnapshot {
    pub tx: Vec<u8>,
    pub logs: Vec<StateLog>,
    pub accum_diffs: CompressedDiffs,
}

impl TxExecutionSnapshot {
    pub fn new(tx: Vec<u8>, logs: Vec<StateLog>, accum_diffs: CompressedDiffs) -> Self {
        Self {
            tx,
            logs,
            accum_diffs,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct SaveData {
    pub chain_id: String,
    pub block_height: u64,
    pub block_time_unix_secs: u64,
    pub tx_snapshots: Vec<TxExecutionSnapshot>,
}
