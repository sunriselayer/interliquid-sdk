use borsh::{BorshDeserialize, BorshSerialize};

pub trait Value: BorshSerialize + BorshDeserialize + Send + Sync + 'static {}

impl<T: BorshSerialize + BorshDeserialize + Send + Sync + 'static> Value for T {}
