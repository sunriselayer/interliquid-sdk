use std::sync::Arc;

use crate::core::{Module, MsgHandlerRegistry, MsgRegistry};

use super::keeper::AuthKeeper;

pub struct AuthModule {
    keeper: Arc<AuthKeeper>,
}

impl AuthModule {
    pub fn new(keeper: Arc<AuthKeeper>) -> Self {
        Self { keeper }
    }

    pub fn keeper(&self) -> &AuthKeeper {
        &self.keeper
    }
}

impl Module for AuthModule {
    fn register_msgs(
        self: Arc<Self>,
        _msg_registry: &mut MsgRegistry,
        _msg_handler_registry: &mut MsgHandlerRegistry,
    ) {
    }
}
