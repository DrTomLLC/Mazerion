//! FFI error types and result structures

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr;

#[repr(C)]
pub struct MazerionError {
    pub code: c_int,
    pub message: *mut c_char,
}

impl MazerionError {
    pub fn success() -> Self {
        Self {
            code: 0,
            message: ptr::null_mut(),
        }
    }

    pub fn new(code: c_int, message: &str) -> Self {
        let msg = match CString::new(message) {
            Ok(s) => s.into_raw(),
            Err(_) => {
                CString::new("Error creating error message")
                    .unwrap_or_else(|_| CString::new("").unwrap_or_default())
                    .into_raw()
            }
        };
        Self { code, message: msg }
    }

    pub fn json_error(msg: &str) -> Self {
        Self::new(6, msg)
    }

    pub fn panic_caught(msg: &str) -> Self {
        Self::new(7, msg)
    }

    pub fn calculator_not_found(id: &str) -> Self {
        Self::new(8, &format!("Calculator not found: {}", id))
    }

    pub fn calculation_error(msg: &str) -> Self {
        Self::new(9, msg)
    }
}

#[repr(C)]
pub struct MazerionResult {
    pub error: MazerionError,
    pub json_output: *mut c_char,
}

impl MazerionResult {
    pub fn success(json: &str) -> Self {
        let json_cstring = match CString::new(json) {
            Ok(s) => s.into_raw(),
            Err(_) => {
                return Self {
                    error: MazerionError::json_error("Invalid JSON output"),
                    json_output: ptr::null_mut(),
                }
            }
        };
        Self {
            error: MazerionError::success(),
            json_output: json_cstring,
        }
    }

    pub fn error(error: MazerionError) -> Self {
        Self {
            error,
            json_output: ptr::null_mut(),
        }
    }
}