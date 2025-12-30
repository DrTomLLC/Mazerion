// crates/ffi/src/error.rs
// Typed error mapping for FFI boundary

use mazerion_core::Error as CoreError;

#[derive(Debug, thiserror::Error, uniffi::Error)]
#[uniffi(flat_error)]
pub enum MazerionError {
    #[error("Invalid input: {msg}")]
    InvalidInput { msg: String },

    #[error("Calculator not found")]
    CalculatorNotFound,

    #[error("Calculation failed: {msg}")]
    CalculationFailed { msg: String },

    #[error("Validation error: {msg}")]
    ValidationError { msg: String },

    #[error("Database error: {msg}")]
    DatabaseError { msg: String },

    #[error("System not ready")]
    SystemNotReady,

    #[error("Input size limit exceeded")]
    InputTooLarge,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

impl MazerionError {
    pub fn from_core_error(err: CoreError) -> Self {
        match err {
            CoreError::Validation(msg) => Self::ValidationError { msg },
            CoreError::OutOfRange(msg) => Self::ValidationError { msg },
            CoreError::MissingInput(msg) => Self::InvalidInput { msg },
            CoreError::Calculation(msg) => Self::CalculationFailed { msg },
            CoreError::Parse(msg) => Self::InvalidInput { msg },
            CoreError::Io(msg) => Self::CalculationFailed { msg },
            CoreError::Config(msg) => Self::CalculationFailed { msg },
            CoreError::DatabaseError(msg) => Self::DatabaseError { msg },
        }
    }
}