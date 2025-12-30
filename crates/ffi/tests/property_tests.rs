// crates/ffi/tests/property_tests.rs
// Property-based tests for robustness - ZERO UNWRAP

#[cfg(test)]
mod property_tests {
    use mazerion_ffi::*;

    // Force linking of calculators
    #[allow(unused_imports)]
    use mazerion_calculators as _;

    #[test]
    fn test_calculator_id_ascii_property() -> Result<(), Box<dyn std::error::Error>> {
        let calculators = list_calculators()?;

        for calc in calculators {
            assert!(calc.id.is_ascii());
            assert!(!calc.id.contains('/'));
            assert!(!calc.id.contains('\\'));
            assert!(!calc.id.contains(' '));
        }

        Ok(())
    }

    #[test]
    fn test_all_categories_valid() -> Result<(), Box<dyn std::error::Error>> {
        let category_map = get_categories()?;
        let valid_categories = vec![
            "Basic", "Advanced", "Brewing", "Beer",
            "Finishing", "Mead Styles", "Utilities"
        ];

        for entry in category_map.entries {
            assert!(
                valid_categories.contains(&entry.category.as_str()),
                "Invalid category: {}",
                entry.category
            );
        }

        Ok(())
    }

    #[test]
    fn test_calc_results_have_consistent_structure() -> Result<(), Box<dyn std::error::Error>> {
        let params = vec![
            CalcParam { key: "og".to_string(), value: "1.080".to_string() },
            CalcParam { key: "fg".to_string(), value: "1.010".to_string() },
        ];

        let result = execute_calculator("abv".to_string(), params)?;

        assert!(!result.value.is_empty());
        assert!(!result.unit.is_empty());
        assert!(!result.display_text.is_empty());
        assert!(result.value.parse::<f64>().is_ok());

        Ok(())
    }
}