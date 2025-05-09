use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::types::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Account {
    pub address: Address,
    pub nonce: u64,
}
