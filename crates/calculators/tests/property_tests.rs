//! PROPERTY-BASED FUZZING TESTS
//! Location: crates/calculators/tests/property_tests.rs
//!
//! These tests use proptest to fuzz calculators with random inputs
//! and verify mathematical invariants hold for all valid inputs

use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator, Measurement};
use proptest::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;

// ============================================================================
// PROPERTY GENERATORS
// ============================================================================

/// Generate valid gravity values (0.990 to 2.000)
fn gravity_strategy() -> impl Strategy<Value = Decimal> {
    (990u32..=2000u32)
        .prop_map(|x| Decimal::from(x) / Decimal::from(1000))
}

/// Generate valid ABV percentages (0.0 to 25.0)
fn abv_strategy() -> impl Strategy<Value = Decimal> {
    (0u32..=2500u32)
        .prop_map(|x| Decimal::from(x) / Decimal::from(100))
}

/// Generate valid Brix values (0.0 to 40.0)
fn brix_strategy() -> impl Strategy<Value = Decimal> {
    (0u32..=4000u32)
        .prop_map(|x| Decimal::from(x) / Decimal::from(100))
}

/// Generate valid volume (0.1 to 1000.0 liters)
fn volume_strategy() -> impl Strategy<Value = Decimal> {
    (10u32..=100000u32)
        .prop_map(|x| Decimal::from(x) / Decimal::from(100))
}

/// Generate valid temperatures (-20 to 120°F)
fn temperature_f_strategy() -> impl Strategy<Value = Decimal> {
    (-20i32..=120i32)
        .prop_map(|x| Decimal::from(x))
}

/// Generate valid pH (1.5 to 8.5)
#[allow(dead_code)]
fn ph_strategy() -> impl Strategy<Value = Decimal> {
    (150u32..=850u32)
        .prop_map(|x| Decimal::from(x) / Decimal::from(100))
}

// ============================================================================
// ABV CALCULATOR PROPERTIES
// ============================================================================

proptest! {
    #[test]
    fn prop_abv_never_panics(
        og in gravity_strategy(),
        fg in gravity_strategy()
    ) {
        let calc = AbvCalculator::default();
        let input = CalcInput::new()
            .add_param("og", &og.to_string())
            .add_param("fg", &fg.to_string());

        // Should never panic - either Ok or Err
        let _ = calc.calculate(input);
    }

    #[test]
    fn prop_abv_higher_og_gives_higher_abv(
        og1 in gravity_strategy(),
        og2 in gravity_strategy(),
        fg in gravity_strategy()
    ) {
        // If OG1 > OG2 and same FG, then ABV1 > ABV2
        if og1 <= fg || og2 <= fg || og1 == og2 {
            return Ok(());
        }

        let calc = AbvCalculator::default();

        let input1 = CalcInput::new()
            .add_param("og", &og1.to_string())
            .add_param("fg", &fg.to_string());

        let input2 = CalcInput::new()
            .add_param("og", &og2.to_string())
            .add_param("fg", &fg.to_string());

        if let (Ok(result1), Ok(result2)) = (calc.calculate(input1), calc.calculate(input2)) {
            if og1 > og2 {
                prop_assert!(result1.output.value > result2.output.value);
            }
        }
    }

    #[test]
    fn prop_abv_is_zero_when_og_equals_fg(
        sg in gravity_strategy()
    ) {
        let calc = AbvCalculator::default();
        let input = CalcInput::new()
            .add_param("og", &sg.to_string())
            .add_param("fg", &sg.to_string());

        if let Ok(result) = calc.calculate(input) {
            // When OG = FG, ABV should be 0
            prop_assert_eq!(result.output.value, Decimal::ZERO);
        }
    }

    #[test]
    fn prop_abv_rejects_fg_greater_than_og(
        og in gravity_strategy(),
        fg in gravity_strategy()
    ) {
        if fg <= og {
            return Ok(());
        }

        let calc = AbvCalculator::default();
        let input = CalcInput::new()
            .add_param("og", &og.to_string())
            .add_param("fg", &fg.to_string());

        // Should reject FG > OG
        prop_assert!(calc.calculate(input).is_err());
    }
}

