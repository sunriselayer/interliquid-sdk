use crate::types::InterLiquidSdkError;

/// Trait for cryptographic signature verification.
/// 
/// This trait defines the interface that all verifying key implementations
/// must provide. It allows different cryptographic algorithms to be used
/// interchangeably within the system.
pub trait VerifyingKey {
    /// Verifies a signature against a message.
    /// 
    /// # Arguments
    /// 
    /// * `msg` - The message bytes that were signed
    /// * `sig` - The signature bytes to verify against the message
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if the signature is valid for the given message,
    /// or an error if verification fails or the inputs are malformed.
    fn verify(&self, msg: &[u8], sig: &[u8]) -> Result<(), InterLiquidSdkError>;
}
