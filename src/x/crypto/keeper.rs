use std::{any::Any, collections::BTreeMap};

use anyhow::anyhow;

use crate::{
    core::Context,
    types::{InterLiquidSdkError, NamedSerializableType, SerializableAny},
};

use super::verifying_key::VerifyingKey;

pub trait CryptoKeeperI {
    fn register_verifying_key<T: VerifyingKey + NamedSerializableType>(
        &mut self,
        downcast: Box<
            dyn Fn(Box<dyn Any>) -> Result<Box<dyn VerifyingKey>, InterLiquidSdkError>
                + Send
                + Sync,
        >,
    ) -> Result<(), InterLiquidSdkError>;

    fn unpack_verifying_key(
        &self,
        ctx: &dyn Context,
        any: &SerializableAny,
    ) -> Result<Box<dyn VerifyingKey>, InterLiquidSdkError>;
}

pub struct CryptoKeeper {
    verifying_key_types: BTreeMap<
        &'static str,
        Box<
            dyn Fn(Box<dyn Any>) -> Result<Box<dyn VerifyingKey>, InterLiquidSdkError>
                + Send
                + Sync,
        >,
    >,
}

impl CryptoKeeperI for CryptoKeeper {
    fn register_verifying_key<T: VerifyingKey + NamedSerializableType>(
        &mut self,
        downcast: Box<
            dyn Fn(Box<dyn Any>) -> Result<Box<dyn VerifyingKey>, InterLiquidSdkError>
                + Send
                + Sync,
        >,
    ) -> Result<(), InterLiquidSdkError> {
        let name = T::type_name();

        if self.verifying_key_types.contains_key(name) {
            return Err(InterLiquidSdkError::AlreadyExists(anyhow!(
                "verifying key type already registered"
            )));
        }

        self.verifying_key_types.insert(name, downcast);

        Ok(())
    }

    fn unpack_verifying_key(
        &self,
        ctx: &dyn Context,
        any: &SerializableAny,
    ) -> Result<Box<dyn VerifyingKey>, InterLiquidSdkError> {
        let name = any.type_.as_str();

        let downcast = self
            .verifying_key_types
            .get(name)
            .ok_or(InterLiquidSdkError::NotFound(anyhow!(
                "type not registered"
            )))?;

        let instance = ctx.type_registry().unpack_any(any)?;
        let casted = downcast(instance)?;

        Ok(casted)
    }
}
