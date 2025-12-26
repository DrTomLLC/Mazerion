use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator, Measurement};
use rust_decimal::Decimal;
use std::str::FromStr;

#[test]
fn test_abv_calculation() {
    let calc = AbvCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.090")
        .add_param("fg", "1.010");

    let result = calc.calculate(input).unwrap();
    let expected = Decimal::from_str("10.5").unwrap();
    assert!((result.output.value - expected).abs() < Decimal::from_str("0.1").unwrap());
}

#[test]
fn test_abv_high_gravity() {
    let calc = AbvCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.160")
        .add_param("fg", "1.000");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::from_str("20.0").unwrap());
    assert!(!result.warnings.is_empty());
}

#[test]
fn test_brix_to_sg() {
    let calc = BrixToSgCalculator::default();
    let brix_measurement = Measurement::brix(Decimal::from_str("20.0").unwrap()).unwrap();
    let input = CalcInput::new().add_measurement(brix_measurement);

    let result = calc.calculate(input).unwrap();
    let expected = Decimal::from_str("1.083").unwrap();
    assert!((result.output.value - expected).abs() < Decimal::from_str("0.001").unwrap());
}

#[test]
fn test_sg_to_brix() {
    let calc = SgToBrixCalculator::default();
    let sg_measurement = Measurement::sg(Decimal::from_str("1.083").unwrap()).unwrap();
    let input = CalcInput::new().add_measurement(sg_measurement);

    let result = calc.calculate(input).unwrap();
    let expected = Decimal::from_str("20.0").unwrap();
    assert!((result.output.value - expected).abs() < Decimal::from_str("0.5").unwrap());
}

#[test]
fn test_dilution_calculator() {
    let calc = DilutionCalculator::default();
    let input = CalcInput::new()
        .add_param("current_volume", "20")
        .add_param("current_abv", "14")
        .add_param("target_abv", "10");

    let result = calc.calculate(input).unwrap();
    let expected = Decimal::from_str("8.0").unwrap();
    assert!((result.output.value - expected).abs() < Decimal::from_str("0.1").unwrap());
}

#[test]
fn test_gravity_from_ingredients() {
    let calc = GravityFromIngredientsCalculator::default();
    let input = CalcInput::new()
        .add_param("honey_weight", "3.6")
        .add_param("water_volume", "15");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::from_str("1.070").unwrap());
    assert!(result.output.value < Decimal::from_str("1.090").unwrap());
}

#[test]
fn test_hydrometer_correction() {
    let calc = HydrometerCorrectionCalculator::default();
    let input = CalcInput::new()
        .add_param("measured_sg", "1.050")
        .add_param("sample_temp", "30")
        .add_param("calibration_temp", "20");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value != Decimal::from_str("1.050").unwrap());
}

#[test]
fn test_abv_missing_params() {
    let calc = AbvCalculator::default();
    let input = CalcInput::new().add_param("og", "1.090");

    let result = calc.calculate(input);
    assert!(result.is_err());
}

#[test]
fn test_dilution_impossible() {
    let calc = DilutionCalculator::default();
    let input = CalcInput::new()
        .add_param("current_volume", "20")
        .add_param("current_abv", "10")
        .add_param("target_abv", "14");

    let result = calc.calculate(input);
    assert!(result.is_err());
}
