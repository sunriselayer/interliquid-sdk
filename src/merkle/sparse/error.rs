use thiserror::Error;

#[derive(Debug, Error)]
pub enum OctRadSparseTreeError {
    #[error("Invalid proof")]
    InvalidProof(anyhow::Error),
    #[error("Empty key set")]
    EmptyKeySet,
    #[error("Empty key suffix")]
    EmptyKeySuffix,
}
