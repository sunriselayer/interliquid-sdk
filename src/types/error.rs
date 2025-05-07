use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterLiquidSdkError {
    #[error("Accessing unrelated state")]
    UnrelatedState,
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
