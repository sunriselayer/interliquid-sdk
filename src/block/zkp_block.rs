use borsh_derive::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputBlock {
    pub tx_root: [u8; 32],
    pub entire_root_prev: [u8; 32],
    pub entire_root_next: [u8; 32],
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PrivateInputBlock {
    pub tx_hashes: Vec<[u8; 32]>,
    pub state_root_prev: [u8; 32],
    pub state_root_next: [u8; 32],
    pub keys_root_prev: [u8; 32],
    pub keys_root_next: [u8; 32],
    pub accum_diffs_hashes: Vec<[u8; 32]>,
    pub tx_proofs: Vec<Vec<u8>>,
    pub state_commit_proof: Vec<Vec<u8>>,
    pub keys_commit_proof: Vec<Vec<u8>>,
}
