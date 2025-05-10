use std::marker::PhantomData;

use crate::tx::{Tx, TxAnteHandler, TxPostHandler};
use crate::types::InterLiquidSdkError;

use super::msg_registry::MsgRegistry;
use super::type_registry::TypeRegistry;
use super::{Context, Module};

pub struct App<TX: Tx> {
    ctx: Box<dyn Context>,
    modules: Vec<Box<dyn Module>>,
    tx_ante_handlers: Vec<Box<dyn TxAnteHandler<TX>>>,
    tx_post_handlers: Vec<Box<dyn TxPostHandler<TX>>>,
    msg_registry: MsgRegistry,
    type_registry: TypeRegistry,
    phantom: PhantomData<TX>,
}

impl<TX: Tx> App<TX> {
    pub fn new(ctx: Box<dyn Context>) -> Self {
        Self {
            ctx,
            modules: Vec::new(),
            tx_ante_handlers: Vec::new(),
            tx_post_handlers: Vec::new(),
            msg_registry: MsgRegistry::new(),
            type_registry: TypeRegistry::new(),
            phantom: PhantomData,
        }
    }

    pub fn load(
        &'static mut self,
        modules: Vec<Box<dyn Module>>,
        tx_ante_handlers: Vec<Box<dyn TxAnteHandler<TX>>>,
        tx_post_handlers: Vec<Box<dyn TxPostHandler<TX>>>,
    ) -> Result<(), InterLiquidSdkError> {
        if !self.modules.is_empty() {
            return Err(InterLiquidSdkError::ModuleAlreadyLoaded);
        }
        self.modules = modules;

        for module in self.modules.iter_mut() {
            module.register_types(&mut self.type_registry);
            module.register_msgs(&mut self.msg_registry);
        }
        self.tx_ante_handlers = tx_ante_handlers;
        self.tx_post_handlers = tx_post_handlers;

        Ok(())
    }

    pub fn execute_tx(&mut self, tx: &TX) -> Result<(), InterLiquidSdkError> {
        for handler in self.tx_ante_handlers.iter_mut() {
            handler.handle(self.ctx.as_mut(), tx)?;
        }

        for handler in self.tx_post_handlers.iter_mut() {
            handler.handle(self.ctx.as_mut(), tx)?;
        }

        Ok(())
    }
}
