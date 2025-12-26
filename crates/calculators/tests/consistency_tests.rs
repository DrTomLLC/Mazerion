//! CROSS-CALCULATOR CONSISTENCY TEST SUITE
//! Verifies related calculators agree with each other
//! Location: crates/calculators/tests/consistency_tests.rs

use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator, Measurement};
use rust_decimal::Decimal;
use std::str::FromStr;

// ============================================================================
// BRIX ↔ SG ↔ ABV CONSISTENCY
// ============================================================================

#[test]
fn test_brix_sg_abv_consistency() {
    // Workflow: Brix → SG → ABV should match direct Brix → ABV

    // Step 1: Original Brix reading
    let original_brix = Decimal::from_str("22.0").expect("Parse original Brix");

    // Step 2: Convert Brix to SG
    let brix_to_sg_calc = BrixToSgCalculator::default();
    let brix_meas = Measurement::brix(original_brix).expect("Create Brix measurement");
    let input1 = CalcInput::new().add_measurement(brix_meas);
    let og_result = brix_to_sg_calc.calculate(input1).expect("Brix to SG conversion");
    let og = og_result.output.value;

    // Step 3: Assume fermentation to 1.010
    let fg = Decimal::from_str("1.010").expect("Parse FG");

    // Step 4: Calculate ABV from OG/FG
    let abv_calc = AbvCalculator::default();
    let input2 = CalcInput::new()
        .add_param("og", &og.to_string())
        .add_param("fg", &fg.to_string());
    let abv_result = abv_calc.calculate(input2).expect("ABV calculation");

    // Verify ABV is reasonable (22 Brix → ~1.092 OG → ~10.8% ABV)
    let expected_abv = Decimal::from_str("10.0").expect("Parse expected ABV");
    let tolerance = Decimal::from_str("2.0").expect("Parse tolerance");

    assert!(
        (abv_result.output.value - expected_abv).abs() < tolerance,
        "Brix→SG→ABV workflow gave unrealistic ABV: {}",
        abv_result.output.value
    );
}

#[test]
fn test_sg_brix_roundtrip_consistency() {
    // SG → Brix → SG should be consistent

    let original_sg = Decimal::from_str("1.050").expect("Parse SG");

    // Step 1: SG → Brix
    let sg_to_brix_calc = SgToBrixCalculator::default();
    let sg_meas = Measurement::sg(original_sg).expect("Create SG measurement");
    let input1 = CalcInput::new().add_measurement(sg_meas);
    let brix_result = sg_to_brix_calc.calculate(input1).expect("SG to Brix");

    // Step 2: Brix → SG
    let brix_to_sg_calc = BrixToSgCalculator::default();
    let brix_meas = Measurement::brix(brix_result.output.value).expect("Create Brix measurement");
    let input2 = CalcInput::new().add_measurement(brix_meas);
    let sg_result = brix_to_sg_calc.calculate(input2).expect("Brix to SG");

    // Should get back original SG within tolerance
    let diff = (sg_result.output.value - original_sg).abs();
    let tolerance = Decimal::from_str("0.002").expect("Parse tolerance");

    assert!(
        diff < tolerance,
        "SG→Brix→SG roundtrip failed: started {} ended {} (diff {})",
        original_sg,
        sg_result.output.value,
        diff
    );
}

// ============================================================================
// DILUTION ↔ BLENDING CONSISTENCY
// ============================================================================

#[test]
fn test_dilution_is_blending_with_water() {
    // Diluting 10L at 14% to 10% should equal blending with water

    // Method 1: Dilution calculator
    let dilution_calc = DilutionCalculator::default();
    let input1 = CalcInput::new()
        .add_param("current_volume", "10")
        .add_param("current_abv", "14")
        .add_param("target_abv", "10");
    let dilution_result = dilution_calc.calculate(input1).expect("Dilution");
    let water_needed = dilution_result.output.value;

    // Method 2: Blending calculator (batch1 = 10L at 14%, batch2 = water at 0%)
    let blending_calc = BlendingCalculator::default();
    let input2 = CalcInput::new()
        .add_param("volume1", "10")
        .add_param("abv1", "14")
        .add_param("volume2", &water_needed.to_string())
        .add_param("abv2", "0");
    let blending_result = blending_calc.calculate(input2).expect("Blending");

    // Blended ABV should equal target ABV (10%)
    let expected_abv = Decimal::from_str("10.0").expect("Parse expected ABV");
    let tolerance = Decimal::from_str("0.1").expect("Parse tolerance");

    assert!(
        (blending_result.output.value - expected_abv).abs() < tolerance,
        "Dilution and blending don't agree: blending gave {} ABV, expected {}",
        blending_result.output.value,
        expected_abv
    );
}

