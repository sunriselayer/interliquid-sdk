use borsh_derive::{BorshDeserialize, BorshSerialize};

/// Represents the blockchain environment context for transaction execution.
/// This struct contains information about the current chain state and block.
///
/// `block_time` is the time of the block in seconds since Unix epoch
#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Environment {
    /// The unique identifier of the blockchain
    pub chain_id: String,
    /// The current block height (block number)
    pub block_height: u64,
    /// The timestamp of the current block in seconds since Unix epoch
    pub block_time: u64,
}

impl Environment {
    /// Creates a new Environment instance.
    ///
    /// # Arguments
    /// * `chain_id` - The unique identifier of the blockchain
    /// * `block_height` - The current block height
    /// * `block_time` - The timestamp of the block in seconds since Unix epoch
    pub fn new(chain_id: String, block_height: u64, block_time: u64) -> Self {
        Self {
            chain_id,
            block_height,
            block_time,
        }
    }
}
