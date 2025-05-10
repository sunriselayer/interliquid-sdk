use thiserror::Error;

#[derive(Debug, Error)]
pub enum OctRadSparseTreeError {
    #[error("Invalid proof")]
    InvalidProof(anyhow::Error),
}
