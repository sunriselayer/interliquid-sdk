use std::{any::Any, collections::BTreeMap};

use crate::{tx::Msg, types::InterLiquidSdkError};

use super::Context;

pub struct MsgRegistry {
    handlers: BTreeMap<
        &'static str,
        Box<dyn Fn(&mut dyn Context, &dyn Any) -> Result<(), InterLiquidSdkError>>,
    >,
}

impl MsgRegistry {
    pub fn new() -> Self {
        Self {
            handlers: BTreeMap::new(),
        }
    }

    pub fn register<T: Msg>(
        &mut self,
        handler: Box<dyn Fn(&mut dyn Context, &T) -> Result<(), InterLiquidSdkError>>,
    ) {
        self.handlers.insert(
            T::type_name(),
            Box::new(move |ctx, any| {
                let msg = any.downcast_ref::<T>().unwrap();
                handler(ctx, msg)
            }),
        );
    }
}
