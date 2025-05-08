use std::{any::Any, collections::BTreeMap};

use crate::types::{InterLiquidSdkError, NamedSerializableType, SerializableAny};

pub struct TypeRegistry {
    from_any: BTreeMap<
        &'static str,
        Box<dyn Fn(SerializableAny) -> Result<Box<dyn Any>, InterLiquidSdkError> + Send + Sync>,
    >,
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self {
            from_any: BTreeMap::new(),
        }
    }

    pub fn register<T: NamedSerializableType>(&mut self) {
        self.from_any.insert(
            T::type_name(),
            Box::new(|any| T::from_any(any).map(Box::new).map(|b| b as Box<dyn Any>)),
        );
    }
}
