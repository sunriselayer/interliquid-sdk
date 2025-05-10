use thiserror::Error;

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
    #[error("Insufficient balance")]
    InsufficientBalance,

    // Store
    #[error("Accessing unrelated state")]
    UnrelatedState,

    // IO
    #[error("IO error")]
    Io(#[from] std::io::Error),

    // SEC1
    #[error("SEC1")]
    Sec1,

    // P256
    #[error("P256")]
    P256Key(#[from] p256::ecdsa::Error),

    // Other
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
