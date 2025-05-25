use borsh_derive::{BorshDeserialize, BorshSerialize};

/// Unix seconds
#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Timestamp(u64);
