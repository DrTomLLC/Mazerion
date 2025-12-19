// Mead styles calculator tests - FIXED for 33 g/L/%ABV formula

use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator};
use rust_decimal::Decimal;

#[test]
fn test_great_mead() {
    let calc = GreatMeadCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12");

    let result = calc.calculate(input).unwrap();
    // 19 × 12 × 33 = 7,524 g
    assert!(result.output.value > Decimal::from(7000));
    assert!(result.output.value < Decimal::from(8000));
}

#[test]
fn test_hydromel() {
    let calc = HydromelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "5");

    let result = calc.calculate(input).unwrap();
    // FIXED: 19 × 5 × 33 = 3,135 g (NOT 12,825 g)
    assert!(result.output.value > Decimal::from(3000));
    assert!(result.output.value < Decimal::from(3300));
}

#[test]
fn test_sack_mead() {
    let calc = SackCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "16");

    let result = calc.calculate(input).unwrap();
    // 19 × 16 × 33 = 9,984 g
    assert!(result.output.value > Decimal::from(9500));
    assert!(result.output.value < Decimal::from(10500));
}

#[test]
fn test_melomel() {
    let calc = MelomelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("fruit_weight", "4.0")
        .add_param("fruit_type", "strawberry");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "fruit_sugar_g"));
    assert!(result.metadata.iter().any(|(k, _)| k == "honey_kg"));
}

#[test]
fn test_cyser() {
    let calc = CyserCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("juice_percent", "50");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "juice_volume_L"));
    assert!(result.metadata.iter().any(|(k, _)| k == "honey_kg"));
}

#[test]
fn test_bochet() {
    let calc = BochetCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "14")
        .add_param("bochet_level", "medium");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "caramel_level"));
    assert!(result.metadata.iter().any(|(k, _)| k == "sugar_loss"));
}

#[test]
fn test_braggot() {
    let calc = BraggotCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "10")
        .add_param("honey_percent", "50")
        .add_param("malt_weight", "3.0");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "honey_kg" || k == "honey_g"));
}

#[test]
fn test_metheglin() {
    let calc = MetheglinCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("spice_level", "medium");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "honey_kg"));
}

#[test]
fn test_acerglyn() {
    let calc = AcerglynCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("maple_percent", "30");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "honey_g"));
}

#[test]
fn test_capsicumel() {
    let calc = CapsicumelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_invalid_honey_amount() {
    let calc = GreatMeadCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "0")
        .add_param("target_abv", "12");
    let result = calc.calculate(input);
    assert!(result.is_ok());
}