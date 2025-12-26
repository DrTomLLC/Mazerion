use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator};
use rust_decimal::Decimal;

#[test]
fn test_carbonation_co2_volumes() {
    let calc = CarbonationCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_co2", "2.5")
        .add_param("temperature", "20")
        .add_param("sugar_type", "corn_sugar");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_carbonation_table_sugar() {
    let calc = CarbonationCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_co2", "2.5")
        .add_param("temperature", "20")
        .add_param("sugar_type", "table_sugar");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_carbonation_dme() {
    let calc = CarbonationCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_co2", "2.5")
        .add_param("temperature", "20")
        .add_param("sugar_type", "dme");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_carbonation_honey() {
    let calc = CarbonationCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_co2", "2.5")
        .add_param("temperature", "20")
        .add_param("sugar_type", "honey");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_carbonation_temperature_correction() {
    let calc = CarbonationCalculator::default();
    let input_cold = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_co2", "2.5")
        .add_param("temperature", "2")
        .add_param("sugar_type", "corn_sugar");

    let input_warm = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_co2", "2.5")
        .add_param("temperature", "25")
        .add_param("sugar_type", "corn_sugar");

    let result_cold = calc.calculate(input_cold).unwrap();
    let result_warm = calc.calculate(input_warm).unwrap();

    assert!(result_cold.output.value < result_warm.output.value);
}

#[test]
fn test_yeast_pitch_rate() {
    let calc = YeastPitchCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("og", "1.080")
        .add_param("yeast_type", "ale");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
    assert!(
        result
            .metadata
            .iter()
            .any(|(key, _)| key == "cells_billion")
    );
}

#[test]
fn test_yeast_pitch_lager() {
    let calc = YeastPitchCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("og", "1.050")
        .add_param("yeast_type", "lager");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_yeast_starter() {
    let calc = YeastStarterCalculator::default();
    let input = CalcInput::new()
        .add_param("cells_needed", "200")
        .add_param("cells_available", "100")
        .add_param("starter_gravity", "1.040");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
    assert!(result.metadata.iter().any(|(key, _)| key == "dme_needed_g"));
}

#[test]
fn test_yeast_starter_validation() {
    let calc = YeastStarterCalculator::default();
    let input = CalcInput::new()
        .add_param("cells_needed", "50")
        .add_param("cells_available", "100");

    let result = calc.calculate(input);
    assert!(result.is_err());
}

#[test]
fn test_carbonation_missing_params() {
    let calc = CarbonationCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_co2", "2.5");

    let result = calc.calculate(input);
    assert!(result.is_err());
}
