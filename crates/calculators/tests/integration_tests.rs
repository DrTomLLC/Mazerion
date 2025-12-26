//! INTEGRATION TESTS - FIXED VERSION
//! Location: crates/calculators/tests/integration_tests.rs

use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator, Measurement};
use rust_decimal::Decimal;
use std::str::FromStr;

// ============================================================================
// COMPLETE MEAD WORKFLOW INTEGRATION TESTS
// ============================================================================

#[test]
fn integration_complete_traditional_mead_workflow() {
    // SCENARIO: Brewer wants to make 5 gallons (19L) of 14% ABV traditional mead

    // Step 1: Calculate honey needed using CORRECT API
    let gravity_calc = GravityFromIngredientsCalculator::default();
    let honey_input = CalcInput::new()
        .add_param("honey_weight", "5.32") // CORRECTED: More honey for 14% ABV target
        .add_param("water_volume", "15"); // liters

    let gravity_result = gravity_calc.calculate(honey_input).unwrap();
    let calculated_og = gravity_result.output.value;

    // With 5.32 kg honey in 15 L:
    // Concentration: 0.3547 kg/L
    // Points: 0.3547 * 292 = 103.57
    // OG: 1.10357 ✓
    assert!(calculated_og > Decimal::from_str("1.090").unwrap());
    assert!(calculated_og < Decimal::from_str("1.120").unwrap());

    // Step 2: Calculate nutrition requirements (TOSNA)
    let nutrition_calc = NutritionCalculator::default();
    let nutrition_input = CalcInput::new()
        .add_param("volume", "19") // 5 gallons ≈ 19L
        .add_param("target_abv", "14");

    let nutrition_result = nutrition_calc.calculate(nutrition_input).unwrap();

    // Verify nutrition schedule exists
    assert!(!nutrition_result.metadata.is_empty());
    assert!(nutrition_result.metadata.iter().any(|(k, _)| k.contains("yan") || k.contains("fermaid")));

    // Step 3: Calculate yeast pitch rate
    let yeast_calc = YeastPitchCalculator::default();
    let yeast_input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("og", &calculated_og.to_string())
        .add_param("yeast_type", "ale");

    let yeast_result = yeast_calc.calculate(yeast_input).unwrap();

    // Verify cell count is calculated
    assert!(yeast_result.output.value > Decimal::ZERO);

    // Step 4: Simulate fermentation completion - calculate final ABV
    let final_gravity = Decimal::from_str("1.010").unwrap();

    let abv_calc = AbvCalculator::default();
    let abv_input = CalcInput::new()
        .add_param("og", &calculated_og.to_string())
        .add_param("fg", &final_gravity.to_string());

    let abv_result = abv_calc.calculate(abv_input).unwrap();

    // Verify ABV is close to target
    assert!(abv_result.output.value > Decimal::from_str("12.0").unwrap());
    assert!(abv_result.output.value < Decimal::from_str("16.0").unwrap());

    // Step 5: Calculate stabilization (sulfites)
    let sulfite_calc = SulfiteCalculator::default();

    // CRITICAL: pH must be a Measurement, not a parameter
    let ph_meas = Measurement::ph(Decimal::from_str("3.5").unwrap()).unwrap();

    let sulfite_input = CalcInput::new()
        .add_measurement(ph_meas)
        .add_param("volume", "19")
        .add_param("target_free_so2", "50"); // Correct parameter name

    let sulfite_result = sulfite_calc.calculate(sulfite_input).unwrap();

    // Verify sulfite amount calculated
    assert!(sulfite_result.output.value > Decimal::ZERO);

    // Step 6: Calculate backsweetening if needed
    let backsweeten_calc = BacksweeteningCalculator::default();

    // CRITICAL: current_sg must be a Measurement, not a parameter
    let current_sg_meas = Measurement::sg(final_gravity).unwrap();

    let backsweeten_input = CalcInput::new()
        .add_measurement(current_sg_meas)
        .add_param("volume", "19")
        .add_param("target_sg", "1.020")
        .add_param("sweetener", "honey"); // Correct parameter name

    let backsweeten_result = backsweeten_calc.calculate(backsweeten_input).unwrap();

    // Verify honey amount for backsweetening
    assert!(backsweeten_result.output.value > Decimal::ZERO);

    println!("✅ Complete mead workflow integration test PASSED");
    println!("   OG: {:.3}", calculated_og);
    println!("   Final ABV: {:.2}%", abv_result.output.value);
    println!("   Nutrients: {} g", nutrition_result.output.value);
    println!("   Yeast cells: {} billion", yeast_result.output.value);
    println!("   Sulfites: {} g", sulfite_result.output.value);
    println!("   Backsweetening: {} g honey", backsweeten_result.output.value);
}

