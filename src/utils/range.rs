use std::marker::PhantomData;

use borsh::BorshDeserialize;

use crate::types::InterLiquidSdkError;

use super::KeyDeclaration;

/// Trait for types that can be used as key prefixes in range queries.
/// 
/// `KeyPrefix` allows you to query for a range of keys that share a common prefix.
/// This is useful for iterating over subsets of data in maps.
/// 
/// # Associated Types
/// - `KeyToExtract`: The full key type that will be extracted from the stored keys
/// 
/// # Required Methods
/// - `to_prefix_bytes`: Converts the prefix to its byte representation
/// 
/// # Provided Methods
/// - `extract`: Extracts the full key from the stored key bytes
pub trait KeyPrefix: Clone + Sized + Send {
    /// The full key type that will be extracted from results
    type KeyToExtract: KeyDeclaration;

    /// Converts this prefix to its byte representation
    fn to_prefix_bytes(&self) -> Vec<u8>;

    /// Extracts the full key from the stored key bytes
    /// 
    /// # Parameters
    /// - `key`: The key bytes to extract from
    /// 
    /// # Returns
    /// The extracted key or an error if deserialization fails
    fn extract<'a>(key: &mut [u8]) -> Result<Self::KeyToExtract, InterLiquidSdkError> {
        Ok(<Self::KeyToExtract as BorshDeserialize>::try_from_slice(
            &key,
        )?)
    }
}

/// A key prefix for tuple keys where only the first element is specified.
/// 
/// This allows you to query all entries in a map with composite keys `(T1, T2)`
/// that have a specific value for the first component.
/// 
/// # Type Parameters
/// - `T1`: The first key component type (the prefix)
/// - `T2`: The second key component type
/// 
/// # Example
/// ```ignore
/// // Query all tokens owned by a specific address
/// let prefix = KeyPrefixTupleOne::<Address, TokenId>::new(&owner_address);
/// for result in map.iter(&mut state, prefix) {
///     let ((address, token_id), value) = result?;
/// }
/// ```
#[derive(Clone)]
pub struct KeyPrefixTupleOne<'a, T1, T2>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    prefix: <T1 as KeyDeclaration>::KeyReference<'a>,
    phantom: PhantomData<(T1, T2)>,
}

impl<'a, T1, T2> KeyPrefixTupleOne<'a, T1, T2>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    /// Creates a new `KeyPrefixTupleOne` with the given first component.
    /// 
    /// # Parameters
    /// - `prefix`: The value for the first key component
    /// 
    /// # Returns
    /// A new `KeyPrefixTupleOne` instance
    pub fn new(prefix: T1::KeyReference<'a>) -> KeyPrefixTupleOne<'a, T1, T2> {
        KeyPrefixTupleOne {
            prefix,
            phantom: PhantomData,
        }
    }
}

impl<'a, T1, T2> KeyPrefix for KeyPrefixTupleOne<'a, T1, T2>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    type KeyToExtract = (T1, T2);

    fn to_prefix_bytes(&self) -> Vec<u8> {
        <T1 as KeyDeclaration>::to_key_bytes(self.prefix)
    }
}

/// A key prefix for triple keys where only the first element is specified.
/// 
/// This allows you to query all entries in a map with composite keys `(T1, T2, T3)`
/// that have a specific value for the first component.
/// 
/// # Type Parameters
/// - `T1`: The first key component type (the prefix)
/// - `T2`: The second key component type
/// - `T3`: The third key component type
#[derive(Clone)]
pub struct KeyPrefixTripleOne<'a, T1, T2, T3>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    T3: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T3 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    prefix: <T1 as KeyDeclaration>::KeyReference<'a>,
    phantom: PhantomData<(T1, T2, T3)>,
}

impl<'a, T1, T2, T3> KeyPrefixTripleOne<'a, T1, T2, T3>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    T3: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T3 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    /// Creates a new `KeyPrefixTripleOne` with the given first component.
    /// 
    /// # Parameters
    /// - `prefix`: The value for the first key component
    /// 
    /// # Returns
    /// A new `KeyPrefixTripleOne` instance
    pub fn new(prefix: T1::KeyReference<'a>) -> KeyPrefixTripleOne<'a, T1, T2, T3> {
        KeyPrefixTripleOne {
            prefix,
            phantom: PhantomData,
        }
    }
}

