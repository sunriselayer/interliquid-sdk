use std::sync::Arc;

use crate::core::{Module, MsgHandlerRegistry, MsgRegistry};

use super::NftKeeper;

pub struct NftModule {
    keeper: Arc<NftKeeper>,
}

impl NftModule {
    pub fn new(keeper: Arc<NftKeeper>) -> Self {
        Self { keeper }
    }

    pub fn keeper(&self) -> &NftKeeper {
        &self.keeper
    }
}

impl Module for NftModule {
    fn register_msgs(
        self: Arc<Self>,
        _msg_registry: &mut MsgRegistry,
        _msg_handler_registry: &mut MsgHandlerRegistry,
    ) {
    }
}
