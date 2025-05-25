use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    state::{AccumulatedLogs, StateLog},
    types::Timestamp,
};

/// Represents a snapshot of state changes after executing a transaction.
/// Contains both the individual state logs and accumulated logs for verification.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct TxExecutionSnapshot {
    pub logs: Vec<StateLog>,
    pub accum_logs: AccumulatedLogs,
}

impl TxExecutionSnapshot {
    /// Creates a new TxExecutionSnapshot instance.
    ///
    /// # Arguments
    /// * `logs` - List of individual state changes made by the transaction
    /// * `accum_logs` - Accumulated state logs for merkle proof generation
    pub fn new(logs: Vec<StateLog>, accum_logs: AccumulatedLogs) -> Self {
        Self { logs, accum_logs }
    }
}

/// Persistent storage structure containing all data needed to reconstruct
/// and verify a block's execution. This data is saved after block processing
/// and used for proof generation.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct SaveData {
    pub chain_id: String,
    pub block_height: u64,
    pub block_time: Timestamp,
    pub state_sparse_tree_root: [u8; 32],
    pub keys_patricia_trie_root: [u8; 32],
    pub tx_snapshots: Vec<TxExecutionSnapshot>,
}

impl SaveData {
    /// Creates a new SaveData instance.
    ///
    /// # Arguments
    /// * `chain_id` - The identifier of the blockchain
    /// * `block_height` - The height of the block
    /// * `block_time` - The Unix timestamp when the block was created
    /// * `state_sparse_tree_root` - The 32-byte root hash of the state sparse merkle tree
    /// * `keys_patricia_trie_root` - The 32-byte root hash of the keys patricia trie
    /// * `tx_snapshots` - List of transaction execution snapshots in the block
    pub fn new(
        chain_id: String,
        block_height: u64,
        block_time: Timestamp,
        state_sparse_tree_root: [u8; 32],
        keys_patricia_trie_root: [u8; 32],
        tx_snapshots: Vec<TxExecutionSnapshot>,
    ) -> Self {
        Self {
            chain_id,
            block_height,
            block_time,
            state_sparse_tree_root,
            keys_patricia_trie_root,
            tx_snapshots,
        }
    }
}
