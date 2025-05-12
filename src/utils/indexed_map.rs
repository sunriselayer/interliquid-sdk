use std::{collections::BTreeMap, marker::PhantomData};

use borsh::BorshSerialize;

use super::{
    key::{join_keys, KeyDeclaration},
    KeyPrefix, Map, Value,
};
use crate::{state::StateManager, types::InterLiquidSdkError};

pub struct IndexedMap<K: KeyDeclaration, V: Value> {
    map: Map<K, V>,
    indexers: BTreeMap<String, Box<dyn IndexerI<K, V>>>,
}

impl<K: KeyDeclaration, V: Value> IndexedMap<K, V> {
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
                let old_indexing_key = indexer.key_bytes_mapping(&old_value)?;
                let new_indexing_key = indexer.key_bytes_mapping(value)?;

                if old_indexing_key != new_indexing_key {
                    indexer._del(state, &old_indexing_key)?;
                    indexer._set(state, &new_indexing_key, &K::to_key_bytes(key))?;
                }
            }
        } else {
            for indexer in self.indexers.values() {
                indexer._set(
                    state,
                    &indexer.key_bytes_mapping(&value)?,
                    &K::to_key_bytes(key),
                )?;
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
                indexer._del(state, &indexer.key_bytes_mapping(&old_value)?)?;
            }

            self.map.del(state, key)?;
        }

        Ok(())
    }

    pub fn iter<'a, B: KeyPrefix>(
        &'a self,
        state: &'a mut dyn StateManager,
        key_prefix: B,
    ) -> Box<dyn Iterator<Item = Result<(B::KeyToExtract, V), InterLiquidSdkError>> + 'a> {
        self.map.iter(state, key_prefix)
    }
}

trait IndexerI<PK: KeyDeclaration, V: Value>: Send + Sync {
    fn _get(
        &self,
        state: &mut dyn StateManager,
        indexing_key: &[u8],
    ) -> Result<Option<PK>, InterLiquidSdkError>;
    fn _set(
        &self,
        state: &mut dyn StateManager,
        indexing_key: &[u8],
        primary_key: &[u8],
    ) -> Result<(), InterLiquidSdkError>;
    fn _del(
        &self,
        state: &mut dyn StateManager,
        indexing_key: &[u8],
    ) -> Result<(), InterLiquidSdkError>;

    fn key_bytes_mapping(&self, value: &V) -> Result<Vec<u8>, InterLiquidSdkError>;
}

pub struct Indexer<'a, IK: KeyDeclaration, PK: KeyDeclaration, V: Value> {
    prefix: Vec<u8>,
    key_mapping: Box<dyn Fn(&V) -> IK::KeyReference<'a> + Send + Sync>,
    phantom: PhantomData<PK>,
}

impl<'a, IK: KeyDeclaration, PK: KeyDeclaration, V: Value> Indexer<'a, IK, PK, V> {
    pub fn new(
        prefix: Vec<u8>,
        key_mapping: impl Fn(&V) -> IK::KeyReference<'a> + Send + Sync + 'static,
    ) -> Self {
        Self {
            prefix,
            key_mapping: Box::new(key_mapping),
            phantom: PhantomData,
        }
    }

    pub fn get<'b>(
        &self,
        state: &mut dyn StateManager,
        indexing_key: IK::KeyReference<'b>,
    ) -> Result<Option<PK>, InterLiquidSdkError> {
        self._get(state, &IK::to_key_bytes(indexing_key))
    }

    pub fn iter<'b, B: KeyPrefix>(
        &'b self,
        state: &'b mut dyn StateManager,
        key_prefix: B,
    ) -> Box<dyn Iterator<Item = Result<(IK, PK), InterLiquidSdkError>> + 'b> {
        let iter = state.iter(key_prefix.to_prefix_bytes());

        Box::new(iter.map(|result| {
            let (ik, pk) = result?;
            let ik = IK::try_from_slice(&ik)?;
            let pk = PK::try_from_slice(&pk)?;

            Ok((ik, pk))
        }))
    }
}

impl<'a, IK: KeyDeclaration, PK: KeyDeclaration, V: Value> IndexerI<PK, V>
    for Indexer<'a, IK, PK, V>
{
    fn _get(
        &self,
        state: &mut dyn StateManager,
        indexing_key: &[u8],
    ) -> Result<Option<PK>, InterLiquidSdkError> {
        let entire_key = join_keys([&self.prefix, indexing_key]);
        let primary_key = state.get(&entire_key)?;

        match primary_key {
            Some(value) => Ok(Some(PK::try_from_slice(&value)?)),
            None => Ok(None),
        }
    }

    fn _set(
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

    fn _del(
        &self,
        state: &mut dyn StateManager,
        indexing_key: &[u8],
    ) -> Result<(), InterLiquidSdkError> {
        let entire_key = join_keys([&self.prefix, indexing_key]);

        state.del(&entire_key)
    }

    fn key_bytes_mapping(&self, value: &V) -> Result<Vec<u8>, InterLiquidSdkError> {
        let mut buf = Vec::new();
        (self.key_mapping)(value).serialize(&mut buf)?;

        Ok(buf)
    }
}
