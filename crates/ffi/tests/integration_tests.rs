// crates/ffi/tests/integration_tests.rs
// End-to-end integration tests - ZERO UNWRAP

use mazerion_ffi::*;
use std::collections::HashSet;

// Force linking of calculators
#[allow(unused_imports)]
use mazerion_calculators as _;

#[test]
fn test_full_workflow() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Get version
    let ver = version();
    assert!(!ver.is_empty());

    // 2. List all calculators
    let all_calcs = list_calculators()?;
    assert!(!all_calcs.is_empty());

    // 3. Get categories
    let categories = get_categories()?;
    assert!(!categories.entries.is_empty());

    // 4. Get calculators in Basic category
    let basic_calcs = get_calculators_by_category("Basic".to_string())?;
    assert!(!basic_calcs.is_empty());

    // 5. Execute a calculation
    let result = execute_calculator(
        "abv".to_string(),
        vec![
            CalcParam { key: "og".to_string(), value: "1.080".to_string() },
            CalcParam { key: "fg".to_string(), value: "1.010".to_string() },
        ],
    )?;

    assert!(!result.value.is_empty());
    assert!(result.unit.contains('%'), "Unit should contain %: {}", result.unit);

    Ok(())
}

#[test]
fn test_all_calculators_are_accessible() -> Result<(), Box<dyn std::error::Error>> {
    let calculators = list_calculators()?;

    for calc in calculators {
        // Verify we can get this calculator by category
        let by_category = get_calculators_by_category(calc.category.clone())?;

        let found = by_category.iter().any(|c| c.id == calc.id);
        assert!(found, "Calculator {} not found in category {}", calc.id, calc.category);
    }

    Ok(())
}

#[test]
fn test_category_counts_match_calculators() -> Result<(), Box<dyn std::error::Error>> {
    let calculators = list_calculators()?;
    let category_map = get_categories()?;

    let mut expected_counts: std::collections::HashMap<String, u32> = std::collections::HashMap::new();

    for calc in calculators {
        *expected_counts.entry(calc.category).or_insert(0) += 1;
    }

    let actual_counts: std::collections::HashMap<String, u32> = category_map
        .entries
        .into_iter()
        .map(|e| (e.category, e.count))
        .collect();

    assert_eq!(expected_counts, actual_counts);

    Ok(())
}

#[test]
fn test_no_duplicate_calculator_ids() -> Result<(), Box<dyn std::error::Error>> {
    let calculators = list_calculators()?;
    let mut ids = HashSet::new();

    for calc in calculators {
        assert!(ids.insert(calc.id.clone()), "Duplicate calculator ID: {}", calc.id);
    }

    Ok(())
}