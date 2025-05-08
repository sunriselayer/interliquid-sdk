use crate::core::{Module, MsgRegistry, TypeRegistry};

use super::{key::VerifyingKeyTraitImpl, p256::VerifyingKeyP256};

pub struct CryptoModule {}

impl CryptoModule {
    pub fn new() -> Self {
        Self {}
    }
}

impl Module for CryptoModule {
    fn register_types(&self, type_registry: &mut TypeRegistry) {
        type_registry
            .register_trait(&VerifyingKeyTraitImpl)
            .unwrap();

        type_registry
            .register_impl::<VerifyingKeyP256>(&VerifyingKeyTraitImpl)
            .unwrap();
    }

    fn register_msgs(&'static self, _msg_registry: &mut MsgRegistry) {}
}
