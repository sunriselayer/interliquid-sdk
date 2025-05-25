use std::marker::PhantomData;

use super::{
    key::{join_keys, KeyDeclaration},
    KeyPrefix, Value,
};
use crate::{state::TracableStateManager, types::InterLiquidSdkError};

/// A key-value map for storing multiple values indexed by keys.
/// 
/// `Map` provides a way to store multiple values, each associated with a unique key.
/// All keys are automatically prefixed to avoid collisions with other storage.
/// 
/// # Type Parameters
/// - `K`: The key type, must implement `KeyDeclaration`
/// - `V`: The value type, must implement `Value`
/// 
/// # Example
/// ```ignore
/// let balances: Map<Address, u64> = Map::new(&[b"balances"]);
/// balances.set(&mut state, &address, &100)?;
/// let balance = balances.get(&mut state, &address)?; // Some(100)
/// ```
pub struct Map<K: KeyDeclaration, V: Value> {
    prefix: Vec<u8>,
    phantom: PhantomData<(K, V)>,
}

impl<K: KeyDeclaration, V: Value> Map<K, V> {
    /// Creates a new `Map` with the given key prefix.
    /// 
    /// # Parameters
    /// - `prefix`: An iterator of byte slices that will be joined to form the key prefix
    /// 
    /// # Returns
    /// A new `Map` instance
    pub fn new<'a, P: IntoIterator<Item = &'a [u8]>>(prefix: P) -> Self {
        Self {
            prefix: join_keys(prefix),
            phantom: PhantomData,
        }
    }

    /// Retrieves a value by its key.
    /// 
    /// # Parameters
    /// - `state`: The state manager to read from
    /// - `key`: The key to look up
    /// 
    /// # Returns
    /// - `Ok(Some(value))` if the key exists
    /// - `Ok(None)` if the key doesn't exist
    /// - `Err` if there was an error reading from state or deserializing
    pub fn get<'a>(
        &self,
        state: &mut dyn TracableStateManager,
        key: K::KeyReference<'a>,
    ) -> Result<Option<V>, InterLiquidSdkError> {
        let entire_key = join_keys([self.prefix.as_slice(), &K::to_key_bytes(key)]);
        let value = state.get(&entire_key)?;

        match value {
            Some(value) => Ok(Some(V::try_from_slice(&value)?)),
            None => Ok(None),
        }
    }

    /// Sets a value for the given key.
    /// 
    /// # Parameters
    /// - `state`: The state manager to write to
    /// - `key`: The key to set
    /// - `value`: The value to store
    /// 
    /// # Returns
    /// - `Ok(())` on success
    /// - `Err` if there was an error serializing or writing to state
    pub fn set<'a>(
        &self,
        state: &mut dyn TracableStateManager,
        key: K::KeyReference<'a>,
        value: &V,
    ) -> Result<(), InterLiquidSdkError> {
        let entire_key = join_keys([self.prefix.as_slice(), &K::to_key_bytes(key)]);
        let mut buf = Vec::new();
        value.serialize(&mut buf)?;

        state.set(&entire_key, &buf)
    }

    /// Deletes a value by its key.
    /// 
    /// # Parameters
    /// - `state`: The state manager to write to
    /// - `key`: The key to delete
    /// 
    /// # Returns
    /// - `Ok(())` on success (even if the key didn't exist)
    /// - `Err` if there was an error accessing state
    pub fn del<'a>(
        &self,
        state: &mut dyn TracableStateManager,
        key: K::KeyReference<'a>,
    ) -> Result<(), InterLiquidSdkError> {
        let entire_key = join_keys([self.prefix.as_slice(), &K::to_key_bytes(key)]);

        state.del(&entire_key)
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
        let iter = state.iter(key_prefix.to_prefix_bytes());

        Box::new(iter.map(|result| {
            let (mut k, v) = result?;
            let key = B::extract(&mut k[self.prefix.len()..])?;
            let value = V::try_from_slice(&v)?;

            Ok((key, value))
        }))
    }
}
