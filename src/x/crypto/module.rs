use std::sync::Arc;

use crate::core::{Module, MsgHandlerRegistry, MsgRegistry};

use super::{keeper::CryptoKeeper, p256::VerifyingKeyP256};

pub struct CryptoModule {
    keeper: CryptoKeeper,
}

impl CryptoModule {
    pub fn new(mut keeper: CryptoKeeper) -> Self {
        keeper.register_verifying_key::<VerifyingKeyP256>().unwrap();

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
