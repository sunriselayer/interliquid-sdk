use std::marker::PhantomData;

use super::{key::join_keys, Value};
use crate::{state::TracableStateManager, types::InterLiquidSdkError};

/// `Item` stores one value for the designated key in the state.
/// 
/// This is the simplest storage primitive, storing a single value at a specific key.
/// Use this when you need to store configuration data, counters, or any other
/// single piece of data.
/// 
/// # Type Parameters
/// - `V`: The value type, must implement `Value`
/// 
/// # Example
/// ```ignore
/// let counter: Item<u64> = Item::new(&[b"counter"]);
/// counter.set(&mut state, &42)?;
/// let value = counter.get(&mut state)?; // Some(42)
/// ```
pub struct Item<V: Value> {
    key: Vec<u8>,
    phantom: PhantomData<V>,
}

impl<V: Value> Item<V> {
    /// Creates a new `Item` with the given key.
    /// 
    /// # Parameters
    /// - `key`: An iterator of byte slices that will be joined to form the storage key
    /// 
    /// # Returns
    /// A new `Item` instance
    pub fn new<'a, P: IntoIterator<Item = &'a [u8]>>(key: P) -> Self {
        Self {
            key: join_keys(key),
            phantom: PhantomData,
        }
    }

    /// Retrieves the stored value.
    /// 
    /// # Parameters
    /// - `state`: The state manager to read from
    /// 
    /// # Returns
    /// - `Ok(Some(value))` if a value is stored
    /// - `Ok(None)` if no value is stored
    /// - `Err` if there was an error reading from state or deserializing
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

    /// Stores a value, replacing any existing value.
    /// 
    /// # Parameters
    /// - `state`: The state manager to write to
    /// - `value`: The value to store
    /// 
    /// # Returns
    /// - `Ok(())` on success
    /// - `Err` if there was an error serializing or writing to state
    pub fn set<S: TracableStateManager>(
        &self,
        state: &mut S,
        value: &V,
    ) -> Result<(), InterLiquidSdkError> {
        let mut buf = Vec::new();
        value.serialize(&mut buf)?;

        state.set(&self.key, &buf)
    }

    /// Deletes the stored value.
    /// 
    /// # Parameters
    /// - `state`: The state manager to write to
    /// 
    /// # Returns
    /// - `Ok(())` on success (even if no value was stored)
    /// - `Err` if there was an error accessing state
    pub fn del<S: TracableStateManager>(&self, state: &mut S) -> Result<(), InterLiquidSdkError> {
        state.del(&self.key)
    }
}
