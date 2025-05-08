use std::{collections::BTreeMap, ops::RangeBounds};

use borsh::{BorshDeserialize, BorshSerialize};

use super::{
    key::{join_keys, KeyDeclaration},
    IntoObjectSafeRangeBounds, Map, PrefixBound,
};
use crate::{state::StateManager, types::InterLiquidSdkError};

pub struct IndexedMap<K: KeyDeclaration, V: BorshSerialize + BorshDeserialize> {
    map: Map<K, V>,
    indexers: BTreeMap<String, Indexer<V>>,
}

impl<K: KeyDeclaration, V: BorshSerialize + BorshDeserialize> IndexedMap<K, V> {
    pub fn new<'a, P: IntoIterator<Item = &'a [u8]>>(prefix: P) -> Self {
        Self {
            map: Map::new(prefix),
            indexers: BTreeMap::new(),
        }
    }

    pub fn get<'a>(
        &self,
        state: &mut dyn StateManager,
        key: K::KeyReference<'a>,
    ) -> Result<Option<V>, InterLiquidSdkError> {
        self.map.get(state, key)
    }

    pub fn set<'a>(
        &self,
        state: &mut dyn StateManager,
        key: K::KeyReference<'a>,
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
                    indexer.set(state, &new_indexing_key, &K::to_key_bytes(key))?;
                }
            }
        } else {
            for indexer in self.indexers.values() {
                indexer.set(state, &indexer.key_mapping(value), &K::to_key_bytes(key))?;
            }
        }

        Ok(())
    }

    pub fn del<'a>(
        &self,
        state: &mut dyn StateManager,
        key: K::KeyReference<'a>,
    ) -> Result<(), InterLiquidSdkError> {
        let old_value = self.map.get(state, key)?;

        if let Some(old_value) = old_value {
            for indexer in self.indexers.values() {
                indexer.del(state, &indexer.key_mapping(&old_value))?;
            }

            self.map.del(state, key)?;
        }

        Ok(())
    }

    pub fn iter<'a, B: PrefixBound>(
        &'a self,
        state: &'a mut dyn StateManager,
        range: impl RangeBounds<B>,
    ) -> Box<dyn Iterator<Item = Result<(B::KeyToExtract, V), InterLiquidSdkError>> + 'a> {
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

    pub fn get(
        &self,
        state: &mut dyn StateManager,
        indexing_key: &[u8],
    ) -> Result<Option<V>, InterLiquidSdkError> {
        let entire_key = join_keys([&self.prefix, indexing_key]);
        let primary_key = state.get(&entire_key)?;

        match primary_key {
            Some(value) => Ok(Some(V::deserialize(&mut &value[..])?)),
            None => Ok(None),
        }
    }

    fn set(
        &self,
        state: &mut dyn StateManager,
        indexing_key: &[u8],
        primary_key: &[u8],
    ) -> Result<(), InterLiquidSdkError> {
        let entire_key = join_keys([&self.prefix, indexing_key]);
        let mut buf = Vec::new();
        primary_key.serialize(&mut buf)?;

        state.set(&entire_key, &buf)
    }

    fn del(
        &self,
        state: &mut dyn StateManager,
        indexing_key: &[u8],
    ) -> Result<(), InterLiquidSdkError> {
        let entire_key = join_keys([&self.prefix, indexing_key]);

        state.del(&entire_key)
    }

    pub fn iter<'a>(
        &'a self,
        state: &'a mut dyn StateManager,
        range: impl IntoObjectSafeRangeBounds<Vec<u8>>,
    ) -> Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a> {
        let iter = state.iter(range.into_object_safe_range_bounds());

        Box::new(iter.map(|result| {
            let (mut indexing_key, primary_key) = result?;
            let indexing_key = indexing_key.split_off(self.prefix.len());

            Ok((indexing_key, primary_key))
        }))
    }
}
