use crate::core::{Context, Module, MsgHandlerRegistry, MsgRegistry};

use super::{msg_send::MsgSend, BankKeeper};

pub struct BankModule<C: Context> {
    keeper: BankKeeper<C>,
}

impl<C: Context> BankModule<C> {
    pub fn new(keeper: BankKeeper<C>) -> Self {
        Self { keeper }
    }
}

impl<C: Context> Module<C> for BankModule<C> {
    fn register_msgs(
        &'static self,
        msg_registry: &mut MsgRegistry,
        msg_handler_registry: &mut MsgHandlerRegistry<C>,
    ) {
        msg_registry.register::<MsgSend>();
        msg_handler_registry
            .register::<MsgSend>(Box::new(move |ctx, msg| self.keeper.msg_send(ctx, msg)));
    }
}
