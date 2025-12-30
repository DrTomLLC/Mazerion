// crates/ffi/tests/unit_tests.rs
// Unit tests for FFI functions - ZERO UNWRAP, ZERO PANIC

use mazerion_ffi::*;

// Force linking of calculators
#[allow(unused_imports)]
use mazerion_calculators as _;

#[test]
fn test_version_returns_semver() {
    let ver = version();
    assert!(ver.contains('.'));
    assert!(!ver.is_empty());
}

#[test]
fn test_list_calculators_succeeds() -> Result<(), Box<dyn std::error::Error>> {
    let result = list_calculators();
    assert!(result.is_ok());

    let calculators = result?;
    assert!(!calculators.is_empty());

    for calc in calculators {
        assert!(!calc.id.is_empty());
        assert!(!calc.name.is_empty());
        assert!(!calc.category.is_empty());
    }

    Ok(())
}

#[test]
fn test_get_categories_returns_valid_map() -> Result<(), Box<dyn std::error::Error>> {
    let result = get_categories();
    assert!(result.is_ok());

    let category_map = result?;
    assert!(!category_map.entries.is_empty());

    for entry in category_map.entries {
        assert!(!entry.category.is_empty());
        assert!(entry.count > 0);
    }

    Ok(())
}

#[test]
fn test_get_calculators_by_category() -> Result<(), Box<dyn std::error::Error>> {
    let result = get_calculators_by_category("Basic".to_string());
    assert!(result.is_ok());

    let calculators = result?;
    assert!(!calculators.is_empty());

    for calc in calculators {
        assert_eq!(calc.category, "Basic");
    }

    Ok(())
}

#[test]
fn test_execute_abv_calculator() -> Result<(), Box<dyn std::error::Error>> {
    let params = vec![
        CalcParam {
            key: "og".to_string(),
            value: "1.080".to_string(),
        },
        CalcParam {
            key: "fg".to_string(),
            value: "1.010".to_string(),
        },
    ];

    let result = execute_calculator("abv".to_string(), params);
    assert!(result.is_ok());

    let calc_result = result?;
    assert!(!calc_result.value.is_empty());
    assert!(!calc_result.unit.is_empty());
    assert!(!calc_result.display_text.is_empty());

    Ok(())
}

#[test]
fn test_execute_calculator_not_found() {
    let params = vec![];
    let result = execute_calculator("nonexistent_calc".to_string(), params);
    assert!(result.is_err());

    if let Err(MazerionError::CalculatorNotFound) = result {
        // Expected error
    } else {
        assert!(false, "Expected CalculatorNotFound error, got: {:?}", result.err());
    }
}

#[test]
fn test_execute_calculator_invalid_input() {
    let params = vec![
        CalcParam {
            key: "og".to_string(),
            value: "invalid".to_string(),
        },
    ];

    let result = execute_calculator("abv".to_string(), params);
    assert!(result.is_err());
}