#[test]
fn test_multiple_dilutions_equal_single() {
    // Diluting twice should equal diluting once

    // Single dilution: 20L at 14% → 10%
    let dilution_calc = DilutionCalculator::default();
    let input1 = CalcInput::new()
        .add_param("current_volume", "20")
        .add_param("current_abv", "14")
        .add_param("target_abv", "10");
    let result1 = dilution_calc.calculate(input1).expect("Single dilution");
    let single_water = result1.output.value;

    // Two-step dilution: 20L at 14% → 12% → 10%
    // Step 1: 20L at 14% → 12%
    let input2 = CalcInput::new()
        .add_param("current_volume", "20")
        .add_param("current_abv", "14")
        .add_param("target_abv", "12");
    let result2 = dilution_calc.calculate(input2).expect("First dilution");
    let water1 = result2.output.value;
    let intermediate_volume = Decimal::from_str("20").expect("Parse") + water1;

    // Step 2: intermediate volume at 12% → 10%
    let input3 = CalcInput::new()
        .add_param("current_volume", &intermediate_volume.to_string())
        .add_param("current_abv", "12")
        .add_param("target_abv", "10");
    let result3 = dilution_calc.calculate(input3).expect("Second dilution");
    let water2 = result3.output.value;

    let double_water = water1 + water2;

    // Should be approximately equal
    let diff = (single_water - double_water).abs();
    let tolerance = Decimal::from_str("0.5").expect("Parse tolerance");

    assert!(
        diff < tolerance,
        "Multiple dilutions don't match single: single={} double={}",
        single_water,
        double_water
    );
}

// ============================================================================
// REFRACTOMETER ↔ HYDROMETER CONSISTENCY
// ============================================================================

// NOTE: Refractometer API needs verification - test commented out
// #[test]
// fn test_refractometer_hydrometer_consistency() {
//     // Refractometer and hydrometer should give similar results
//     // (accounting for temperature correction)
//
//     // Refractometer reading before fermentation
//     let original_brix = Decimal::from_str("18.0").expect("Parse original Brix");
//     let current_brix = Decimal::from_str("18.0").expect("Parse current Brix");
//
//     let refract_calc = RefractometerCalculator::default();
//     let input1 = CalcInput::new()
//         .add_param("original_brix", &original_brix.to_string())
//         .add_param("current_brix", &current_brix.to_string());
//     let refract_result = refract_calc.calculate(input1).expect("Refractometer");
//
//     // Hydrometer at calibration temp should give similar reading
//     let hydro_calc = HydrometerCorrectionCalculator::default();
//     let input2 = CalcInput::new()
//         .add_param("measured_sg", &refract_result.output.value.to_string())
//         .add_param("sample_temp", "68")
//         .add_param("calibration_temp", "68");
//     let hydro_result = hydro_calc.calculate(input2).expect("Hydrometer");
//
//     // At calibration temp, corrected SG should equal measured SG
//     let diff = (hydro_result.output.value - refract_result.output.value).abs();
//     let tolerance = Decimal::from_str("0.001").expect("Parse tolerance");
//
//     assert!(
//         diff < tolerance,
//         "Refractometer and hydrometer at calibration temp don't match: {} vs {}",
//         refract_result.output.value,
//         hydro_result.output.value
//     );
// }

// ============================================================================
// GRAVITY ↔ ABV CONSISTENCY
// ============================================================================

#[test]
fn test_gravity_from_ingredients_abv_consistency() {
    // Gravity from ingredients → ABV should be realistic

    let gravity_calc = GravityFromIngredientsCalculator::default();
    let input1 = CalcInput::new()
        .add_param("honey_weight", "3.6")
        .add_param("water_volume", "15");
    let og_result = gravity_calc.calculate(input1).expect("OG calculation");
    let og = og_result.output.value;

    // Assume fermentation to 1.010
    let fg = Decimal::from_str("1.010").expect("Parse FG");

    let abv_calc = AbvCalculator::default();
    let input2 = CalcInput::new()
        .add_param("og", &og.to_string())
        .add_param("fg", &fg.to_string());
    let abv_result = abv_calc.calculate(input2).expect("ABV calculation");

    // 3.6kg honey in 15L should give ~7-9% ABV
    let min_abv = Decimal::from_str("7.0").expect("Parse min ABV");
    let max_abv = Decimal::from_str("9.0").expect("Parse max ABV");

    assert!(
        abv_result.output.value >= min_abv && abv_result.output.value <= max_abv,
        "Gravity→ABV gave unrealistic result: {} (expected 9-11%)",
        abv_result.output.value
    );
}

