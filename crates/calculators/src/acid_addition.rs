//! Acid addition calculator for pH adjustment.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

/// Calculate acid additions to adjust pH.
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

    fn description(&self) -> &'static str {
        "Calculate acid additions to adjust pH"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let ph_meas = input.get_measurement(Unit::Ph)?;
        let current_ph = ph_meas.value;

        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;

        let target_ph = input.get_param("target_ph")
            .ok_or_else(|| Error::MissingInput("target_ph required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid target_ph".into()))?;

        let acid_type = input.get_param("acid_type").unwrap_or("tartaric");

        if target_ph >= current_ph {
            return Err(Error::Validation("Target pH must be lower than current pH".into()));
        }

        let ph_drop = current_ph - target_ph;

        let acid_factor = match acid_type {
            "tartaric" => Decimal::new(15, 1),
            "citric" => Decimal::new(17, 1),
            "malic" => Decimal::new(18, 1),
            "lactic" => Decimal::new(22, 1),
            _ => Decimal::new(17, 1),
        };

        let acid_needed = (ph_drop / Decimal::new(1, 1)) * acid_factor * volume;

        let mut result = CalcResult::new(Measurement::grams(acid_needed)?);

        if ph_drop > Decimal::new(5, 1) {
            result = result.with_warning("Large pH adjustment - add in increments");
        }

        if target_ph < Decimal::new(3, 0) {
            result = result.with_warning("Very low target pH - may affect flavor");
        }

        let acid_name = match acid_type {
            "tartaric" => "Tartaric Acid (strongest, wine)",
            "citric" => "Citric Acid (bright, fruity)",
            "malic" => "Malic Acid (soft, apple-like)",
            "lactic" => "Lactic Acid (smooth, creamy)",
            _ => "Acid",
        };

        result = result
            .with_meta("acid_type", acid_name)
            .with_meta("current_ph", current_ph.to_string())
            .with_meta("target_ph", target_ph.to_string())
            .with_meta("ph_drop", format!("{:.2}", ph_drop))
            .with_meta("volume", format!("{} L", volume));

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        input.get_measurement(Unit::Ph)?;
        let required = ["volume", "target_ph"];
        for param in &required {
            if input.get_param(param).is_none() {
                return Err(Error::MissingInput(format!("{} required", param)));
            }
        }
        Ok(())
    }
}

register_calculator!(AcidAdditionCalculator);