use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct BlendingCalculator;

impl BlendingCalculator {
    pub const ID: &'static str = "blending";
}

impl Calculator for BlendingCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Blending Calculator" }
    fn description(&self) -> &'static str { "Calculate final ABV when mixing two batches" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let vol1 = input.get_param("volume1")
            .ok_or_else(|| Error::MissingInput("volume1 required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume1: {}", e)))?;
        let abv1 = input.get_param("abv1")
            .ok_or_else(|| Error::MissingInput("abv1 required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid abv1: {}", e)))?;
        let vol2 = input.get_param("volume2")
            .ok_or_else(|| Error::MissingInput("volume2 required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume2: {}", e)))?;
        let abv2 = input.get_param("abv2")
            .ok_or_else(|| Error::MissingInput("abv2 required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid abv2: {}", e)))?;

        let total_vol = vol1 + vol2;
        let total_alcohol = (vol1 * abv1) + (vol2 * abv2);
        let blended_abv = total_alcohol / total_vol;

        let mut result = CalcResult::new(Measurement::new(blended_abv, Unit::Abv));
        result = result
            .with_meta("total_volume", format!("{:.2}L", total_vol))
            .with_meta("total_alcohol", format!("{:.2}L", total_alcohol / Decimal::from(100)));
        Ok(result)
    }
}

register_calculator!(BlendingCalculator);
