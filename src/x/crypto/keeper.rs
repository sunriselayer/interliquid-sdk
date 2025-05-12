use std::collections::BTreeMap;

use anyhow::anyhow;

use crate::types::{InterLiquidSdkError, NamedSerializableType, SerializableAny};

use super::verifying_key::VerifyingKey;

pub trait CryptoKeeperI {
    fn register_verifying_key<T: VerifyingKey + NamedSerializableType>(
        &mut self,
    ) -> Result<(), InterLiquidSdkError>;

    fn unpack_verifying_key(
        &self,
        any: &SerializableAny,
    ) -> Result<Box<dyn VerifyingKey>, InterLiquidSdkError>;
}

pub struct CryptoKeeper {
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
    pub fn new() -> Self {
        Self {
            unpack: BTreeMap::new(),
        }
    }
}

impl CryptoKeeperI for CryptoKeeper {
    fn register_verifying_key<T: VerifyingKey + NamedSerializableType>(
        &mut self,
    ) -> Result<(), InterLiquidSdkError> {
        let name = T::type_name();

        if self.unpack.contains_key(name) {
            return Err(InterLiquidSdkError::AlreadyExists(anyhow!(
                "verifying key type already registered"
            )));
        }

        self.unpack.insert(
            name,
            Box::new(|any| {
                let verifying_key = T::deserialize(&mut &any.value[..])?;

                Ok(Box::new(verifying_key))
            }),
        );

        Ok(())
    }

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
