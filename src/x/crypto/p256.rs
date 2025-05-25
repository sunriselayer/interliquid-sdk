use crate::p256::ecdsa::signature::Verifier;
use crate::p256::ecdsa::{Signature, VerifyingKey as P256VerifyingKey};
use crate::p256::EncodedPoint;
use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::types::{InterLiquidSdkError, NamedSerializableType};

use super::verifying_key::VerifyingKey;

/// P256 elliptic curve verifying key implementation.
/// 
/// This struct wraps a P256 public key in compressed format (33 bytes)
/// and provides signature verification functionality using the ECDSA
/// algorithm on the P256 curve.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct VerifyingKeyP256 {
    /// The compressed public key bytes (33 bytes)
    key: [u8; 33],
}

impl VerifyingKeyP256 {
    /// Creates a new `VerifyingKeyP256` from compressed public key bytes.
    /// 
    /// # Arguments
    /// 
    /// * `key` - A 33-byte array containing the compressed P256 public key
    /// 
    /// # Returns
    /// 
    /// Returns a new `VerifyingKeyP256` instance.
    pub fn new(key: [u8; 33]) -> Self {
        Self { key }
    }
}

impl VerifyingKey for VerifyingKeyP256 {
    /// Verifies a signature against a message using P256 ECDSA.
    /// 
    /// # Arguments
    /// 
    /// * `msg` - The message bytes that were signed
    /// * `sig` - The signature bytes to verify
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if the signature is valid, or an error if verification
    /// fails or the key/signature format is invalid.
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
    /// Returns the type name identifier for P256 verifying keys.
    /// 
    /// # Returns
    /// 
    /// Returns "verifying_key_p256" as the type identifier.
    const TYPE_NAME: &'static str = "verifying_key_p256";
}
