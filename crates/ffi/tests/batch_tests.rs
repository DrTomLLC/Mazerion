// crates/ffi/tests/batch_tests.rs
// Batch operation tests for efficiency - ZERO UNWRAP, ZERO PANIC

use mazerion_ffi::*;

// Force linking of calculators
#[allow(unused_imports)]
use mazerion_calculators as _;

#[test]
fn test_execute_batch_success() -> Result<(), Box<dyn std::error::Error>> {
    let requests = vec![
        BatchCalculatorRequest {
            calculator_id: "abv".to_string(),
            params: vec![
                CalcParam { key: "og".to_string(), value: "1.080".to_string() },
                CalcParam { key: "fg".to_string(), value: "1.010".to_string() },
            ],
        },
        BatchCalculatorRequest {
            calculator_id: "abv".to_string(),
            params: vec![
                CalcParam { key: "og".to_string(), value: "1.050".to_string() },
                CalcParam { key: "fg".to_string(), value: "1.000".to_string() },
            ],
        },
    ];

    let result = execute_batch(requests);
    assert!(result.is_ok(), "Batch execution failed: {:?}", result.err());

    let results = result?;
    assert_eq!(results.len(), 2);

    for (idx, batch_result) in results.iter().enumerate() {
        if batch_result.result.is_none() {
            eprintln!("Batch {} failed with error: {:?}", idx, batch_result.error);
        }
        assert!(
            batch_result.result.is_some(),
            "Batch {} failed: {:?}",
            idx,
            batch_result.error
        );
        assert!(batch_result.error.is_none());
    }

    Ok(())
}

#[test]
fn test_execute_batch_partial_failure() -> Result<(), Box<dyn std::error::Error>> {
    let requests = vec![
        BatchCalculatorRequest {
            calculator_id: "abv".to_string(),
            params: vec![
                CalcParam { key: "og".to_string(), value: "1.080".to_string() },
                CalcParam { key: "fg".to_string(), value: "1.010".to_string() },
            ],
        },
        BatchCalculatorRequest {
            calculator_id: "nonexistent".to_string(),
            params: vec![],
        },
        BatchCalculatorRequest {
            calculator_id: "abv".to_string(),
            params: vec![
                CalcParam { key: "og".to_string(), value: "invalid".to_string() },
            ],
        },
    ];

    let result = execute_batch(requests);
    assert!(result.is_ok());

    let results = result?;
    assert_eq!(results.len(), 3);

    if results[0].result.is_none() {
        eprintln!("First request failed: {:?}", results[0].error);
    }
    assert!(results[0].result.is_some(), "First valid request should succeed: {:?}", results[0].error);
    assert!(results[1].error.is_some(), "Nonexistent calculator should fail");
    assert!(results[2].error.is_some(), "Invalid input should fail");

    Ok(())
}

#[test]
fn test_execute_batch_empty_rejects() {
    let result = execute_batch(vec![]);
    assert!(result.is_err());
}

#[test]
fn test_execute_batch_too_large_rejects() {
    let mut requests = Vec::new();
    for i in 0..150 {
        requests.push(BatchCalculatorRequest {
            calculator_id: format!("calc{}", i),
            params: vec![],
        });
    }

    let result = execute_batch(requests);
    assert!(result.is_err());

    if let Err(MazerionError::InputTooLarge) = result {
        // Expected error
    } else {
        assert!(false, "Expected InputTooLarge error, got: {:?}", result.err());
    }
}

#[test]
fn test_simple_abv_single() -> Result<(), Box<dyn std::error::Error>> {
    // Test single ABV calculation to verify parameter format
    let result = execute_calculator(
        "abv".to_string(),
        vec![
            CalcParam { key: "og".to_string(), value: "1.080".to_string() },
            CalcParam { key: "fg".to_string(), value: "1.010".to_string() },
        ],
    );

    match &result {
        Ok(r) => {
            println!("ABV calculation succeeded: {} {}", r.value, r.unit);
        }
        Err(e) => {
            eprintln!("ABV calculation failed: {:?}", e);
        }
    }

    assert!(result.is_ok(), "Single ABV calculation should work: {:?}", result.err());
    Ok(())
}