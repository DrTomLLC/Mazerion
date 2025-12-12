//! Memory management for FFI

use crate::error::MazerionResult;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

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
pub unsafe extern "C" fn mazerion_free_result(result: MazerionResult) {
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
    match CString::new("0.10.4") {
        Ok(s) => s.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}