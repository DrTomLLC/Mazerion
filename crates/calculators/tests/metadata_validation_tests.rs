//! METADATA VALIDATION TEST SUITE
//! Verifies ALL calculators return proper metadata
//! Location: crates/calculators/tests/metadata_validation_tests.rs

use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator, Measurement};
use rust_decimal::Decimal;
use std::str::FromStr;

// ============================================================================
// ABV CALCULATOR METADATA
// ============================================================================

#[test]
fn test_abv_metadata_fields() {
    let calc = AbvCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.050")
        .add_param("fg", "1.010");

    let result = calc.calculate(input).expect("ABV calculation");

    // Verify expected metadata fields
    assert!(result.metadata.iter().any(|(k, _)| k == "og"), "Missing 'og' field");
    assert!(result.metadata.iter().any(|(k, _)| k == "fg"), "Missing 'fg' field");
    assert!(result.metadata.iter().any(|(k, _)| k == "formula"), "Missing 'formula' field");
}

#[test]
fn test_abv_metadata_values() {
    let calc = AbvCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.050")
        .add_param("fg", "1.010");

    let result = calc.calculate(input).expect("ABV calculation");

    // Verify metadata values are accurate
    let og_meta = result.metadata.iter().find(|(k, _)| k == "og").map(|(_, v)| v);
    assert_eq!(og_meta, Some(&"1.050".to_string()));

    let fg_meta = result.metadata.iter().find(|(k, _)| k == "fg").map(|(_, v)| v);
    assert_eq!(fg_meta, Some(&"1.010".to_string()));
}

// ============================================================================
// BRIX/SG CALCULATOR METADATA
// ============================================================================

#[test]
fn test_brix_to_sg_metadata() {
    let calc = BrixToSgCalculator::default();
    let brix_meas = Measurement::brix(Decimal::from_str("20.0").expect("Parse Brix"))
        .expect("Create Brix measurement");
    let input = CalcInput::new().add_measurement(brix_meas);

    let result = calc.calculate(input).expect("Brix to SG calculation");

    // Should have input and formula metadata
    assert!(result.metadata.iter().any(|(k, _)| k == "brix"), "Missing 'brix' field");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("formula") || k.contains("method")),
            "Missing formula/method field");
}

#[test]
fn test_sg_to_brix_metadata() {
    let calc = SgToBrixCalculator::default();
    let sg_meas = Measurement::sg(Decimal::from_str("1.083").expect("Parse SG"))
        .expect("Create SG measurement");
    let input = CalcInput::new().add_measurement(sg_meas);

    let result = calc.calculate(input).expect("SG to Brix calculation");

    // Should have input metadata
    assert!(result.metadata.iter().any(|(k, _)| k == "sg" || k.contains("specific")),
            "Missing SG field");
}

// ============================================================================
// DILUTION CALCULATOR METADATA
// ============================================================================

#[test]
fn test_dilution_metadata_fields() {
    let calc = DilutionCalculator::default();
    let input = CalcInput::new()
        .add_param("current_volume", "20")
        .add_param("current_abv", "14")
        .add_param("target_abv", "10");

    let result = calc.calculate(input).expect("Dilution calculation");

    // Should include current and target values
    assert!(result.metadata.iter().any(|(k, _)| k.contains("current") || k.contains("volume")),
            "Missing current volume info");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("target") || k.contains("abv")),
            "Missing target ABV info");
}

// ============================================================================
// BLENDING CALCULATOR METADATA
// ============================================================================

#[test]
fn test_blending_metadata() {
    let calc = BlendingCalculator::default();
    let input = CalcInput::new()
        .add_param("volume1", "10")
        .add_param("abv1", "12")
        .add_param("volume2", "10")
        .add_param("abv2", "8");

    let result = calc.calculate(input).expect("Blending calculation");

    // Should include batch information
    assert!(result.metadata.iter().any(|(k, _)| k.contains("batch") || k.contains("volume")),
            "Missing batch info");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("total")),
            "Missing total volume");
}

// ============================================================================
// IBU CALCULATOR METADATA
// ============================================================================

#[test]
fn test_ibu_metadata_comprehensive() {
    let calc = IbuCalculator::default();
    let input = CalcInput::new()
        .add_param("hop_weight_g", "28")
        .add_param("alpha_acid", "10.0")
        .add_param("boil_time", "60")
        .add_param("volume_l", "23")
        .add_param("boil_gravity", "1.050");

    let result = calc.calculate(input).expect("IBU calculation");

    // Should include hop details
    assert!(result.metadata.iter().any(|(k, _)| k.contains("hop") || k.contains("weight")),
            "Missing hop weight");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("alpha") || k.contains("acid")),
            "Missing alpha acid");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("boil") || k.contains("time")),
            "Missing boil time");
}

// ============================================================================
// CARBONATION CALCULATOR METADATA
// ============================================================================

