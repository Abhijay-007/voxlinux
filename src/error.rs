
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IntentError {
    #[error("Failed to parse intent")]
    ParseError,

    #[error("Invalid parameters")]
    InvalidParameters,

    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Process failed")]
    ProcessFailed,
}
