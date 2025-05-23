use borsh_derive::{BorshDeserialize, BorshSerialize};
use crate::sha2::{Digest, Sha256};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Header {
    pub chain_id: u64,
    pub height: u64,
    pub time: u64,

    pub header_hash_prev: [u8; 32],

    pub txs_root: [u8; 32],

    pub state_root: [u8; 32],
    pub keys_root: [u8; 32],

    pub sequencer_hash: [u8; 32],
    pub sequencer_hash_next: [u8; 32],
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Block {
    pub header: Header,
    pub txs: Vec<Vec<u8>>,
    pub sequencer_signature: Vec<u8>,
}

pub fn entire_root(state_root: &[u8; 32], keys_root: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(state_root);
    hasher.update(keys_root);
    hasher.finalize().into()
}