#[test]
fn integration_melomel_with_fruit_workflow() {
    // SCENARIO: Making a raspberry melomel (fruit mead)

    // Step 1: Calculate melomel with CORRECT API
    let melomel_calc = MelomelCalculator::default();
    let melomel_input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12")
        .add_param("fruit_weight", "2.5") // CORRECTED: fruit_weight not fruit_ratio
        .add_param("fruit_type", "raspberry");

    let melomel_result = melomel_calc.calculate(melomel_input).unwrap();

    // Verify honey calculation accounts for fruit sugar
    assert!(melomel_result.output.value > Decimal::ZERO);

    // Verify metadata includes fruit info
    assert!(melomel_result.metadata.iter().any(|(k, _)| k == "fruit_sugar_g"));
    assert!(melomel_result.metadata.iter().any(|(k, _)| k == "fruit_abv"));

    // Step 2: Calculate nutrition (more nutrients needed with fruit)
    let nutrition_calc = NutritionCalculator::default();
    let nutrition_input = CalcInput::new()
        .add_param("volume", "19")
        .add_param("target_abv", "12");

    let nutrition_result = nutrition_calc.calculate(nutrition_input).unwrap();
    assert!(nutrition_result.output.value > Decimal::ZERO);

    println!("✅ Melomel workflow integration test PASSED");
    println!("   Honey needed: {} g", melomel_result.output.value);
    println!("   Fruit: 2.5 kg raspberry");
}

// ============================================================================
// BEER BREWING WORKFLOW INTEGRATION TESTS
// ============================================================================

#[test]
fn integration_complete_beer_brewing_workflow() {
    // SCENARIO: Brewing a 5-gallon IPA

    // Step 1: Calculate IBUs from hop additions
    let ibu_calc = IbuCalculator::default();

    // First hop addition (bittering)
    let bitter_input = CalcInput::new()
        .add_param("hop_weight_g", "56") // 2 oz
        .add_param("alpha_acid", "12.5")
        .add_param("boil_time", "60")
        .add_param("volume_l", "23")
        .add_param("boil_gravity", "1.060");

    let bitter_result = ibu_calc.calculate(bitter_input).unwrap();
    let bitter_ibus = bitter_result.output.value;

    // Second hop addition (flavor)
    let flavor_input = CalcInput::new()
        .add_param("hop_weight_g", "28") // 1 oz
        .add_param("alpha_acid", "8.0")
        .add_param("boil_time", "15")
        .add_param("volume_l", "23")
        .add_param("boil_gravity", "1.060");

    let flavor_result = ibu_calc.calculate(flavor_input).unwrap();
    let flavor_ibus = flavor_result.output.value;

    let total_ibus = bitter_ibus + flavor_ibus;

    // Verify total IBUs in IPA range
    assert!(total_ibus > Decimal::from_str("40.0").unwrap());
    assert!(total_ibus < Decimal::from_str("100.0").unwrap());

    // Step 2: Calculate yeast pitch rate for beer
    let yeast_calc = YeastPitchCalculator::default();
    let yeast_input = CalcInput::new()
        .add_param("volume", "23")
        .add_param("og", "1.060")
        .add_param("yeast_type", "ale");

    let yeast_result = yeast_calc.calculate(yeast_input).unwrap();

    // Verify ale yeast pitch rate
    assert!(yeast_result.output.value > Decimal::ZERO);

    // Step 3: Simulate fermentation - calculate final ABV
    let abv_calc = AbvCalculator::default();
    let abv_input = CalcInput::new()
        .add_param("og", "1.060")
        .add_param("fg", "1.012");

    let abv_result = abv_calc.calculate(abv_input).unwrap();

    // IPA should be 5-7% ABV
    assert!(abv_result.output.value > Decimal::from_str("5.0").unwrap());
    assert!(abv_result.output.value < Decimal::from_str("7.0").unwrap());

    // Step 4: Calculate carbonation (priming sugar)
    let carb_calc = CarbonationCalculator::default();
    let carb_input = CalcInput::new()
        .add_param("volume", "23")
        .add_param("temperature", "20")
        .add_param("target_co2", "2.5") // Typical for IPA
        .add_param("method", "priming");

    let carb_result = carb_calc.calculate(carb_input).unwrap();

    // Verify priming sugar calculated
    assert!(carb_result.output.value > Decimal::ZERO);
    assert!(carb_result.output.value < Decimal::from_str("300.0").unwrap());

    println!("✅ Complete beer brewing workflow PASSED");
    println!("   Total IBUs: {:.1}", total_ibus);
    println!("   ABV: {:.2}%", abv_result.output.value);
    println!("   Yeast cells: {} billion", yeast_result.output.value);
    println!("   Priming sugar: {} g", carb_result.output.value);
}

