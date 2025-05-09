use crate::tx::{TxAnteHandler, TxPostHandler};
use crate::types::InterLiquidSdkError;

use super::msg_registry::MsgRegistry;
use super::type_registry::TypeRegistry;
use super::Module;

pub struct BaseApp {
    msg_registry: MsgRegistry,
    type_registry: TypeRegistry,
    modules: Vec<Box<dyn Module>>,
    tx_ante_handlers: Vec<Box<dyn TxAnteHandler>>,
    tx_post_handlers: Vec<Box<dyn TxPostHandler>>,
}

impl BaseApp {
    pub fn new() -> Self {
        Self {
            msg_registry: MsgRegistry::new(),
            type_registry: TypeRegistry::new(),
            modules: Vec::new(),
            tx_ante_handlers: Vec::new(),
            tx_post_handlers: Vec::new(),
        }
    }

    pub fn load(
        &'static mut self,
        modules: Vec<Box<dyn Module>>,
        tx_ante_handlers: Vec<Box<dyn TxAnteHandler>>,
        tx_post_handlers: Vec<Box<dyn TxPostHandler>>,
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
}
