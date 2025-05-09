use std::{any::Any, collections::BTreeMap};

use anyhow::anyhow;

use crate::types::{InterLiquidSdkError, NamedSerializableType, SerializableAny};

pub struct TypeRegistry {
    unpack_any: BTreeMap<
        &'static str,
        Box<dyn Fn(&SerializableAny) -> Result<Box<dyn Any>, InterLiquidSdkError> + Send + Sync>,
    >,
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self {
            unpack_any: BTreeMap::new(),
        }
    }

    pub fn register<T: NamedSerializableType>(&mut self) {
        self.unpack_any.insert(
            T::type_name(),
            Box::new(|any| T::unpack_any(any).map(Box::new).map(|b| b as Box<dyn Any>)),
        );
    }

    pub fn unpack_any(&self, any: &SerializableAny) -> Result<Box<dyn Any>, InterLiquidSdkError> {
        let name = any.type_.as_str();

        let unpack_any = self
            .unpack_any
            .get(name)
            .ok_or(InterLiquidSdkError::NotFound(anyhow!(
                "type not registered"
            )))?;

        unpack_any(any)
    }
}
