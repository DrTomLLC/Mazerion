use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator};
use rust_decimal::Decimal;

#[test]
fn test_ibu_tinseth_60min() {
    let calc = IbuCalculator::default();
    let input = CalcInput::new()
        .add_param("volume_l", "19")
        .add_param("hop_weight_g", "30")
        .add_param("alpha_acid", "6.5")
        .add_param("boil_time", "60")
        .add_param("boil_gravity", "1.050");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::ZERO);
    assert!(result.output.value < Decimal::from(100));
}

#[test]
fn test_ibu_tinseth_90min() {
    let calc = IbuCalculator::default();
    let input = CalcInput::new()
        .add_param("volume_l", "19")
        .add_param("hop_weight_g", "30")
        .add_param("alpha_acid", "6.5")
        .add_param("boil_time", "90")
        .add_param("boil_gravity", "1.050");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value > Decimal::from(20));
}

#[test]
fn test_ibu_high_gravity_effect() {
    let calc = IbuCalculator::default();
    let input_low = CalcInput::new()
        .add_param("volume_l", "19")
        .add_param("hop_weight_g", "30")
        .add_param("alpha_acid", "6.5")
        .add_param("boil_time", "60")
        .add_param("boil_gravity", "1.040");

    let input_high = CalcInput::new()
        .add_param("volume_l", "19")
        .add_param("hop_weight_g", "30")
        .add_param("alpha_acid", "6.5")
        .add_param("boil_time", "60")
        .add_param("boil_gravity", "1.080");

    let result_low = calc.calculate(input_low).unwrap();
    let result_high = calc.calculate(input_high).unwrap();

    assert!(result_low.output.value > result_high.output.value);
}

#[test]
fn test_ibu_short_boil() {
    let calc = IbuCalculator::default();
    let input = CalcInput::new()
        .add_param("volume_l", "19")
        .add_param("hop_weight_g", "30")
        .add_param("alpha_acid", "6.5")
        .add_param("boil_time", "15")
        .add_param("boil_gravity", "1.050");

    let result = calc.calculate(input).unwrap();
    assert!(result.output.value < Decimal::from(15));
}

#[test]
fn test_ibu_multiple_additions() {
    let calc = IbuCalculator::default();

    let input1 = CalcInput::new()
        .add_param("volume_l", "19")
        .add_param("hop_weight_g", "20")
        .add_param("alpha_acid", "6.5")
        .add_param("boil_time", "60")
        .add_param("boil_gravity", "1.050");

    let input2 = CalcInput::new()
        .add_param("volume_l", "19")
        .add_param("hop_weight_g", "10")
        .add_param("alpha_acid", "6.5")
        .add_param("boil_time", "15")
        .add_param("boil_gravity", "1.050");

    let result1 = calc.calculate(input1).unwrap();
    let result2 = calc.calculate(input2).unwrap();

    let total_ibu = result1.output.value + result2.output.value;
    assert!(total_ibu > Decimal::ZERO);
}

#[test]
fn test_ibu_validation_missing_params() {
    let calc = IbuCalculator::default();
    let input = CalcInput::new()
        .add_param("volume_l", "19")
        .add_param("hop_weight_g", "30");

    let result = calc.calculate(input);
    assert!(result.is_err());
}

#[test]
fn test_ibu_validation_invalid_alpha_acid() {
    let calc = IbuCalculator::default();
    let input = CalcInput::new()
        .add_param("volume_l", "19")
        .add_param("hop_weight_g", "30")
        .add_param("alpha_acid", "25")
        .add_param("boil_time", "60")
        .add_param("boil_gravity", "1.050");

    let result = calc.calculate(input);
    assert!(result.is_err() || !result.unwrap().warnings.is_empty());
}

#[test]
fn test_ibu_high_amount_warning() {
    let calc = IbuCalculator::default();
    let input = CalcInput::new()
        .add_param("volume_l", "19")
        .add_param("hop_weight_g", "200")
        .add_param("alpha_acid", "12")
        .add_param("boil_time", "90")
        .add_param("boil_gravity", "1.050");

    let result = calc.calculate(input).unwrap();
    assert!(!result.warnings.is_empty());
}
