use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::zkp::WitnessTx;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub enum RunnerMessage {
    TxReceived(MessageTxReceived),
    TxProofReady(MessageTxProofReady),
    TxProofAggregationReady(MessageTxProofAggregationReady),
    CommitStateProofReady(MessageCommitStateProofReady),
    CommitKeysProofReady(MessageCommitKeysProofReady),
    BlockCommitted(MessageBlockCommitted),
    TxProved(MessageTxProved),
    TxProofAggregated(MessageTxProofAggregated),
    CommitStateProved(MessageCommitStateProved),
    CommitKeysProved(MessageCommitKeysProved),
    BlockProved(MessageBlockProved),
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageTxReceived {
    pub tx: Vec<u8>,
}

impl MessageTxReceived {
    pub fn new(tx: Vec<u8>) -> Self {
        Self { tx }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageTxProofReady {
    pub chain_id: String,
    pub block_height: u64,
    pub block_time_unix_secs: u64,
    pub tx_index: usize,
    pub witness: WitnessTx,
}

impl MessageTxProofReady {
    pub fn new(
        chain_id: String,
        block_height: u64,
        block_time_unix_secs: u64,
        tx_index: usize,
        witness: WitnessTx,
    ) -> Self {
        Self {
            chain_id,
            block_height,
            block_time_unix_secs,
            tx_index,
            witness,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageTxProofAggregationReady {
    pub chain_id: String,
    pub block_height: u64,
    pub tx_index: (usize, usize),
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageCommitStateProofReady {
    pub chain_id: String,
    pub block_height: u64,
    pub state_root: [u8; 32],
}

impl MessageCommitStateProofReady {
    pub fn new(chain_id: String, block_height: u64, state_root: [u8; 32]) -> Self {
        Self {
            chain_id,
            block_height,
            state_root,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageCommitKeysProofReady {
    pub chain_id: String,
    pub block_height: u64,
    pub keys_root: [u8; 32],
}

impl MessageCommitKeysProofReady {
    pub fn new(chain_id: String, block_height: u64, keys_root: [u8; 32]) -> Self {
        Self {
            chain_id,
            block_height,
            keys_root,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageBlockCommitted {
    pub chain_id: String,
    pub block_height: u64,
}

impl MessageBlockCommitted {
    pub fn new(chain_id: String, block_height: u64) -> Self {
        Self {
            chain_id,
            block_height,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageTxProved {
    pub chain_id: String,
    pub block_height: u64,
    pub tx_index: usize,
    pub proof: Vec<u8>,
}

impl MessageTxProved {
    pub fn new(chain_id: String, block_height: u64, tx_index: usize, proof: Vec<u8>) -> Self {
        Self {
            chain_id,
            block_height,
            tx_index,
            proof,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageTxProofAggregated {
    pub chain_id: String,
    pub block_height: u64,
    pub tx_index: (usize, usize),
    pub proof: Vec<u8>,
}

impl MessageTxProofAggregated {
    pub fn new(
        chain_id: String,
        block_height: u64,
        tx_index: (usize, usize),
        proof: Vec<u8>,
    ) -> Self {
        Self {
            chain_id,
            block_height,
            tx_index,
            proof,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageCommitStateProved {
    pub chain_id: String,
    pub block_height: u64,
    pub state_root: [u8; 32],
}

impl MessageCommitStateProved {
    pub fn new(chain_id: String, block_height: u64, state_root: [u8; 32]) -> Self {
        Self {
            chain_id,
            block_height,
            state_root,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageCommitKeysProved {
    pub chain_id: String,
    pub block_height: u64,
    pub keys_root: [u8; 32],
}

impl MessageCommitKeysProved {
    pub fn new(chain_id: String, block_height: u64, keys_root: [u8; 32]) -> Self {
        Self {
            chain_id,
            block_height,
            keys_root,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageBlockProved {
    pub chain_id: String,
    pub block_height: u64,
    pub entire_root: [u8; 32],
}

impl MessageBlockProved {
    pub fn new(chain_id: String, block_height: u64, entire_root: [u8; 32]) -> Self {
        Self {
            chain_id,
            block_height,
            entire_root,
        }
    }
}
