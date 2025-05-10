use std::marker::PhantomData;

use anyhow::anyhow;

use crate::tx::{Tx, TxAnteHandler, TxPostHandler};
use crate::types::InterLiquidSdkError;

use super::{Context, Module, MsgHandlerRegistry, MsgRegistry};

pub struct App<C: Context, TX: Tx> {
    modules: Vec<Box<dyn Module<C>>>,
    tx_ante_handlers: Vec<Box<dyn TxAnteHandler<C, TX>>>,
    tx_post_handlers: Vec<Box<dyn TxPostHandler<C, TX>>>,
    msg_registry: MsgRegistry,
    msg_handler_registry: MsgHandlerRegistry<C>,
    phantom: PhantomData<(C, TX)>,
}

impl<C: Context, TX: Tx> App<C, TX> {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            tx_ante_handlers: Vec::new(),
            tx_post_handlers: Vec::new(),
            msg_registry: MsgRegistry::new(),
            msg_handler_registry: MsgHandlerRegistry::new(),
            phantom: PhantomData,
        }
    }

    pub fn load(
        &'static mut self,
        modules: Vec<Box<dyn Module<C>>>,
        tx_ante_handlers: Vec<Box<dyn TxAnteHandler<C, TX>>>,
        tx_post_handlers: Vec<Box<dyn TxPostHandler<C, TX>>>,
    ) -> Result<(), InterLiquidSdkError> {
        if !self.modules.is_empty() {
            return Err(InterLiquidSdkError::ModuleAlreadyLoaded);
        }
        self.modules = modules;

        for module in self.modules.iter_mut() {
            module.register_msgs(&mut self.msg_registry, &mut self.msg_handler_registry);
        }
        self.tx_ante_handlers = tx_ante_handlers;
        self.tx_post_handlers = tx_post_handlers;

        Ok(())
    }

    pub fn execute_tx(&mut self, ctx: &mut C, tx: &TX) -> Result<(), InterLiquidSdkError> {
        for handler in self.tx_ante_handlers.iter_mut() {
            handler.handle(ctx, tx)?;
        }

        for msg in tx.msgs() {
            let type_name = msg.type_.as_str();
            let msg = self.msg_registry.unpack(&msg)?;
            let handler = self.msg_handler_registry.get(&type_name).ok_or(
                InterLiquidSdkError::InvalidRequest(anyhow!(
                    "msg handler not found: {}",
                    type_name
                )),
            )?;

            handler(ctx, &msg)?;
        }

        for handler in self.tx_post_handlers.iter_mut() {
            handler.handle(ctx, tx)?;
        }

        Ok(())
    }
}

pub trait AppI<C: Context, TX: Tx> {
    fn execute_tx(&mut self, ctx: &mut C, tx: &TX) -> Result<(), InterLiquidSdkError>;
}

impl<C: Context, TX: Tx> AppI<C, TX> for App<C, TX> {
    fn execute_tx(&mut self, ctx: &mut C, tx: &TX) -> Result<(), InterLiquidSdkError> {
        self.execute_tx(ctx, tx)
    }
}
