use std::{collections::BTreeMap, ops::RangeBounds};

use borsh::{BorshDeserialize, BorshSerialize};

use super::{
    key::{join_keys, KeySerializable},
    Map,
};
use crate::{state::StateManager, types::InterLiquidSdkError};

pub struct IndexedMap<K: KeySerializable, V: BorshSerialize + BorshDeserialize> {
    map: Map<K, V>,
    indexers: BTreeMap<String, Indexer<V>>,
}

impl<K: KeySerializable, V: BorshSerialize + BorshDeserialize> IndexedMap<K, V> {
    pub fn new<'a, P: IntoIterator<Item = &'a [u8]>>(prefix: P) -> Self {
        Self {
            map: Map::new(prefix),
            indexers: BTreeMap::new(),
        }
    }

    pub fn get<S: StateManager>(
        &self,
        state: &mut S,
        key: &K,
    ) -> Result<Option<V>, InterLiquidSdkError> {
        self.map.get(state, key)
    }

    pub fn set<S: StateManager>(
        &self,
        state: &mut S,
        key: &K,
        value: &V,
    ) -> Result<(), InterLiquidSdkError> {
        let old_value = self.map.get(state, key)?;

        self.map.set(state, key, value)?;

        if let Some(old_value) = old_value {
            for indexer in self.indexers.values() {
                let old_indexing_key = indexer.key_mapping(&old_value);
                let new_indexing_key = indexer.key_mapping(value);

                if old_indexing_key != new_indexing_key {
                    indexer.del(state, &old_indexing_key)?;
                    indexer.set(state, &new_indexing_key, &key.to_key_bytes())?;
                }
            }
        } else {
            for indexer in self.indexers.values() {
                indexer.set(state, &indexer.key_mapping(value), &key.to_key_bytes())?;
            }
        }

        Ok(())
    }

    pub fn del<S: StateManager>(&self, state: &mut S, key: &K) -> Result<(), InterLiquidSdkError> {
        let old_value = self.map.get(state, key)?;

        if let Some(old_value) = old_value {
            for indexer in self.indexers.values() {
                indexer.del(state, &indexer.key_mapping(&old_value))?;
            }

            self.map.del(state, key)?;
        }

        Ok(())
    }

    pub fn iter<'a, S: StateManager>(
        &'a self,
        state: &'a mut S,
        range: impl RangeBounds<Vec<u8>>,
    ) -> impl Iterator<Item = Result<(Vec<u8>, V), InterLiquidSdkError>> + 'a {
        self.map.iter(state, range)
    }
}

pub struct Indexer<V: BorshSerialize + BorshDeserialize> {
    prefix: Vec<u8>,
    key_mapping: Box<dyn Fn(&V) -> Vec<u8>>,
}

impl<V: BorshSerialize + BorshDeserialize> Indexer<V> {
    pub fn new(prefix: Vec<u8>, key_mapping: impl Fn(&V) -> Vec<u8> + 'static) -> Self {
        Self {
            prefix,
            key_mapping: Box::new(key_mapping),
        }
    }

    fn key_mapping(&self, value: &V) -> Vec<u8> {
        (self.key_mapping)(value)
    }

    pub fn get<S: StateManager>(
        &self,
        state: &mut S,
        indexing_key: &[u8],
    ) -> Result<Option<V>, InterLiquidSdkError> {
        let entire_key = join_keys([&self.prefix, indexing_key]);
        let primary_key = state.get(&entire_key)?;

        match primary_key {
            Some(value) => Ok(Some(V::deserialize(&mut &value[..])?)),
            None => Ok(None),
        }
    }

    fn set<S: StateManager>(
        &self,
        state: &mut S,
        indexing_key: &[u8],
        primary_key: &[u8],
    ) -> Result<(), InterLiquidSdkError> {
        let entire_key = join_keys([&self.prefix, indexing_key]);
        let mut buf = Vec::new();
        primary_key.serialize(&mut buf)?;

        state.set(&entire_key, &buf)
    }

    fn del<S: StateManager>(
        &self,
        state: &mut S,
        indexing_key: &[u8],
    ) -> Result<(), InterLiquidSdkError> {
        let entire_key = join_keys([&self.prefix, indexing_key]);

        state.del(&entire_key)
    }

    pub fn iter<'a, S: StateManager>(
        &'a self,
        state: &'a mut S,
        range: impl RangeBounds<Vec<u8>>,
    ) -> impl Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a {
        let iter = state.iter(range);

        iter.map(|result| {
            let (indexing_key, primary_key) = result?;

            Ok((indexing_key, primary_key))
        })
    }
}
