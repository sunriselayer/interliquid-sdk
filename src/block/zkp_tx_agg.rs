use borsh_derive::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputTxAgg {
    pub tx_root: [u8; 32],
    pub accum_diffs_hash_left_prev: [u8; 32],
    pub accum_diffs_hash_right_next: [u8; 32],
    pub entire_state_root: [u8; 32],
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PrivateInputTx {
    pub tx_root_left: [u8; 32],
    pub tx_root_right: [u8; 32],
    pub accum_diffs_hash_mid: [u8; 32],
    pub proof_left: Vec<u8>,
    pub proof_right: Vec<u8>,
}
