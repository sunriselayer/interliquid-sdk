use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::types::Address;

/// Represents a user account in the blockchain.
/// Each account has a unique address and a nonce to prevent replay attacks.
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Account {
    /// The unique address identifying this account.
    pub address: Address,
    /// The current nonce value, incremented with each transaction.
    pub nonce: u64,
}

impl Account {
    /// Creates a new account with the given address and initial nonce of 0.
    /// 
    /// # Arguments
    /// * `address` - The address for the new account
    pub fn new(address: Address) -> Self {
        Self { address, nonce: 0 }
    }
}
