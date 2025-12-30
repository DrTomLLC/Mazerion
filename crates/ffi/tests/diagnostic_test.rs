// crates/ffi/tests/diagnostic_test.rs
// Diagnostic tests to understand calculator behavior - ZERO UNWRAP

use mazerion_ffi::*;

// Force linking of calculators
#[allow(unused_imports)]
use mazerion_calculators as _;

#[test]
fn test_list_all_calculators() -> Result<(), Box<dyn std::error::Error>> {
    let calculators = list_calculators()?;

    println!("\n=== Available Calculators ===");
    for calc in &calculators {
        println!("ID: {}, Name: {}, Category: {}", calc.id, calc.name, calc.category);
    }

    // Find ABV calculator
    let abv = calculators.iter().find(|c| c.id == "abv");
    assert!(abv.is_some(), "ABV calculator should exist");

    if let Some(abv_calc) = abv {
        println!("\nABV Calculator: {:?}", abv_calc);
    }

    Ok(())
}

#[test]
fn test_abv_calculator_with_different_params() {
    println!("\n=== Testing ABV Calculator ===");

    // Try with og/fg
    let result1 = execute_calculator(
        "abv".to_string(),
        vec![
            CalcParam { key: "og".to_string(), value: "1.080".to_string() },
            CalcParam { key: "fg".to_string(), value: "1.010".to_string() },
        ],
    );
    println!("og/fg result: {:?}", result1);

    // Try with original_gravity/final_gravity
    let result2 = execute_calculator(
        "abv".to_string(),
        vec![
            CalcParam { key: "original_gravity".to_string(), value: "1.080".to_string() },
            CalcParam { key: "final_gravity".to_string(), value: "1.010".to_string() },
        ],
    );
    println!("original_gravity/final_gravity result: {:?}", result2);
}

#[test]
fn test_calculator_categories() -> Result<(), Box<dyn std::error::Error>> {
    let categories = get_categories()?;

    println!("\n=== Calculator Categories ===");
    for entry in &categories.entries {
        println!("{}: {} calculators", entry.category, entry.count);
    }

    Ok(())
}