// ============================================================================
// BRIX/SG CONVERSION PROPERTIES
// ============================================================================

proptest! {
    #[test]
    fn prop_brix_to_sg_never_panics(
        brix in brix_strategy()
    ) {
        let calc = BrixToSgCalculator::default();
        let brix_meas = Measurement::brix(brix);

        if let Ok(measurement) = brix_meas {
            let input = CalcInput::new().add_measurement(measurement);
            let _ = calc.calculate(input);
        }
    }

    #[test]
    fn prop_brix_sg_roundtrip_preserves_value(
        brix in brix_strategy()
    ) {
        let brix_to_sg = BrixToSgCalculator::default();
        let sg_to_brix = SgToBrixCalculator::default();

        if let Ok(brix_meas) = Measurement::brix(brix) {
            let input1 = CalcInput::new().add_measurement(brix_meas);

            if let Ok(sg_result) = brix_to_sg.calculate(input1) {
                if let Ok(sg_meas) = Measurement::sg(sg_result.output.value) {
                    let input2 = CalcInput::new().add_measurement(sg_meas);

                    if let Ok(brix_result) = sg_to_brix.calculate(input2) {
                        // Roundtrip should preserve value within tolerance
                        let diff = (brix_result.output.value - brix).abs();

                        // Handle zero case: use absolute tolerance instead of relative
                        let tolerance = if brix == Decimal::ZERO {
                            Decimal::from_str("0.01").unwrap() // 0.01 Brix absolute tolerance
                        } else {
                            brix.abs() * Decimal::from_str("0.01").unwrap() // 1% relative tolerance
                        };

                        prop_assert!(diff <= tolerance);
                    }
                }
            }
        }
    }

    #[test]
    fn prop_higher_brix_gives_higher_sg(
        brix1 in brix_strategy(),
        brix2 in brix_strategy()
    ) {
        if brix1 == brix2 {
            return Ok(());
        }

        let calc = BrixToSgCalculator::default();

        if let (Ok(m1), Ok(m2)) = (Measurement::brix(brix1), Measurement::brix(brix2)) {
            let input1 = CalcInput::new().add_measurement(m1);
            let input2 = CalcInput::new().add_measurement(m2);

            if let (Ok(result1), Ok(result2)) = (calc.calculate(input1), calc.calculate(input2)) {
                if brix1 > brix2 {
                    prop_assert!(result1.output.value > result2.output.value);
                }
            }
        }
    }
}

// ============================================================================
// DILUTION CALCULATOR PROPERTIES
// ============================================================================

proptest! {
    #[test]
    fn prop_dilution_never_panics(
        volume in volume_strategy(),
        current_abv in abv_strategy(),
        target_abv in abv_strategy()
    ) {
        let calc = DilutionCalculator::default();
        let input = CalcInput::new()
            .add_param("current_volume", &volume.to_string())
            .add_param("current_abv", &current_abv.to_string())
            .add_param("target_abv", &target_abv.to_string());

        // Should never panic
        let _ = calc.calculate(input);
    }

    #[test]
    fn prop_dilution_requires_target_less_than_current(
        volume in volume_strategy(),
        current_abv in abv_strategy(),
        target_abv in abv_strategy()
    ) {
        if target_abv >= current_abv {
            let calc = DilutionCalculator::default();
            let input = CalcInput::new()
                .add_param("current_volume", &volume.to_string())
                .add_param("current_abv", &current_abv.to_string())
                .add_param("target_abv", &target_abv.to_string());

            // Should reject target >= current
            prop_assert!(calc.calculate(input).is_err());
        }
    }

    #[test]
    fn prop_dilution_water_is_positive_when_valid(
        volume in volume_strategy(),
        current_abv in (1u32..=2000u32).prop_map(|x| Decimal::from(x) / Decimal::from(100)),
        ratio in (10u32..=90u32).prop_map(|x| Decimal::from(x) / Decimal::from(100))
    ) {
        let target_abv = current_abv * ratio;

        let calc = DilutionCalculator::default();
        let input = CalcInput::new()
            .add_param("current_volume", &volume.to_string())
            .add_param("current_abv", &current_abv.to_string())
            .add_param("target_abv", &target_abv.to_string());

        if let Ok(result) = calc.calculate(input) {
            // Water needed should be positive
            prop_assert!(result.output.value > Decimal::ZERO);
        }
    }
}

