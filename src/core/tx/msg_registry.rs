use std::collections::BTreeMap;

use anyhow::anyhow;

use crate::types::{InterLiquidSdkError, NamedSerializableType, SerializableAny};

use super::Msg;

/// The registry for unpacking `SerializableAny` of Tx's Msg types.
///
/// This registry provides functionality to deserialize messages from their
/// serialized `SerializableAny` representation back into concrete message types.
/// It maintains a mapping of type names to deserialization functions.
pub struct MsgRegistry {
    unpack: BTreeMap<
        &'static str,
        Box<dyn Fn(&SerializableAny) -> Result<Box<dyn Msg>, InterLiquidSdkError> + Send + Sync>,
    >,
}

impl MsgRegistry {
    /// Creates a new empty message registry.
    ///
    /// # Returns
    /// A new `MsgRegistry` with no registered message types.
    pub fn new() -> Self {
        Self {
            unpack: BTreeMap::new(),
        }
    }

    /// Registers a message type for deserialization.
    ///
    /// This method registers the deserialization logic for a specific message type.
    /// Once registered, messages of this type can be unpacked from `SerializableAny`.
    ///
    /// # Type Parameters
    /// - `T`: The message type that must implement both `Msg` and `NamedSerializableType`
    pub fn register<T: Msg + NamedSerializableType>(&mut self) {
        let name = T::type_name();

        self.unpack.insert(
            name,
            Box::new(|any| {
                let msg = T::try_from_slice(&any.value)?;

                Ok(Box::new(msg))
            }),
        );
    }

    /// Unpacks a `SerializableAny` into a concrete message type.
    ///
    /// This method deserializes the provided `SerializableAny` into its original
    /// message type based on the type name stored in the `SerializableAny`.
    ///
    /// # Parameters
    /// - `any`: The serialized message to unpack
    ///
    /// # Returns
    /// - `Ok(Box<dyn Msg>)` containing the deserialized message
    /// - `Err(InterLiquidSdkError::NotFound)` if the message type is not registered
    /// - `Err(InterLiquidSdkError)` if deserialization fails
    pub fn unpack(&self, any: &SerializableAny) -> Result<Box<dyn Msg>, InterLiquidSdkError> {
        let name = any.type_.as_str();

        let unpack = self
            .unpack
            .get(name)
            .ok_or(InterLiquidSdkError::NotFound(anyhow!(
                "msg type not registered"
            )))?;

        Ok(unpack(any)?)
    }
}
