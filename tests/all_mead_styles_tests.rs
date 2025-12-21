//! COMPREHENSIVE Mead Styles calculator tests - ALL 13 STYLES

use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator};
use rust_decimal::Decimal;

// ============================================================================
// TRADITIONAL MEAD
// ============================================================================

#[test]
fn test_traditional_basic() {
    let calc = GreatMeadCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12");
    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_traditional_low_abv() {
    let calc = GreatMeadCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "8");
    let result = calc.calculate(input).unwrap();
    // 19 L × 8% × 33 g/L/% = 5016 g
    assert_eq!(result.output.value, Decimal::from(5016));
}

#[test]
fn test_traditional_high_abv() {
    let calc = GreatMeadCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "18");
    let result = calc.calculate(input).unwrap();
    assert!(!result.warnings.is_empty());
}

// ============================================================================
// HYDROMEL
// ============================================================================

#[test]
fn test_hydromel_basic() {
    let calc = HydromelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "5");
    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_hydromel_session() {
    let calc = HydromelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "3.5");
    let result = calc.calculate(input).unwrap();
    // 19 L × 3.5% × 33 g/L/% = 2191.5 g
    assert!(result.output.value > Decimal::from(2000));
}

// ============================================================================
// SACK MEAD
// ============================================================================

#[test]
fn test_sack_basic() {
    let calc = SackCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "16");
    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::from(9000));
}

#[test]
fn test_sack_extreme() {
    let calc = SackCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "18");
    let result = calc.calculate(input).unwrap();
    assert!(!result.warnings.is_empty());
}

// ============================================================================
// MELOMEL
// ============================================================================

#[test]
fn test_melomel_strawberry() {
    let calc = MelomelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("fruit_weight", "2")
        .add_param("fruit_type", "strawberry");
    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
    assert!(result.metadata.iter().any(|(k, _)| k == "fruit_sugar_g"));
}

#[test]
fn test_melomel_blueberry_high_sugar() {
    let calc = MelomelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("fruit_weight", "3")
        .add_param("fruit_type", "blueberry");
    let result = calc.calculate(input).unwrap();
    // Blueberry is 10% sugar
    assert!(result.metadata.iter().any(|(k, v)| k == "fruit_sugar_g" && v.contains("300")));
}

#[test]
fn test_melomel_cherry() {
    let calc = MelomelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("fruit_weight", "2.5")
        .add_param("fruit_type", "cherry");
    let result = calc.calculate(input).unwrap();
    // Cherry is 12% sugar
    assert!(result.metadata.iter().any(|(k, _)| k == "fruit_sugar_g"));
}

// ============================================================================
// CYSER
// ============================================================================

#[test]
fn test_cyser_basic() {
    let calc = CyserCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("juice_percent", "40");
    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
    assert!(result.metadata.iter().any(|(k, _)| k == "juice_volume_L"));
}

#[test]
fn test_cyser_high_juice() {
    let calc = CyserCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("juice_percent", "60");
    let result = calc.calculate(input).unwrap();
    assert!(!result.warnings.is_empty());
}

// ============================================================================
// PYMENT
// ============================================================================

#[test]
fn test_pyment_basic() {
    let calc = PymentCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "13")
        .add_param("juice_percent", "40");
    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_pyment_wine_style() {
    let calc = PymentCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "14")
        .add_param("juice_percent", "50");
    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "abv_from_juice"));
}

// ============================================================================
// ACERGLYN
// ============================================================================

#[test]
fn test_acerglyn_basic() {
    let calc = AcerglynCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("maple_percent", "30");
    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_acerglyn_high_maple() {
    let calc = AcerglynCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("maple_percent", "50");
    let result = calc.calculate(input).unwrap();
    assert!(!result.warnings.is_empty());
}

// ============================================================================
// BOCHET
// ============================================================================

#[test]
fn test_bochet_light() {
    let calc = BochetCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "14")
        .add_param("bochet_level", "light");
    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, v)| k == "caramel_level" && v.contains("light")));
}

#[test]
fn test_bochet_dark() {
    let calc = BochetCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "14")
        .add_param("bochet_level", "dark");
    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, v)| k == "sugar_loss" && v.contains("15")));
}

// ============================================================================
// BRAGGOT
// ============================================================================

#[test]
fn test_braggot_balanced() {
    let calc = BraggotCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "10")
        .add_param("honey_percent", "50")
        .add_param("malt_weight", "3");
    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_braggot_honey_forward() {
    let calc = BraggotCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("honey_percent", "70")
        .add_param("malt_weight", "2");
    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "honey_kg"));
}

// ============================================================================
// CAPSICUMEL
// ============================================================================

#[test]
fn test_capsicumel_basic() {
    let calc = CapsicumelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12");
    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

// ============================================================================
// METHEGLIN
// ============================================================================

#[test]
fn test_metheglin_basic() {
    let calc = MetheglinCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("spice_level", "medium");
    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

#[test]
fn test_metheglin_heavy_spice() {
    let calc = MetheglinCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("spice_level", "heavy");
    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "honey_kg"));
}

// ============================================================================
// LACTOMEL
// ============================================================================

#[test]
fn test_lactomel_light() {
    let calc = LactomelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("lactose_level", "light");
    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, v)| k == "lactose_g" && v.contains("950")));
}

#[test]
fn test_lactomel_medium() {
    let calc = LactomelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("lactose_level", "medium");
    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, v)| k == "lactose_g" && v.contains("1900")));
}

#[test]
fn test_lactomel_heavy() {
    let calc = LactomelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("lactose_level", "heavy");
    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, v)| k == "lactose_g" && v.contains("2850")));
}

// ============================================================================
// OXYMEL
// ============================================================================

#[test]
fn test_oxymel_traditional_ratio() {
    let calc = OxymelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "1")
        .add_param("vinegar_percent", "80")
        .add_param("honey_percent", "20");
    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, v)| k == "ratio" && v.contains("4.0:1")));
}

#[test]
fn test_oxymel_balanced() {
    let calc = OxymelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "1")
        .add_param("vinegar_percent", "40")
        .add_param("honey_percent", "20");
    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, v)| k == "balance"));
}

#[test]
fn test_oxymel_sweet_tart() {
    let calc = OxymelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "1")
        .add_param("vinegar_percent", "30")
        .add_param("honey_percent", "30");
    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
}

// ============================================================================
// ERROR HANDLING
// ============================================================================

#[test]
fn test_melomel_missing_fruit_type() {
    let calc = MelomelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("fruit_weight", "2");
    let result = calc.calculate(input);
    assert!(result.is_err());
}

#[test]
fn test_cyser_missing_juice_percent() {
    let calc = CyserCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12");
    // Should work - juice_percent defaults to some value
    let result = calc.calculate(input);
    assert!(result.is_ok());
}

#[test]
fn test_oxymel_invalid_percentages() {
    let calc = OxymelCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "1")
        .add_param("vinegar_percent", "60")
        .add_param("honey_percent", "50");
    let result = calc.calculate(input);
    assert!(result.is_err());
}

#[test]
fn test_lactomel_missing_volume() {
    let calc = LactomelCalculator::default();
    let input = CalcInput::new()
        .add_param("target_abv", "12");
    let result = calc.calculate(input);
    assert!(result.is_err());
}