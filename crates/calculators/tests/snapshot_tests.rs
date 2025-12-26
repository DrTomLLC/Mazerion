//! SNAPSHOT/REGRESSION TESTS - CORRECT APIS VERSION
//! Location: crates/calculators/tests/snapshot_tests.rs

use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator};
use rust_decimal::Decimal;
use std::str::FromStr;

// ============================================================================
// ABV FORMULA SNAPSHOTS
// ============================================================================

#[test]
fn snapshot_abv_standard_cases() {
    let calc = AbvCalculator::default();

    // Standard ale: OG 1.050 -> FG 1.010
    let input1 = CalcInput::new()
        .add_param("og", "1.050")
        .add_param("fg", "1.010");
    let result1 = calc.calculate(input1).expect("Standard ale calculation failed");

    // ABV = (1.050 - 1.010) × 131.25 = 5.25%
    let expected1 = Decimal::from_str("5.25").expect("Parse 5.25 failed");
    assert_eq!(result1.output.value, expected1, "Standard ale ABV mismatch");

    // High gravity beer: OG 1.080 -> FG 1.015
    let input2 = CalcInput::new()
        .add_param("og", "1.080")
        .add_param("fg", "1.015");
    let result2 = calc.calculate(input2).expect("High gravity calculation failed");

    // ABV = (1.080 - 1.015) × 131.25 = 8.53125%
    let expected2 = Decimal::from_str("8.53125").expect("Parse 8.53125 failed");
    assert_eq!(result2.output.value, expected2, "High gravity ABV mismatch");

    // Traditional mead: OG 1.110 -> FG 1.010
    let input3 = CalcInput::new()
        .add_param("og", "1.110")
        .add_param("fg", "1.010");
    let result3 = calc.calculate(input3).expect("Mead calculation failed");

    // ABV = (1.110 - 1.010) × 131.25 = 13.125%
    let expected3 = Decimal::from_str("13.125").expect("Parse 13.125 failed");
    assert_eq!(result3.output.value, expected3, "Traditional mead ABV mismatch");
}

// ============================================================================
// BRIX/SG CONVERSION KNOWN VALUES
// ============================================================================

#[test]
fn snapshot_brix_to_sg_known_values() {
    let calc = BrixToSgCalculator::default();

    use mazerion_core::Measurement;

    // Test known Brix to SG conversions with ACTUAL formula results
    let test_cases = vec![
        ("0.0", "1.000"),
        ("10.0", "1.040"),
        ("20.0", "1.083"),
        ("25.0", "1.106"),
    ];

    for (brix_str, expected_sg_str) in test_cases {
        let brix = Decimal::from_str(brix_str).expect("Parse Brix failed");
        let brix_meas = Measurement::brix(brix).expect("Create Brix measurement failed");
        let input = CalcInput::new().add_measurement(brix_meas);

        let result = calc.calculate(input).expect("Brix to SG calculation failed");
        let expected_sg = Decimal::from_str(expected_sg_str).expect("Parse expected SG failed");

        // Allow 0.001 tolerance for rounding
        let diff = (result.output.value - expected_sg).abs();
        let tolerance = Decimal::from_str("0.001").expect("Parse tolerance failed");

        assert!(
            diff < tolerance,
            "Brix {} -> SG: expected {}, got {}",
            brix_str,
            expected_sg_str,
            result.output.value
        );
    }
}

#[test]
fn snapshot_sg_to_brix_known_values() {
    let calc = SgToBrixCalculator::default();

    use mazerion_core::Measurement;

    // Test known SG to Brix conversions
    let test_cases = vec![
        ("1.000", "0.0"),
        ("1.040", "10.0"),
        ("1.083", "20.0"),
    ];

    for (sg_str, expected_brix_str) in test_cases {
        let sg = Decimal::from_str(sg_str).expect("Parse SG failed");
        let sg_meas = Measurement::sg(sg).expect("Create SG measurement failed");
        let input = CalcInput::new().add_measurement(sg_meas);

        let result = calc.calculate(input).expect("SG to Brix calculation failed");
        let expected_brix = Decimal::from_str(expected_brix_str).expect("Parse expected Brix failed");

        // Allow 0.1 tolerance for rounding
        let diff = (result.output.value - expected_brix).abs();
        let tolerance = Decimal::from_str("0.1").expect("Parse tolerance failed");

        assert!(
            diff < tolerance,
            "SG {} -> Brix: expected {}, got {}",
            sg_str,
            expected_brix_str,
            result.output.value
        );
    }
}

