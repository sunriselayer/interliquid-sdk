use thiserror::Error;

#[derive(Debug, Error)]
pub enum OctRadPatriciaTrieError {
    #[error("Invalid proof")]
    InvalidProof(anyhow::Error),
}