impl<'a, T1, T2, T3> KeyPrefix for KeyPrefixTripleOne<'a, T1, T2, T3>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    T3: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T3 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    type KeyToExtract = (T1, T2, T3);

    fn to_prefix_bytes(&self) -> Vec<u8> {
        <T1 as KeyDeclaration>::to_key_bytes(self.prefix)
    }
}

/// A key prefix for triple keys where the first two elements are specified.
/// 
/// This allows you to query all entries in a map with composite keys `(T1, T2, T3)`
/// that have specific values for the first two components.
/// 
/// # Type Parameters
/// - `T1`: The first key component type
/// - `T2`: The second key component type  
/// - `T3`: The third key component type
/// 
/// # Example
/// ```ignore
/// // Query all items in a specific collection owned by a specific address
/// let prefix = KeyPrefixTripleTwo::<Address, CollectionId, ItemId>::new((&owner, &collection));
/// for result in map.iter(&mut state, prefix) {
///     let ((address, collection_id, item_id), value) = result?;
/// }
/// ```
#[derive(Clone)]
pub struct KeyPrefixTripleTwo<'a, T1, T2, T3>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    T3: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T3 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    prefix: (T1::KeyReference<'a>, T2::KeyReference<'a>),
    phantom: PhantomData<(T1, T2, T3)>,
}

impl<'a, T1, T2, T3> KeyPrefixTripleTwo<'a, T1, T2, T3>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    T3: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T3 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    /// Creates a new `KeyPrefixTripleTwo` with the given first two components.
    /// 
    /// # Parameters
    /// - `prefix`: A tuple containing values for the first two key components
    /// 
    /// # Returns
    /// A new `KeyPrefixTripleTwo` instance
    pub fn new(
        prefix: (T1::KeyReference<'a>, T2::KeyReference<'a>),
    ) -> KeyPrefixTripleTwo<'a, T1, T2, T3> {
        KeyPrefixTripleTwo {
            prefix,
            phantom: PhantomData,
        }
    }
}

impl<'a, T1, T2, T3> KeyPrefix for KeyPrefixTripleTwo<'a, T1, T2, T3>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    T3: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T3 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    type KeyToExtract = (T1, T2, T3);

    fn to_prefix_bytes(&self) -> Vec<u8> {
        <(T1, T2) as KeyDeclaration>::to_key_bytes(self.prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use borsh::{BorshDeserialize, BorshSerialize};
    use borsh_derive::{BorshDeserialize, BorshSerialize};

    // Define a simple test type that implements KeyDeclaration
    #[derive(Clone, BorshSerialize, BorshDeserialize)]
    struct TestKey(u32);

    impl KeyDeclaration for TestKey {
        type KeyReference<'a> = &'a TestKey;

        fn to_key_bytes(key: Self::KeyReference<'_>) -> Vec<u8> {
            let mut buf = Vec::new();
            key.serialize(&mut buf).unwrap();
            buf
        }
    }

    #[test]
    fn test_key_prefix_tuple_one() {
        let test_key = TestKey(42);
        let prefix = KeyPrefixTupleOne::<TestKey, TestKey>::new(&test_key);

        // Test to_prefix_bytes
        let bytes = prefix.to_prefix_bytes();
        let deserialized = TestKey::try_from_slice(&bytes).unwrap();
        assert_eq!(deserialized.0, 42);
    }

    #[test]
    fn test_key_prefix_triple_one() {
        let test_key = TestKey(42);
        let prefix = KeyPrefixTripleOne::<TestKey, TestKey, TestKey>::new(&test_key);

        // Test to_prefix_bytes
        let bytes = prefix.to_prefix_bytes();
        let deserialized = TestKey::try_from_slice(&bytes).unwrap();
        assert_eq!(deserialized.0, 42);
    }

    #[test]
    fn test_key_prefix_triple_two() {
        let test_key1 = TestKey(42);
        let test_key2 = TestKey(24);
        let prefix = KeyPrefixTripleTwo::<TestKey, TestKey, TestKey>::new((&test_key1, &test_key2));

        // Test to_prefix_bytes
        let bytes = prefix.to_prefix_bytes();
        let deserialized: (TestKey, TestKey) = BorshDeserialize::try_from_slice(&bytes).unwrap();
        assert_eq!(deserialized.0 .0, 42);
        assert_eq!(deserialized.1 .0, 24);
    }
}
