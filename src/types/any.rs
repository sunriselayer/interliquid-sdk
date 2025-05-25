use std::any::Any;

use borsh::{BorshDeserialize, BorshSerialize};
use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::types::InterLiquidSdkError;

/// A wrapper struct that allows serialization of any type along with its type information.
/// This enables type-safe deserialization by storing both the type name and serialized data.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct SerializableAny {
    /// The name of the type being serialized
    pub type_: String,
    /// The serialized binary data of the value
    pub value: Vec<u8>,
}

impl SerializableAny {
    /// Creates a new SerializableAny instance.
    ///
    /// # Arguments
    /// * `type_` - The name of the type being serialized
    /// * `value` - The serialized binary data
    pub fn new(type_: String, value: Vec<u8>) -> Self {
        Self { type_, value }
    }
}

/// Trait for types that can be serialized into SerializableAny with type information.
/// Types implementing this trait can be safely serialized and deserialized with type checking.
pub trait NamedSerializableType: Any + BorshSerialize + BorshDeserialize {
    /// Returns the static name of the type.
    /// This name is used to identify the type during deserialization.
    const TYPE_NAME: &'static str;

    /// Packs the current instance into a SerializableAny.
    ///
    /// # Returns
    /// A SerializableAny containing the type name and serialized data.
    ///
    /// # Errors
    /// Returns an error if serialization fails.
    fn pack_any(&self) -> Result<SerializableAny, InterLiquidSdkError> {
        let mut buf = vec![];
        self.serialize(&mut buf)?;

        let any = SerializableAny::new(Self::TYPE_NAME.to_owned(), buf);

        Ok(any)
    }

    /// Unpacks a SerializableAny into an instance of the implementing type.
    ///
    /// # Arguments
    /// * `any` - The SerializableAny to unpack
    ///
    /// # Returns
    /// An instance of the implementing type.
    ///
    /// # Errors
    /// Returns an error if deserialization fails.
    fn unpack_any(any: &SerializableAny) -> Result<Self, InterLiquidSdkError> {
        let value = Self::try_from_slice(&any.value)?;

        Ok(value)
    }
}
