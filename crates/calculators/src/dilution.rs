//! Dilution calculator for ABV adjustment.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

/// Calculate water needed to reduce ABV.
#[derive(Default)]
pub struct DilutionCalculator;

impl DilutionCalculator {
    pub const ID: &'static str = "dilution";
}

impl Calculator for DilutionCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Dilution Calculator"
    }

    fn description(&self) -> &'static str {
        "Calculate water needed to reduce ABV to target level"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let current_volume = input.get_param("current_volume")
            .ok_or_else(|| Error::MissingInput("current_volume required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid current volume".into()))?;

        let current_abv = input.get_param("current_abv")
            .ok_or_else(|| Error::MissingInput("current_abv required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid current ABV".into()))?;

        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid target ABV".into()))?;

        if current_volume <= Decimal::ZERO {
            return Err(Error::OutOfRange("Volume must be positive".into()));
        }

        if current_abv <= target_abv {
            return Err(Error::Validation("Current ABV must be greater than target ABV".into()));
        }

        if target_abv < Decimal::ZERO {
            return Err(Error::OutOfRange("Target ABV cannot be negative".into()));
        }

        let water_needed = current_volume * ((current_abv / target_abv) - Decimal::ONE);
        let final_volume = current_volume + water_needed;

        let mut result = CalcResult::new(Measurement::liters(water_needed)?);

        if water_needed / current_volume > Decimal::new(5, 1) {
            result = result.with_warning("Adding more than 50% water - may affect flavor");
        }

        result = result
            .with_meta("current_volume", format!("{} L", current_volume))
            .with_meta("current_abv", format!("{}%", current_abv))
            .with_meta("target_abv", format!("{}%", target_abv))
            .with_meta("final_volume", format!("{} L", final_volume));

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("current_volume").is_none() {
            return Err(Error::MissingInput("current_volume required".into()));
        }
        if input.get_param("current_abv").is_none() {
            return Err(Error::MissingInput("current_abv required".into()));
        }
        if input.get_param("target_abv").is_none() {
            return Err(Error::MissingInput("target_abv required".into()));
        }
        Ok(())
    }
}

register_calculator!(DilutionCalculator);