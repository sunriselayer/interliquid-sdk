use crate::core::{Module, MsgRegistry, TypeRegistry};

use super::keeper::AuthKeeper;

pub struct AuthModule {
    keeper: AuthKeeper,
}

impl AuthModule {
    pub fn new(keeper: AuthKeeper) -> Self {
        Self { keeper }
    }
}

impl Module for AuthModule {
    fn register_types(&self, type_registry: &mut TypeRegistry) {}

    fn register_msgs(&'static self, msg_registry: &mut MsgRegistry) {}
}
