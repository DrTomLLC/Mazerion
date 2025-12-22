use mazerion_core::{Calculator, CalcInput, CalcResult, Error, Measurement, Unit};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

/// Gallons to Bottles with Racking Losses Calculator
///
/// Uses CORRECT compounding math where each racking loses a percentage of the
/// CURRENT volume, not the original volume.
///
/// Formula: final_volume = initial_volume × (1 - loss_rate)^number_of_rackings
///
/// Example: 5 gallons, 3 rackings, 5% loss per racking
///   - After racking 1: 5 × 0.95 = 4.75 gal
///   - After racking 2: 4.75 × 0.95 = 4.5125 gal
///   - After racking 3: 4.5125 × 0.95 = 4.286875 gal
///   - Final bottles: 4.286875 × 5.047 = 21.64 bottles
///   - NOT 21.45 bottles (which is what simple addition would give)
#[derive(Default)]
pub struct GallonsToBottlesWithLossesCalculator;

impl Calculator for GallonsToBottlesWithLossesCalculator {
    fn id(&self) -> &'static str {
        "gallons_to_bottles_with_losses"
    }

    fn name(&self) -> &'static str {
        "Gallons to Bottles (with Losses)"
    }

    fn category(&self) -> &'static str {
        "conversions"
    }

    fn description(&self) -> &'static str {
        "Calculate final bottle count after accounting for racking losses. \
         Losses compound with each racking - each racking loses a percentage \
         of the current volume, not the original volume. Standard 750ml bottles assumed."
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult, Error> {
        // Helper function to extract and parse parameters from input
        let get_param = |name: &str| -> Result<Decimal, Error> {
            input.params
                .iter()
                .find(|(k, _)| k == name)
                .map(|(_, v)| v.as_str())
                .ok_or_else(|| Error::MissingInput(name.to_string()))?
                .parse::<Decimal>()
                .map_err(|_| Error::Calculation(format!("Invalid value for parameter '{}'", name)))
        };

        // Extract input parameters
        let initial_volume_gal = get_param("initial_volume")?;
        let loss_rate_percent = get_param("loss_rate_percent")?;
        let num_rackings = get_param("num_rackings")?;

        // Comprehensive input validation
        if initial_volume_gal <= Decimal::ZERO {
            return Err(Error::Calculation(
                "Initial volume must be greater than zero".to_string()
            ));
        }

        if initial_volume_gal > Decimal::from(1000) {
            return Err(Error::Calculation(
                "Initial volume seems unreasonably large (max 1000 gallons)".to_string()
            ));
        }

        if loss_rate_percent < Decimal::ZERO {
            return Err(Error::Calculation(
                "Loss rate cannot be negative".to_string()
            ));
        }

        if loss_rate_percent >= Decimal::from(100) {
            return Err(Error::Calculation(
                "Loss rate must be less than 100%".to_string()
            ));
        }

        if num_rackings < Decimal::ONE {
            return Err(Error::Calculation(
                "Must have at least 1 racking".to_string()
            ));
        }

        if num_rackings > Decimal::from(20) {
            return Err(Error::Calculation(
                "Number of rackings seems unreasonably high (max 20)".to_string()
            ));
        }

        // Convert num_rackings to u32 for loop iteration
        let num_rackings_u32 = num_rackings
            .to_u32()
            .ok_or_else(|| Error::Calculation("Invalid number of rackings".to_string()))?;

        // Constants for conversions
        let liters_per_gallon = Decimal::new(3785411784, 9); // 3.785411784 L/gal
        let liters_per_bottle = Decimal::new(75, 2); // 0.75 L (750ml standard wine bottle)
        let bottles_per_gallon = liters_per_gallon / liters_per_bottle; // 5.047215712 bottles/gal

        // Convert loss rate from percentage to decimal (5% -> 0.05)
        let loss_rate_decimal = loss_rate_percent / Decimal::from(100);

        // Calculate retention rate (what remains after each racking)
        let retention_rate = Decimal::ONE - loss_rate_decimal;

        // CRITICAL: CORRECT COMPOUNDING MATH
        // Each racking loses a percentage of the CURRENT volume, not the original
        // This is mathematically equivalent to: (1 - loss_rate)^num_rackings
        // We multiply in a loop because Decimal doesn't have a power function
        let mut compounded_retention = Decimal::ONE;
        for _ in 0..num_rackings_u32 {
            compounded_retention *= retention_rate;
        }

        // Calculate final volume after all rackings
        let final_volume_gal = initial_volume_gal * compounded_retention;
        let final_volume_l = final_volume_gal * liters_per_gallon;

        // Calculate total bottles from final volume
        let total_bottles = final_volume_gal * bottles_per_gallon;

        // Calculate total losses
        let total_loss_gal = initial_volume_gal - final_volume_gal;
        let total_loss_l = total_loss_gal * liters_per_gallon;
        let total_loss_percent = (total_loss_gal / initial_volume_gal) * Decimal::from(100);

        // Build comprehensive metadata with all calculation details
        let mut metadata = vec![
            // Initial conditions
            ("Initial Volume (gal)".to_string(), format!("{:.3}", initial_volume_gal)),
            ("Initial Volume (L)".to_string(), format!("{:.3}", initial_volume_gal * liters_per_gallon)),
            ("Initial Bottles (theoretical)".to_string(), format!("{:.2}", initial_volume_gal * bottles_per_gallon)),

            // Loss parameters
            ("Loss Rate Per Racking".to_string(), format!("{:.2}%", loss_rate_percent)),
            ("Number of Rackings".to_string(), format!("{}", num_rackings_u32)),
            ("Retention Rate Per Racking".to_string(), format!("{:.4}", retention_rate)),
            ("Compounded Retention".to_string(), format!("{:.6}", compounded_retention)),

            // Final results
            ("Final Volume (gal)".to_string(), format!("{:.3}", final_volume_gal)),
            ("Final Volume (L)".to_string(), format!("{:.3}", final_volume_l)),
            ("Final Bottles (actual)".to_string(), format!("{:.2}", total_bottles)),

            // Loss calculations
            ("Total Loss (gal)".to_string(), format!("{:.3}", total_loss_gal)),
            ("Total Loss (L)".to_string(), format!("{:.3}", total_loss_l)),
            ("Total Loss (%)".to_string(), format!("{:.2}%", total_loss_percent)),
            ("Total Loss (bottles)".to_string(), format!("{:.2}", total_loss_gal * bottles_per_gallon)),

            // Conversion reference
            ("Bottles per Gallon".to_string(), format!("{:.4}", bottles_per_gallon)),
            ("Bottle Size".to_string(), "750ml (standard wine bottle)".to_string()),
        ];

        // Add detailed racking-by-racking breakdown
        metadata.push(("".to_string(), "".to_string())); // Spacer
        metadata.push(("Racking Breakdown".to_string(), "".to_string()));
        metadata.push(("─────────────────".to_string(), "".to_string()));

        let mut current_volume = initial_volume_gal;
        for i in 1..=num_rackings_u32 {
            let volume_before = current_volume;
            current_volume *= retention_rate;
            let loss_this_racking = volume_before - current_volume;
            let bottles_after = current_volume * bottles_per_gallon;
            let bottles_lost = loss_this_racking * bottles_per_gallon;

            metadata.push((
                format!("Racking {}", i),
                format!(
                    "Volume: {:.3} gal ({:.2} bottles) | Lost: {:.3} gal ({:.2} bottles)",
                    current_volume, bottles_after, loss_this_racking, bottles_lost
                ),
            ));
        }

        // Add comparison to incorrect additive method
        let wrong_total_loss_percent = loss_rate_percent * Decimal::from(num_rackings_u32);
        let wrong_final_volume = initial_volume_gal * (Decimal::ONE - (wrong_total_loss_percent / Decimal::from(100)));
        let wrong_bottles = wrong_final_volume * bottles_per_gallon;
        let difference = total_bottles - wrong_bottles;

        metadata.push(("".to_string(), "".to_string())); // Spacer
        metadata.push(("Math Comparison".to_string(), "".to_string()));
        metadata.push(("───────────────".to_string(), "".to_string()));
        metadata.push((
            "Correct (Compounding)".to_string(),
            format!("{:.2} bottles | {:.2}% total loss", total_bottles, total_loss_percent)
        ));
        metadata.push((
            "Wrong (Additive)".to_string(),
            format!("{:.2} bottles | {:.2}% total loss", wrong_bottles, wrong_total_loss_percent)
        ));
        metadata.push((
            "Difference".to_string(),
            format!("{:.2} bottles ({:.2}% error)", difference, (difference / total_bottles * Decimal::from(100)).abs())
        ));

        // Generate comprehensive warnings
        let mut warnings = Vec::new();

        // Warning for high loss rate per racking
        if loss_rate_percent > Decimal::from(10) {
            warnings.push(format!(
                "⚠️ Loss rate of {:.1}% per racking is quite high. Typical losses are 3-8% per racking. \
                 Consider improving racking technique or checking for equipment issues.",
                loss_rate_percent
            ));
        }

        // Warning for very low loss rate
        if loss_rate_percent < Decimal::from(2) {
            warnings.push(format!(
                "ℹ️ Loss rate of {:.1}% is very low - excellent racking technique! \
                 This assumes very careful siphoning with minimal lees/sediment.",
                loss_rate_percent
            ));
        }

        // Warning for excessive total loss
        if total_loss_percent > Decimal::from(30) {
            warnings.push(format!(
                "⚠️ Total loss of {:.1}% is very high. You're losing {:.2} bottles! \
                 Consider reducing the number of rackings or improving technique.",
                total_loss_percent, total_loss_gal * bottles_per_gallon
            ));
        }

        // Warning for too many rackings
        if num_rackings_u32 > 5 {
            warnings.push(format!(
                "⚠️ {} rackings is quite a lot. Most meads only need 2-4 rackings. \
                 Each additional racking increases oxidation risk and losses.",
                num_rackings_u32
            ));
        }

        // Warning for very few rackings with high losses
        if num_rackings_u32 == 1 && loss_rate_percent > Decimal::from(8) {
            warnings.push(
                "⚠️ Single racking with >8% loss suggests heavy sediment or poor technique. \
                 Consider cold-crashing before racking to compact sediment.".to_string()
            );
        }

        // Informational notes
        if num_rackings_u32 >= 3 && loss_rate_percent <= Decimal::from(5) {
            warnings.push(
                "✓ Good racking technique! Keeping losses ≤5% per racking is excellent practice.".to_string()
            );
        }

        // Add bottle size alternatives note
        warnings.push(
            "ℹ️ This calculation assumes standard 750ml wine bottles. \
             For 375ml (half bottles): multiply by 2. \
             For 1.5L (magnums): divide by 2. \
             For 187ml (splits): multiply by 4.".to_string()
        );

        // Return complete result
        Ok(CalcResult {
            output: Measurement {
                value: total_bottles.round_dp(2),
                unit: Unit::Liters, // TEMPORARY - bottles as "liters" until Count added // Bottle count - ADD unit.rs to crates/core/src/ first!
            },
            metadata,
            warnings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5gal_3rackings_5percent_compounding() {
        let calc = GallonsToBottlesWithLossesCalculator;
        let input = CalcInput::new()
            .add_param("initial_volume", "5")
            .add_param("loss_rate_percent", "5")
            .add_param("num_rackings", "3");

        let result = calc.calculate(input).unwrap();

        // Expected: 5 × (0.95)^3 = 4.286875 gal
        // 4.286875 × 5.047215712 = 21.64 bottles
        assert_eq!(result.output.value.round_dp(2), Decimal::new(2164, 2));
    }

    #[test]
    fn test_compounding_not_additive() {
        let calc = GallonsToBottlesWithLossesCalculator;
        let input = CalcInput::new()
            .add_param("initial_volume", "5")
            .add_param("loss_rate_percent", "5")
            .add_param("num_rackings", "3");

        let result = calc.calculate(input).unwrap();

        // WRONG (additive): 5% + 5% + 5% = 15% total
        // 5 × 0.85 = 4.25 gal × 5.047 = 21.45 bottles (WRONG)
        let wrong_result = Decimal::new(2145, 2);

        // CORRECT (compounding): 5 × (0.95)^3 = 4.287 gal × 5.047 = 21.64 bottles
        let correct_result = Decimal::new(2164, 2);

        let actual = result.output.value.round_dp(2);

        // Verify we're using compounding, not addition
        assert_eq!(actual, correct_result);
        assert_ne!(actual, wrong_result);
    }

    #[test]
    fn test_single_racking() {
        let calc = GallonsToBottlesWithLossesCalculator;
        let input = CalcInput::new()
            .add_param("initial_volume", "5")
            .add_param("loss_rate_percent", "5")
            .add_param("num_rackings", "1");

        let result = calc.calculate(input).unwrap();

        // 5 × 0.95 = 4.75 gal × 5.047 = 23.97 bottles
        assert_eq!(result.output.value.round_dp(2), Decimal::new(2397, 2));
    }

    #[test]
    fn test_high_loss_rate() {
        let calc = GallonsToBottlesWithLossesCalculator;
        let input = CalcInput::new()
            .add_param("initial_volume", "6")
            .add_param("loss_rate_percent", "10")
            .add_param("num_rackings", "4");

        let result = calc.calculate(input).unwrap();

        // 6 × (0.90)^4 = 3.9366 gal × 5.047 = 19.87 bottles
        let expected = Decimal::new(1987, 2);
        let actual = result.output.value.round_dp(2);
        assert!((actual - expected).abs() < Decimal::new(5, 2)); // Within 0.05

        // Should have warnings for high loss
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_invalid_inputs() {
        let calc = GallonsToBottlesWithLossesCalculator;

        // Negative volume
        let input = CalcInput::new()
            .add_param("initial_volume", "-5")
            .add_param("loss_rate_percent", "5")
            .add_param("num_rackings", "3");
        assert!(calc.calculate(input).is_err());

        // Loss rate >= 100%
        let input = CalcInput::new()
            .add_param("initial_volume", "5")
            .add_param("loss_rate_percent", "100")
            .add_param("num_rackings", "3");
        assert!(calc.calculate(input).is_err());

        // Zero rackings
        let input = CalcInput::new()
            .add_param("initial_volume", "5")
            .add_param("loss_rate_percent", "5")
            .add_param("num_rackings", "0");
        assert!(calc.calculate(input).is_err());
    }

    #[test]
    fn test_metadata_includes_breakdown() {
        let calc = GallonsToBottlesWithLossesCalculator;
        let input = CalcInput::new()
            .add_param("initial_volume", "5")
            .add_param("loss_rate_percent", "5")
            .add_param("num_rackings", "2");

        let result = calc.calculate(input).unwrap();

        // Should have detailed racking breakdowns
        let has_racking_1 = result.metadata.iter().any(|(k, _)| k.contains("Racking 1"));
        let has_racking_2 = result.metadata.iter().any(|(k, _)| k.contains("Racking 2"));
        let has_comparison = result.metadata.iter().any(|(k, _)| k.contains("Wrong"));

        assert!(has_racking_1);
        assert!(has_racking_2);
        assert!(has_comparison);
    }
}