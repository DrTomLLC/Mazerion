// crates/ffi/tests/validation_tests.rs
// Security-critical validation tests - ZERO UNWRAP, ZERO PANIC

use mazerion_ffi::*;

// Force linking of calculators
#[allow(unused_imports)]
use mazerion_calculators as _;

#[test]
fn test_reject_empty_calculator_id() {
    let result = execute_calculator("".to_string(), vec![]);
    assert!(result.is_err());
}

#[test]
fn test_reject_oversized_calculator_id() {
    let long_id = "a".repeat(200);
    let result = execute_calculator(long_id, vec![]);
    assert!(result.is_err());

    if let Err(MazerionError::InputTooLarge) = result {
        // Expected error
    } else {
        assert!(false, "Expected InputTooLarge error, got: {:?}", result.err());
    }
}

#[test]
fn test_reject_invalid_calculator_id_chars() {
    let result = execute_calculator("calc/../../etc/passwd".to_string(), vec![]);
    assert!(result.is_err());
}

#[test]
fn test_reject_too_many_params() {
    let mut params = Vec::new();
    for i in 0..100 {
        params.push(CalcParam {
            key: format!("param{}", i),
            value: "value".to_string(),
        });
    }

    let result = execute_calculator("abv".to_string(), params);
    assert!(result.is_err());

    if let Err(MazerionError::InputTooLarge) = result {
        // Expected error
    } else {
        assert!(false, "Expected InputTooLarge error, got: {:?}", result.err());
    }
}

#[test]
fn test_reject_oversized_param_value() {
    let params = vec![CalcParam {
        key: "og".to_string(),
        value: "1".repeat(2000),
    }];

    let result = execute_calculator("abv".to_string(), params);
    assert!(result.is_err());

    if let Err(MazerionError::InputTooLarge) = result {
        // Expected error
    } else {
        assert!(false, "Expected InputTooLarge error, got: {:?}", result.err());
    }
}

#[test]
fn test_reject_empty_param_key() {
    let params = vec![CalcParam {
        key: "".to_string(),
        value: "1.080".to_string(),
    }];

    let result = execute_calculator("abv".to_string(), params);
    assert!(result.is_err());
}

#[test]
fn test_reject_oversized_category_name() {
    let long_category = "a".repeat(100);
    let result = get_calculators_by_category(long_category);
    assert!(result.is_err());
}