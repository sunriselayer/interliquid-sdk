use borsh_derive::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputTxAgg {
    pub tx_root: [u8; 32],
    pub accum_diffs_hash_left_prev: [u8; 32],
    pub accum_diffs_hash_right_next: [u8; 32],
    pub entire_state_root: [u8; 32],
}

impl PublicInputTxAgg {
    pub fn new(
        tx_root: [u8; 32],
        accum_diffs_hash_left_prev: [u8; 32],
        accum_diffs_hash_right_next: [u8; 32],
        entire_state_root: [u8; 32],
    ) -> Self {
        Self {
            tx_root,
            accum_diffs_hash_left_prev,
            accum_diffs_hash_right_next,
            entire_state_root,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct WitnessTxAgg {
    pub tx_root_left: [u8; 32],
    pub tx_root_right: [u8; 32],
    pub accum_diffs_hash_left_prev: [u8; 32],
    pub accum_diffs_hash_mid: [u8; 32],
    pub accum_diffs_hash_right_next: [u8; 32],
    pub entire_state_root: [u8; 32],
    pub proof_left: Vec<u8>,
    pub proof_right: Vec<u8>,
}

impl WitnessTxAgg {
    pub fn new(
        tx_root_left: [u8; 32],
        tx_root_right: [u8; 32],
        accum_diffs_hash_left_prev: [u8; 32],
        accum_diffs_hash_mid: [u8; 32],
        accum_diffs_hash_right_next: [u8; 32],
        entire_state_root: [u8; 32],
        proof_left: Vec<u8>,
        proof_right: Vec<u8>,
    ) -> Self {
        Self {
            tx_root_left,
            tx_root_right,
            accum_diffs_hash_left_prev,
            accum_diffs_hash_mid,
            accum_diffs_hash_right_next,
            entire_state_root,
            proof_left,
            proof_right,
        }
    }
}
