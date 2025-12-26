//! COMPREHENSIVE ERROR HANDLING TEST SUITE
//! Tests EVERY error path for EVERY calculator
//! Location: crates/calculators/tests/error_handling_tests.rs

use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator, Measurement};
use rust_decimal::Decimal;
use std::str::FromStr;

// ============================================================================
// ABV CALCULATOR ERROR TESTS
// ============================================================================

#[test]
fn test_abv_missing_og() {
    let calc = AbvCalculator::default();
    let input = CalcInput::new().add_param("fg", "1.010");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when OG missing");
}

#[test]
fn test_abv_missing_fg() {
    let calc = AbvCalculator::default();
    let input = CalcInput::new().add_param("og", "1.050");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when FG missing");
}

#[test]
fn test_abv_invalid_og() {
    let calc = AbvCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "not_a_number")
        .add_param("fg", "1.010");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error on invalid OG");
}

#[test]
fn test_abv_invalid_fg() {
    let calc = AbvCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.050")
        .add_param("fg", "not_a_number");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error on invalid FG");
}

#[test]
fn test_abv_fg_greater_than_og() {
    let calc = AbvCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.010")
        .add_param("fg", "1.050");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when FG > OG");
}

#[test]
fn test_abv_negative_gravity() {
    let calc = AbvCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "-1.050")
        .add_param("fg", "1.010");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error on negative gravity");
}

// ============================================================================
// BRIX/SG CALCULATOR ERROR TESTS
// ============================================================================

#[test]
fn test_brix_to_sg_missing_measurement() {
    let calc = BrixToSgCalculator::default();
    let input = CalcInput::new(); // No measurement

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when Brix measurement missing");
}

#[test]
fn test_brix_to_sg_negative_brix() {
    let _calc = BrixToSgCalculator::default();
    let brix = Decimal::from_str("-10.0").expect("Parse negative Brix");

    // Measurement::brix should reject negative values
    let result = Measurement::brix(brix);
    assert!(result.is_err(), "Should error on negative Brix");
}

#[test]
fn test_sg_to_brix_missing_measurement() {
    let calc = SgToBrixCalculator::default();
    let input = CalcInput::new(); // No measurement

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when SG measurement missing");
}

#[test]
fn test_sg_to_brix_invalid_sg() {
    let _calc = SgToBrixCalculator::default();

    // SG way out of range
    let sg = Decimal::from_str("5.000").expect("Parse invalid SG");
    let result = Measurement::sg(sg);

    assert!(result.is_err(), "Should error on SG > 2.0");
}

// ============================================================================
// DILUTION CALCULATOR ERROR TESTS
// ============================================================================

#[test]
fn test_dilution_missing_current_volume() {
    let calc = DilutionCalculator::default();
    let input = CalcInput::new()
        .add_param("current_abv", "14")
        .add_param("target_abv", "10");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when current_volume missing");
}

#[test]
fn test_dilution_missing_current_abv() {
    let calc = DilutionCalculator::default();
    let input = CalcInput::new()
        .add_param("current_volume", "20")
        .add_param("target_abv", "10");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when current_abv missing");
}

#[test]
fn test_dilution_missing_target_abv() {
    let calc = DilutionCalculator::default();
    let input = CalcInput::new()
        .add_param("current_volume", "20")
        .add_param("current_abv", "14");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when target_abv missing");
}

#[test]
fn test_dilution_target_greater_than_current() {
    let calc = DilutionCalculator::default();
    let input = CalcInput::new()
        .add_param("current_volume", "20")
        .add_param("current_abv", "10")
        .add_param("target_abv", "14");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when target ABV > current ABV");
}

#[test]
fn test_dilution_negative_volume() {
    let calc = DilutionCalculator::default();
    let input = CalcInput::new()
        .add_param("current_volume", "-20")
        .add_param("current_abv", "14")
        .add_param("target_abv", "10");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error on negative volume");
}

#[test]
fn test_dilution_zero_volume() {
    let calc = DilutionCalculator::default();
    let input = CalcInput::new()
        .add_param("current_volume", "0")
        .add_param("current_abv", "14")
        .add_param("target_abv", "10");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error on zero volume");
}

#[test]
fn test_dilution_zero_target_abv() {
    let calc = DilutionCalculator::default();
    let input = CalcInput::new()
        .add_param("current_volume", "20")
        .add_param("current_abv", "14")
        .add_param("target_abv", "0");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when target ABV = 0 (division by zero)");
}

// ============================================================================
// BLENDING CALCULATOR ERROR TESTS
// ============================================================================

