use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct VolumeAdjustmentCalculator;

impl VolumeAdjustmentCalculator {
    pub const ID: &'static str = "volume_adjustment";
}

impl Calculator for VolumeAdjustmentCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Volume Adjustment" }
    fn description(&self) -> &'static str { "Scale recipe to different batch size" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let original_vol = input.get_param("original_volume")
            .ok_or_else(|| Error::MissingInput("original_volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid original_volume: {}", e)))?;
        let target_vol = input.get_param("target_volume")
            .ok_or_else(|| Error::MissingInput("target_volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid target_volume: {}", e)))?;
        let ingredient_amt = input.get_param("ingredient_amount")
            .ok_or_else(|| Error::MissingInput("ingredient_amount required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid ingredient_amount: {}", e)))?;

        let scale_factor = target_vol / original_vol;
        let scaled_amt = ingredient_amt * scale_factor;
        let mut result = CalcResult::new(Measurement::new(scaled_amt, Unit::Grams));
        result = result.with_meta("scale_factor", format!("{:.2}", scale_factor));
        Ok(result)
    }
}

register_calculator!(VolumeAdjustmentCalculator);
