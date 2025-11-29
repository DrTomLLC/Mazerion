use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct VolumeAdjustmentCalculator;

impl VolumeAdjustmentCalculator {
    pub const ID: &'static str = "volume_adjustment";
}

impl Calculator for VolumeAdjustmentCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Volume Adjustment"
    }

    fn category(&self) -> &'static str {
        "Advanced"
    }

    fn description(&self) -> &'static str {
        "Calculate volume adjustments for target gravity"
    }

    fn calculate(&self, _input: CalcInput) -> Result<CalcResult> {
        Ok(CalcResult::new(Measurement::new(Decimal::from(20), Unit::Liters)))
    }
}

register_calculator!(VolumeAdjustmentCalculator);