#[test]
fn test_blending_missing_volume1() {
    let calc = BlendingCalculator::default();
    let input = CalcInput::new()
        .add_param("abv1", "12")
        .add_param("volume2", "10")
        .add_param("abv2", "8");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when volume1 missing");
}

#[test]
fn test_blending_missing_abv1() {
    let calc = BlendingCalculator::default();
    let input = CalcInput::new()
        .add_param("volume1", "10")
        .add_param("volume2", "10")
        .add_param("abv2", "8");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when abv1 missing");
}

#[test]
fn test_blending_negative_volume() {
    let calc = BlendingCalculator::default();
    let input = CalcInput::new()
        .add_param("volume1", "-10")
        .add_param("abv1", "12")
        .add_param("volume2", "10")
        .add_param("abv2", "8");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error on negative volume");
}

#[test]
fn test_blending_zero_total_volume() {
    let calc = BlendingCalculator::default();
    let input = CalcInput::new()
        .add_param("volume1", "0")
        .add_param("abv1", "12")
        .add_param("volume2", "0")
        .add_param("abv2", "8");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when total volume = 0");
}

// ============================================================================
// IBU CALCULATOR ERROR TESTS
// ============================================================================

#[test]
fn test_ibu_missing_hop_weight() {
    let calc = IbuCalculator::default();
    let input = CalcInput::new()
        .add_param("alpha_acid", "10.0")
        .add_param("boil_time", "60")
        .add_param("volume_l", "23")
        .add_param("boil_gravity", "1.050");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when hop_weight_g missing");
}

#[test]
fn test_ibu_missing_alpha_acid() {
    let calc = IbuCalculator::default();
    let input = CalcInput::new()
        .add_param("hop_weight_g", "28")
        .add_param("boil_time", "60")
        .add_param("volume_l", "23")
        .add_param("boil_gravity", "1.050");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when alpha_acid missing");
}

#[test]
fn test_ibu_invalid_alpha_acid() {
    let calc = IbuCalculator::default();
    let input = CalcInput::new()
        .add_param("hop_weight_g", "28")
        .add_param("alpha_acid", "150.0") // Way too high
        .add_param("boil_time", "60")
        .add_param("volume_l", "23")
        .add_param("boil_gravity", "1.050");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error on invalid alpha acid %");
}

#[test]
fn test_ibu_zero_volume() {
    let calc = IbuCalculator::default();
    let input = CalcInput::new()
        .add_param("hop_weight_g", "28")
        .add_param("alpha_acid", "10.0")
        .add_param("boil_time", "60")
        .add_param("volume_l", "0")
        .add_param("boil_gravity", "1.050");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error on zero volume");
}

// ============================================================================
// CARBONATION CALCULATOR ERROR TESTS
// ============================================================================

#[test]
fn test_carbonation_missing_temperature() {
    let calc = CarbonationCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_co2", "2.5")
        .add_param("method", "priming");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when temperature missing");
}

#[test]
fn test_carbonation_missing_volume() {
    let calc = CarbonationCalculator::default();
    let input = CalcInput::new()
        .add_param("temperature", "20")
        .add_param("target_co2", "2.5")
        .add_param("method", "priming");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when volume missing");
}

#[test]
fn test_carbonation_missing_target_co2() {
    let calc = CarbonationCalculator::default();
    let input = CalcInput::new()
        .add_param("temperature", "20")
        .add_param("volume", "19")
        .add_param("method", "priming");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when target_co2 missing");
}

#[test]
fn test_carbonation_invalid_method() {
    let calc = CarbonationCalculator::default();
    let input = CalcInput::new()
        .add_param("temperature", "20")
        .add_param("volume", "19")
        .add_param("target_co2", "2.5")
        .add_param("method", "invalid_method");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error on invalid method");
}

// ============================================================================
// HYDROMETER CORRECTION ERROR TESTS
// ============================================================================

#[test]
fn test_hydrometer_missing_measured_sg() {
    let calc = HydrometerCorrectionCalculator::default();
    let input = CalcInput::new()
        .add_param("sample_temp", "75")
        .add_param("calibration_temp", "68");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when measured_sg missing");
}

#[test]
fn test_hydrometer_missing_sample_temp() {
    let calc = HydrometerCorrectionCalculator::default();
    let input = CalcInput::new()
        .add_param("measured_sg", "1.050")
        .add_param("calibration_temp", "68");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when sample_temp missing");
}

