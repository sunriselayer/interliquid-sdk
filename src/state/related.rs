use std::collections::BTreeMap;

use crate::types::InterLiquidSdkError;

use super::manager::StateManager;

/// A state manager implementation that stores key-value pairs in memory.
/// Tracks which keys are accessed to ensure all state dependencies are recorded.
pub struct RelatedState {
    /// The in-memory storage of key-value pairs for this state
    pub map: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl RelatedState {
    /// Creates a new RelatedState instance with the given initial state.
    /// 
    /// # Arguments
    /// 
    /// * `map` - Initial key-value pairs to populate the state
    pub fn new(map: BTreeMap<Vec<u8>, Vec<u8>>) -> Self {
        Self { map }
    }
}

impl StateManager for RelatedState {
    /// Retrieves a value from the related state.
    /// Returns an error if the key is not in the tracked state.
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError> {
        if let Some(value) = self.map.get(key) {
            Ok(Some(value.clone()))
        } else {
            Err(InterLiquidSdkError::UnrelatedState)
        }
    }

    /// Sets a key-value pair in the related state.
    /// Adds the key to the tracked state if not already present.
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError> {
        self.map.insert(key.to_vec(), value.to_vec());

        Ok(())
    }

    /// Removes a key from the related state.
    /// No-op if the key doesn't exist.
    fn del(&mut self, key: &[u8]) -> Result<(), InterLiquidSdkError> {
        self.map.remove(key);

        Ok(())
    }

    /// Creates an iterator over key-value pairs with the given prefix.
    /// Only returns keys that are in the tracked state.
    fn iter<'a>(
        &'a self,
        key_prefix: Vec<u8>,
    ) -> Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a> {
        let iter = bytes_prefix_range(&self.map, key_prefix);

        Box::new(iter.map(|(k, v)| Ok((k, v))))
    }
}

/// Creates an iterator over entries in a BTreeMap that match a given prefix.
/// Handles edge cases for empty prefixes and prefixes ending with 0xFF.
/// 
/// # Arguments
/// 
/// * `map` - The BTreeMap to iterate over
/// * `key_prefix` - The prefix to filter keys by
/// 
/// # Returns
/// 
/// An iterator yielding (key, value) pairs where keys start with the given prefix
pub fn bytes_prefix_range<'a, T: Clone>(
    map: &'a BTreeMap<Vec<u8>, T>,
    key_prefix: Vec<u8>,
) -> Box<dyn Iterator<Item = (Vec<u8>, T)> + 'a> {
    if key_prefix.len() == 0 {
        Box::new(map.iter().map(|(k, v)| (k.clone(), v.clone())))
    } else if key_prefix.iter().all(|&b| b == 0xFF) {
        Box::new(map.range(key_prefix..).map(|(k, v)| (k.clone(), v.clone())))
    } else {
        let mut key_prefix_next = key_prefix.clone();
        *key_prefix_next.last_mut().unwrap() += 1; // len > 0

        Box::new(
            map.range(key_prefix..key_prefix_next)
                .map(|(k, v)| (k.clone(), v.clone())),
        )
    }
}