// ============================================================================
// CROSS-CALCULATOR CONSISTENCY TESTS
// ============================================================================

#[test]
fn integration_gravity_abv_consistency() {
    // Test that gravity→ABV→gravity calculations are consistent

    let og = Decimal::from_str("1.080").unwrap();
    let fg = Decimal::from_str("1.015").unwrap();

    // Calculate ABV from gravities
    let abv_calc = AbvCalculator::default();
    let abv_input = CalcInput::new()
        .add_param("og", &og.to_string())
        .add_param("fg", &fg.to_string());

    let abv_result = abv_calc.calculate(abv_input).unwrap();
    let calculated_abv = abv_result.output.value;

    // Verify ABV formula: (OG - FG) × 131.25
    let expected_abv = (og - fg) * Decimal::from_str("131.25").unwrap();
    let diff = (calculated_abv - expected_abv).abs();

    assert!(diff < Decimal::from_str("0.1").unwrap(),
            "ABV calculation inconsistent: {} vs expected {}",
            calculated_abv, expected_abv);
}

#[test]
fn integration_brix_sg_abv_workflow() {
    // Workflow: Brix reading → SG → ABV calculation

    // Step 1: Convert initial Brix to SG
    let brix_to_sg = BrixToSgCalculator::default();
    let og_brix = Decimal::from_str("19.0").unwrap();
    let brix_meas = Measurement::brix(og_brix).unwrap();
    let input1 = CalcInput::new().add_measurement(brix_meas);

    let sg_result = brix_to_sg.calculate(input1).unwrap();
    let og = sg_result.output.value;

    // Step 2: Simulate fermentation to FG
    let fg_brix = Decimal::from_str("5.0").unwrap();
    let fg_brix_meas = Measurement::brix(fg_brix).unwrap();
    let input2 = CalcInput::new().add_measurement(fg_brix_meas);

    let fg_result = brix_to_sg.calculate(input2).unwrap();
    let fg = fg_result.output.value;

    // Step 3: Calculate ABV
    let abv_calc = AbvCalculator::default();
    let abv_input = CalcInput::new()
        .add_param("og", &og.to_string())
        .add_param("fg", &fg.to_string());

    let abv_result = abv_calc.calculate(abv_input).unwrap();

    // Verify reasonable ABV from Brix readings
    assert!(abv_result.output.value > Decimal::ZERO);
    assert!(abv_result.output.value < Decimal::from_str("20.0").unwrap());

    println!("✅ Brix→SG→ABV workflow PASSED");
    println!("   OG Brix: {}° → SG: {:.3}", og_brix, og);
    println!("   FG Brix: {}° → SG: {:.3}", fg_brix, fg);
    println!("   Final ABV: {:.2}%", abv_result.output.value);
}

// ============================================================================
// BLENDING AND DILUTION INTEGRATION TESTS
// ============================================================================

#[test]
fn integration_blending_then_dilution_workflow() {
    // SCENARIO: Blend two batches, then dilute to target ABV

    // Step 1: Blend two batches
    let blend_calc = BlendingCalculator::default();
    let blend_input = CalcInput::new()
        .add_param("volume1", "10")
        .add_param("abv1", "16")
        .add_param("volume2", "10")
        .add_param("abv2", "12");

    let blend_result = blend_calc.calculate(blend_input).unwrap();
    let blended_abv = blend_result.output.value;
    let blended_volume = Decimal::from_str("20.0").unwrap(); // Sum of volumes

    // Verify blended ABV is average of inputs
    assert_eq!(blended_abv, Decimal::from_str("14.0").unwrap());

    // Step 2: Dilute blended batch to 12% ABV
    let dilution_calc = DilutionCalculator::default();
    let dilution_input = CalcInput::new()
        .add_param("current_volume", &blended_volume.to_string())
        .add_param("current_abv", &blended_abv.to_string())
        .add_param("target_abv", "12");

    let dilution_result = dilution_calc.calculate(dilution_input).unwrap();
    let water_needed = dilution_result.output.value;

    // Verify water amount is reasonable
    assert!(water_needed > Decimal::ZERO);
    assert!(water_needed < blended_volume); // Shouldn't need more water than original volume

    println!("✅ Blending→Dilution workflow PASSED");
    println!("   Blended: 20L at 14% ABV");
    println!("   Water needed: {} L to reach 12% ABV", water_needed);
}

