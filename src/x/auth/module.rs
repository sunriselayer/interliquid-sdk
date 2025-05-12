use std::sync::Arc;

use crate::core::{Module, MsgHandlerRegistry, MsgRegistry};

use super::keeper::AuthKeeper;

pub struct AuthModule {
    _keeper: AuthKeeper,
}

impl AuthModule {
    pub fn new(_keeper: AuthKeeper) -> Self {
        Self { _keeper }
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