// ============================================================================
// DILUTION KNOWN CALCULATIONS
// ============================================================================

#[test]
fn snapshot_dilution_known_calculations() {
    let calc = DilutionCalculator::default();

    // Known case: 20L at 14% ABV diluted to 10% ABV
    let input = CalcInput::new()
        .add_param("current_volume", "20")
        .add_param("current_abv", "14")
        .add_param("target_abv", "10");

    let result = calc.calculate(input).expect("Dilution calculation failed");

    // Water needed = 20 × (14 - 10) / 10 = 8 L
    let expected = Decimal::from_str("8.0").expect("Parse expected water failed");

    // Allow 0.1L tolerance
    let diff = (result.output.value - expected).abs();
    let tolerance = Decimal::from_str("0.1").expect("Parse tolerance failed");

    assert!(
        diff < tolerance,
        "Dilution: expected {} L water, got {} L",
        expected,
        result.output.value
    );
}

// ============================================================================
// IBU TINSETH FORMULA SNAPSHOT
// ============================================================================

#[test]
fn snapshot_ibu_tinseth_formula() {
    let calc = IbuCalculator::default();

    // Test case: 28g hops, 10% AA, 60min boil, 23L, OG 1.050
    let input = CalcInput::new()
        .add_param("hop_weight_g", "28")
        .add_param("alpha_acid", "10.0")
        .add_param("boil_time", "60")
        .add_param("volume_l", "23")
        .add_param("boil_gravity", "1.050");

    let result = calc.calculate(input).expect("IBU calculation failed");

    // Expected IBU: ~28 (based on actual test output)
    let min_ibu = Decimal::from_str("27.0").expect("Parse min IBU failed");
    let max_ibu = Decimal::from_str("29.0").expect("Parse max IBU failed");

    assert!(
        result.output.value >= min_ibu && result.output.value <= max_ibu,
        "IBU calculation outside expected range: {} (expected 27-29)",
        result.output.value
    );
}

// ============================================================================
// CARBONATION PRIMING SUGAR SNAPSHOT
// ============================================================================

#[test]
fn snapshot_carbonation_priming_sugar() {
    let calc = CarbonationCalculator::default();

    // 19L batch at 20°C to 2.5 volumes CO2
    let input = CalcInput::new()
        .add_param("temperature", "20")
        .add_param("volume", "19")
        .add_param("target_co2", "2.5")
        .add_param("method", "priming");

    let result = calc.calculate(input).expect("Carbonation calculation failed");

    // Expected priming sugar: 120-130g for 19L
    let min_sugar = Decimal::from_str("120.0").expect("Parse min sugar failed");
    let max_sugar = Decimal::from_str("130.0").expect("Parse max sugar failed");

    assert!(
        result.output.value >= min_sugar && result.output.value <= max_sugar,
        "Priming sugar outside expected range: {} g (expected 120-130g)",
        result.output.value
    );
}

// ============================================================================
// REGRESSION TESTS - FORMULA DETECTION
// ============================================================================

#[test]
fn regression_abv_formula_unchanged() {
    let calc = AbvCalculator::default();

    // Test that ABV formula remains: (OG - FG) × 131.25
    let og = Decimal::from_str("1.060").expect("Parse OG failed");
    let fg = Decimal::from_str("1.015").expect("Parse FG failed");

    let input = CalcInput::new()
        .add_param("og", "1.060")
        .add_param("fg", "1.015");

    let result = calc.calculate(input).expect("ABV calculation failed");

    // Manual calculation: (1.060 - 1.015) × 131.25 = 5.90625
    let expected = (og - fg) * Decimal::from_str("131.25").expect("Parse multiplier failed");

    assert_eq!(
        result.output.value, expected,
        "ABV formula has changed! Expected (OG-FG)×131.25"
    );
}

