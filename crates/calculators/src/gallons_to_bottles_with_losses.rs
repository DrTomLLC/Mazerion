//! Gallons to Bottles calculator with integrated loss calculation
//! Accounts for gross lees, fine lees, racking losses, and vessel geometry

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Default)]
pub struct GallonsToBottlesWithLosses;

impl GallonsToBottlesWithLosses {
    pub const ID: &'static str = "gallons_to_bottles_with_losses";
}

impl Calculator for GallonsToBottlesWithLosses {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Gallons to Bottles (with Losses)"
    }

    fn description(&self) -> &'static str {
        "Calculate bottle count accounting for brewing losses"
    }

    fn category(&self) -> &'static str {
        "Utilities"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        // Get initial volume in liters
        let volume_str = input.params.iter()
            .find(|(k, _)| k == "initial_volume")
            .map(|(_, v)| v.as_str())
            .ok_or(Error::MissingInput("initial_volume".into()))?;
        let mut volume_l = Decimal::from_str(volume_str)
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;

        if volume_l <= Decimal::ZERO {
            return Err(Error::Validation("Volume must be positive".into()));
        }

        // Get vessel type (default: carboy)
        let vessel_type = input.params.iter()
            .find(|(k, _)| k == "vessel_type")
            .map(|(_, v)| v.as_str())
            .unwrap_or("carboy");

        // Get number of rackings (default: 0)
        let num_rackings_str = input.params.iter()
            .find(|(k, _)| k == "num_rackings")
            .map(|(_, v)| v.as_str())
            .unwrap_or("0");
        let num_rackings = u32::from_str(num_rackings_str)
            .map_err(|e| Error::Parse(format!("Invalid num_rackings: {}", e)))?;

        if num_rackings > 10 {
            return Err(Error::Validation("Maximum 10 rackings allowed".into()));
        }

        // Get process type (default: standard)
        let process_type = input.params.iter()
            .find(|(k, _)| k == "process_type")
            .map(|(_, v)| v.as_str())
            .unwrap_or("standard");

        // Calculate losses
        let initial_volume = volume_l;

        // 1. Gross lees (primary fermentation) - vessel dependent
        let gross_lees_pct = match vessel_type {
            "carboy" => Decimal::new(6, 2),      // 6%
            "bucket" => Decimal::new(8, 2),      // 8%
            "keg" => Decimal::new(5, 2),         // 5%
            "barrel" => Decimal::new(7, 2),      // 7%
            _ => Decimal::new(6, 2),             // Default 6%
        };
        let gross_lees_loss = volume_l * gross_lees_pct;
        volume_l -= gross_lees_loss;

        // 2. Fine lees (secondary rackings) - progressive loss
        let mut fine_lees_loss = Decimal::ZERO;
        let mut transfer_loss = Decimal::ZERO;

        if num_rackings > 0 {
            // First racking after primary (typically higher loss)
            let first_racking_pct = Decimal::new(2, 2);  // 2%
            let first_loss = volume_l * first_racking_pct;
            fine_lees_loss += first_loss;
            volume_l -= first_loss;

            // Transfer loss for first racking
            let first_transfer = volume_l * Decimal::new(5, 3);  // 0.5%
            transfer_loss += first_transfer;
            volume_l -= first_transfer;

            // Subsequent rackings (diminishing loss)
            for i in 1..num_rackings {
                let racking_pct = if i == 1 {
                    Decimal::new(15, 3)  // 1.5% for second racking
                } else {
                    Decimal::new(1, 2)   // 1% for subsequent rackings
                };

                let racking_loss = volume_l * racking_pct;
                fine_lees_loss += racking_loss;
                volume_l -= racking_loss;

                // Transfer loss each time
                let trans = volume_l * Decimal::new(5, 3);  // 0.5%
                transfer_loss += trans;
                volume_l -= trans;
            }
        }

        // 3. Clarification losses
        let clarification_pct = match process_type {
            "standard" => Decimal::new(1, 2),    // 1%
            "fined" => Decimal::new(2, 2),       // 2%
            "filtered" => Decimal::new(4, 2),    // 4%
            "none" => Decimal::ZERO,
            _ => Decimal::new(1, 2),
        };
        let clarification_loss = volume_l * clarification_pct;
        volume_l -= clarification_loss;

        // Calculate total loss
        let total_loss = initial_volume - volume_l;
        let loss_pct = (total_loss / initial_volume) * Decimal::from(100);

        // Now calculate bottles from final volume
        let vol_ml = volume_l * Decimal::from(1000);

        // Bottle sizes in mL
        let bottle_12oz = Decimal::new(35488, 2);
        let bottle_375ml = Decimal::from(375);
        let bottle_500ml = Decimal::from(500);
        let bottle_750ml = Decimal::from(750);
        let bottle_1l = Decimal::from(1000);
        let bottle_1_5l = Decimal::from(1500);
        let bottle_3l = Decimal::from(3000);
        let bottle_5l = Decimal::from(5000);
        let bottle_6l = Decimal::from(6000);

        // Calculate bottle counts
        let bottles_12oz = (vol_ml / bottle_12oz).round_dp(0);
        let bottles_375ml = (vol_ml / bottle_375ml).round_dp(0);
        let bottles_500ml = (vol_ml / bottle_500ml).round_dp(0);
        let bottles_750ml = (vol_ml / bottle_750ml).round_dp(0);
        let bottles_1l = (vol_ml / bottle_1l).round_dp(0);
        let bottles_1_5l = (vol_ml / bottle_1_5l).round_dp(0);
        let bottles_3l = (vol_ml / bottle_3l).round_dp(0);
        let bottles_5l = (vol_ml / bottle_5l).round_dp(0);
        let bottles_6l = (vol_ml / bottle_6l).round_dp(0);

        // Calculate cases
        let cases_12oz = (bottles_12oz / Decimal::from(24)).ceil();
        let cases_375ml = (bottles_375ml / Decimal::from(12)).ceil();
        let cases_750ml = (bottles_750ml / Decimal::from(12)).ceil();
        let cases_1l = (bottles_1l / Decimal::from(12)).ceil();

        // Primary output: final volume in liters after losses
        let output = Measurement::new(volume_l, Unit::Liters);

        let mut result = CalcResult::new(output)
            .with_meta("initial_volume", format!("{:.2} L", initial_volume))
            .with_meta("final_volume", format!("{:.2} L", volume_l))
            .with_meta("total_loss", format!("{:.2} L ({:.1}%)", total_loss, loss_pct))
            .with_meta("gross_lees_loss", format!("{:.2} L ({:.1}%)", gross_lees_loss, gross_lees_pct * Decimal::from(100)))
            .with_meta("fine_lees_loss", format!("{:.2} L", fine_lees_loss))
            .with_meta("transfer_loss", format!("{:.2} L", transfer_loss))
            .with_meta("clarification_loss", format!("{:.2} L", clarification_loss))
            .with_meta("vessel_type", vessel_type)
            .with_meta("num_rackings", format!("{}", num_rackings))
            .with_meta("bottles_12oz", format!("{} bottles (12 oz / 355 mL)", bottles_12oz))
            .with_meta("bottles_375ml", format!("{} bottles (375 mL / half-bottle)", bottles_375ml))
            .with_meta("bottles_500ml", format!("{} bottles (500 mL)", bottles_500ml))
            .with_meta("bottles_750ml", format!("{} bottles (750 mL / standard wine)", bottles_750ml))
            .with_meta("bottles_1L", format!("{} bottles (1 L / magnum)", bottles_1l))
            .with_meta("bottles_1.5L", format!("{} bottles (1.5 L)", bottles_1_5l))
            .with_meta("bottles_3L", format!("{} bottles (3 L / double magnum)", bottles_3l))
            .with_meta("bottles_5L", format!("{} bottles (5 L / jeroboam)", bottles_5l))
            .with_meta("bottles_6L", format!("{} bottles (6 L / imperial)", bottles_6l))
            .with_meta("cases_12oz", format!("{} cases (24 × 12oz)", cases_12oz))
            .with_meta("cases_375ml", format!("{} cases (12 × 375mL)", cases_375ml))
            .with_meta("cases_750ml", format!("{} cases (12 × 750mL)", cases_750ml))
            .with_meta("cases_1L", format!("{} cases (12 × 1L)", cases_1l))
            .with_meta("volume_gallons", format!("{:.2} gal", volume_l * Decimal::new(264172, 6)))
            .with_meta("volume_liters", format!("{:.2} L", volume_l))
            .with_meta("volume_quarts", format!("{:.2} qt", volume_l * Decimal::new(105669, 5)))
            .with_meta("volume_fluid_ounces", format!("{:.1} fl oz", volume_l * Decimal::new(338140, 4)));

        // Warnings
        if loss_pct > Decimal::from(25) {
            result = result.with_warning(format!("⚠️ High total loss: {:.1}%", loss_pct));
        }

        if num_rackings > 4 {
            result = result.with_warning(format!("⚠️ Many rackings ({}) - consider reducing", num_rackings));
        }

        let recovery_pct = (volume_l / initial_volume) * Decimal::from(100);
        if recovery_pct < Decimal::from(60) {
            result = result.with_warning(format!("⚠️ Low recovery: {:.1}%", recovery_pct));
        }

        Ok(result)
    }
}

register_calculator!(GallonsToBottlesWithLosses);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5gal_carboy_3_rackings_standard() {
        let calc = GallonsToBottlesWithLosses;
        let input = CalcInput::new()
            .add_param("initial_volume", "18.93")
            .add_param("vessel_type", "carboy")
            .add_param("num_rackings", "3")
            .add_param("process_type", "standard");

        let result = calc.calculate(input).unwrap();
        assert!(result.output.value > Decimal::ZERO);
    }
}