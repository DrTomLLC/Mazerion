//! Comprehensive tests for ALL mead calculators - UPDATED with correct formula
//! Place in: crates/calculators/tests/all_mead_styles_tests.rs

use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator};
use rust_decimal::Decimal;

// TRADITIONAL MEAD TESTS

#[test]
fn test_traditional_2gal_12abv() {
    let calc = GreatMeadCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "7.57")
        .add_param("target_abv", "12");

    let result = calc.calculate(input).unwrap();

    // 7.57 × 12 × 33 = 2,998 g = 6.6 lb
    let expected = Decimal::from(2998);
    assert!(
        (result.output.value - expected).abs() < Decimal::from(150),
        "Got {} g, expected {} g",
        result.output.value,
        expected
    );
}

#[test]
fn test_traditional_5gal_12abv() {
    let calc = GreatMeadCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "18.93")
        .add_param("target_abv", "12");

    let result = calc.calculate(input).unwrap();

    // 18.93 × 12 × 33 = 7,496 g = 16.5 lb
    let expected = Decimal::from(7496);
    assert!(
        (result.output.value - expected).abs() < Decimal::from(200),
        "Got {} g, expected {} g",
        result.output.value,
        expected
    );
}

// HYDROMEL TESTS

#[test]
fn test_hydromel_5gal_5abv() {
    let calc = HydromelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "18.93")
        .add_param("target_abv", "5");

    let result = calc.calculate(input).unwrap();

    // 18.93 × 5 × 33 = 3,123 g
    let expected = Decimal::from(3123);
    assert!((result.output.value - expected).abs() < Decimal::from(100));
}

// SACK MEAD TESTS

#[test]
fn test_sack_5gal_16abv() {
    let calc = SackCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "18.93")
        .add_param("target_abv", "16");

    let result = calc.calculate(input).unwrap();

    // 18.93 × 16 × 33 = 9,994 g = 22 lb
    let expected = Decimal::from(9994);
    assert!((result.output.value - expected).abs() < Decimal::from(200));
}

// MELOMEL TESTS

#[test]
fn test_melomel_5gal_12abv_6lb_raspberry() {
    let calc = MelomelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "18.93")
        .add_param("target_abv", "12")
        .add_param("fruit_weight", "2.72")
        .add_param("fruit_type", "raspberry");

    let result = calc.calculate(input).unwrap();

    // Total: 18.93 × 12 × 33 = 7,496 g
    // Fruit sugar: 2720 × 0.05 = 136 g
    // Honey: 7,496 - 136 = 7,360 g = 16.2 lb
    let expected = Decimal::from(7360);
    assert!(
        (result.output.value - expected).abs() < Decimal::from(200),
        "Got {} g, expected {} g",
        result.output.value,
        expected
    );
}

#[test]
fn test_melomel_strawberry() {
    let calc = MelomelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("fruit_weight", "3.0")
        .add_param("fruit_type", "strawberry");

    let result = calc.calculate(input).unwrap();

    // Total: 19 × 12 × 33 = 7,524 g
    // Fruit: 3000 × 0.06 = 180 g
    // Honey: 7,344 g
    let expected = Decimal::from(7344);
    assert!((result.output.value - expected).abs() < Decimal::from(200));
}

#[test]
fn test_melomel_cherry() {
    let calc = MelomelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("fruit_weight", "4.0")
        .add_param("fruit_type", "cherry");

    let result = calc.calculate(input).unwrap();

    // Total: 7,524 g
    // Fruit: 4000 × 0.12 = 480 g
    // Honey: 7,044 g
    let expected = Decimal::from(7044);
    assert!((result.output.value - expected).abs() < Decimal::from(200));
}

// CYSER TESTS

#[test]
fn test_cyser_50percent() {
    let calc = CyserCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("juice_percent", "50");

    let result = calc.calculate(input).unwrap();

    // Total: 19 × 12 × 33 = 7,524 g
    // Juice: 9.5 L × 104 g/L = 988 g
    // Honey: 6,536 g
    let expected = Decimal::from(6536);
    assert!((result.output.value - expected).abs() < Decimal::from(200));
}

// ACERGLYN TESTS

#[test]
fn test_acerglyn_30percent() {
    let calc = AcerglynCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("maple_percent", "30");

    let result = calc.calculate(input).unwrap();

    // Total: 7,524 g sugar
    // 70% from honey = 5,267 g
    let expected = Decimal::from(5267);
    assert!((result.output.value - expected).abs() < Decimal::from(200));
}

// BOCHET TESTS

#[test]
fn test_bochet_medium() {
    let calc = BochetCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "14")
        .add_param("bochet_level", "medium");

    let result = calc.calculate(input).unwrap();

    // Need: 19 × 14 × 33 = 8,778 g
    // Medium = 10% loss
    // Start: 8,778 / 0.90 = 9,753 g
    let expected = Decimal::from(9753);
    assert!((result.output.value - expected).abs() < Decimal::from(200));
}

// BRAGGOT TESTS

#[test]
fn test_braggot_50percent() {
    let calc = BraggotCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "10")
        .add_param("honey_percent", "50")
        .add_param("malt_weight", "3.0");

    let result = calc.calculate(input).unwrap();

    // Total: 19 × 10 × 33 = 6,270 g
    // 50% from honey = 3,135 g
    let expected = Decimal::from(3135);
    assert!((result.output.value - expected).abs() < Decimal::from(150));
}

// CAPSICUMEL TESTS

#[test]
fn test_capsicumel() {
    let calc = CapsicumelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12");

    let result = calc.calculate(input).unwrap();

    // 19 × 12 × 33 = 7,524 g
    let expected = Decimal::from(7524);
    assert!((result.output.value - expected).abs() < Decimal::from(150));
}

// METHEGLIN TESTS

#[test]
fn test_metheglin() {
    let calc = MetheglinCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("spice_level", "medium");

    let result = calc.calculate(input).unwrap();

    // 19 × 12 × 33 = 7,524 g
    let expected = Decimal::from(7524);
    assert!((result.output.value - expected).abs() < Decimal::from(150));
}