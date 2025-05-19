use crate::sha2::{Digest, Sha256};
use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::types::InterLiquidSdkError;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputTxAgg {
    pub txs_root: [u8; 32],
    pub env_hash: [u8; 32],
    pub accum_diffs_hash_left_prev: [u8; 32],
    pub accum_diffs_hash_right_next: [u8; 32],
    pub entire_root: [u8; 32],
}

impl PublicInputTxAgg {
    pub fn new(
        txs_root: [u8; 32],
        env_hash: [u8; 32],
        accum_diffs_hash_left_prev: [u8; 32],
        accum_diffs_hash_right_next: [u8; 32],
        entire_root: [u8; 32],
    ) -> Self {
        Self {
            txs_root,
            env_hash,
            accum_diffs_hash_left_prev,
            accum_diffs_hash_right_next,
            entire_root,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct WitnessTxAgg {
    pub txs_root_left: [u8; 32],
    pub txs_root_right: [u8; 32],
    pub env_hash: [u8; 32],
    pub accum_diffs_hash_left_prev: [u8; 32],
    pub accum_diffs_hash_mid: [u8; 32],
    pub accum_diffs_hash_right_next: [u8; 32],
    pub entire_root: [u8; 32],
    pub proof_left: Vec<u8>,
    pub proof_right: Vec<u8>,
}

impl WitnessTxAgg {
    pub fn new(
        txs_root_left: [u8; 32],
        txs_root_right: [u8; 32],
        env_hash: [u8; 32],
        accum_diffs_hash_left_prev: [u8; 32],
        accum_diffs_hash_mid: [u8; 32],
        accum_diffs_hash_right_next: [u8; 32],
        entire_root: [u8; 32],
        proof_left: Vec<u8>,
        proof_right: Vec<u8>,
    ) -> Self {
        Self {
            txs_root_left,
            txs_root_right,
            env_hash,
            accum_diffs_hash_left_prev,
            accum_diffs_hash_mid,
            accum_diffs_hash_right_next,
            entire_root,
            proof_left,
            proof_right,
        }
    }
}

pub fn circuit_tx_agg(witness: WitnessTxAgg) -> Result<PublicInputTxAgg, InterLiquidSdkError> {
    let mut hasher = Sha256::new();
    hasher.update(witness.txs_root_left);
    hasher.update(witness.txs_root_right);
    let tx_root = hasher.finalize().into();

    let input = PublicInputTxAgg::new(
        tx_root,
        witness.env_hash,
        witness.accum_diffs_hash_left_prev,
        witness.accum_diffs_hash_right_next,
        witness.entire_root,
    );

    Ok(input)
}