// ============================================================================
// HYDROMETER CORRECTION PROPERTIES
// ============================================================================

proptest! {
    #[test]
    fn prop_hydrometer_never_panics(
        sg in gravity_strategy(),
        sample_temp in temperature_f_strategy(),
        cal_temp in temperature_f_strategy()
    ) {
        let calc = HydrometerCorrectionCalculator::default();
        let input = CalcInput::new()
            .add_param("measured_sg", &sg.to_string())
            .add_param("sample_temp", &sample_temp.to_string())
            .add_param("calibration_temp", &cal_temp.to_string());

        // Should never panic
        let _ = calc.calculate(input);
    }

    #[test]
    fn prop_hydrometer_equals_measured_at_calibration_temp(
        sg in gravity_strategy(),
        temp in temperature_f_strategy()
    ) {
        let calc = HydrometerCorrectionCalculator::default();
        let input = CalcInput::new()
            .add_param("measured_sg", &sg.to_string())
            .add_param("sample_temp", &temp.to_string())
            .add_param("calibration_temp", &temp.to_string());

        if let Ok(result) = calc.calculate(input) {
            // At calibration temp, corrected = measured
            let diff = (result.output.value - sg).abs();
            prop_assert!(diff < Decimal::from_str("0.001").unwrap());
        }
    }

    #[test]
    fn prop_hydrometer_hot_increases_sg(
        sg in gravity_strategy(),
        cal_temp in (60i32..=70i32).prop_map(Decimal::from),
        temp_diff in (5i32..=30i32).prop_map(Decimal::from)
    ) {
        let sample_temp = cal_temp + temp_diff;

        let calc = HydrometerCorrectionCalculator::default();
        let input = CalcInput::new()
            .add_param("measured_sg", &sg.to_string())
            .add_param("sample_temp", &sample_temp.to_string())
            .add_param("calibration_temp", &cal_temp.to_string());

        if let Ok(result) = calc.calculate(input) {
            // Hot sample reads low, corrected should be higher
            prop_assert!(result.output.value >= sg);
        }
    }
}

// ============================================================================
// BLENDING CALCULATOR PROPERTIES
// ============================================================================

proptest! {
    #[test]
    fn prop_blending_never_panics(
        vol1 in volume_strategy(),
        vol2 in volume_strategy(),
        abv1 in abv_strategy(),
        abv2 in abv_strategy()
    ) {
        let calc = BlendingCalculator::default();
        let input = CalcInput::new()
            .add_param("volume1", &vol1.to_string())
            .add_param("volume2", &vol2.to_string())
            .add_param("abv1", &abv1.to_string())
            .add_param("abv2", &abv2.to_string());

        // Should never panic
        let _ = calc.calculate(input);
    }

    #[test]
    fn prop_blending_result_between_inputs(
        vol1 in volume_strategy(),
        vol2 in volume_strategy(),
        abv1 in abv_strategy(),
        abv2 in abv_strategy()
    ) {
        let calc = BlendingCalculator::default();
        let input = CalcInput::new()
            .add_param("volume1", &vol1.to_string())
            .add_param("volume2", &vol2.to_string())
            .add_param("abv1", &abv1.to_string())
            .add_param("abv2", &abv2.to_string());

        if let Ok(result) = calc.calculate(input) {
            let min_abv = if abv1 < abv2 { abv1 } else { abv2 };
            let max_abv = if abv1 > abv2 { abv1 } else { abv2 };

            // Blended ABV should be between the two inputs
            prop_assert!(result.output.value >= min_abv);
            prop_assert!(result.output.value <= max_abv);
        }
    }

    #[test]
    fn prop_blending_equal_volumes_gives_average(
        vol in volume_strategy(),
        abv1 in abv_strategy(),
        abv2 in abv_strategy()
    ) {
        let calc = BlendingCalculator::default();
        let input = CalcInput::new()
            .add_param("volume1", &vol.to_string())
            .add_param("volume2", &vol.to_string())
            .add_param("abv1", &abv1.to_string())
            .add_param("abv2", &abv2.to_string());

        if let Ok(result) = calc.calculate(input) {
            let expected = (abv1 + abv2) / Decimal::from(2);
            let diff = (result.output.value - expected).abs();

            // Equal volumes should give average ABV
            prop_assert!(diff < Decimal::from_str("0.1").unwrap());
        }
    }
}

