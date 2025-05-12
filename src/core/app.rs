use std::marker::PhantomData;
use std::sync::Arc;

use anyhow::anyhow;

use crate::tx::{Tx, TxAnteHandler, TxPostHandler};
use crate::types::InterLiquidSdkError;

use super::{Context, Module, MsgHandlerRegistry, MsgRegistry};

pub struct App<TX: Tx> {
    tx_ante_handlers: Vec<Box<dyn TxAnteHandler<TX>>>,
    tx_post_handlers: Vec<Box<dyn TxPostHandler<TX>>>,
    msg_registry: MsgRegistry,
    msg_handler_registry: MsgHandlerRegistry,
    phantom: PhantomData<TX>,
}

impl<TX: Tx> App<TX> {
    pub fn new(
        modules: Vec<Arc<dyn Module>>,
        tx_ante_handlers: Vec<Box<dyn TxAnteHandler<TX>>>,
        tx_post_handlers: Vec<Box<dyn TxPostHandler<TX>>>,
    ) -> Self {
        let mut msg_registry = MsgRegistry::new();
        let mut msg_handler_registry = MsgHandlerRegistry::new();

        for module in modules.iter().cloned() {
            module.register_msgs(&mut msg_registry, &mut msg_handler_registry);
        }

        Self {
            tx_ante_handlers,
            tx_post_handlers,
            msg_registry,
            msg_handler_registry,
            phantom: PhantomData,
        }
    }

    pub fn execute_tx(&self, ctx: &mut dyn Context, tx: &[u8]) -> Result<(), InterLiquidSdkError> {
        let tx = TX::try_from_slice(tx)?;

        for handler in self.tx_ante_handlers.iter() {
            handler.handle(ctx, &self.msg_registry, &tx)?;
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

        for handler in self.tx_post_handlers.iter() {
            handler.handle(ctx, &self.msg_registry, &tx)?;
        }

        Ok(())
    }
}
