use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator};
use rust_decimal::Decimal;
use std::str::FromStr;

#[test]
fn test_cost_calculator_basic() {
    let calc = CostCalculator::default();
    let input = CalcInput::new()
        .add_param("honey_kg", "3.6")
        .add_param("honey_price_per_kg", "25")
        .add_param("batch_size", "19");

    let result = calc.calculate(input).unwrap();
    let expected = Decimal::from_str("90.0").unwrap();
    assert!((result.output.value - expected).abs() < Decimal::ONE);
}

#[test]
fn test_cost_calculator_with_extras() {
    let calc = CostCalculator::default();
    let input = CalcInput::new()
        .add_param("honey_kg", "3.6")
        .add_param("honey_price_per_kg", "25")
        .add_param("fruit_kg", "4.0")
        .add_param("fruit_price_per_kg", "5")
        .add_param("yeast_cost", "10")
        .add_param("nutrient_cost", "8")
        .add_param("other_cost", "12")
        .add_param("batch_size", "19");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::from(100));
    assert!(result.metadata.iter().any(|(k, _)| k == "cost_per_liter"));
}

#[test]
fn test_priming_alternatives_corn_sugar() {
    let calc = PrimingAlternativesCalculator::default();
    let input = CalcInput::new()
        .add_param("corn_sugar_g", "120");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "table_sugar"));
    assert!(result.metadata.iter().any(|(k, _)| k == "dme"));
    assert!(result.metadata.iter().any(|(k, _)| k == "honey"));
}

#[test]
fn test_priming_alternatives_table_sugar() {
    let calc = PrimingAlternativesCalculator::default();
    let input = CalcInput::new()
        .add_param("table_sugar_g", "100");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "corn_sugar"));
}

#[test]
fn test_water_chemistry_basic() {
    let calc = WaterChemistryCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("calcium_ppm", "50")
        .add_param("magnesium_ppm", "10")
        .add_param("sodium_ppm", "20");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "total_hardness"));
}

#[test]
fn test_water_chemistry_with_additions() {
    let calc = WaterChemistryCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("calcium_ppm", "50")
        .add_param("target_calcium", "100")
        .add_param("gypsum_g", "5");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "calcium_from_gypsum"));
}

#[test]
fn test_bench_trials() {
    let calc = BenchTrialsCalculator::default();
    let input = CalcInput::new()
        .add_param("trial_volume", "100")
        .add_param("trial_addition", "0.5")
        .add_param("batch_volume", "19000");

    let result = calc.calculate(input).unwrap();
    let expected = Decimal::from_str("95.0").unwrap();
    assert!((result.output.value - expected).abs() < Decimal::ONE);
}

#[test]
fn test_bench_trials_validation() {
    let calc = BenchTrialsCalculator::default();
    let input = CalcInput::new()
        .add_param("trial_volume", "100");

    let result = calc.calculate(input);
    assert!(result.is_err());
}

#[test]
fn test_cost_per_bottle() {
    let calc = CostCalculator::default();
    let input = CalcInput::new()
        .add_param("honey_kg", "3.6")
        .add_param("honey_price_per_kg", "25")
        .add_param("batch_size", "19")
        .add_param("bottle_size", "750");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "cost_per_bottle"));
}

#[test]
fn test_water_adjustment_calcium_chloride() {
    let calc = WaterChemistryCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("calcium_ppm", "30")
        .add_param("target_calcium", "100")
        .add_param("cacl2_g", "8");

    let result = calc.calculate(input).unwrap();
    assert!(result.metadata.iter().any(|(k, _)| k == "calcium_from_cacl2"));
}