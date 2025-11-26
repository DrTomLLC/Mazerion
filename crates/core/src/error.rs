//! Error types for Mazerion.

use thiserror::Error;

/// Main error type.
#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Out of range: {0}")]
    OutOfRange(String),

    #[error("Missing input: {0}")]
    MissingInput(String),

    #[error("Calculation error: {0}")]
    Calculation(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Database error: {0}")]
    DatabaseError(String),
}

/// Result type alias.
pub type Result<T> = std::result::Result<T, Error>;