#[test]
fn test_hydrometer_invalid_sg() {
    let calc = HydrometerCorrectionCalculator::default();
    let input = CalcInput::new()
        .add_param("measured_sg", "not_a_number")
        .add_param("sample_temp", "75")
        .add_param("calibration_temp", "68");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error on invalid SG");
}

// ============================================================================
// NUTRITION CALCULATOR ERROR TESTS
// ============================================================================

#[test]
fn test_nutrition_missing_volume() {
    let calc = NutritionCalculator::default();
    let input = CalcInput::new()
        .add_param("starting_gravity", "1.100")
        .add_param("target_abv", "14")
        .add_param("protocol", "tosna");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when volume missing");
}

#[test]
fn test_nutrition_missing_starting_gravity() {
    // starting_gravity is now OPTIONAL - calculates from target_abv
    let calc = NutritionCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "14")
        .add_param("protocol", "tosna");

    let result = calc.calculate(input);

    // Should succeed - starting_gravity is optional
    assert!(result.is_ok(), "starting_gravity is optional - calculates from ABV");

    let output = result.unwrap();
    assert!(output.output.value > Decimal::ZERO, "Should calculate nutrient amount");
}

#[test]
fn test_nutrition_invalid_abv() {
    let calc = NutritionCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("starting_gravity", "1.100")
        .add_param("target_abv", "50") // Way too high
        .add_param("protocol", "tosna");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error on unrealistic ABV");
}

// ============================================================================
// BACKSWEETENING CALCULATOR ERROR TESTS
// ============================================================================

#[test]
fn test_backsweetening_missing_measurement() {
    let calc = BacksweeteningCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("sweetener", "honey")
        .add_param("target_gravity", "1.015");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when current SG measurement missing");
}

#[test]
fn test_backsweetening_missing_volume() {
    let calc = BacksweeteningCalculator::default();
    let sg_meas = Measurement::sg(Decimal::from_str("1.000").expect("Parse SG"))
        .expect("Create SG measurement");
    let input = CalcInput::new()
        .add_measurement(sg_meas)
        .add_param("sweetener", "honey")
        .add_param("target_gravity", "1.015");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when volume missing");
}

#[test]
fn test_backsweetening_target_less_than_current() {
    let calc = BacksweeteningCalculator::default();
    let sg_meas = Measurement::sg(Decimal::from_str("1.010").expect("Parse SG"))
        .expect("Create SG measurement");
    let input = CalcInput::new()
        .add_measurement(sg_meas)
        .add_param("volume", "19")
        .add_param("sweetener", "honey")
        .add_param("target_gravity", "1.005"); // Less than current

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when target < current gravity");
}

// ============================================================================
// SULFITE CALCULATOR ERROR TESTS
// ============================================================================

#[test]
fn test_sulfite_missing_ph_measurement() {
    let calc = SulfiteCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_free_so2", "50")
        .add_param("sulfite_type", "kmeta");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when pH measurement missing");
}

#[test]
fn test_sulfite_missing_volume() {
    let calc = SulfiteCalculator::default();
    let ph_meas = Measurement::ph(Decimal::from_str("3.5").expect("Parse pH"))
        .expect("Create pH measurement");
    let input = CalcInput::new()
        .add_measurement(ph_meas)
        .add_param("target_free_so2", "50")
        .add_param("sulfite_type", "kmeta");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when volume missing");
}

#[test]
fn test_sulfite_invalid_ph() {
    let _calc = SulfiteCalculator::default();

    // pH 10 is way out of range for mead/wine
    let ph = Decimal::from_str("10.0").expect("Parse invalid pH");
    let result = Measurement::ph(ph);

    // Should error on invalid pH
    assert!(result.is_err(), "Should error on pH > 8.5");
}

// ============================================================================
// REFRACTOMETER CALCULATOR ERROR TESTS
// ============================================================================

#[test]
fn test_refractometer_missing_original_brix() {
    let calc = RefractometerCalculator::default();
    let input = CalcInput::new()
        .add_param("current_brix", "8.5");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when original_brix missing");
}

#[test]
fn test_refractometer_missing_current_brix() {
    let calc = RefractometerCalculator::default();
    let input = CalcInput::new()
        .add_param("original_brix", "18.0");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error when current_brix missing");
}

#[test]
fn test_refractometer_invalid_brix() {
    let calc = RefractometerCalculator::default();
    let input = CalcInput::new()
        .add_param("original_brix", "not_a_number")
        .add_param("current_brix", "8.5");

    let result = calc.calculate(input);
    assert!(result.is_err(), "Should error on invalid Brix");
}