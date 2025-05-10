use std::marker::PhantomData;

use crate::core::{Context, Module, MsgHandlerRegistry, MsgRegistry};

use super::{
    keeper::{CryptoKeeper, CryptoKeeperI},
    p256::VerifyingKeyP256,
};

pub struct CryptoModule<C: Context> {
    keeper: CryptoKeeper<C>,
    phantom: PhantomData<C>,
}

impl<C: Context> CryptoModule<C> {
    pub fn new(mut keeper: CryptoKeeper<C>) -> Self {
        keeper.register_verifying_key::<VerifyingKeyP256>().unwrap();

        Self {
            keeper,
            phantom: PhantomData,
        }
    }
}

impl<C: Context> Module<C> for CryptoModule<C> {
    fn register_msgs(
        &'static self,
        _msg_registry: &mut MsgRegistry,
        _msg_handler_registry: &mut MsgHandlerRegistry<C>,
    ) {
    }
}
