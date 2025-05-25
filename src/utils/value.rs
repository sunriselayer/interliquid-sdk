use borsh::{BorshDeserialize, BorshSerialize};

/// Trait for types that can be stored as values in state.
/// 
/// This trait is automatically implemented for all types that meet the requirements:
/// - Can be serialized and deserialized with Borsh
/// - Are thread-safe (`Send + Sync`)
/// - Have a `'static` lifetime
/// 
/// The trait acts as a marker to ensure that only suitable types can be stored
/// in the SDK's storage primitives (Item, Map, IndexedMap).
pub trait Value: BorshSerialize + BorshDeserialize + Send + Sync + 'static {}

/// Blanket implementation for all types that meet the requirements
impl<T: BorshSerialize + BorshDeserialize + Send + Sync + 'static> Value for T {}
