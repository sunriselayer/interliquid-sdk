use interliquid_sdk::utils::range::{KeyPrefix, KeyPrefixTupleOne, KeyPrefixTripleOne, KeyPrefixTripleTwo};
use interliquid_sdk::utils::KeyDeclaration;
use borsh::{BorshDeserialize, BorshSerialize};
use borsh_derive::{BorshSerialize, BorshDeserialize};

// Define a simple test type that implements KeyDeclaration
#[derive(Clone, BorshSerialize, BorshDeserialize)]
struct TestKey(u32);

impl KeyDeclaration for TestKey {
    type KeyReference<'a> = &'a TestKey;

    fn to_key_bytes(key: Self::KeyReference<'_>) -> Vec<u8> {
        (*key).try_to_vec().unwrap_or_default()
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
    assert_eq!(deserialized.0.0, 42);
    assert_eq!(deserialized.1.0, 24);
} 