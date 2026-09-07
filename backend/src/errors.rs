use anyhow::Error as AnyError;


#[derive(thiserror::Error, Debug)]
pub enum SeceretDropError {
    #[error("Secret Not Found")]
    NotFound,

    #[error("Secret has expired")]
    Expired,

    #[error("Decryption failed")]
    DecryptionFailed,

    #[error("Validation Error: {0}")]
    ValidationError(String),

    #[error("Rate limit exceeded")]
    RateLimited,

    #[error("Internal Server Error")]
    InternalError(#[from] AnyError),
}
