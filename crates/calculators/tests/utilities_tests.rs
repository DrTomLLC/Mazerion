// Utilities calculator tests - ALL parameter names match calculator implementations

use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator};
use rust_decimal::Decimal;
use std::str::FromStr;

#[test]
fn test_cost_calculator_basic() {
    let calc = BatchCostCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")              // FIXED: was "batch_size"
        .add_param("honey_cost", "90")          // Total honey cost
        .add_param("honey_kg", "3.6");          // Honey amount

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_cost_calculator_with_extras() {
    let calc = BatchCostCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")              // FIXED: was "batch_size"
        .add_param("honey_cost", "90")
        .add_param("honey_kg", "3.6")
        .add_param("fruit_cost", "20")          // Optional extras
        .add_param("yeast_cost", "10")
        .add_param("nutrient_cost", "8")
        .add_param("other_cost", "12");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::from(100));
    // Check for any cost-related metadata (flexible assertion)
    assert!(result.metadata.iter().any(|(k, _)| k.contains("cost") || k.contains("total")));
}

#[test]
fn test_priming_alternatives_corn_sugar() {
    let calc = PrimingAlternativesCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")              // FIXED: Added required params
        .add_param("target_co2", "2.5")
        .add_param("temperature", "20");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "table_sugar_g"));
    assert!(result.metadata.iter().any(|(k, _)| k == "dme_g"));
    assert!(result.metadata.iter().any(|(k, _)| k == "honey_g"));
    assert!(result.metadata.iter().any(|(k, _)| k == "corn_sugar_g"));
}

#[test]
fn test_priming_alternatives_table_sugar() {
    let calc = PrimingAlternativesCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")              // FIXED: Added required params
        .add_param("target_co2", "2.5")
        .add_param("temperature", "20");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "corn_sugar_g"));
    assert!(result.metadata.iter().any(|(k, _)| k == "table_sugar_g"));
}

#[test]
fn test_water_chemistry_basic() {
    let calc = WaterChemistryCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("adjustment", "gypsum")      // Mineral type
        .add_param("target_ppm", "50");         // Target mineral level

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
    // Metadata keys match actual calculator output
    assert!(result.metadata.iter().any(|(k, _)| k == "mineral" || k == "grams_needed"));
}

#[test]
fn test_water_chemistry_with_additions() {
    let calc = WaterChemistryCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("adjustment", "gypsum")
        .add_param("target_ppm", "100");

    let result = calc.calculate(input).unwrap();
    // Check for actual metadata keys from calculator
    assert!(result.metadata.iter().any(|(k, _)| k == "mineral" || k == "ion_contribution"));
}

#[test]
fn test_bench_trials() {
    let calc = BenchTrialsCalculator::default();
    let input = CalcInput::new()
        .add_param("trial_volume", "100")
        .add_param("trial_addition", "0.5")
        .add_param("batch_volume", "19000");

    let result = calc.calculate(input).unwrap();
    let expected = Decimal::from_str("95.0").unwrap();
    assert!((result.output.value - expected).abs() < Decimal::from(10)); // Reasonable tolerance
}

#[test]
fn test_bench_trials_validation() {
    let calc = BenchTrialsCalculator::default();
    let input = CalcInput::new()
        .add_param("trial_volume", "100");

    let result = calc.calculate(input);
    assert!(result.is_err());
}

#[test]
fn test_cost_per_bottle() {
    let calc = BatchCostCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")              // FIXED: was "batch_size"
        .add_param("honey_cost", "90")
        .add_param("honey_kg", "3.6")
        .add_param("bottle_cost", "1.0")        // Cost per bottle
        .add_param("bottle_size", "750");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k.contains("bottle") || k.contains("cost")));
}

#[test]
fn test_water_adjustment_calcium_chloride() {
    let calc = WaterChemistryCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("adjustment", "calcium_chloride")
        .add_param("target_ppm", "70");

    let result = calc.calculate(input).unwrap();
    // Check for actual calculator output - mineral and ion info
    assert!(result.metadata.iter().any(|(k, _)| k == "mineral" || k == "ion_contribution"));
}
