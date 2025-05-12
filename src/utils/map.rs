use std::marker::PhantomData;

use super::{
    key::{join_keys, KeyDeclaration},
    KeyPrefix, Value,
};
use crate::{state::StateManager, types::InterLiquidSdkError};

pub struct Map<K: KeyDeclaration, V: Value> {
    prefix: Vec<u8>,
    phantom: PhantomData<(K, V)>,
}

impl<K: KeyDeclaration, V: Value> Map<K, V> {
    pub fn new<'a, P: IntoIterator<Item = &'a [u8]>>(prefix: P) -> Self {
        Self {
            prefix: join_keys(prefix),
            phantom: PhantomData,
        }
    }

    pub fn get<'a>(
        &self,
        state: &mut dyn StateManager,
        key: K::KeyReference<'a>,
    ) -> Result<Option<V>, InterLiquidSdkError> {
        let entire_key = join_keys([self.prefix.as_slice(), &K::to_key_bytes(key)]);
        let value = state.get(&entire_key)?;

        match value {
            Some(value) => Ok(Some(V::deserialize(&mut &value[..])?)),
            None => Ok(None),
        }
    }

    pub fn set<'a>(
        &self,
        state: &mut dyn StateManager,
        key: K::KeyReference<'a>,
        value: &V,
    ) -> Result<(), InterLiquidSdkError> {
        let entire_key = join_keys([self.prefix.as_slice(), &K::to_key_bytes(key)]);
        let mut buf = Vec::new();
        value.serialize(&mut buf)?;

        state.set(&entire_key, &buf)
    }

    pub fn del<'a>(
        &self,
        state: &mut dyn StateManager,
        key: K::KeyReference<'a>,
    ) -> Result<(), InterLiquidSdkError> {
        let entire_key = join_keys([self.prefix.as_slice(), &K::to_key_bytes(key)]);

        state.del(&entire_key)
    }

    pub fn iter<'a, B: KeyPrefix>(
        &'a self,
        state: &'a mut dyn StateManager,
        key_prefix: B,
    ) -> Box<dyn Iterator<Item = Result<(B::KeyToExtract, V), InterLiquidSdkError>> + 'a> {
        let iter = state.iter(key_prefix.to_prefix_bytes());

        Box::new(iter.map(|result| {
            let (mut k, v) = result?;
            let key = B::extract(&mut k[self.prefix.len()..]);
            let value = V::deserialize(&mut &v[..])?;

            Ok((key, value))
        }))
    }
}
