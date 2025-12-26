//! ADD THESE TESTS TO: crates/calculators/tests/brewing_tests.rs
//! LOCATION: crates/calculators/tests/brewing_tests.rs
//!
//! Copy everything below and ADD to the END of your existing brewing_tests.rs file
//! THIS VERSION HAS ALL COMPILATION ERRORS FIXED!

use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator};
use rust_decimal::Decimal;
use std::str::FromStr;

// ============================================================================
// TOSNA/NUTRITION DETAILED TESTS
// ============================================================================

#[test]
fn test_nutrition_calculator_has_schedule() {
    let calc = NutritionCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "14");

    let result = calc.calculate(input).unwrap();

    // Should have schedule information in metadata
    assert!(result.metadata.iter().any(|(k, _)|
        k.contains("fermaid") || k.contains("schedule") || k.contains("yan") || k.contains("addition")
    ));
}

#[test]
fn test_nutrition_calculator_low_abv() {
    let calc = NutritionCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "8");

    let result = calc.calculate(input).unwrap();

    // Low ABV needs less nutrients
    assert!(result.output.value >= Decimal::ZERO);
}

#[test]
fn test_nutrition_calculator_high_abv() {
    let calc = NutritionCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "18");

    let result = calc.calculate(input).unwrap();

    // High ABV needs more nutrients
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_nutrition_calculator_yan_target() {
    let calc = NutritionCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "14");

    let result = calc.calculate(input).unwrap();

    // Should provide YAN target information
    assert!(result.metadata.iter().any(|(k, _)| k.to_lowercase().contains("yan")));
}

#[test]
fn test_nutrition_calculator_fermaid_o() {
    let calc = NutritionCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "14");

    let result = calc.calculate(input).unwrap();

    // TOSNA protocol uses Fermaid O
    assert!(result.metadata.iter().any(|(k, v)|
        k.to_lowercase().contains("fermaid") || v.to_lowercase().contains("fermaid")
    ));
}

// ============================================================================
// CARBONATION DETAILED TESTS
// ============================================================================

#[test]
fn test_carbonation_low_co2() {
    let calc = CarbonationCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("temperature", "20")
        .add_param("target_co2", "2.0") // Low carbonation
        .add_param("method", "priming");

    let result = calc.calculate(input).unwrap();

    // Should calculate priming sugar amount
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_carbonation_medium_co2() {
    let calc = CarbonationCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("temperature", "20")
        .add_param("target_co2", "2.5") // Medium carbonation
        .add_param("method", "priming");

    let result = calc.calculate(input).unwrap();

    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_carbonation_high_co2() {
    let calc = CarbonationCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("temperature", "20")
        .add_param("target_co2", "4.0") // High carbonation (Belgian)
        .add_param("method", "priming");

    let result = calc.calculate(input).unwrap();

    // High CO2 should need substantial priming sugar
    assert!(result.output.value > Decimal::from_str("100.0").unwrap());
}

#[test]
fn test_carbonation_temperature_effect() {
    let calc = CarbonationCalculator::default();

    let cold_input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("temperature", "5") // Cold
        .add_param("target_co2", "2.5")
        .add_param("method", "priming");

    let warm_input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("temperature", "20") // Warm
        .add_param("target_co2", "2.5")
        .add_param("method", "priming");

    let cold_result = calc.calculate(cold_input).unwrap();
    let warm_result = calc.calculate(warm_input).unwrap();

    // Warm beer has less dissolved CO2, needs more priming sugar
    assert!(warm_result.output.value > cold_result.output.value);
}

#[test]
fn test_carbonation_force_method() {
    let calc = CarbonationCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("temperature", "2") // Keg temp
        .add_param("target_co2", "2.5")
        .add_param("method", "force");

    let result = calc.calculate(input);

    // Force carbonation should calculate PSI
    assert!(result.is_ok());
}

// ============================================================================
// YEAST PITCH RATE TESTS
// ============================================================================

#[test]
fn test_yeast_pitch_standard_gravity() {
    let calc = YeastPitchCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("og", "1.060")
        .add_param("yeast_type", "ale");

    let result = calc.calculate(input).unwrap();

    // Should calculate cell count
    assert!(result.output.value > Decimal::ZERO);
    assert!(result.metadata.iter().any(|(k, _)| k.contains("cells") || k.contains("billion")));
}

#[test]
fn test_yeast_pitch_high_gravity() {
    let calc = YeastPitchCalculator::default();

    let standard = CalcInput::new()
        .add_param("volume", "19")
        .add_param("og", "1.060")
        .add_param("yeast_type", "ale");

    let high_gravity = CalcInput::new()
        .add_param("volume", "19")
        .add_param("og", "1.100")
        .add_param("yeast_type", "ale");

    let standard_result = calc.calculate(standard).unwrap();
    let high_result = calc.calculate(high_gravity).unwrap();

    // High gravity needs more cells
    assert!(high_result.output.value > standard_result.output.value);
}

#[test]
fn test_yeast_pitch_lager_vs_ale() {
    let calc = YeastPitchCalculator::default();

    let ale = CalcInput::new()
        .add_param("volume", "19")
        .add_param("og", "1.050")
        .add_param("yeast_type", "ale");

    let lager = CalcInput::new()
        .add_param("volume", "19")
        .add_param("og", "1.050")
        .add_param("yeast_type", "lager");

    let ale_result = calc.calculate(ale).unwrap();
    let lager_result = calc.calculate(lager).unwrap();

    // Lager needs ~2x more cells than ale
    assert!(lager_result.output.value > ale_result.output.value);
}

// ============================================================================
// LABEL VERIFICATION TESTS
// ============================================================================

#[test]
fn test_nutrition_calculator_id() {
    let calc = NutritionCalculator::default();
    assert!(calc.id().contains("nutrit") || calc.id().contains("tosna"));
}

#[test]
fn test_carbonation_calculator_id() {
    let calc = CarbonationCalculator::default();
    assert!(calc.id().contains("carbon") || calc.id().contains("prim"));
}

#[test]
fn test_yeast_pitch_calculator_id() {
    let calc = YeastPitchCalculator::default();
    assert!(calc.id().contains("yeast") || calc.id().contains("pitch"));
}