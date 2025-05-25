use std::collections::BTreeMap;

use anyhow::anyhow;

use crate::types::{InterLiquidSdkError, NamedSerializableType, SerializableAny};

use super::verifying_key::VerifyingKey;

/// Trait defining the interface for the crypto keeper.
/// 
/// The crypto keeper is responsible for managing and unpacking different types
/// of verifying keys in the system.
pub trait CryptoKeeperI: Send + Sync {
    /// Unpacks a serialized verifying key from a `SerializableAny` container.
    /// 
    /// # Arguments
    /// 
    /// * `any` - A serialized container holding the verifying key data
    /// 
    /// # Returns
    /// 
    /// Returns a boxed trait object implementing `VerifyingKey` on success,
    /// or an error if the type is not registered or unpacking fails.
    fn unpack_verifying_key(
        &self,
        any: &SerializableAny,
    ) -> Result<Box<dyn VerifyingKey>, InterLiquidSdkError>;
}

/// The main crypto keeper implementation that manages verifying keys.
/// 
/// This struct maintains a registry of verifying key types and their
/// corresponding unpacking functions. It allows registration of new
/// verifying key types and provides functionality to unpack them from
/// serialized data.
pub struct CryptoKeeper {
    /// Map from type names to unpacking functions for verifying keys
    unpack: BTreeMap<
        &'static str,
        Box<
            dyn Fn(&SerializableAny) -> Result<Box<dyn VerifyingKey>, InterLiquidSdkError>
                + Send
                + Sync,
        >,
    >,
}

impl CryptoKeeper {
    /// Creates a new `CryptoKeeper` instance with an empty registry.
    /// 
    /// # Returns
    /// 
    /// Returns a new `CryptoKeeper` with no registered verifying key types.
    pub fn new() -> Self {
        Self {
            unpack: BTreeMap::new(),
        }
    }

    /// Registers a new verifying key type in the keeper.
    /// 
    /// This method adds a new verifying key type to the registry, allowing it
    /// to be unpacked from serialized data. The type must implement both
    /// `VerifyingKey` and `NamedSerializableType` traits.
    /// 
    /// # Type Parameters
    /// 
    /// * `T` - The verifying key type to register
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on successful registration, or an error if the type
    /// is already registered.
    pub fn register_verifying_key<T: VerifyingKey + NamedSerializableType>(
        &mut self,
    ) -> Result<(), InterLiquidSdkError> {
        let name = T::TYPE_NAME;

        if self.unpack.contains_key(name) {
            return Err(InterLiquidSdkError::AlreadyExists(anyhow!(
                "verifying key type already registered"
            )));
        }

        self.unpack.insert(
            name,
            Box::new(|any| {
                let verifying_key = T::try_from_slice(&any.value)?;

                Ok(Box::new(verifying_key))
            }),
        );

        Ok(())
    }
}

impl CryptoKeeperI for CryptoKeeper {
    /// Unpacks a verifying key from serialized data.
    /// 
    /// Looks up the appropriate unpacking function based on the type name
    /// in the `SerializableAny` container and uses it to deserialize the
    /// verifying key.
    /// 
    /// # Arguments
    /// 
    /// * `any` - The serialized container with type information and data
    /// 
    /// # Returns
    /// 
    /// Returns the unpacked verifying key as a trait object, or an error if
    /// the type is not registered or unpacking fails.
    fn unpack_verifying_key(
        &self,
        any: &SerializableAny,
    ) -> Result<Box<dyn VerifyingKey>, InterLiquidSdkError> {
        let name = any.type_.as_str();

        let unpack = self
            .unpack
            .get(name)
            .ok_or(InterLiquidSdkError::NotFound(anyhow!(
                "verifying key type not registered"
            )))?;

        Ok(unpack(any)?)
    }
}
