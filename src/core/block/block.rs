use borsh_derive::{BorshDeserialize, BorshSerialize};

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
