use thiserror::Error;

#[derive(Error, Debug)]
pub enum FastFindError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid regex pattern: {0}")]
    Regex(#[from] regex::Error),

    #[error("Invalid UTF-8 in file: {0}")]
    InvalidUtf8(String),

    #[error("Binary file detected: {0}")]
    BinaryFile(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

pub type Result<T> = std::result::Result<T, FastFindError>; 