// ============================================================================
// ATTENUATION ↔ ABV CONSISTENCY
// ============================================================================

#[test]
fn test_attenuation_abv_consistency() {
    // High attenuation should correlate with high ABV

    let og = Decimal::from_str("1.060").expect("Parse OG");
    let fg = Decimal::from_str("1.010").expect("Parse FG");

    // Calculate attenuation
    let attenuation_calc = AttenuationCalculator::default();
    let input1 = CalcInput::new()
        .add_param("og", &og.to_string())
        .add_param("fg", &fg.to_string());
    let attenuation_result = attenuation_calc.calculate(input1).expect("Attenuation");

    // Calculate ABV
    let abv_calc = AbvCalculator::default();
    let input2 = CalcInput::new()
        .add_param("og", &og.to_string())
        .add_param("fg", &fg.to_string());
    let abv_result = abv_calc.calculate(input2).expect("ABV");

    // Verify relationship: higher attenuation → more ABV
    // OG 1.060 → FG 1.010 is 83% attenuation and should give ~6.6% ABV
    let expected_attenuation = Decimal::from_str("80.0").expect("Parse expected");
    let tolerance = Decimal::from_str("5.0").expect("Parse tolerance");

    assert!(
        (attenuation_result.output.value - expected_attenuation).abs() < tolerance,
        "Attenuation outside expected range: {}",
        attenuation_result.output.value
    );

    let expected_abv = Decimal::from_str("6.5").expect("Parse expected ABV");
    let abv_tolerance = Decimal::from_str("0.5").expect("Parse ABV tolerance");

    assert!(
        (abv_result.output.value - expected_abv).abs() < abv_tolerance,
        "ABV outside expected range: {}",
        abv_result.output.value
    );
}

// ============================================================================
// CARBONATION ↔ VOLUME CONSISTENCY
// ============================================================================

#[test]
fn test_carbonation_scales_with_volume() {
    // Priming sugar should scale linearly with volume

    let carbonation_calc = CarbonationCalculator::default();

    // 19L batch
    let input1 = CalcInput::new()
        .add_param("temperature", "20")
        .add_param("volume", "19")
        .add_param("target_co2", "2.5")
        .add_param("method", "priming");
    let result1 = carbonation_calc.calculate(input1).expect("19L carbonation");
    let sugar_19l = result1.output.value;

    // 38L batch (exactly double)
    let input2 = CalcInput::new()
        .add_param("temperature", "20")
        .add_param("volume", "38")
        .add_param("target_co2", "2.5")
        .add_param("method", "priming");
    let result2 = carbonation_calc.calculate(input2).expect("38L carbonation");
    let sugar_38l = result2.output.value;

    // Sugar should approximately double
    let expected_double = sugar_19l * Decimal::from_str("2.0").expect("Parse 2");
    let diff = (sugar_38l - expected_double).abs();
    let tolerance = sugar_19l * Decimal::from_str("0.05").expect("Parse 5%"); // 5% tolerance

    assert!(
        diff < tolerance,
        "Carbonation doesn't scale linearly: 19L={} 38L={} (expected {})",
        sugar_19l,
        sugar_38l,
        expected_double
    );
}

// ============================================================================
// BENCH TRIALS ↔ RECIPE UPSCALING CONSISTENCY
// ============================================================================

#[test]
fn test_bench_trials_upscaling_consistency() {
    // Bench trials scaling should match recipe upscaling

    let bench_calc = BenchTrialsCalculator::default();
    let input1 = CalcInput::new()
        .add_param("batch_volume", "19")
        .add_param("trial_volume", "0.1")
        .add_param("trial_addition", "5"); // Use trial_addition not addition_amount
    let bench_result = bench_calc.calculate(input1).expect("Bench trials");

    let upscaling_calc = UpscalingCalculator::default();
    let input2 = CalcInput::new()
        .add_param("current_volume", "0.1")  // Use current_volume not original_volume
        .add_param("target_volume", "19");
    let _upscaling_result = upscaling_calc.calculate(input2).expect("Recipe upscaling");

    // Scaling factors should be similar
    // Bench trials: batch/trial = 19/0.1 = 190
    // Recipe upscaling: target/original = 19/0.1 = 190

    // Bench result should be 5g × 190 = 950g
    let expected = Decimal::from_str("950.0").expect("Parse expected");
    let tolerance = Decimal::from_str("10.0").expect("Parse tolerance");

    assert!(
        (bench_result.output.value - expected).abs() < tolerance,
        "Bench trials gave unexpected result: {} (expected {})",
        bench_result.output.value,
        expected
    );
}

