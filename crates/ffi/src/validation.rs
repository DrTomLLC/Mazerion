// crates/ffi/src/validation.rs
// Input validation with security-critical bounds checking

use crate::error::MazerionError;
use crate::types::CalcParam;

// HARD SECURITY LIMITS
const MAX_CALCULATOR_ID_LEN: usize = 100;
const MAX_PARAM_KEY_LEN: usize = 100;
const MAX_PARAM_VALUE_LEN: usize = 1000;
const MAX_PARAMS_COUNT: usize = 50;
const MAX_BATCH_SIZE: usize = 100;

pub fn check_system_ready() -> Result<(), MazerionError> {
    let count = mazerion_core::traits::calculator_count();
    if count == 0 {
        return Err(MazerionError::SystemNotReady);
    }
    Ok(())
}

pub fn validate_calculator_id(id: &str) -> Result<(), MazerionError> {
    if id.is_empty() {
        return Err(MazerionError::InvalidInput {
            msg: "Calculator ID cannot be empty".to_string(),
        });
    }

    if id.len() > MAX_CALCULATOR_ID_LEN {
        return Err(MazerionError::InputTooLarge);
    }

    if !id.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(MazerionError::InvalidInput {
            msg: "Calculator ID contains invalid characters".to_string(),
        });
    }

    Ok(())
}

pub fn validate_params(params: &[CalcParam]) -> Result<(), MazerionError> {
    if params.len() > MAX_PARAMS_COUNT {
        return Err(MazerionError::InputTooLarge);
    }

    for param in params {
        if param.key.is_empty() {
            return Err(MazerionError::InvalidInput {
                msg: "Parameter key cannot be empty".to_string(),
            });
        }

        if param.key.len() > MAX_PARAM_KEY_LEN {
            return Err(MazerionError::InputTooLarge);
        }

        if param.value.len() > MAX_PARAM_VALUE_LEN {
            return Err(MazerionError::InputTooLarge);
        }
    }

    Ok(())
}

pub fn validate_batch_size(size: usize) -> Result<(), MazerionError> {
    if size == 0 {
        return Err(MazerionError::InvalidInput {
            msg: "Batch cannot be empty".to_string(),
        });
    }

    if size > MAX_BATCH_SIZE {
        return Err(MazerionError::InputTooLarge);
    }

    Ok(())
}