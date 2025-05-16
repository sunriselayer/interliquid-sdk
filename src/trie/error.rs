use thiserror::Error;

#[derive(Debug, Error)]
pub enum NibblePatriciaTrieError {
    // Trie
    #[error("Not found")]
    NotFound,
    #[error("Invalid node")]
    InvalidNode,
    #[error("Invalid hash")]
    InvalidHash,
    #[error("Empty proof")]
    EmptyProof,
    #[error("Invalid proof")]
    InvalidProof,

    // IO
    #[error("IO error")]
    Io(#[from] std::io::Error),

    // Other
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
