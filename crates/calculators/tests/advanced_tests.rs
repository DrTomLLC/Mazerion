use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator};
use rust_decimal::Decimal;
use std::str::FromStr;

#[test]
fn test_blending_equal_volumes() {
    let calc = BlendingCalculator::default();
    let input = CalcInput::new()
        .add_param("volume1", "10")
        .add_param("abv1", "12")
        .add_param("volume2", "10")
        .add_param("abv2", "14");

    let result = calc.calculate(input).unwrap();
    let expected = Decimal::from(13);
    assert!((result.output.value - expected).abs() < Decimal::from_str("0.1").unwrap());
}

#[test]
fn test_blending_different_volumes() {
    let calc = BlendingCalculator::default();
    let input = CalcInput::new()
        .add_param("volume1", "15")
        .add_param("abv1", "10")
        .add_param("volume2", "5")
        .add_param("abv2", "18");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::from(10));
    assert!(result.output.value < Decimal::from(18));
}

#[test]
fn test_blending_three_batches() {
    let calc = BlendingCalculator::default();
    let input = CalcInput::new()
        .add_param("volume1", "10")
        .add_param("abv1", "12")
        .add_param("volume2", "10")
        .add_param("abv2", "14")
        .add_param("volume3", "5")
        .add_param("abv3", "10");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "total_volume"));
}

#[test]
fn test_attenuation_apparent() {
    let calc = AttenuationCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.080")
        .add_param("fg", "1.015");

    let result = calc.calculate(input).unwrap();
    let expected = Decimal::from(81);
    assert!((result.output.value - expected).abs() < Decimal::ONE);
}

#[test]
fn test_attenuation_real() {
    let calc = AttenuationCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.080")
        .add_param("fg", "1.015")
        .add_param("real", "true");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "real_attenuation"));
}

#[test]
fn test_attenuation_low() {
    let calc = AttenuationCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.080")
        .add_param("fg", "1.040");

    let result = calc.calculate(input).unwrap();
    assert!(!result.warnings.is_empty());
}

#[test]
fn test_volume_adjustment_dilution() {
    let calc = VolumeAdjustmentCalculator::default();
    let input = CalcInput::new()
        .add_param("current_volume", "18")
        .add_param("current_gravity", "1.060")
        .add_param("target_gravity", "1.050");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_volume_adjustment_concentration() {
    let calc = VolumeAdjustmentCalculator::default();
    let input = CalcInput::new()
        .add_param("current_volume", "20")
        .add_param("current_gravity", "1.040")
        .add_param("target_gravity", "1.050");

    // Calculator only supports dilution (target < current), so this should error
    let result = calc.calculate(input);
    assert!(result.is_err());
}

#[test]
fn test_alcohol_tolerance_low() {
    let calc = AlcoholToleranceCalculator::default();
    let input = CalcInput::new()
        .add_param("yeast_strain", "WLP001")
        .add_param("og", "1.120");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "max_abv"));
}

#[test]
fn test_alcohol_tolerance_high() {
    let calc = AlcoholToleranceCalculator::default();
    let input = CalcInput::new()
        .add_param("yeast_strain", "EC-1118")
        .add_param("og", "1.140");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "max_abv"));
}

#[test]
fn test_blending_validation_missing_params() {
    let calc = BlendingCalculator::default();
    let input = CalcInput::new()
        .add_param("volume1", "10")
        .add_param("abv1", "12");

    let result = calc.calculate(input);
    assert!(result.is_err());
}

#[test]
fn test_attenuation_impossible() {
    let calc = AttenuationCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.050")
        .add_param("fg", "1.060");

    let result = calc.calculate(input);
    assert!(result.is_err());
}

#[test]
fn test_volume_adjustment_negative_water() {
    let calc = VolumeAdjustmentCalculator::default();
    let input = CalcInput::new()
        .add_param("current_volume", "15")
        .add_param("current_gravity", "1.040")
        .add_param("target_gravity", "1.060");

    let result = calc.calculate(input);
    assert!(result.is_err() || !result.unwrap().warnings.is_empty());
}