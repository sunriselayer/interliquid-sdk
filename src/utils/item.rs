use std::marker::PhantomData;

use borsh::{BorshDeserialize, BorshSerialize};

use super::key::join_keys;
use crate::{state::StateManager, types::InterLiquidSdkError};

pub struct Item<V: BorshSerialize + BorshDeserialize> {
    key: Vec<u8>,
    phantom: PhantomData<V>,
}

impl<V: BorshSerialize + BorshDeserialize> Item<V> {
    pub fn new<'a, P: IntoIterator<Item = &'a [u8]>>(key: P) -> Self {
        Self {
            key: join_keys(key),
            phantom: PhantomData,
        }
    }

    pub fn get<S: StateManager>(&self, state: &mut S) -> Result<Option<V>, InterLiquidSdkError> {
        let value = state.get(&self.key)?;

        match value {
            Some(value) => Ok(Some(V::deserialize(&mut &value[..])?)),
            None => Ok(None),
        }
    }

    pub fn set<S: StateManager>(
        &self,
        state: &mut S,
        value: &V,
    ) -> Result<(), InterLiquidSdkError> {
        let mut buf = Vec::new();
        value.serialize(&mut buf)?;

        state.set(&self.key, &buf)
    }

    pub fn del<S: StateManager>(&self, state: &mut S) -> Result<(), InterLiquidSdkError> {
        state.del(&self.key)
    }
}
