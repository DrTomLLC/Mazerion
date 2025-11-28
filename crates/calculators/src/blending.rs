//! Blending calculator - mix two batches.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

/// Calculate blended properties.
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
        let vol1 = input
            .get_param("volume1")
            .ok_or_else(|| Error::MissingInput("volume1 required".into()))?;
        let abv1 = input
            .get_param("abv1")
            .ok_or_else(|| Error::MissingInput("abv1 required".into()))?;
        let vol2 = input
            .get_param("volume2")
            .ok_or_else(|| Error::MissingInput("volume2 required".into()))?;
        let abv2 = input
            .get_param("abv2")
            .ok_or_else(|| Error::MissingInput("abv2 required".into()))?;

        let v1: Decimal = vol1
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid volume1: {}", vol1)))?;
        let a1: Decimal = abv1
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid abv1: {}", abv1)))?;
        let v2: Decimal = vol2
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid volume2: {}", vol2)))?;
        let a2: Decimal = abv2
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid abv2: {}", abv2)))?;

        let total_volume = v1 + v2;
        let blended_abv = (v1 * a1 + v2 * a2) / total_volume;

        let mut result = CalcResult::new(Measurement::new(blended_abv, Unit::Abv));
        result = result
            .with_meta("batch1", format!("{} L @ {}%", v1, a1))
            .with_meta("batch2", format!("{} L @ {}%", v2, a2))
            .with_meta("total_volume", format!("{} L", total_volume));

        Ok(result)
    }
}

register_calculator!(BlendingCalculator);