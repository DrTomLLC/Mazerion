//! Sulfite calculator with pH-dependent effectiveness.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

/// Calculate K-meta additions with pH-dependent effectiveness.
#[derive(Default)]
pub struct SulfiteCalculator;

impl SulfiteCalculator {
    pub const ID: &'static str = "sulfite";
}

impl Calculator for SulfiteCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Sulfite Calculator"
    }

    fn description(&self) -> &'static str {
        "Calculate K-meta additions with pH-dependent effectiveness"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let ph_meas = input.get_measurement(Unit::Ph)?;
        let ph = ph_meas.value;

        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;

        let target_free_so2 = input.get_param("target_free_so2")
            .ok_or_else(|| Error::MissingInput("target_free_so2 required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid target_free_so2".into()))?;

        if volume <= Decimal::ZERO {
            return Err(Error::OutOfRange("Volume must be positive".into()));
        }

        let ph_diff = ph - Decimal::new(181, 2);
        let molecular_percent = Decimal::ONE / (Decimal::ONE + Decimal::from(10));

        let k_meta_strength = Decimal::new(576, 3);

        let total_so2_needed = target_free_so2 / molecular_percent;

        let k_meta_grams = (total_so2_needed * volume) / (Decimal::from(1000) * k_meta_strength);

        let mut result = CalcResult::new(Measurement::grams(k_meta_grams)?);

        if ph > Decimal::new(38, 1) {
            result = result.with_warning("pH > 3.8 - sulfite less effective");
        }

        if target_free_so2 > Decimal::from(50) {
            result = result.with_warning("High free SO2 target - may affect flavor");
        }

        result = result
            .with_meta("ph", ph.to_string())
            .with_meta("volume", format!("{} L", volume))
            .with_meta("target_free_so2", format!("{} ppm", target_free_so2))
            .with_meta("total_so2", format!("{:.1} ppm", total_so2_needed))
            .with_meta("note", "Add potassium metabisulfite (K-meta)");

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        input.get_measurement(Unit::Ph)?;
        let required = ["volume", "target_free_so2"];
        for param in &required {
            if input.get_param(param).is_none() {
                return Err(Error::MissingInput(format!("{} required", param)));
            }
        }
        Ok(())
    }
}

register_calculator!(SulfiteCalculator);