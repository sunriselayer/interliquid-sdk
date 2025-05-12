use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{block::PrivateInputTx, state::StateLog};

use super::savedata::TxExecutionSnapshot;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub enum RunnerMessage {
    TxReceived(MessageTxReceived),
    TxProofReady(MessageTxProofReady),
    BlockCommitted(MessageBlockCommitted),
    TxProved(MessageTxProved),
    TxProofAggregated(MessageTxProofAggregated),
    StateRootProved(MessageStateRootProved),
    KeysRootProved(MessageKeysRootProved),
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
    pub tx_index: usize,
    pub inputs: PrivateInputTx,
}

impl MessageTxProofReady {
    pub fn new(
        chain_id: String,
        block_height: u64,
        tx_index: usize,
        inputs: PrivateInputTx,
    ) -> Self {
        Self {
            chain_id,
            block_height,
            tx_index,
            inputs,
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
pub struct MessageStateRootProved {
    pub chain_id: String,
    pub block_height: u64,
    pub state_root: [u8; 32],
}

impl MessageStateRootProved {
    pub fn new(chain_id: String, block_height: u64, state_root: [u8; 32]) -> Self {
        Self {
            chain_id,
            block_height,
            state_root,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageKeysRootProved {
    pub chain_id: String,
    pub block_height: u64,
    pub keys_root: [u8; 32],
}

impl MessageKeysRootProved {
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
    pub entire_state_root: [u8; 32],
}

impl MessageBlockProved {
    pub fn new(chain_id: String, block_height: u64, entire_state_root: [u8; 32]) -> Self {
        Self {
            chain_id,
            block_height,
            entire_state_root,
        }
    }
}
