use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{core::entire_root, types::InterLiquidSdkError};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputBlock {
    pub txs_root: [u8; 32],
    pub entire_root_prev: [u8; 32],
    pub entire_root_next: [u8; 32],
}

impl PublicInputBlock {
    pub fn new(txs_root: [u8; 32], entire_root_prev: [u8; 32], entire_root_next: [u8; 32]) -> Self {
        Self {
            txs_root,
            entire_root_prev,
            entire_root_next,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct WitnessBlock {
    pub txs_root: [u8; 32],
    pub state_root_prev: [u8; 32],
    pub state_root_next: [u8; 32],
    pub keys_root_prev: [u8; 32],
    pub keys_root_next: [u8; 32],
    pub accum_logs_hash: [u8; 32],
    pub proof_tx_agg: Vec<u8>,
    pub proof_commit_state: Vec<u8>,
    pub proof_commit_keys: Vec<u8>,
}

impl WitnessBlock {
    pub fn new(
        txs_root: [u8; 32],
        state_root_prev: [u8; 32],
        state_root_next: [u8; 32],
        keys_root_prev: [u8; 32],
        keys_root_next: [u8; 32],
        accum_logs_hash: [u8; 32],
        proof_tx_agg: Vec<u8>,
        proof_commit_state: Vec<u8>,
        proof_commit_keys: Vec<u8>,
    ) -> Self {
        Self {
            txs_root,
            state_root_prev,
            state_root_next,
            keys_root_prev,
            keys_root_next,
            accum_logs_hash,
            proof_tx_agg,
            proof_commit_state,
            proof_commit_keys,
        }
    }
}

pub fn circuit_block(witness: WitnessBlock) -> Result<PublicInputBlock, InterLiquidSdkError> {
    let entire_root_prev = entire_root(&witness.state_root_prev, &witness.keys_root_prev);

    let entire_root_next = entire_root(&witness.state_root_next, &witness.keys_root_next);

    let input = PublicInputBlock::new(witness.txs_root, entire_root_prev, entire_root_next);

    Ok(input)
}
