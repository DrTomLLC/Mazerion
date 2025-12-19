//! Waste/Loss Calculator
//! Track losses through entire brewing process - racking, lees, clarification, filtration

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct WasteCalculator;

impl WasteCalculator {
    pub const ID: &'static str = "waste";
}

impl Calculator for WasteCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Waste/Loss Calculator"
    }

    fn category(&self) -> &'static str {
        "Utilities"
    }

    fn description(&self) -> &'static str {
        "Calculate expected losses through brewing process from start to bottle"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let initial_volume = input.get_param("initial_volume")
            .ok_or_else(|| Error::MissingInput("initial_volume required".into()))?;

        let num_rackings = input.get_param("num_rackings")
            .unwrap_or("3"); // Default 3 rackings

        let vessel_type = input.get_param("vessel_type")
            .unwrap_or("carboy"); // carboy, bucket, keg, barrel

        let process_type = input.get_param("process_type")
            .unwrap_or("standard"); // standard, filtered, fined, none

        let vol_l: Decimal = initial_volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;

        let rackings: i32 = num_rackings.parse()
            .map_err(|_| Error::Parse("Invalid number of rackings".into()))?;

        if vol_l <= Decimal::ZERO {
            return Err(Error::Validation("Volume must be positive".into()));
        }

        if rackings < 0 || rackings > 10 {
            return Err(Error::Validation("Rackings must be 0-10".into()));
        }

        // LOSS RATES (liters per racking for 19L/5gal batch - scale proportionally)
        // Based on real-world brewing experience

        // Gross lees (primary fermentation) - highest loss
        let gross_lees_loss_l = match vessel_type {
            "bucket" => vol_l * Decimal::new(8, 2),    // 8% - wide bottom
            "carboy" => vol_l * Decimal::new(6, 2),    // 6% - narrow neck
            "keg" => vol_l * Decimal::new(5, 2),       // 5% - conical
            "barrel" => vol_l * Decimal::new(7, 2),    // 7% - wood absorption
            _ => vol_l * Decimal::new(6, 2),
        };

        // Fine lees (secondary+ rackings) - lower loss per racking
        let fine_lees_loss_per_racking_l = match vessel_type {
            "bucket" => vol_l * Decimal::new(3, 2),    // 3% per racking
            "carboy" => vol_l * Decimal::new(2, 2),    // 2% per racking
            "keg" => vol_l * Decimal::new(15, 3),      // 1.5% per racking
            "barrel" => vol_l * Decimal::new(25, 3),   // 2.5% per racking
            _ => vol_l * Decimal::new(2, 2),
        };

        // Process losses
        let clarification_loss_l = match process_type {
            "filtered" => vol_l * Decimal::new(4, 2),   // 4% - filter pads absorb
            "fined" => vol_l * Decimal::new(2, 2),      // 2% - fining agents settle
            "standard" => vol_l * Decimal::new(1, 2),   // 1% - natural settling
            "none" => Decimal::ZERO,
            _ => vol_l * Decimal::new(1, 2),
        };

        // Vessel transfer losses (hose deadspace, spillage, etc.)
        let transfer_loss_per_racking_l = vol_l * Decimal::new(5, 3); // 0.5% per transfer

        // Calculate cumulative losses
        let total_racking_loss_l = if rackings == 0 {
            Decimal::ZERO
        } else if rackings == 1 {
            gross_lees_loss_l + transfer_loss_per_racking_l
        } else {
            // First racking is gross lees, rest are fine lees
            gross_lees_loss_l +
                (fine_lees_loss_per_racking_l * Decimal::from(rackings - 1)) +
                (transfer_loss_per_racking_l * Decimal::from(rackings))
        };

        let total_loss_l = total_racking_loss_l + clarification_loss_l;
        let final_volume_l = vol_l - total_loss_l;
        let loss_percent = (total_loss_l / vol_l) * Decimal::from(100);

        // Individual loss breakdown
        let gross_lees_percent = (gross_lees_loss_l / vol_l) * Decimal::from(100);
        let fine_lees_total_l = if rackings > 1 {
            fine_lees_loss_per_racking_l * Decimal::from(rackings - 1)
        } else {
            Decimal::ZERO
        };
        let fine_lees_percent = (fine_lees_total_l / vol_l) * Decimal::from(100);
        let transfer_total_l = transfer_loss_per_racking_l * Decimal::from(rackings);
        let transfer_percent = (transfer_total_l / vol_l) * Decimal::from(100);
        let clarification_percent = (clarification_loss_l / vol_l) * Decimal::from(100);

        let mut result = CalcResult::new(Measurement::new(final_volume_l, Unit::Grams))
            .with_meta("initial_volume", format!("{:.2} L", vol_l))
            .with_meta("final_volume", format!("{:.2} L", final_volume_l))
            .with_meta("total_loss", format!("{:.2} L ({:.1}%)", total_loss_l, loss_percent))
            .with_meta("num_rackings", format!("{}", rackings))
            .with_meta("vessel_type", vessel_type)
            .with_meta("process_type", process_type);

        // Detailed breakdown
        if rackings > 0 {
            result = result.with_meta("loss_gross_lees", format!("{:.2} L ({:.1}%) - Primary fermentation sediment", gross_lees_loss_l, gross_lees_percent));
        }

        if rackings > 1 {
            result = result.with_meta("loss_fine_lees", format!("{:.2} L ({:.1}%) - {} secondary rackings", fine_lees_total_l, fine_lees_percent, rackings - 1));
        }

        if rackings > 0 {
            result = result.with_meta("loss_transfers", format!("{:.2} L ({:.1}%) - Hose deadspace & spillage", transfer_total_l, transfer_percent));
        }

        if clarification_loss_l > Decimal::ZERO {
            result = result.with_meta("loss_clarification", format!("{:.2} L ({:.1}%) - {}", clarification_loss_l, clarification_percent, process_type));
        }

        // Per-racking details
        if rackings > 0 {
            result = result.with_meta("racking_1", format!("Primary → Secondary: {:.2} L loss (gross lees + transfer)", gross_lees_loss_l + transfer_loss_per_racking_l));
        }

        for i in 2..=rackings {
            let loss = fine_lees_loss_per_racking_l + transfer_loss_per_racking_l;
            result = result.with_meta(
                format!("racking_{}", i),
                format!("Racking {}: {:.2} L loss (fine lees + transfer)", i, loss)
            );
        }

        // Warnings
        if loss_percent > Decimal::from(25) {
            result = result.with_warning("⚠️ High loss rate (>25%) - consider fewer rackings or different vessel");
        }

        if rackings > 4 {
            result = result.with_warning("ℹ️ Many rackings - ensure benefits outweigh losses");
        }

        if final_volume_l < vol_l * Decimal::new(6, 1) {
            result = result.with_warning("⚠️ Less than 60% recovery - process may be too aggressive");
        }

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("initial_volume").is_none() {
            return Err(Error::MissingInput("initial_volume required".into()));
        }
        Ok(())
    }
}

register_calculator!(WasteCalculator);