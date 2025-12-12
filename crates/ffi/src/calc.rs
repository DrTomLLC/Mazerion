//! Calculator operations for FFI

use crate::error::{MazerionError, MazerionResult};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::panic;

#[unsafe(no_mangle)]
pub extern "C" fn mazerion_list_calculators() -> MazerionResult {
    let result = panic::catch_unwind(|| {
        let calc_info = mazerion_api::list_calculators();

        let calc_list: Vec<serde_json::Value> = calc_info
            .iter()
            .map(|info| {
                serde_json::json!({
                    "id": info.id,
                    "name": info.name,
                    "description": info.description,
                    "category": info.category
                })
            })
            .collect();

        match serde_json::to_string(&calc_list) {
            Ok(json) => MazerionResult::success(&json),
            Err(e) => MazerionResult::error(MazerionError::json_error(&e.to_string())),
        }
    });

    match result {
        Ok(r) => r,
        Err(_) => MazerionResult::error(MazerionError::panic_caught("Panic in list")),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mazerion_calculate(
    calculator_id: *const c_char,
    json_input: *const c_char,
) -> MazerionResult {
    let result = panic::catch_unwind(|| {
        if calculator_id.is_null() {
            return MazerionResult::error(MazerionError::new(
                1,
                "Calculator ID pointer is null",
            ));
        }
        if json_input.is_null() {
            return MazerionResult::error(MazerionError::new(2, "JSON input pointer is null"));
        }

        // SAFETY: Caller must ensure calculator_id is a valid null-terminated C string
        let calc_id = match unsafe { CStr::from_ptr(calculator_id) }.to_str() {
            Ok(s) => s,
            Err(_) => {
                return MazerionResult::error(MazerionError::new(
                    3,
                    "Invalid UTF-8 in calculator ID",
                ))
            }
        };

        // SAFETY: Caller must ensure json_input is a valid null-terminated C string
        let json_str = match unsafe { CStr::from_ptr(json_input) }.to_str() {
            Ok(s) => s,
            Err(_) => {
                return MazerionResult::error(MazerionError::new(
                    4,
                    "Invalid UTF-8 in JSON input",
                ))
            }
        };

        let request: mazerion_api::ApiRequest = match serde_json::from_str(json_str) {
            Ok(r) => r,
            Err(e) => {
                return MazerionResult::error(MazerionError::json_error(&format!(
                    "Failed to parse JSON input: {}",
                    e
                )))
            }
        };

        if request.calculator_id != calc_id {
            return MazerionResult::error(MazerionError::new(
                5,
                "Calculator ID mismatch between parameter and JSON",
            ));
        }

        match mazerion_api::execute_calculation(request) {
            Ok(response) => match serde_json::to_string(&response) {
                Ok(json) => MazerionResult::success(&json),
                Err(e) => MazerionResult::error(MazerionError::json_error(&format!(
                    "Failed to serialize result: {}",
                    e
                ))),
            },
            Err(e) => MazerionResult::error(MazerionError::calculation_error(&e.to_string())),
        }
    });

    match result {
        Ok(r) => r,
        Err(_) => MazerionResult::error(MazerionError::panic_caught("Panic in calculate")),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mazerion_get_categories() -> MazerionResult {
    let result = panic::catch_unwind(|| {
        let calc_info = mazerion_api::list_calculators();
        let mut categories = std::collections::HashMap::new();

        for info in calc_info {
            *categories.entry(info.category).or_insert(0) += 1;
        }

        match serde_json::to_string(&categories) {
            Ok(json) => MazerionResult::success(&json),
            Err(e) => MazerionResult::error(MazerionError::json_error(&e.to_string())),
        }
    });

    match result {
        Ok(r) => r,
        Err(_) => MazerionResult::error(MazerionError::panic_caught("Panic")),
    }
}