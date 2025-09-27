use thiserror::Error;

pub type Result<T> = std::result::Result<T, BlockchainError>;

#[derive(Error, Debug)]
pub enum BlockchainError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid block: {message}")]
    InvalidBlock { message: String },

    #[error("Chain validation failed: {message}")]
    ChainValidation { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Empty blockchain")]
    EmptyBlockchain,

    #[error("Invalid transaction: {message}")]
    InvalidTransaction { message: String },
}