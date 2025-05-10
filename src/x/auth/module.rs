use std::marker::PhantomData;

use crate::core::{Context, Module, MsgHandlerRegistry, MsgRegistry};

use super::keeper::AuthKeeper;

pub struct AuthModule<C: Context> {
    _keeper: AuthKeeper<C>,
    phantom: PhantomData<C>,
}

impl<C: Context> AuthModule<C> {
    pub fn new(_keeper: AuthKeeper<C>) -> Self {
        Self {
            _keeper,
            phantom: PhantomData,
        }
    }
}

impl<C: Context> Module<C> for AuthModule<C> {
    fn register_msgs(
        &'static self,
        _msg_registry: &mut MsgRegistry,
        _msg_handler_registry: &mut MsgHandlerRegistry<C>,
    ) {
    }
}
