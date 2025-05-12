use std::{any::Any, collections::BTreeMap};

use crate::types::{InterLiquidSdkError, NamedSerializableType};

use super::{Context, Msg};

pub struct MsgHandlerRegistry {
    handlers: BTreeMap<
        &'static str,
        Box<dyn Fn(&mut dyn Context, &dyn Any) -> Result<(), InterLiquidSdkError> + Send + Sync>,
    >,
}

impl MsgHandlerRegistry {
    pub fn new() -> Self {
        Self {
            handlers: BTreeMap::new(),
        }
    }

    pub fn register<T: Msg + NamedSerializableType>(
        &mut self,
        handler: Box<dyn Fn(&mut dyn Context, &T) -> Result<(), InterLiquidSdkError> + Send + Sync>,
    ) {
        let name = T::type_name();

        self.handlers.insert(
            name,
            Box::new(move |ctx, any| {
                let msg = any.downcast_ref::<T>().unwrap();
                handler(ctx, msg)
            }),
        );
    }

    pub fn get(
        &self,
        name: &str,
    ) -> Option<
        &Box<dyn Fn(&mut dyn Context, &dyn Any) -> Result<(), InterLiquidSdkError> + Send + Sync>,
    > {
        self.handlers.get(name)
    }
}
