use std::sync::Arc;

use crate::core::{Module, MsgHandlerRegistry, MsgRegistry};

use super::keeper::AuthKeeper;

pub struct AuthModule<'a> {
    keeper: AuthKeeper<'a>,
}

impl<'a> AuthModule<'a> {
    pub fn new(keeper: AuthKeeper<'a>) -> Self {
        Self { keeper }
    }

    pub fn keeper(&self) -> &AuthKeeper<'a> {
        &self.keeper
    }
}

impl<'a> Module for AuthModule<'a> {
    fn register_msgs(
        self: Arc<Self>,
        _msg_registry: &mut MsgRegistry,
        _msg_handler_registry: &mut MsgHandlerRegistry,
    ) {
    }
}
