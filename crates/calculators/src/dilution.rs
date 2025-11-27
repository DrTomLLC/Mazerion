//! Dilution calculator - adjust ABV by water addition.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;
use std::str::FromStr;

/// Calculate water needed to dilute to target ABV.
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
        "Calculate water needed to reduce ABV"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let current_vol = input
            .get_param("current_volume")
            .ok_or_else(|| Error::MissingInput("current_volume required".into()))?;
        let current_abv = input
            .get_param("current_abv")
            .ok_or_else(|| Error::MissingInput("current_abv required".into()))?;
        let target_abv = input
            .get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;

        let vol: Decimal = current_vol
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid volume: {}", current_vol)))?;
        let cur_abv: Decimal = current_abv
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid current ABV: {}", current_abv)))?;
        let tgt_abv: Decimal = target_abv
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid target ABV: {}", target_abv)))?;

        if cur_abv <= tgt_abv {
            return Err(Error::Validation(
                "Current ABV must be greater than target ABV".into(),
            ));
        }

        if tgt_abv <= Decimal::ZERO {
            return Err(Error::Validation("Target ABV must be positive".into()));
        }

        // Water needed = current_vol × (current_abv / target_abv - 1)
        let water_needed = vol * (cur_abv / tgt_abv - Decimal::ONE);

        let mut result = CalcResult::new(Measurement::new(water_needed, Unit::Liters));

        result = result
            .with_meta("current_volume", format!("{} L", vol))
            .with_meta("current_abv", format!("{}%", cur_abv))
            .with_meta("target_abv", format!("{}%", tgt_abv))
            .with_meta("final_volume", format!("{} L", vol + water_needed));

        Ok(result)
    }
}

register_calculator!(DilutionCalculator);