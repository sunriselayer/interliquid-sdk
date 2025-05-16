use std::sync::Arc;

use crate::core::{Module, MsgHandlerRegistry, MsgRegistry};

use super::keeper::CryptoKeeper;

pub struct CryptoModule {
    keeper: Arc<CryptoKeeper>,
}

impl CryptoModule {
    pub fn new(keeper: Arc<CryptoKeeper>) -> Self {
        Self { keeper }
    }

    pub fn keeper(&self) -> &CryptoKeeper {
        &self.keeper
    }
}

impl Module for CryptoModule {
    fn register_msgs(
        self: Arc<Self>,
        _msg_registry: &mut MsgRegistry,
        _msg_handler_registry: &mut MsgHandlerRegistry,
    ) {
    }
}
