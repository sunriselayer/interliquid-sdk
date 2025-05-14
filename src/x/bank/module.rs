use std::sync::Arc;

use crate::core::{Module, MsgHandlerRegistry, MsgRegistry};

use super::{msg_send::MsgSend, BankKeeper};

pub struct BankModule {
    keeper: Arc<BankKeeper>,
}

impl BankModule {
    pub fn new(keeper: Arc<BankKeeper>) -> Self {
        Self { keeper }
    }

    pub fn keeper(&self) -> &BankKeeper {
        &self.keeper
    }
}

impl Module for BankModule {
    fn register_msgs(
        self: Arc<Self>,
        msg_registry: &mut MsgRegistry,
        msg_handler_registry: &mut MsgHandlerRegistry,
    ) {
        msg_registry.register::<MsgSend>();

        let module = self.clone();
        msg_handler_registry
            .register::<MsgSend>(Box::new(move |ctx, msg| module.keeper.msg_send(ctx, msg)));
    }
}
