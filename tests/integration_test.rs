//! FFI Integration Tests

use mazerion_ffi::*;
use std::ffi::{CStr, CString};

#[test]
fn test_init() {
    let result = mazerion_init();
    assert_eq!(result, 0, "Initialization should succeed");
}

#[test]
fn test_version() {
    let version_ptr = mazerion_version();
    assert!(!version_ptr.is_null(), "Version should not be null");

    unsafe {
        let version = CStr::from_ptr(version_ptr).to_str().unwrap();
        assert!(version.contains("0.10"), "Version should be 0.10.x");
        mazerion_free_string(version_ptr);
    }
}

#[test]
fn test_list_calculators() {
    mazerion_init();

    let result = mazerion_list_calculators();
    assert_eq!(result.error.code, 0, "Should succeed");
    assert!(!result.json_output.is_null(), "Should return JSON");

    unsafe {
        let json = CStr::from_ptr(result.json_output).to_str().unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        assert!(parsed.len() > 0, "Should have calculators");

        // Verify structure
        let first = &parsed[0];
        assert!(first["id"].is_string());
        assert!(first["name"].is_string());
        assert!(first["description"].is_string());
        assert!(first["category"].is_string());

        mazerion_free_result(result);
    }
}

#[test]
fn test_get_categories() {
    mazerion_init();

    let result = mazerion_get_categories();
    assert_eq!(result.error.code, 0, "Should succeed");
    assert!(!result.json_output.is_null(), "Should return JSON");

    unsafe {
        let json = CStr::from_ptr(result.json_output).to_str().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(json).unwrap();
        assert!(parsed.is_object(), "Should be object");
        assert!(parsed.as_object().unwrap().len() > 0, "Should have categories");

        mazerion_free_result(result);
    }
}

#[test]
fn test_calculate_abv() {
    mazerion_init();

    let calc_id = CString::new("abv").unwrap();
    let json_input = CString::new(
        r#"{
            "calculator_id": "abv",
            "params": {
                "og": "1.090",
                "fg": "1.010"
            }
        }"#,
    )
        .unwrap();

    let result = mazerion_calculate(calc_id.as_ptr(), json_input.as_ptr());

    assert_eq!(result.error.code, 0, "Calculation should succeed");
    assert!(!result.json_output.is_null(), "Should return result");

    unsafe {
        let json = CStr::from_ptr(result.json_output).to_str().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(json).unwrap();

        assert!(parsed["value"].is_string());
        assert!(parsed["unit"].is_string());
        assert!(parsed["warnings"].is_array());
        assert!(parsed["metadata"].is_object());

        let abv: f64 = parsed["value"].as_str().unwrap().parse().unwrap();
        assert!(abv > 10.0 && abv < 11.0, "ABV should be ~10.5%");

        mazerion_free_result(result);
    }
}

#[test]
fn test_calculate_dilution() {
    mazerion_init();

    let calc_id = CString::new("dilution").unwrap();
    let json_input = CString::new(
        r#"{
            "calculator_id": "dilution",
            "params": {
                "current_volume": "20",
                "current_abv": "14",
                "target_abv": "10"
            }
        }"#,
    )
        .unwrap();

    let result = mazerion_calculate(calc_id.as_ptr(), json_input.as_ptr());

    assert_eq!(result.error.code, 0, "Calculation should succeed");

    unsafe {
        let json = CStr::from_ptr(result.json_output).to_str().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(json).unwrap();

        let water: f64 = parsed["value"].as_str().unwrap().parse().unwrap();
        assert!(water > 7.0 && water < 9.0, "Water should be ~8L");

        mazerion_free_result(result);
    }
}

#[test]
fn test_null_calculator_id() {
    let json_input = CString::new("{}").unwrap();
    let result = mazerion_calculate(std::ptr::null(), json_input.as_ptr());

    assert_eq!(result.error.code, 1, "Should return null pointer error");
    assert!(!result.error.message.is_null(), "Should have error message");

    unsafe {
        mazerion_free_result(result);
    }
}

#[test]
fn test_null_json_input() {
    let calc_id = CString::new("abv").unwrap();
    let result = mazerion_calculate(calc_id.as_ptr(), std::ptr::null());

    assert_eq!(result.error.code, 2, "Should return null pointer error");
    assert!(!result.error.message.is_null(), "Should have error message");

    unsafe {
        mazerion_free_result(result);
    }
}

#[test]
fn test_invalid_json() {
    mazerion_init();

    let calc_id = CString::new("abv").unwrap();
    let json_input = CString::new("not valid json").unwrap();

    let result = mazerion_calculate(calc_id.as_ptr(), json_input.as_ptr());

    assert_eq!(result.error.code, 6, "Should return JSON parse error");
    assert!(!result.error.message.is_null(), "Should have error message");

    unsafe {
        mazerion_free_result(result);
    }
}

#[test]
fn test_calculator_not_found() {
    mazerion_init();

    let calc_id = CString::new("nonexistent").unwrap();
    let json_input = CString::new(
        r#"{
            "calculator_id": "nonexistent",
            "params": {}
        }"#,
    )
        .unwrap();

    let result = mazerion_calculate(calc_id.as_ptr(), json_input.as_ptr());

    assert_eq!(result.error.code, 9, "Should return calculation error");
    assert!(!result.error.message.is_null(), "Should have error message");

    unsafe {
        let msg = CStr::from_ptr(result.error.message).to_str().unwrap();
        assert!(
            msg.contains("not found") || msg.contains("Not found"),
            "Error should mention calculator not found"
        );
        mazerion_free_result(result);
    }
}

#[test]
fn test_calculator_id_mismatch() {
    mazerion_init();

    let calc_id = CString::new("abv").unwrap();
    let json_input = CString::new(
        r#"{
            "calculator_id": "dilution",
            "params": {}
        }"#,
    )
        .unwrap();

    let result = mazerion_calculate(calc_id.as_ptr(), json_input.as_ptr());

    assert_eq!(result.error.code, 5, "Should return mismatch error");

    unsafe {
        mazerion_free_result(result);
    }
}

#[test]
fn test_missing_required_params() {
    mazerion_init();

    let calc_id = CString::new("abv").unwrap();
    let json_input = CString::new(
        r#"{
            "calculator_id": "abv",
            "params": {}
        }"#,
    )
        .unwrap();

    let result = mazerion_calculate(calc_id.as_ptr(), json_input.as_ptr());

    assert_eq!(result.error.code, 9, "Should return calculation error");

    unsafe {
        mazerion_free_result(result);
    }
}