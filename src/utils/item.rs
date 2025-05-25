use std::marker::PhantomData;

use super::{key::join_keys, Value};
use crate::{state::TracableStateManager, types::InterLiquidSdkError};

/// `Item` stores one value for the designated key in the state.
pub struct Item<V: Value> {
    key: Vec<u8>,
    phantom: PhantomData<V>,
}

impl<V: Value> Item<V> {
    pub fn new<'a, P: IntoIterator<Item = &'a [u8]>>(key: P) -> Self {
        Self {
            key: join_keys(key),
            phantom: PhantomData,
        }
    }

    pub fn get<S: TracableStateManager>(
        &self,
        state: &mut S,
    ) -> Result<Option<V>, InterLiquidSdkError> {
        let value = state.get(&self.key)?;

        match value {
            Some(value) => Ok(Some(V::try_from_slice(&value)?)),
            None => Ok(None),
        }
    }

    pub fn set<S: TracableStateManager>(
        &self,
        state: &mut S,
        value: &V,
    ) -> Result<(), InterLiquidSdkError> {
        let mut buf = Vec::new();
        value.serialize(&mut buf)?;

        state.set(&self.key, &buf)
    }

    pub fn del<S: TracableStateManager>(&self, state: &mut S) -> Result<(), InterLiquidSdkError> {
        state.del(&self.key)
    }
}
