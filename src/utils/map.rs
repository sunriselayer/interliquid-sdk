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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::RelatedState;
    use std::collections::BTreeMap;
    use borsh::BorshSerialize;
    use borsh_derive::{BorshSerialize as BorshSerializeDerive, BorshDeserialize as BorshDeserializeDerive};

    // Test key type
    #[derive(Debug, Clone, PartialEq, BorshSerializeDerive, BorshDeserializeDerive)]
    struct TestKey {
        id: u32,
    }

    impl KeyDeclaration for TestKey {
        type KeyReference<'a> = &'a TestKey;

        fn to_key_bytes(key: &TestKey) -> Vec<u8> {
            let mut buf = Vec::new();
            key.serialize(&mut buf).unwrap();
            buf
        }
    }

    // Test value type - automatically implements Value trait
    #[derive(Debug, Clone, PartialEq, BorshSerializeDerive, BorshDeserializeDerive)]
    struct TestValue {
        name: String,
        amount: u64,
    }

    #[test]
    fn test_basic_operations() {
        let mut state = RelatedState::new(BTreeMap::new());
        let map: Map<TestKey, u64> = Map::new(vec![&b"test"[..], &b"map"[..]]);

        let key1 = TestKey { id: 1 };
        let key2 = TestKey { id: 2 };

        // Test initial get returns None
        assert_eq!(map.get(&mut state, &key1).unwrap(), None);
        assert_eq!(map.get(&mut state, &key2).unwrap(), None);

        // Test set and get
        map.set(&mut state, &key1, &100).unwrap();
        map.set(&mut state, &key2, &200).unwrap();
        assert_eq!(map.get(&mut state, &key1).unwrap(), Some(100));
        assert_eq!(map.get(&mut state, &key2).unwrap(), Some(200));

        // Test overwrite
        map.set(&mut state, &key1, &150).unwrap();
        assert_eq!(map.get(&mut state, &key1).unwrap(), Some(150));
        assert_eq!(map.get(&mut state, &key2).unwrap(), Some(200)); // key2 unchanged

        // Test delete
        map.del(&mut state, &key1).unwrap();
        assert_eq!(map.get(&mut state, &key1).unwrap(), None);
        assert_eq!(map.get(&mut state, &key2).unwrap(), Some(200)); // key2 still exists
    }

    #[test]
    fn test_complex_values() {
        let mut state = RelatedState::new(BTreeMap::new());
        let map: Map<TestKey, TestValue> = Map::new(vec![&b"complex"[..]]);

        let key = TestKey { id: 42 };
        let value = TestValue {
            name: "Alice".to_string(),
            amount: 1000,
        };

        // Test set and get complex value
        map.set(&mut state, &key, &value).unwrap();
        let retrieved = map.get(&mut state, &key).unwrap().unwrap();
        assert_eq!(retrieved, value);

        // Test update complex value
        let updated_value = TestValue {
            name: "Bob".to_string(),
            amount: 2000,
        };
        map.set(&mut state, &key, &updated_value).unwrap();
        let retrieved = map.get(&mut state, &key).unwrap().unwrap();
        assert_eq!(retrieved, updated_value);
    }

    #[test]
    fn test_string_keys() {
        let mut state = RelatedState::new(BTreeMap::new());
        let map: Map<String, u32> = Map::new(vec![&b"string_map"[..]]);

        // Test with string keys
        map.set(&mut state, &"key1".to_string(), &10).unwrap();
        map.set(&mut state, &"key2".to_string(), &20).unwrap();
        
        assert_eq!(map.get(&mut state, &"key1".to_string()).unwrap(), Some(10));
        assert_eq!(map.get(&mut state, &"key2".to_string()).unwrap(), Some(20));
        assert_eq!(map.get(&mut state, &"key3".to_string()).unwrap(), None);

        // Test with empty string key
        map.set(&mut state, &"".to_string(), &30).unwrap();
        assert_eq!(map.get(&mut state, &"".to_string()).unwrap(), Some(30));
    }

    #[test]
    fn test_delete_nonexistent() {
        let mut state = RelatedState::new(BTreeMap::new());
        let map: Map<TestKey, u64> = Map::new(vec![&b"test_del"[..]]);

        let key = TestKey { id: 999 };

        // Deleting non-existent key should not error
        assert!(map.del(&mut state, &key).is_ok());
        assert_eq!(map.get(&mut state, &key).unwrap(), None);
    }

    #[test]
    fn test_multiple_maps_isolation() {
        let mut state = RelatedState::new(BTreeMap::new());
        let map1: Map<TestKey, u64> = Map::new(vec![&b"map1"[..]]);
        let map2: Map<TestKey, u64> = Map::new(vec![&b"map2"[..]]);

        let key = TestKey { id: 1 };

        // Set same key in different maps
        map1.set(&mut state, &key, &100).unwrap();
        map2.set(&mut state, &key, &200).unwrap();

        // Verify they store different values
        assert_eq!(map1.get(&mut state, &key).unwrap(), Some(100));
        assert_eq!(map2.get(&mut state, &key).unwrap(), Some(200));

        // Delete from one map shouldn't affect the other
        map1.del(&mut state, &key).unwrap();
        assert_eq!(map1.get(&mut state, &key).unwrap(), None);
        assert_eq!(map2.get(&mut state, &key).unwrap(), Some(200));
    }

    // Test key prefix implementation for iteration
    #[derive(Clone)]
    struct TestKeyPrefix;
    
    impl KeyPrefix for TestKeyPrefix {
        type KeyToExtract = TestKey;

        fn to_prefix_bytes(&self) -> Vec<u8> {
            vec![]
        }
    }

    #[test]
    fn test_iteration() {
        let mut state = RelatedState::new(BTreeMap::new());
        let map: Map<TestKey, u64> = Map::new(vec![&b"iter_test"[..]]);

        // Insert multiple values
        let keys_values = vec![
            (TestKey { id: 1 }, 100),
            (TestKey { id: 2 }, 200),
            (TestKey { id: 3 }, 300),
        ];

        for (key, value) in &keys_values {
            map.set(&mut state, key, value).unwrap();
        }

        // Collect all items through iteration
        let mut collected: Vec<(TestKey, u64)> = map.iter(&mut state, TestKeyPrefix)
            .map(|result| result.unwrap())
            .collect();

        // Sort by id for consistent comparison
        collected.sort_by_key(|(k, _)| k.id);

        assert_eq!(collected.len(), 3);
        for (i, (key, value)) in collected.iter().enumerate() {
            assert_eq!(key.id, keys_values[i].0.id);
            assert_eq!(*value, keys_values[i].1);
        }
    }

    #[test]
    fn test_vec_keys() {
        let mut state = RelatedState::new(BTreeMap::new());
        let map: Map<Vec<u8>, String> = Map::new(vec![&b"vec_key_map"[..]]);

        // Test with byte vector keys
        let key1 = vec![1, 2, 3];
        let key2 = vec![4, 5, 6];
        let key3 = vec![]; // empty vec

        map.set(&mut state, &key1, &"value1".to_string()).unwrap();
        map.set(&mut state, &key2, &"value2".to_string()).unwrap();
        map.set(&mut state, &key3, &"empty_key".to_string()).unwrap();

        assert_eq!(map.get(&mut state, &key1).unwrap(), Some("value1".to_string()));
        assert_eq!(map.get(&mut state, &key2).unwrap(), Some("value2".to_string()));
        assert_eq!(map.get(&mut state, &key3).unwrap(), Some("empty_key".to_string()));
    }

    #[test]
    fn test_large_values() {
        let mut state = RelatedState::new(BTreeMap::new());
        let map: Map<u32, Vec<u8>> = Map::new(vec![&b"large_values"[..]]);

        // Test with large value
        let large_value = vec![0u8; 10000];
        map.set(&mut state, 1u32, &large_value).unwrap();
        
        let retrieved = map.get(&mut state, 1u32).unwrap().unwrap();
        assert_eq!(retrieved.len(), 10000);
        assert_eq!(retrieved, large_value);
    }
}
