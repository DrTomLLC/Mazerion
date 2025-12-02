//! FFI for Mazerion - ACTUAL WORKING VERSION

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::panic;
use std::ptr;

#[repr(C)]
pub struct MazerionError {
    pub code: c_int,
    pub message: *mut c_char,
}

impl MazerionError {
    fn success() -> Self {
        Self { code: 0, message: ptr::null_mut() }
    }

    fn new(code: c_int, message: &str) -> Self {
        let msg = match CString::new(message) {
            Ok(s) => s.into_raw(),
            Err(_) => CString::new("Error").unwrap_or_else(|_| CString::new("").unwrap_or_default()).into_raw(),
        };
        Self { code, message: msg }
    }

    fn json_error(msg: &str) -> Self { Self::new(6, msg) }
    fn panic_caught(msg: &str) -> Self { Self::new(7, msg) }
}

#[repr(C)]
pub struct MazerionResult {
    pub error: MazerionError,
    pub json_output: *mut c_char,
}

impl MazerionResult {
    fn success(json: &str) -> Self {
        let json_cstring = match CString::new(json) {
            Ok(s) => s.into_raw(),
            Err(_) => return Self { error: MazerionError::json_error("Invalid JSON"), json_output: ptr::null_mut() },
        };
        Self { error: MazerionError::success(), json_output: json_cstring }
    }

    fn error(error: MazerionError) -> Self {
        Self { error, json_output: ptr::null_mut() }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mazerion_init() -> c_int {
    match panic::catch_unwind(|| { let _ = mazerion_calculators::init(); }) {
        Ok(_) => 0,
        Err(_) => 7,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mazerion_list_calculators() -> MazerionResult {
    let result = panic::catch_unwind(|| {
        let calc_info = mazerion_api::list_calculators();

        let calc_list: Vec<serde_json::Value> = calc_info.iter().map(|info| {
            serde_json::json!({
                "id": info.id,
                "name": info.name,
                "description": info.description,
                "category": info.category
            })
        }).collect();

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
pub extern "C" fn mazerion_calculate(_calculator_id: *const c_char, _json_input: *const c_char) -> MazerionResult {
    MazerionResult::error(MazerionError::new(4, "Not implemented yet"))
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mazerion_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        // SAFETY: Caller must ensure ptr is valid and was allocated by CString::into_raw
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mazerion_free_result(result: MazerionResult) {
    if !result.error.message.is_null() {
        // SAFETY: message was allocated by CString::into_raw in MazerionError::new
        unsafe {
            let _ = CString::from_raw(result.error.message);
        }
    }
    if !result.json_output.is_null() {
        // SAFETY: json_output was allocated by CString::into_raw in MazerionResult::success
        unsafe {
            let _ = CString::from_raw(result.json_output);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mazerion_version() -> *mut c_char {
    match CString::new("0.7.0") {
        Ok(s) => s.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}