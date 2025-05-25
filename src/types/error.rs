use thiserror::Error;

use crate::trie::NibblePatriciaTrieError;

/// The main error type for the InterLiquid SDK.
/// This enum encompasses all possible errors that can occur within the SDK.
#[derive(Debug, Error)]
pub enum InterLiquidSdkError {
    // Core
    #[error("Module already loaded")]
    ModuleAlreadyLoaded,

    // General
    #[error("Invalid request")]
    InvalidRequest(anyhow::Error),
    #[error("Not found")]
    NotFound(anyhow::Error),
    #[error("Already exists")]
    AlreadyExists(anyhow::Error),
    #[error("Unauthorized")]
    Unauthorized(anyhow::Error),

    // Token
    #[error("Invalid denom")]
    InvalidDenom,
    #[error("Zero amount")]
    ZeroAmount,
    #[error("Denom mismatch")]
    DenomMismatch,
    #[error("Overflow")]
    Overflow,
    #[error("Underflow")]
    Underflow,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Insufficient balance")]
    InsufficientBalance,

    // Store
    #[error("Accessing unrelated state")]
    UnrelatedState,

    // Trie
    #[error("Trie error")]
    Trie(#[from] NibblePatriciaTrieError),

    // IO
    #[error("IO error")]
    Io(#[from] std::io::Error),

    // SEC1
    #[error("SEC1")]
    Sec1,

    // P256
    #[error("P256")]
    P256Key(#[from] crate::p256::ecdsa::Error),

    // Other
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
