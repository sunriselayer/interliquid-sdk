use crate::core::{Module, MsgRegistry, TypeRegistry};

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
    fn register_types(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<MsgSend>();
    }

    fn register_msgs(&'static self, msg_registry: &mut MsgRegistry) {
        msg_registry.register::<MsgSend>(Box::new(move |ctx, msg| self.keeper.msg_send(ctx, msg)));
    }
}
