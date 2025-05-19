use borsh_derive::{BorshDeserialize, BorshSerialize};

/// `block_time` is the time of the block in seconds since Unix epoch
#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Environment {
    pub chain_id: String,
    pub block_height: u64,
    pub block_time: u64,
}

impl Environment {
    pub fn new(chain_id: String, block_height: u64, block_time: u64) -> Self {
        Self {
            chain_id,
            block_height,
            block_time,
        }
    }
}
