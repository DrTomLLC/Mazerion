//! Acid addition calculator - SAFETY CRITICAL
//! Different acids have different strengths - MUST account for this

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct AcidAdditionCalculator;

impl AcidAdditionCalculator {
    pub const ID: &'static str = "acid_addition";
}

impl Calculator for AcidAdditionCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Acid Addition"
    }

    fn category(&self) -> &'static str {
        "Finishing"
    }

    fn description(&self) -> &'static str {
        "Calculate acid additions to adjust pH - accounts for different acid strengths"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let ph_meas = input.get_measurement(Unit::Ph)?;
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required (in liters)".into()))?;
        let target_ph = input.get_param("target_ph")
            .ok_or_else(|| Error::MissingInput("target_ph required".into()))?;
        let acid_type = input.get_param("acid_type").unwrap_or("tartaric");

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let curr_ph = ph_meas.value;
        let targ_ph: Decimal = target_ph.parse()
            .map_err(|_| Error::Parse("Invalid target_ph".into()))?;

        if targ_ph >= curr_ph {
            return Err(Error::Validation("Target pH must be lower than current pH (acid lowers pH)".into()));
        }

        let ph_diff = curr_ph - targ_ph;

        // CRITICAL: Different acids have different strengths
        // Values are g/L needed per 0.1 pH drop
        let acid_strength_factor = match acid_type {
            "tartaric" => Decimal::new(15, 2),   // 0.15 g/L per 0.1 pH (strongest)
            "citric" => Decimal::new(17, 2),     // 0.17 g/L per 0.1 pH
            "malic" => Decimal::new(19, 2),      // 0.19 g/L per 0.1 pH
            "lactic" => Decimal::new(22, 2),     // 0.22 g/L per 0.1 pH (weakest)
            _ => Decimal::new(15, 2),            // default to tartaric
        };

        // Acid needed (g) = volume (L) × pH_diff × strength_factor × 10
        // The ×10 converts from "per 0.1 pH" to "per 1.0 pH"
        let acid_needed_g = vol * ph_diff * acid_strength_factor * Decimal::from(10);

        let acid_name = match acid_type {
            "tartaric" => "Tartaric Acid (wine standard, strongest)",
            "citric" => "Citric Acid (bright, fruity character)",
            "malic" => "Malic Acid (soft, apple-like)",
            "lactic" => "Lactic Acid (smooth, creamy)",
            _ => "Tartaric Acid",
        };

        let mut result = CalcResult::new(Measurement::new(acid_needed_g, Unit::Grams));

        if ph_diff > Decimal::new(5, 1) {
            result = result.with_warning("Large pH drop (>0.5) - add in stages and taste between additions");
        }

        if ph_diff > Decimal::ONE {
            result = result.with_warning("Very large pH drop (>1.0) - MUST add in multiple stages over days");
        }

        if targ_ph < Decimal::new(30, 1) {
            result = result.with_warning("Target pH <3.0 - will taste very tart/sour");
        }

        result = result.with_warning("Always add acid gradually - easy to add, impossible to remove");

        result = result
            .with_meta("acid_type", acid_name)
            .with_meta("acid_needed_g", format!("{:.2}", acid_needed_g))
            .with_meta("acid_needed_tsp", format!("{:.2}", acid_needed_g / Decimal::new(5, 0)))
            .with_meta("current_ph", format!("{:.2}", curr_ph))
            .with_meta("target_ph", format!("{:.2}", targ_ph))
            .with_meta("ph_change", format!("{:.2}", ph_diff))
            .with_meta("strength_factor", format!("{:.2}", acid_strength_factor))
            .with_meta("volume_liters", format!("{:.2}", vol));

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        input.get_measurement(Unit::Ph)?;
        if input.get_param("volume").is_none() {
            return Err(Error::MissingInput("volume required".into()));
        }
        if input.get_param("target_ph").is_none() {
            return Err(Error::MissingInput("target_ph required".into()));
        }
        Ok(())
    }
}

register_calculator!(AcidAdditionCalculator);