#[test]
fn regression_hydrometer_correction_formula() {
    let calc = HydrometerCorrectionCalculator::default();

    // At calibration temp (68°F), correction should be zero
    // HydrometerCorrectionCalculator uses STRING PARAMS not Measurements!
    let input = CalcInput::new()
        .add_param("measured_sg", "1.050")
        .add_param("sample_temp", "68")
        .add_param("calibration_temp", "68");

    let result = calc.calculate(input).expect("Hydrometer calculation failed");

    // At calibration temp, corrected SG should equal measured SG
    let measured_sg = Decimal::from_str("1.050").expect("Parse measured SG failed");
    let diff = (result.output.value - measured_sg).abs();
    let tolerance = Decimal::from_str("0.001").expect("Parse tolerance failed");

    assert!(
        diff < tolerance,
        "Hydrometer correction at calibration temp should be zero. Got diff: {}",
        diff
    );
}

#[test]
fn regression_decimal_precision_maintained() {
    let calc = AbvCalculator::default();

    // Test that rust_decimal precision is maintained
    let input = CalcInput::new()
        .add_param("og", "1.0505")
        .add_param("fg", "1.0100");

    let result = calc.calculate(input).expect("ABV calculation failed");

    // Should preserve decimal precision
    assert!(result.output.value.to_string().len() > 4, "Decimal precision lost");
}

// ============================================================================
// EDGE CASE SNAPSHOTS
// ============================================================================

#[test]
fn snapshot_edge_cases_minimum_values() {
    let calc = AbvCalculator::default();

    // Minimum ABV: OG 1.001 -> FG 1.000
    let input = CalcInput::new()
        .add_param("og", "1.001")
        .add_param("fg", "1.000");

    let result = calc.calculate(input).expect("Min ABV calculation failed");

    // ABV = 0.001 × 131.25 = 0.13125%
    let expected = Decimal::from_str("0.13125").expect("Parse expected failed");
    assert_eq!(result.output.value, expected, "Minimum ABV case failed");
}

#[test]
fn snapshot_edge_cases_maximum_values() {
    let calc = AbvCalculator::default();

    // Maximum ABV: OG 1.150 -> FG 1.000
    let input = CalcInput::new()
        .add_param("og", "1.150")
        .add_param("fg", "1.000");

    let result = calc.calculate(input).expect("Max ABV calculation failed");

    // ABV = 0.150 × 131.25 = 19.6875%
    let expected = Decimal::from_str("19.6875").expect("Parse expected failed");
    assert_eq!(result.output.value, expected, "Maximum ABV case failed");

    // ABV calculator may or may not generate warnings - don't check for them
}

// ============================================================================
// METADATA CONSISTENCY TESTS
// ============================================================================

#[test]
fn snapshot_metadata_fields_present() {
    let calc = AbvCalculator::default();

    let input = CalcInput::new()
        .add_param("og", "1.050")
        .add_param("fg", "1.010");

    let result = calc.calculate(input).expect("Metadata test calculation failed");

    // Verify expected metadata fields exist
    let has_og = result.metadata.iter().any(|(k, _)| k == "og");
    let has_fg = result.metadata.iter().any(|(k, _)| k == "fg");
    let has_formula = result.metadata.iter().any(|(k, _)| k == "formula");

    assert!(has_og, "Missing 'og' metadata field");
    assert!(has_fg, "Missing 'fg' metadata field");
    assert!(has_formula, "Missing 'formula' metadata field");
}

// ============================================================================
// VERSION COMPATIBILITY TEST
// ============================================================================

#[test]
fn compatibility_v0_25_0_calculations() {
    // Ensure v0.25.0 calculations remain identical
    let abv_calc = AbvCalculator::default();

    // v0.25.0 test vector: OG 1.070 -> FG 1.012
    let input = CalcInput::new()
        .add_param("og", "1.070")
        .add_param("fg", "1.012");

    let result = abv_calc.calculate(input).expect("v0.25.0 compatibility test failed");

    // v0.25.0 result: (1.070 - 1.012) × 131.25 = 7.6125%
    let expected = Decimal::from_str("7.6125").expect("Parse v0.25.0 expected failed");

    assert_eq!(
        result.output.value, expected,
        "v0.25.0 compatibility broken! ABV calculation changed"
    );
}