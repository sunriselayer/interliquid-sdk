use std::collections::BTreeMap;

use anyhow::anyhow;

use crate::types::{InterLiquidSdkError, NamedSerializableType, SerializableAny};

use super::Msg;

// The registry for unpacking `SerializableAny` of Tx's Msg types.
pub struct MsgRegistry {
    unpack: BTreeMap<
        &'static str,
        Box<dyn Fn(&SerializableAny) -> Result<Box<dyn Msg>, InterLiquidSdkError> + Send + Sync>,
    >,
}

impl MsgRegistry {
    pub fn new() -> Self {
        Self {
            unpack: BTreeMap::new(),
        }
    }

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