// ============================================================================
// CARBONATION CALCULATOR PROPERTIES
// ============================================================================

proptest! {
    #[test]
    fn prop_carbonation_never_panics(
        volume in volume_strategy(),
        temp in (0i32..=30i32).prop_map(Decimal::from),
        co2 in (15u32..=50u32).prop_map(|x| Decimal::from(x) / Decimal::from(10))
    ) {
        let calc = CarbonationCalculator::default();
        let input = CalcInput::new()
            .add_param("volume", &volume.to_string())
            .add_param("temperature", &temp.to_string())
            .add_param("target_co2", &co2.to_string())
            .add_param("method", "priming");

        // Should never panic
        let _ = calc.calculate(input);
    }

    #[test]
    fn prop_carbonation_higher_co2_needs_more_sugar(
        volume in volume_strategy(),
        temp in (0i32..=30i32).prop_map(Decimal::from),
        co2_1 in (20u32..=30u32).prop_map(|x| Decimal::from(x) / Decimal::from(10)),
        co2_2 in (31u32..=45u32).prop_map(|x| Decimal::from(x) / Decimal::from(10))
    ) {
        let calc = CarbonationCalculator::default();

        let input1 = CalcInput::new()
            .add_param("volume", &volume.to_string())
            .add_param("temperature", &temp.to_string())
            .add_param("target_co2", &co2_1.to_string())
            .add_param("method", "priming");

        let input2 = CalcInput::new()
            .add_param("volume", &volume.to_string())
            .add_param("temperature", &temp.to_string())
            .add_param("target_co2", &co2_2.to_string())
            .add_param("method", "priming");

        if let (Ok(result1), Ok(result2)) = (calc.calculate(input1), calc.calculate(input2)) {
            // Higher CO2 needs more priming sugar
            prop_assert!(result2.output.value > result1.output.value);
        }
    }
}

// ============================================================================
// NUTRITION CALCULATOR PROPERTIES
// ============================================================================

proptest! {
    #[test]
    fn prop_nutrition_never_panics(
        volume in volume_strategy(),
        abv in abv_strategy()
    ) {
        let calc = NutritionCalculator::default();
        let input = CalcInput::new()
            .add_param("volume", &volume.to_string())
            .add_param("target_abv", &abv.to_string());

        // Should never panic
        let _ = calc.calculate(input);
    }

    #[test]
    fn prop_nutrition_higher_abv_needs_more_nutrients(
        volume in volume_strategy(),
        abv1 in (5u32..=10u32).prop_map(Decimal::from),
        abv2 in (15u32..=20u32).prop_map(Decimal::from)
    ) {
        let calc = NutritionCalculator::default();

        let input1 = CalcInput::new()
            .add_param("volume", &volume.to_string())
            .add_param("target_abv", &abv1.to_string());

        let input2 = CalcInput::new()
            .add_param("volume", &volume.to_string())
            .add_param("target_abv", &abv2.to_string());

        if let (Ok(result1), Ok(result2)) = (calc.calculate(input1), calc.calculate(input2)) {
            // Higher ABV needs more nutrients
            prop_assert!(result2.output.value >= result1.output.value);
        }
    }
}