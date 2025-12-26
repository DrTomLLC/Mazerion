use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;

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

    fn category(&self) -> &'static str {
        "Advanced"
    }

    fn description(&self) -> &'static str {
        "Calculate final properties when blending two batches"
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
            .map_err(|_| Error::Parse("Invalid volume1".into()))?;
        let a1: Decimal = abv1
            .parse()
            .map_err(|_| Error::Parse("Invalid abv1".into()))?;
        let v2: Decimal = vol2
            .parse()
            .map_err(|_| Error::Parse("Invalid volume2".into()))?;
        let a2: Decimal = abv2
            .parse()
            .map_err(|_| Error::Parse("Invalid abv2".into()))?;

        if v1 < Decimal::ZERO || v2 < Decimal::ZERO {
            return Err(Error::Validation("Volumes must be non-negative".into()));
        }

        let total_vol = v1 + v2;

        if total_vol == Decimal::ZERO {
            return Err(Error::Validation(
                "Total volume cannot be zero - at least one batch must have volume".into(),
            ));
        }

        let blended_abv = (v1 * a1 + v2 * a2) / total_vol;

        Ok(CalcResult::new(Measurement::new(blended_abv, Unit::Abv))
            .with_meta("total_volume", total_vol.to_string())
            .with_meta("blended_abv", blended_abv.to_string()))
    }
}

register_calculator!(BlendingCalculator);