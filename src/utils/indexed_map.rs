use std::{collections::BTreeMap, marker::PhantomData};

use borsh::BorshSerialize;

use super::{
    key::{join_keys, KeyDeclaration},
    KeyPrefix, Map, Value,
};
use crate::{state::TracableStateManager, types::InterLiquidSdkError};

/// A map structure that supports secondary indexes for efficient lookups.
///
/// `IndexedMap` wraps a regular `Map` and maintains additional indexes that allow
/// querying values by secondary keys. This is useful when you need to look up
/// values by attributes other than the primary key.
///
/// # Type Parameters
/// - `K`: The primary key type, must implement `KeyDeclaration`
/// - `V`: The value type, must implement `Value`
pub struct IndexedMap<K: KeyDeclaration, V: Value> {
    map: Map<K, V>,
    indexers: BTreeMap<String, Box<dyn IndexerI<K, V>>>,
}

impl<K: KeyDeclaration, V: Value> IndexedMap<K, V> {
    /// Creates a new `IndexedMap` with the given key prefix.
    ///
    /// # Parameters
    /// - `prefix`: An iterator of byte slices that will be joined to form the key prefix
    ///
    /// # Returns
    /// A new `IndexedMap` instance with no indexes configured
    pub fn new<'a, P: IntoIterator<Item = &'a [u8]>>(prefix: P) -> Self {
        Self {
            map: Map::new(prefix),
            indexers: BTreeMap::new(),
        }
    }

    /// Retrieves a value by its primary key.
    ///
    /// # Parameters
    /// - `state`: The state manager to read from
    /// - `key`: The primary key to look up
    ///
    /// # Returns
    /// - `Ok(Some(value))` if the key exists
    /// - `Ok(None)` if the key doesn't exist
    /// - `Err` if there was an error reading from state
    pub fn get<'a>(
        &self,
        state: &mut dyn TracableStateManager,
        key: K::KeyReference<'a>,
    ) -> Result<Option<V>, InterLiquidSdkError> {
        self.map.get(state, key)
    }

    /// Sets a value for the given primary key and updates all secondary indexes.
    ///
    /// If a value already exists for the key, the old index entries are removed
    /// and new ones are created based on the new value.
    ///
    /// # Parameters
    /// - `state`: The state manager to write to
    /// - `key`: The primary key to set
    /// - `value`: The value to store
    ///
    /// # Returns
    /// - `Ok(())` on success
    /// - `Err` if there was an error writing to state or updating indexes
    pub fn set<'a>(
        &self,
        state: &mut dyn TracableStateManager,
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

    /// Deletes a value by its primary key and removes all associated index entries.
    ///
    /// # Parameters
    /// - `state`: The state manager to write to
    /// - `key`: The primary key to delete
    ///
    /// # Returns
    /// - `Ok(())` on success (even if the key didn't exist)
    /// - `Err` if there was an error accessing state
    pub fn del<'a>(
        &self,
        state: &mut dyn TracableStateManager,
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

    /// Returns an iterator over key-value pairs matching the given key prefix.
    ///
    /// # Parameters
    /// - `state`: The state manager to read from
    /// - `key_prefix`: The key prefix to filter by
    ///
    /// # Returns
    /// An iterator that yields `Result<(key, value)>` pairs
    pub fn iter<'a, B: KeyPrefix>(
        &'a self,
        state: &'a mut dyn TracableStateManager,
        key_prefix: B,
    ) -> Box<dyn Iterator<Item = Result<(B::KeyToExtract, V), InterLiquidSdkError>> + 'a> {
        self.map.iter(state, key_prefix)
    }
}

/// Internal trait for indexer implementations.
///
/// This trait defines the interface that all indexers must implement to work
/// with `IndexedMap`. It provides methods for managing the index mappings.
///
/// # Type Parameters
/// - `PK`: The primary key type
/// - `V`: The value type
trait IndexerI<PK: KeyDeclaration, V: Value>: Send + Sync {
    fn _get(
        &self,
        state: &mut dyn TracableStateManager,
        indexing_key: &[u8],
    ) -> Result<Option<PK>, InterLiquidSdkError>;
    fn _set(
        &self,
        state: &mut dyn TracableStateManager,
        indexing_key: &[u8],
        primary_key: &[u8],
    ) -> Result<(), InterLiquidSdkError>;
    fn _del(
        &self,
        state: &mut dyn TracableStateManager,
        indexing_key: &[u8],
    ) -> Result<(), InterLiquidSdkError>;

    fn key_bytes_mapping(&self, value: &V) -> Result<Vec<u8>, InterLiquidSdkError>;
}

/// A secondary index for `IndexedMap` that maps from index keys to primary keys.
///
/// `Indexer` allows you to create secondary indexes on values stored in an `IndexedMap`.
/// You provide a function that extracts an index key from a value, and the indexer
/// maintains a mapping from index keys to primary keys.
///
/// # Type Parameters
/// - `IK`: The index key type
/// - `PK`: The primary key type
/// - `V`: The value type
pub struct Indexer<'a, IK: KeyDeclaration, PK: KeyDeclaration, V: Value> {
    prefix: Vec<u8>,
    key_mapping: Box<dyn Fn(&V) -> IK::KeyReference<'a> + Send + Sync>,
    phantom: PhantomData<PK>,
}

impl<'a, IK: KeyDeclaration, PK: KeyDeclaration, V: Value> Indexer<'a, IK, PK, V> {
    /// Creates a new `Indexer` with the given prefix and key mapping function.
    ///
    /// # Parameters
    /// - `prefix`: The key prefix for this index
    /// - `key_mapping`: A function that extracts the index key from a value
    ///
    /// # Returns
    /// A new `Indexer` instance
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

    /// Retrieves the primary key associated with the given index key.
    ///
    /// # Parameters
    /// - `state`: The state manager to read from
    /// - `indexing_key`: The index key to look up
    ///
    /// # Returns
    /// - `Ok(Some(primary_key))` if the index key exists
    /// - `Ok(None)` if the index key doesn't exist
    /// - `Err` if there was an error reading from state
    pub fn get<'b>(
        &self,
        state: &mut dyn TracableStateManager,
        indexing_key: IK::KeyReference<'b>,
    ) -> Result<Option<PK>, InterLiquidSdkError> {
        self._get(state, &IK::to_key_bytes(indexing_key))
    }

    /// Returns an iterator over index key to primary key mappings.
    ///
    /// # Parameters
    /// - `state`: The state manager to read from
    /// - `key_prefix`: The key prefix to filter by
    ///
    /// # Returns
    /// An iterator that yields `Result<(index_key, primary_key)>` pairs
    pub fn iter<'b, B: KeyPrefix>(
        &'b self,
        state: &'b mut dyn TracableStateManager,
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
        state: &mut dyn TracableStateManager,
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
        state: &mut dyn TracableStateManager,
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
        state: &mut dyn TracableStateManager,
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