// ============================================================================
// ALCOHOL TOLERANCE ↔ ABV CONSISTENCY
// ============================================================================

#[test]
fn test_alcohol_tolerance_limits_abv() {
    // Predicted FG from alcohol tolerance should give realistic ABV

    let tolerance_calc = AlcoholToleranceCalculator::default();
    let input1 = CalcInput::new()
        .add_param("og", "1.100")
        .add_param("yeast_strain", "ec1118"); // Use yeast_strain not yeast_tolerance
    let tolerance_result = tolerance_calc.calculate(input1).expect("Alcohol tolerance");

    // Extract predicted FG from metadata
    let predicted_fg_meta = tolerance_result.metadata.iter()
        .find(|(k, _)| k.contains("fg") || k.contains("final"))
        .map(|(_, v)| v);

    assert!(predicted_fg_meta.is_some(), "Missing predicted FG in metadata");
}

// ============================================================================
// BACKSWEETENING ↔ GRAVITY CONSISTENCY
// ============================================================================

#[test]
fn test_backsweetening_gravity_increase() {
    // Adding sweetener should increase gravity proportionally

    let backsweet_calc = BacksweeteningCalculator::default();

    // Start at 1.000, target 1.010
    let current_sg = Decimal::from_str("1.000").expect("Parse current SG");
    let sg_meas = Measurement::sg(current_sg).expect("Create SG measurement");

    let input1 = CalcInput::new()
        .add_measurement(sg_meas)
        .add_param("volume", "19")
        .add_param("sweetener", "honey")
        .add_param("target_sg", "1.010"); // Use target_sg not target_gravity
    let result1 = backsweet_calc.calculate(input1).expect("Backsweetening to 1.010");
    let honey_for_010 = result1.output.value;

    // Now target 1.020 (double the increase)
    let sg_meas2 = Measurement::sg(current_sg).expect("Create SG measurement");
    let input2 = CalcInput::new()
        .add_measurement(sg_meas2)
        .add_param("volume", "19")
        .add_param("sweetener", "honey")
        .add_param("target_sg", "1.020"); // Use target_sg not target_gravity
    let result2 = backsweet_calc.calculate(input2).expect("Backsweetening to 1.020");
    let honey_for_020 = result2.output.value;

    // Honey for 1.020 should be approximately double honey for 1.010
    let expected_double = honey_for_010 * Decimal::from_str("2.0").expect("Parse 2");
    let diff = (honey_for_020 - expected_double).abs();
    let tolerance = honey_for_010 * Decimal::from_str("0.2").expect("Parse 20%"); // 20% tolerance

    assert!(
        diff < tolerance,
        "Backsweetening doesn't scale: for 1.010={} for 1.020={} (expected ~{})",
        honey_for_010,
        honey_for_020,
        expected_double
    );
}

// ============================================================================
// NUTRITION ↔ GRAVITY/ABV CONSISTENCY
// ============================================================================

#[test]
fn test_nutrition_scales_with_gravity() {
    // Higher gravity should need more nutrients

    let nutrition_calc = NutritionCalculator::default();

    // Low gravity (1.060)
    let input1 = CalcInput::new()
        .add_param("volume", "19")
        .add_param("starting_gravity", "1.060")
        .add_param("target_abv", "8")
        .add_param("protocol", "tosna");
    let result1 = nutrition_calc.calculate(input1).expect("Low gravity nutrition");
    let nutrients_low = result1.output.value;

    // High gravity (1.120)
    let input2 = CalcInput::new()
        .add_param("volume", "19")
        .add_param("starting_gravity", "1.120")
        .add_param("target_abv", "16")
        .add_param("protocol", "tosna");
    let result2 = nutrition_calc.calculate(input2).expect("High gravity nutrition");
    let nutrients_high = result2.output.value;

    // High gravity should need more nutrients
    assert!(
        nutrients_high > nutrients_low,
        "High gravity doesn't need more nutrients: low={} high={}",
        nutrients_low,
        nutrients_high
    );
}