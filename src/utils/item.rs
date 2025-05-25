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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::RelatedState;
    use std::collections::BTreeMap;
    use borsh_derive::{BorshSerialize, BorshDeserialize};

    // Test value type that automatically implements Value trait
    #[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
    struct TestValue {
        id: u32,
        name: String,
    }

    #[test]
    fn test_basic_operations() {
        let mut state = RelatedState::new(BTreeMap::new());
        let item: Item<u64> = Item::new(&[b"test", b"counter"]);

        // Test initial get returns None
        assert_eq!(item.get(&mut state).unwrap(), None);

        // Test set and get
        item.set(&mut state, &42).unwrap();
        assert_eq!(item.get(&mut state).unwrap(), Some(42));

        // Test overwrite
        item.set(&mut state, &100).unwrap();
        assert_eq!(item.get(&mut state).unwrap(), Some(100));

        // Test delete
        item.del(&mut state).unwrap();
        assert_eq!(item.get(&mut state).unwrap(), None);
    }

    #[test]
    fn test_complex_value() {
        let mut state = RelatedState::new(BTreeMap::new());
        let item: Item<TestValue> = Item::new(&[b"test", b"complex"]);

        let value = TestValue {
            id: 1,
            name: "Test".to_string(),
        };

        // Test set and get complex value
        item.set(&mut state, &value).unwrap();
        let retrieved = item.get(&mut state).unwrap().unwrap();
        assert_eq!(retrieved, value);

        // Test update complex value
        let updated_value = TestValue {
            id: 2,
            name: "Updated".to_string(),
        };
        item.set(&mut state, &updated_value).unwrap();
        let retrieved = item.get(&mut state).unwrap().unwrap();
        assert_eq!(retrieved, updated_value);
    }

    #[test]
    fn test_different_keys() {
        let mut state = RelatedState::new(BTreeMap::new());
        let item1: Item<u64> = Item::new(&[b"counter1"]);
        let item2: Item<u64> = Item::new(&[b"counter2"]);

        // Test that different items don't interfere
        item1.set(&mut state, &10).unwrap();
        item2.set(&mut state, &20).unwrap();

        assert_eq!(item1.get(&mut state).unwrap(), Some(10));
        assert_eq!(item2.get(&mut state).unwrap(), Some(20));

        // Delete one shouldn't affect the other
        item1.del(&mut state).unwrap();
        assert_eq!(item1.get(&mut state).unwrap(), None);
        assert_eq!(item2.get(&mut state).unwrap(), Some(20));
    }

    #[test]
    fn test_string_values() {
        let mut state = RelatedState::new(BTreeMap::new());
        let item: Item<String> = Item::new(&[b"test", b"string"]);

        // Test with string values
        item.set(&mut state, &"Hello, World!".to_string()).unwrap();
        assert_eq!(item.get(&mut state).unwrap(), Some("Hello, World!".to_string()));

        // Test with empty string
        item.set(&mut state, &"".to_string()).unwrap();
        assert_eq!(item.get(&mut state).unwrap(), Some("".to_string()));

        // Test with Unicode string
        item.set(&mut state, &"🦀 Rust".to_string()).unwrap();
        assert_eq!(item.get(&mut state).unwrap(), Some("🦀 Rust".to_string()));
    }

    #[test]
    fn test_delete_nonexistent() {
        let mut state = RelatedState::new(BTreeMap::new());
        let item: Item<u64> = Item::new(&[b"nonexistent"]);

        // Deleting non-existent item should not error
        assert!(item.del(&mut state).is_ok());
        assert_eq!(item.get(&mut state).unwrap(), None);
    }

    #[test]
    fn test_key_composition() {
        let mut state = RelatedState::new(BTreeMap::new());
        
        // Test single key component
        let item1: Item<u32> = Item::new(vec![&b"single"[..]]);
        item1.set(&mut state, &1).unwrap();
        assert_eq!(item1.get(&mut state).unwrap(), Some(1));

        // Test multiple key components
        let item2: Item<u32> = Item::new(vec![&b"multi"[..], &b"part"[..], &b"key"[..]]);
        item2.set(&mut state, &2).unwrap();
        assert_eq!(item2.get(&mut state).unwrap(), Some(2));

        // Test empty key components are handled
        let item3: Item<u32> = Item::new(vec![&b"prefix"[..], &b""[..], &b"suffix"[..]]);
        item3.set(&mut state, &3).unwrap();
        assert_eq!(item3.get(&mut state).unwrap(), Some(3));
    }

    #[test]
    fn test_vec_values() {
        let mut state = RelatedState::new(BTreeMap::new());
        let item: Item<Vec<u8>> = Item::new(&[b"bytes"]);

        // Test with empty vec
        item.set(&mut state, &vec![]).unwrap();
        assert_eq!(item.get(&mut state).unwrap(), Some(vec![]));

        // Test with byte data
        let data = vec![1, 2, 3, 4, 5];
        item.set(&mut state, &data).unwrap();
        assert_eq!(item.get(&mut state).unwrap(), Some(data));

        // Test with large vec
        let large_data = vec![0u8; 1000];
        item.set(&mut state, &large_data).unwrap();
        assert_eq!(item.get(&mut state).unwrap(), Some(large_data));
    }

    #[test]
    fn test_bool_values() {
        let mut state = RelatedState::new(BTreeMap::new());
        let item: Item<bool> = Item::new(&[b"flag"]);

        // Test boolean values
        item.set(&mut state, &true).unwrap();
        assert_eq!(item.get(&mut state).unwrap(), Some(true));

        item.set(&mut state, &false).unwrap();
        assert_eq!(item.get(&mut state).unwrap(), Some(false));
    }
}
