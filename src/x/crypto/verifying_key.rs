use crate::types::InterLiquidSdkError;

pub trait VerifyingKey {
    fn verify(&self, msg: &[u8], sig: &[u8]) -> Result<(), InterLiquidSdkError>;
}
