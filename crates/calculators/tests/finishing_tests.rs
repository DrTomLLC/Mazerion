use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator};
use rust_decimal::Decimal;
use std::str::FromStr;

#[test]
fn test_backsweetening_basic() {
    let calc = BacksweeteningCalculator::default();
    let input = CalcInput::new()
        .add_param("current_volume", "19")
        .add_param("target_gravity", "1.020")
        .add_param("honey_sg", "1.425");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_backsweetening_high_gravity() {
    let calc = BacksweeteningCalculator::default();
    let input = CalcInput::new()
        .add_param("current_volume", "19")
        .add_param("target_gravity", "1.100")
        .add_param("honey_sg", "1.425");

    let result = calc.calculate(input).unwrap();
    assert!(!result.warnings.is_empty());
}

#[test]
fn test_sulfite_campden() {
    let calc = SulfiteCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("sulfite_type", "campden")
        .add_param("dose_ppm", "50");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_sulfite_kmeta() {
    let calc = SulfiteCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("sulfite_type", "kmeta")
        .add_param("dose_ppm", "50");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_acid_addition_tartaric() {
    let calc = AcidAdditionCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("current_ph", "3.8")
        .add_param("target_ph", "3.4")
        .add_param("acid_type", "tartaric");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_acid_addition_citric() {
    let calc = AcidAdditionCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("current_ph", "3.8")
        .add_param("target_ph", "3.4")
        .add_param("acid_type", "citric");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_acid_addition_malic() {
    let calc = AcidAdditionCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("current_ph", "3.8")
        .add_param("target_ph", "3.4")
        .add_param("acid_type", "malic");

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
        .add_param("total_volume", "19")
        .add_param("bottle_size", "750");

    let result = calc.calculate(input).unwrap();
    let expected = Decimal::from(25);
    assert_eq!(result.output.value.round(), expected);
}

#[test]
fn test_acid_invalid_ph_range() {
    let calc = AcidAdditionCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("current_ph", "3.2")
        .add_param("target_ph", "3.8")
        .add_param("acid_type", "tartaric");

    let result = calc.calculate(input);
    assert!(result.is_err());
}
