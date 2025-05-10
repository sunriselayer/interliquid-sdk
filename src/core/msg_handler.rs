use std::{any::Any, collections::BTreeMap};

use crate::{
    tx::Msg,
    types::{InterLiquidSdkError, NamedSerializableType},
};

use super::Context;

pub struct MsgHandlerRegistry<C: Context> {
    handlers:
        BTreeMap<&'static str, Box<dyn Fn(&mut C, &dyn Any) -> Result<(), InterLiquidSdkError>>>,
}

impl<C: Context> MsgHandlerRegistry<C> {
    pub fn new() -> Self {
        Self {
            handlers: BTreeMap::new(),
        }
    }

    pub fn register<T: Msg + NamedSerializableType>(
        &mut self,
        handler: Box<dyn Fn(&mut C, &T) -> Result<(), InterLiquidSdkError>>,
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
    ) -> Option<&Box<dyn Fn(&mut C, &dyn Any) -> Result<(), InterLiquidSdkError>>> {
        self.handlers.get(name)
    }
}
