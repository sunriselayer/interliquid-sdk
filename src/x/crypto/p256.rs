use borsh_derive::{BorshDeserialize, BorshSerialize};
use p256::ecdsa::signature::Verifier;
use p256::ecdsa::{Signature, VerifyingKey as P256VerifyingKey};
use p256::EncodedPoint;

use crate::types::{InterLiquidSdkError, NamedSerializableType};

use super::key::VerifyingKey;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct VerifyingKeyP256 {
    key: [u8; 33],
}

impl VerifyingKeyP256 {
    pub fn new(key: [u8; 33]) -> Self {
        Self { key }
    }
}

impl VerifyingKey for VerifyingKeyP256 {
    fn verify(&self, msg: &[u8], sig: &[u8]) -> Result<(), InterLiquidSdkError> {
        let encoded_point =
            EncodedPoint::from_bytes(&self.key).map_err(|_| InterLiquidSdkError::Sec1)?;
        let key = P256VerifyingKey::from_encoded_point(&encoded_point)?;
        let signature = Signature::from_bytes(sig.into()).map_err(|_| InterLiquidSdkError::Sec1)?;

        key.verify(msg, &signature)?;

        Ok(())
    }
}

impl NamedSerializableType for VerifyingKeyP256 {
    fn type_name() -> &'static str {
        "verifying_key_p256"
    }
}
