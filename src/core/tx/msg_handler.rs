use std::{any::Any, collections::BTreeMap};

use crate::{
    core::Context,
    types::{InterLiquidSdkError, NamedSerializableType},
};

use super::Msg;

/// The registry for the handlers of Tx's Msg executions.
///
/// This registry maintains a mapping between message type names and their
/// corresponding execution handlers. It allows the system to dynamically
/// dispatch message execution based on the message type at runtime.
pub struct MsgHandlerRegistry {
    handlers: BTreeMap<
        &'static str,
        Box<dyn Fn(&mut dyn Context, &dyn Any) -> Result<(), InterLiquidSdkError> + Send + Sync>,
    >,
}

impl MsgHandlerRegistry {
    /// Creates a new empty message handler registry.
    ///
    /// # Returns
    /// A new `MsgHandlerRegistry` with no registered handlers.
    pub fn new() -> Self {
        Self {
            handlers: BTreeMap::new(),
        }
    }

    /// Registers a handler for a specific message type.
    ///
    /// This method associates a message type with its execution handler.
    /// The handler will be called when a message of type `T` needs to be executed.
    ///
    /// # Type Parameters
    /// - `T`: The message type that must implement both `Msg` and `NamedSerializableType`
    ///
    /// # Parameters
    /// - `handler`: A boxed function that processes messages of type `T`
    pub fn register<T: Msg + NamedSerializableType>(
        &mut self,
        handler: Box<dyn Fn(&mut dyn Context, &T) -> Result<(), InterLiquidSdkError> + Send + Sync>,
    ) {
        let name = T::TYPE_NAME;

        self.handlers.insert(
            name,
            Box::new(move |ctx, any| {
                let msg = any.downcast_ref::<T>().unwrap();
                handler(ctx, msg)
            }),
        );
    }

    /// Retrieves a handler for the specified message type name.
    ///
    /// # Parameters
    /// - `name`: The type name of the message
    ///
    /// # Returns
    /// - `Some(&handler)` if a handler is registered for the given name
    /// - `None` if no handler is registered for the given name
    pub fn get(
        &self,
        name: &str,
    ) -> Option<
        &Box<dyn Fn(&mut dyn Context, &dyn Any) -> Result<(), InterLiquidSdkError> + Send + Sync>,
    > {
        self.handlers.get(name)
    }
}
