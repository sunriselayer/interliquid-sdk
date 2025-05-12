use crate::core::{Module, MsgHandlerRegistry, MsgRegistry};

use super::{msg_send::MsgSend, BankKeeper};

pub struct BankModule {
    keeper: BankKeeper,
}

impl BankModule {
    pub fn new(keeper: BankKeeper) -> Self {
        Self { keeper }
    }
}

impl Module for BankModule {
    fn register_msgs(
        &'static self,
        msg_registry: &mut MsgRegistry,
        msg_handler_registry: &mut MsgHandlerRegistry,
    ) {
        msg_registry.register::<MsgSend>();
        msg_handler_registry
            .register::<MsgSend>(Box::new(|ctx, msg| self.keeper.msg_send(ctx, msg)));
    }
}
