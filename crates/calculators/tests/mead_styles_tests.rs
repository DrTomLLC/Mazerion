// Mead styles calculator tests - ALL parameter names match calculator implementations

use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator};
use rust_decimal::Decimal;

#[test]
fn test_great_mead() {
    let calc = GreatMeadCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")              // FIXED: was "batch_size"
        .add_param("target_abv", "12");         // FIXED: was "target_gravity"
    // Removed: honey_sg not used by calculator

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::from(1000)); // Should be > 1kg in grams
    assert!(result.metadata.iter().any(|(k, _)| k == "target_abv"));
}

#[test]
fn test_hydromel() {
    let calc = HydromelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")              // FIXED: was "batch_size"
        .add_param("target_abv", "5");          // Session mead ABV

    let result = calc.calculate(input).unwrap();
    // Math: 19L × 5% ABV × 135g/L/% = 12,825g = 12.825kg
    assert!(result.output.value > Decimal::from(10000)); // Should be ~12.8kg
    assert!(result.output.value < Decimal::from(15000)); // Reasonable range for 19L at 5%
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
        .add_param("volume", "19")              // FIXED: was "batch_size"
        .add_param("target_abv", "12")          // Added
        .add_param("fruit_weight", "4.0")       // FIXED: was "fruit_kg"
        .add_param("fruit_type", "strawberry"); // FIXED: was "fruit_sugar"

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "fruit_sugar_g"));
    assert!(result.metadata.iter().any(|(k, _)| k == "honey_kg"));
}

#[test]
fn test_cyser() {
    let calc = CyserCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")              // FIXED: was "batch_size"
        .add_param("target_abv", "12")          // Added
        .add_param("juice_percent", "50");      // FIXED: was "juice_liters" and "juice_sg"

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "juice_volume_L"));
    assert!(result.metadata.iter().any(|(k, _)| k == "honey_kg"));
}

#[test]
fn test_bochet() {
    let calc = BochetCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")              // FIXED: was "batch_size"
        .add_param("target_abv", "14")          // Added
        .add_param("bochet_level", "medium");   // FIXED: was "caramelize_time" and "honey_kg"

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "caramel_level"));
    assert!(result.metadata.iter().any(|(k, _)| k == "sugar_loss"));
}

#[test]
fn test_braggot() {
    let calc = BraggotCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")              // FIXED: was "batch_size"
        .add_param("target_abv", "10")          // Added
        .add_param("honey_percent", "50")       // FIXED: was "honey_kg", "malt_kg", "malt_ppg", "efficiency"
        .add_param("malt_weight", "3.0");       // Added for malt contribution

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "honey_kg" || k == "honey_g"));
}

#[test]
fn test_metheglin() {
    let calc = MetheglinCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")              // FIXED: was "batch_size"
        .add_param("target_abv", "12")          // Added
        .add_param("spice_level", "medium");    // FIXED: was "honey_kg" and "spice_amount"

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "honey_kg"));
}

#[test]
fn test_acerglyn() {
    let calc = AcerglynCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("maple_percent", "30");      // Maple percentage of fermentables

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "honey_g")); // FIXED: was "honey"
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
        .add_param("volume", "19")              // FIXED: was "batch_size"
        .add_param("target_abv", "2");          // FIXED: use too low ABV instead of impossible gravity

    let result = calc.calculate(input);
    // Should either error for out of range ABV or succeed with warning
    assert!(result.is_err() || result.unwrap().warnings.len() > 0);
}