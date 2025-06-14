use thiserror::Error;

#[derive(Error, Debug)]
pub enum FastFindError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid regex pattern: {0}")]
    Regex(#[from] regex::Error),

    #[error("Binary file detected: {0}")]
    BinaryFile(String),
}

pub type Result<T> = std::result::Result<T, FastFindError>; 