#[test]
fn test_carbonation_metadata_fields() {
    let calc = CarbonationCalculator::default();
    let input = CalcInput::new()
        .add_param("temperature", "20")
        .add_param("volume", "19")
        .add_param("target_co2", "2.5")
        .add_param("method", "priming");

    let result = calc.calculate(input).expect("Carbonation calculation");

    // Should include temperature and CO2 info
    assert!(result.metadata.iter().any(|(k, _)| k.contains("temp")),
            "Missing temperature");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("co2") || k.contains("volume")),
            "Missing CO2 info");
}

// ============================================================================
// HYDROMETER CORRECTION METADATA
// ============================================================================

#[test]
fn test_hydrometer_metadata_comprehensive() {
    let calc = HydrometerCorrectionCalculator::default();
    let input = CalcInput::new()
        .add_param("measured_sg", "1.050")
        .add_param("sample_temp", "75")
        .add_param("calibration_temp", "68");

    let result = calc.calculate(input).expect("Hydrometer correction");

    // Should include temperatures and correction
    assert!(result.metadata.iter().any(|(k, _)| k.contains("sample") || k.contains("temp")),
            "Missing sample temp");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("calibration")),
            "Missing calibration temp");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("correction")),
            "Missing correction value");
}

// ============================================================================
// NUTRITION CALCULATOR METADATA
// ============================================================================

#[test]
fn test_nutrition_metadata_schedule() {
    let calc = NutritionCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("starting_gravity", "1.100")
        .add_param("target_abv", "14")
        .add_param("protocol", "tosna");

    let result = calc.calculate(input).expect("Nutrition calculation");

    // Should include schedule and protocol info
    assert!(result.metadata.iter().any(|(k, _)| k.contains("schedule") || k.contains("addition")),
            "Missing schedule info");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("protocol") || k.contains("tosna")),
            "Missing protocol info");
}

// ============================================================================
// YEAST PITCH CALCULATOR METADATA
// ============================================================================

#[test]
fn test_yeast_pitch_metadata() {
    let calc = YeastPitchCalculator::default();
    let input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("gravity", "1.060")
        .add_param("yeast_type", "ale");

    let result = calc.calculate(input).expect("Yeast pitch calculation");

    // Should include cell count and pitch rate
    assert!(result.metadata.iter().any(|(k, _)| k.contains("cells") || k.contains("count")),
            "Missing cell count");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("rate") || k.contains("pitch")),
            "Missing pitch rate");
}

// ============================================================================
// BACKSWEETENING CALCULATOR METADATA
// ============================================================================

#[test]
fn test_backsweetening_metadata() {
    let calc = BacksweeteningCalculator::default();
    let sg_meas = Measurement::sg(Decimal::from_str("1.000").expect("Parse SG"))
        .expect("Create SG measurement");
    let input = CalcInput::new()
        .add_measurement(sg_meas)
        .add_param("volume", "19")
        .add_param("sweetener", "honey")
        .add_param("target_gravity", "1.015");

    let result = calc.calculate(input).expect("Backsweetening calculation");

    // Should include current, target, and sweetener info
    assert!(result.metadata.iter().any(|(k, _)| k.contains("current") || k.contains("sg")),
            "Missing current SG");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("target")),
            "Missing target gravity");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("sweetener")),
            "Missing sweetener type");
}

// ============================================================================
// SULFITE CALCULATOR METADATA
// ============================================================================

#[test]
fn test_sulfite_metadata_comprehensive() {
    let calc = SulfiteCalculator::default();
    let ph_meas = Measurement::ph(Decimal::from_str("3.5").expect("Parse pH"))
        .expect("Create pH measurement");
    let input = CalcInput::new()
        .add_measurement(ph_meas)
        .add_param("volume", "19")
        .add_param("target_free_so2", "50")
        .add_param("sulfite_type", "kmeta");

    let result = calc.calculate(input).expect("Sulfite calculation");

    // Should include pH, target SO2, and type
    assert!(result.metadata.iter().any(|(k, _)| k.contains("ph")),
            "Missing pH");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("so2") || k.contains("sulfite")),
            "Missing SO2 info");
}

// ============================================================================
// ACID ADDITION CALCULATOR METADATA
// ============================================================================

#[test]
fn test_acid_addition_metadata() {
    let calc = AcidAdditionCalculator::default();
    let ph_meas = Measurement::ph(Decimal::from_str("3.8").expect("Parse pH"))
        .expect("Create pH measurement");
    let input = CalcInput::new()
        .add_measurement(ph_meas)
        .add_param("volume", "19")
        .add_param("target_ph", "3.4")
        .add_param("acid_type", "tartaric");

    let result = calc.calculate(input).expect("Acid addition calculation");

    // Should include pH values and acid type
    assert!(result.metadata.iter().any(|(k, _)| k.contains("ph") || k.contains("current")),
            "Missing current pH");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("target")),
            "Missing target pH");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("acid")),
            "Missing acid type");
}

