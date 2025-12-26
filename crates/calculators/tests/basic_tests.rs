//! ADD THESE TESTS TO: crates/calculators/tests/basic_tests.rs
//! LOCATION: crates/calculators/tests/basic_tests.rs
//!
//! COMPLETE VERSION - ALL 20 TESTS INCLUDING DILUTION ZERO TEST
//! Use AFTER fixing dilution.rs calculator

use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator, Measurement, Unit};
use rust_decimal::Decimal;
use std::str::FromStr;

// ============================================================================
// LABEL VERIFICATION TESTS
// ============================================================================

#[test]
fn test_abv_calculator_id() {
    let calc = AbvCalculator::default();
    assert_eq!(calc.id(), "abv");
}

#[test]
fn test_abv_calculator_category() {
    let calc = AbvCalculator::default();
    assert_eq!(calc.category(), "Basic");
}

#[test]
fn test_brix_to_sg_calculator_id() {
    let calc = BrixToSgCalculator::default();
    assert!(calc.id().contains("brix"));
}

#[test]
fn test_sg_to_brix_calculator_id() {
    let calc = SgToBrixCalculator::default();
    assert!(calc.id().contains("brix"));
}

#[test]
fn test_dilution_calculator_id() {
    let calc = DilutionCalculator::default();
    assert_eq!(calc.id(), "dilution");
}

// ============================================================================
// RESULT UNIT TESTS
// ============================================================================

#[test]
fn test_abv_result_unit() {
    let calc = AbvCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.060")
        .add_param("fg", "1.010");
    let result = calc.calculate(input).unwrap();
    assert_eq!(result.output.unit, Unit::Abv);
}

#[test]
fn test_brix_to_sg_result_unit() {
    let calc = BrixToSgCalculator::default();
    let brix = Measurement::brix(Decimal::from_str("20.0").unwrap()).unwrap();
    let input = CalcInput::new().add_measurement(brix);
    let result = calc.calculate(input).unwrap();
    assert_eq!(result.output.unit, Unit::SpecificGravity);
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_abv_very_high_gravity() {
    let calc = AbvCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.200")
        .add_param("fg", "1.050");
    let result = calc.calculate(input).unwrap();

    // Should be around 19.7%
    assert!(result.output.value > Decimal::from_str("19.0").unwrap());
    assert!(result.output.value < Decimal::from_str("21.0").unwrap());
}

#[test]
fn test_abv_extreme_attenuation() {
    let calc = AbvCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.100")
        .add_param("fg", "0.990");
    let result = calc.calculate(input).unwrap();

    // Super dry fermentation
    assert!(result.output.value > Decimal::from_str("14.0").unwrap());
}

#[test]
fn test_brix_to_sg_zero() {
    let calc = BrixToSgCalculator::default();
    let brix = Measurement::brix(Decimal::ZERO).unwrap();
    let input = CalcInput::new().add_measurement(brix);
    let result = calc.calculate(input).unwrap();

    // 0°Brix = 1.000 SG
    assert_eq!(result.output.value, Decimal::ONE);
}

#[test]
fn test_brix_to_sg_very_high() {
    let calc = BrixToSgCalculator::default();
    let brix = Measurement::brix(Decimal::from_str("30.0").unwrap()).unwrap();
    let input = CalcInput::new().add_measurement(brix);
    let result = calc.calculate(input).unwrap();

    // 30°Brix should be around 1.129 SG
    assert!(result.output.value > Decimal::from_str("1.120").unwrap());
    assert!(result.output.value < Decimal::from_str("1.135").unwrap());
}

#[test]
fn test_dilution_to_zero_abv() {
    let calc = DilutionCalculator::default();
    let input = CalcInput::new()
        .add_param("current_volume", "5")
        .add_param("current_abv", "14")
        .add_param("target_abv", "0");

    // NOW FIXED: Should return proper error, NOT panic
    let result = calc.calculate(input);

    // Should error - cannot dilute to 0% ABV
    assert!(result.is_err(), "Dilution to 0% ABV should return error");

    if let Err(e) = result {
        let err_msg = format!("{}", e);
        assert!(
            err_msg.contains("0%") || err_msg.contains("impossible") || err_msg.contains("Cannot"),
            "Error message should explain why: {}",
            err_msg
        );
    }
}

#[test]
fn test_dilution_minimal_change() {
    let calc = DilutionCalculator::default();
    let input = CalcInput::new()
        .add_param("current_volume", "19")
        .add_param("current_abv", "14.0")
        .add_param("target_abv", "13.9");

    let result = calc.calculate(input).unwrap();

    // Very small dilution
    assert!(result.output.value < Decimal::from_str("1.0").unwrap());
}

// ============================================================================
// ROUNDTRIP TESTS
// ============================================================================

#[test]
fn test_brix_sg_roundtrip() {
    let brix_to_sg = BrixToSgCalculator::default();
    let sg_to_brix = SgToBrixCalculator::default();

    // Start with 20°Brix
    let original_brix = Decimal::from_str("20.0").unwrap();
    let brix_measurement = Measurement::brix(original_brix).unwrap();
    let input1 = CalcInput::new().add_measurement(brix_measurement);

    // Convert to SG
    let sg_result = brix_to_sg.calculate(input1).unwrap();
    let sg_value = sg_result.output.value;

    // Convert back to Brix
    let sg_measurement = Measurement::sg(sg_value).unwrap();
    let input2 = CalcInput::new().add_measurement(sg_measurement);
    let brix_result = sg_to_brix.calculate(input2).unwrap();

    // Should be close to original (within 0.5°Brix)
    let diff = (brix_result.output.value - original_brix).abs();
    assert!(diff < Decimal::from_str("0.5").unwrap());
}

// ============================================================================
// PRECISION TESTS
// ============================================================================

#[test]
fn test_abv_decimal_precision() {
    let calc = AbvCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.0605")
        .add_param("fg", "1.0102");

    let result = calc.calculate(input).unwrap();

    // Should handle decimal precision
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_brix_decimal_precision() {
    let calc = BrixToSgCalculator::default();
    let brix = Measurement::brix(Decimal::from_str("20.5").unwrap()).unwrap();
    let input = CalcInput::new().add_measurement(brix);

    let result = calc.calculate(input).unwrap();

    // Should handle decimal Brix
    assert!(result.output.value > Decimal::ONE);
}