use crate::core::{Module, MsgRegistry, TypeRegistry};

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
    fn register_types(&self, _type_registry: &mut TypeRegistry) {}

    fn register_msgs(&'static self, _msg_registry: &mut MsgRegistry) {}
}