// ============================================================================
// REFRACTOMETER CALCULATOR METADATA
// ============================================================================

#[test]
fn test_refractometer_metadata() {
    let calc = RefractometerCalculator::default();
    let input = CalcInput::new()
        .add_param("original_brix", "18.0")
        .add_param("current_brix", "8.5");

    let result = calc.calculate(input).expect("Refractometer calculation");

    // Should include original and current Brix
    assert!(result.metadata.iter().any(|(k, _)| k.contains("original") || k.contains("brix")),
            "Missing original Brix");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("current")),
            "Missing current Brix");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("formula") || k.contains("method")),
            "Missing formula/method");
}

// ============================================================================
// GRAVITY FROM INGREDIENTS METADATA
// ============================================================================

#[test]
fn test_gravity_from_ingredients_metadata() {
    let calc = GravityFromIngredientsCalculator::default();
    let input = CalcInput::new()
        .add_param("honey_weight", "3.6")
        .add_param("water_volume", "15");

    let result = calc.calculate(input).expect("Gravity calculation");

    // Should include ingredient details
    assert!(result.metadata.iter().any(|(k, _)| k.contains("honey") || k.contains("weight")),
            "Missing honey weight");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("water") || k.contains("volume")),
            "Missing water volume");
}

// ============================================================================
// ATTENUATION CALCULATOR METADATA
// ============================================================================

#[test]
fn test_attenuation_metadata() {
    let calc = AttenuationCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.060")
        .add_param("fg", "1.015");

    let result = calc.calculate(input).expect("Attenuation calculation");

    // Should include OG, FG, and attenuation type
    assert!(result.metadata.iter().any(|(k, _)| k == "og"),
            "Missing OG");
    assert!(result.metadata.iter().any(|(k, _)| k == "fg"),
            "Missing FG");
}

// ============================================================================
// ALCOHOL TOLERANCE CALCULATOR METADATA
// ============================================================================

#[test]
fn test_alcohol_tolerance_metadata() {
    let calc = AlcoholToleranceCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.100")
        .add_param("yeast_tolerance", "14");

    let result = calc.calculate(input).expect("Alcohol tolerance calculation");

    // Should include tolerance and predicted FG
    assert!(result.metadata.iter().any(|(k, _)| k.contains("tolerance")),
            "Missing tolerance");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("fg") || k.contains("final")),
            "Missing predicted FG");
}

// ============================================================================
// BENCH TRIALS CALCULATOR METADATA
// ============================================================================

#[test]
fn test_bench_trials_metadata() {
    let calc = BenchTrialsCalculator::default();
    let input = CalcInput::new()
        .add_param("batch_volume", "19")
        .add_param("trial_volume", "0.1")
        .add_param("trial_addition", "5");  // Use trial_addition not addition_amount

    let result = calc.calculate(input).expect("Bench trials calculation");

    // Should include scaling info
    assert!(result.metadata.iter().any(|(k, _)| k.contains("batch") || k.contains("volume")),
            "Missing batch volume");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("trial")),
            "Missing trial volume");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("scaling") || k.contains("factor")),
            "Missing scaling factor");
}

// ============================================================================
// RECIPE UPSCALING CALCULATOR METADATA
// ============================================================================

#[test]
fn test_recipe_upscaling_metadata() {
    let calc = UpscalingCalculator::default();
    let input = CalcInput::new()
        .add_param("current_volume", "5")  // Use current_volume not original_volume
        .add_param("target_volume", "19");

    let result = calc.calculate(input).expect("Recipe upscaling calculation");

    // Should include volumes and scaling factor
    assert!(result.metadata.iter().any(|(k, _)| k.contains("current") || k.contains("volume")),
            "Missing current volume");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("target")),
            "Missing target volume");
    assert!(result.metadata.iter().any(|(k, _)| k.contains("factor") || k.contains("scale")),
            "Missing scaling factor");
}

// ============================================================================
// RECIPE UPSCALING CALCULATOR METADATA
// ============================================================================

#[test]
fn test_metadata_no_empty_values() {
    let calc = AbvCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.050")
        .add_param("fg", "1.010");

    let result = calc.calculate(input).expect("ABV calculation");

    // No metadata value should be empty
    for (key, value) in &result.metadata {
        assert!(!value.is_empty(), "Metadata key '{}' has empty value", key);
    }
}

#[test]
fn test_metadata_keys_not_empty() {
    let calc = AbvCalculator::default();
    let input = CalcInput::new()
        .add_param("og", "1.050")
        .add_param("fg", "1.010");

    let result = calc.calculate(input).expect("ABV calculation");

    // No metadata key should be empty
    for (key, _) in &result.metadata {
        assert!(!key.is_empty(), "Found empty metadata key");
    }
}