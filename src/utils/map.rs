use std::{marker::PhantomData, ops::RangeBounds};

use borsh::{BorshDeserialize, BorshSerialize};

use super::key::{join_keys, KeySerializable};
use crate::{state::StateManager, types::InterLiquidSdkError};

pub struct Map<K: KeySerializable, V: BorshSerialize + BorshDeserialize> {
    prefix: Vec<u8>,
    phantom: PhantomData<(K, V)>,
}

impl<K: KeySerializable, V: BorshSerialize + BorshDeserialize> Map<K, V> {
    pub fn new<'a, P: IntoIterator<Item = &'a [u8]>>(prefix: P) -> Self {
        Self {
            prefix: join_keys(prefix),
            phantom: PhantomData,
        }
    }

    pub fn get<S: StateManager>(
        &self,
        state: &mut S,
        key: &K,
    ) -> Result<Option<V>, InterLiquidSdkError> {
        let entire_key = join_keys([self.prefix.as_slice(), &key.to_key_bytes()]);
        let value = state.get(&entire_key)?;

        match value {
            Some(value) => Ok(Some(V::deserialize(&mut &value[..])?)),
            None => Ok(None),
        }
    }

    pub fn set<S: StateManager>(
        &self,
        state: &mut S,
        key: &K,
        value: &V,
    ) -> Result<(), InterLiquidSdkError> {
        let entire_key = join_keys([self.prefix.as_slice(), &key.to_key_bytes()]);
        let mut buf = Vec::new();
        value.serialize(&mut buf)?;

        state.set(&entire_key, &buf)
    }

    pub fn del<S: StateManager>(&self, state: &mut S, key: &K) -> Result<(), InterLiquidSdkError> {
        let entire_key = join_keys([self.prefix.as_slice(), &key.to_key_bytes()]);

        state.del(&entire_key)
    }

    pub fn iter<'a, S: StateManager>(
        &'a self,
        state: &'a mut S,
        range: impl RangeBounds<Vec<u8>>,
    ) -> impl Iterator<Item = Result<(Vec<u8>, V), InterLiquidSdkError>> + 'a {
        let iter = state.iter(range);

        iter.map(|result| {
            let (k, v) = result?;
            let value = V::deserialize(&mut &v[..])?;

            Ok((k, value))
        })
    }
}
