//! Blending calculator for mixing two batches.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

/// Calculate final properties when blending two batches.
#[derive(Default)]
pub struct BlendingCalculator;

impl BlendingCalculator {
    pub const ID: &'static str = "blending";
}

impl Calculator for BlendingCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Blending Calculator"
    }

    fn description(&self) -> &'static str {
        "Calculate final ABV when mixing two batches"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume1 = input.get_param("volume1")
            .ok_or_else(|| Error::MissingInput("volume1 required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid volume1".into()))?;

        let abv1 = input.get_param("abv1")
            .ok_or_else(|| Error::MissingInput("abv1 required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid abv1".into()))?;

        let volume2 = input.get_param("volume2")
            .ok_or_else(|| Error::MissingInput("volume2 required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid volume2".into()))?;

        let abv2 = input.get_param("abv2")
            .ok_or_else(|| Error::MissingInput("abv2 required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid abv2".into()))?;

        if volume1 <= Decimal::ZERO || volume2 <= Decimal::ZERO {
            return Err(Error::OutOfRange("Volumes must be positive".into()));
        }

        let total_volume = volume1 + volume2;
        let total_alcohol = (volume1 * abv1) + (volume2 * abv2);
        let blended_abv = total_alcohol / total_volume;

        let mut result = CalcResult::new(Measurement::percent(blended_abv)?);

        let abv_diff = (abv1 - abv2).abs();
        if abv_diff > Decimal::from(5) {
            result = result.with_warning("Large ABV difference - blend gradually");
        }

        result = result
            .with_meta("batch1", format!("{} L at {}% ABV", volume1, abv1))
            .with_meta("batch2", format!("{} L at {}% ABV", volume2, abv2))
            .with_meta("total_volume", format!("{} L", total_volume));

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        let required = ["volume1", "abv1", "volume2", "abv2"];
        for param in &required {
            if input.get_param(param).is_none() {
                return Err(Error::MissingInput(format!("{} required", param)));
            }
        }
        Ok(())
    }
}

register_calculator!(BlendingCalculator);