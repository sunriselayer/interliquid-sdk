use crate::{core::IdentifiableTrait, types::InterLiquidSdkError};

pub trait VerifyingKey: IdentifiableTrait {
    fn verify(&self, msg: &[u8], sig: &[u8]) -> Result<(), InterLiquidSdkError>;
}

impl<K: VerifyingKey> IdentifiableTrait for K {
    fn identifier(&self) -> &'static str {
        "verifying_key"
    }
}

pub struct VerifyingKeyTraitImpl;

impl VerifyingKey for VerifyingKeyTraitImpl {
    fn verify(&self, _msg: &[u8], _sig: &[u8]) -> Result<(), InterLiquidSdkError> {
        unimplemented!()
    }
}
