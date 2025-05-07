use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterLiquidSdkError {
    #[error("Key not found")]
    KeyNotFound,
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
