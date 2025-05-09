use crate::core::{Module, MsgRegistry, TypeRegistry};

use super::{keeper::CryptoKeeper, p256::VerifyingKeyP256};

pub struct CryptoModule {
    keeper: CryptoKeeper,
}

impl CryptoModule {
    pub fn new(keeper: CryptoKeeper) -> Self {
        Self { keeper }
    }
}

impl Module for CryptoModule {
    fn register_types(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<VerifyingKeyP256>();
    }

    fn register_msgs(&'static self, _msg_registry: &mut MsgRegistry) {}
}
