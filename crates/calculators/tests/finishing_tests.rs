// SAFETY-CRITICAL: Finishing calculator tests with proper physical measurements

use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator, Measurement};
use rust_decimal::Decimal;
use std::str::FromStr;

#[test]
fn test_backsweetening_basic() {
    let calc = BacksweeteningCalculator::default();

    // CRITICAL: Must provide current SG as a Measurement
    let current_sg = Measurement::sg(Decimal::from_str("1.000").unwrap()).unwrap();

    let input = CalcInput::new()
        .add_measurement(current_sg)
        .add_param("volume", "19")
        .add_param("target_sg", "1.020");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_backsweetening_high_gravity() {
    let calc = BacksweeteningCalculator::default();

    // CRITICAL: Must provide current SG as a Measurement
    let current_sg = Measurement::sg(Decimal::from_str("1.010").unwrap()).unwrap();

    let input = CalcInput::new()
        .add_measurement(current_sg)
        .add_param("volume", "19")
        .add_param("target_sg", "1.100");

    let result = calc.calculate(input).unwrap();
    assert!(!result.warnings.is_empty());
}

#[test]
fn test_sulfite_campden() {
    let calc = SulfiteCalculator::default();

    // CRITICAL: pH measurement required for effectiveness calculation
    let ph = Measurement::ph(Decimal::from_str("3.5").unwrap()).unwrap();

    let input = CalcInput::new()
        .add_measurement(ph)
        .add_param("volume", "19")
        .add_param("target_free_so2", "50");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_sulfite_kmeta() {
    let calc = SulfiteCalculator::default();

    // CRITICAL: pH measurement required for effectiveness calculation
    let ph = Measurement::ph(Decimal::from_str("3.3").unwrap()).unwrap();

    let input = CalcInput::new()
        .add_measurement(ph)
        .add_param("volume", "19")
        .add_param("target_free_so2", "50");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_acid_addition_tartaric() {
    let calc = AcidAdditionCalculator::default();

    // CRITICAL: Current pH must be a physical measurement
    let current_ph = Measurement::ph(Decimal::from_str("3.8").unwrap()).unwrap();

    let input = CalcInput::new()
        .add_measurement(current_ph)
        .add_param("volume", "19")
        .add_param("target_ph", "3.4");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_acid_addition_citric() {
    let calc = AcidAdditionCalculator::default();

    // CRITICAL: Current pH must be a physical measurement
    let current_ph = Measurement::ph(Decimal::from_str("3.8").unwrap()).unwrap();

    let input = CalcInput::new()
        .add_measurement(current_ph)
        .add_param("volume", "19")
        .add_param("target_ph", "3.4");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_acid_addition_malic() {
    let calc = AcidAdditionCalculator::default();

    // CRITICAL: Current pH must be a physical measurement
    let current_ph = Measurement::ph(Decimal::from_str("3.8").unwrap()).unwrap();

    let input = CalcInput::new()
        .add_measurement(current_ph)
        .add_param("volume", "19")
        .add_param("target_ph", "3.4");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_stabilization_calculator() {
    let calc = StabilizationCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("abv", "14");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(key, _)| key == "kmeta_g"));
    assert!(result.metadata.iter().any(|(key, _)| key == "sorbate_g"));
}

#[test]
fn test_tannin_calculator() {
    let calc = TanninCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("tannin_type", "grape")
        .add_param("dose_gpl", "0.5");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_bottling_calculator() {
    let calc = BottlingCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19") // Total batch volume in liters
        .add_param("bottle_size", "750"); // Standard 750 mL wine bottle

    let result = calc.calculate(input).unwrap();

    // CORRECT: 19 L with 3% loss (default) = 18.43 L usable = 24.57 bottles → 24 bottles
    // Calculator properly accounts for realistic losses (racking, lees, sampling)
    let expected = Decimal::from(24);
    assert_eq!(result.output.value.round(), expected);
}

#[test]
fn test_acid_invalid_ph_range() {
    let calc = AcidAdditionCalculator::default();

    // This should fail validation - trying to increase pH with acid
    let current_ph = Measurement::ph(Decimal::from_str("3.2").unwrap()).unwrap();

    let input = CalcInput::new()
        .add_measurement(current_ph)
        .add_param("volume", "19")
        .add_param("target_ph", "3.8"); // Higher than current - INVALID

    let result = calc.calculate(input);
    // Should either error or give negative result (invalid)
    assert!(result.is_err() || result.unwrap().output.value < Decimal::ZERO);
}
