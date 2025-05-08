use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterLiquidSdkError {
    // Core
    #[error("Module already loaded")]
    ModuleAlreadyLoaded,

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

    // Other
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
