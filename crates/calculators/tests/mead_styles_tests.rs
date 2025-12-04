use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator};
use rust_decimal::Decimal;

#[test]
fn test_great_mead() {
    let calc = GreatMeadCalculator::default();
    let input = CalcInput::new()
        .add_param("batch_size", "19")
        .add_param("target_gravity", "1.120")
        .add_param("honey_sg", "1.425");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::from(5));
    assert!(result.metadata.iter().any(|(k, _)| k == "estimated_abv"));
}

#[test]
fn test_hydromel() {
    let calc = HydromelCalculator::default();
    let input = CalcInput::new()
        .add_param("batch_size", "19")
        .add_param("target_gravity", "1.050")
        .add_param("honey_sg", "1.425");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
    assert!(result.output.value < Decimal::from(4));
}

#[test]
fn test_sack_mead() {
    let calc = SackCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "16");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_melomel() {
    let calc = MelomelCalculator::default();
    let input = CalcInput::new()
        .add_param("batch_size", "19")
        .add_param("honey_kg", "3.6")
        .add_param("fruit_kg", "4.0")
        .add_param("fruit_sugar", "12")
        .add_param("honey_sg", "1.425");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "total_sugar"));
    assert!(result.metadata.iter().any(|(k, _)| k == "estimated_og"));
}

#[test]
fn test_cyser() {
    let calc = CyserCalculator::default();
    let input = CalcInput::new()
        .add_param("batch_size", "19")
        .add_param("honey_kg", "2.0")
        .add_param("juice_liters", "15")
        .add_param("juice_sg", "1.050")
        .add_param("honey_sg", "1.425");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "estimated_og"));
    assert!(result.metadata.iter().any(|(k, _)| k == "estimated_abv"));
}

#[test]
fn test_bochet() {
    let calc = BochetCalculator::default();
    let input = CalcInput::new()
        .add_param("batch_size", "19")
        .add_param("honey_kg", "4.0")
        .add_param("caramelize_time", "60")
        .add_param("honey_sg", "1.425");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "color_estimate"));
    assert!(result.metadata.iter().any(|(k, _)| k == "estimated_og"));
}

#[test]
fn test_braggot() {
    let calc = BraggotCalculator::default();
    let input = CalcInput::new()
        .add_param("batch_size", "19")
        .add_param("honey_kg", "2.0")
        .add_param("malt_kg", "3.0")
        .add_param("malt_ppg", "37")
        .add_param("efficiency", "75")
        .add_param("honey_sg", "1.425");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "honey_gravity"));
    assert!(result.metadata.iter().any(|(k, _)| k == "malt_gravity"));
    assert!(result.metadata.iter().any(|(k, _)| k == "total_og"));
}

#[test]
fn test_metheglin() {
    let calc = MetheglinCalculator::default();
    let input = CalcInput::new()
        .add_param("batch_size", "19")
        .add_param("honey_kg", "3.6")
        .add_param("spice_amount", "50")
        .add_param("honey_sg", "1.425");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "estimated_og"));
}

#[test]
fn test_acerglyn() {
    let calc = AcerglynCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("maple_percentage", "30");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "honey"));
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
        .add_param("batch_size", "19")
        .add_param("target_gravity", "0.990")
        .add_param("honey_sg", "1.425");

    let result = calc.calculate(input);
    assert!(result.is_err());
}