// ============================================================================
// MULTI-STAGE FERMENTATION WORKFLOW
// ============================================================================

#[test]
fn integration_primary_secondary_bottling_workflow() {
    // Complete fermentation lifecycle

    let initial_volume = Decimal::from_str("23.0").unwrap(); // liters
    let og = Decimal::from_str("1.070").unwrap();

    // Stage 1: Primary fermentation losses (5%)
    let primary_loss = initial_volume * Decimal::from_str("0.05").unwrap();
    let after_primary = initial_volume - primary_loss;

    // Stage 2: Secondary/racking losses (3%)
    let secondary_loss = after_primary * Decimal::from_str("0.03").unwrap();
    let after_secondary = after_primary - secondary_loss;

    // Stage 3: Calculate bottles available
    // Assuming 750mL bottles with 2% bottling loss
    let bottling_loss = after_secondary * Decimal::from_str("0.02").unwrap();
    let bottleable_volume = after_secondary - bottling_loss;

    let bottle_size = Decimal::from_str("0.750").unwrap(); // 750mL
    let num_bottles = bottleable_volume / bottle_size;

    // Verify reasonable bottle count (should be ~28 bottles)
    assert!(num_bottles > Decimal::from_str("25.0").unwrap());
    assert!(num_bottles < Decimal::from_str("32.0").unwrap());

    // Calculate final ABV
    let fg = Decimal::from_str("1.010").unwrap();
    let abv_calc = AbvCalculator::default();
    let abv_input = CalcInput::new()
        .add_param("og", &og.to_string())
        .add_param("fg", &fg.to_string());

    let abv_result = abv_calc.calculate(abv_input).unwrap();

    println!("✅ Multi-stage fermentation workflow PASSED");
    println!("   Started with: {} L", initial_volume);
    println!("   After primary: {} L", after_primary);
    println!("   After secondary: {} L", after_secondary);
    println!("   Bottleable: {} L", bottleable_volume);
    println!("   Bottles: {:.0} × 750mL", num_bottles);
    println!("   Final ABV: {:.2}%", abv_result.output.value);
}

// ============================================================================
// ERROR PROPAGATION TESTS
// ============================================================================

#[test]
fn integration_error_handling_across_workflow() {
    // Test that errors in one stage don't crash subsequent stages

    // Try to calculate ABV with invalid inputs
    let abv_calc = AbvCalculator::default();
    let bad_input = CalcInput::new()
        .add_param("og", "1.010")
        .add_param("fg", "1.050"); // FG > OG - invalid

    let abv_result = abv_calc.calculate(bad_input);

    // Should return error, not panic
    assert!(abv_result.is_err());

    // Continue with valid workflow despite error
    let good_input = CalcInput::new()
        .add_param("og", "1.050")
        .add_param("fg", "1.010");

    let good_result = abv_calc.calculate(good_input);
    assert!(good_result.is_ok());
}

// ============================================================================
// PERFORMANCE / STRESS TESTS
// ============================================================================

#[test]
fn integration_rapid_sequential_calculations() {
    // Test that calculators can handle rapid sequential calls

    let abv_calc = AbvCalculator::default();

    for i in 1..=100 {
        let og = Decimal::from(1000 + i * 10) / Decimal::from(1000);
        let fg = Decimal::from(990 + i) / Decimal::from(1000);

        let input = CalcInput::new()
            .add_param("og", &og.to_string())
            .add_param("fg", &fg.to_string());

        let result = abv_calc.calculate(input);

        // All should succeed
        assert!(result.is_ok(), "Calculation #{} failed", i);
    }

    println!("✅ Completed 100 sequential calculations without error");
}