use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct WaterChemistryCalculator;

impl WaterChemistryCalculator {
    pub const ID: &'static str = "water_chemistry";
}

impl Calculator for WaterChemistryCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Water Chemistry"
    }

    fn category(&self) -> &'static str {
        "Utilities"
    }

    fn description(&self) -> &'static str {
        "Calculate water chemistry adjustments (mineral additions)"
    }

    fn calculate(&self, _input: CalcInput) -> Result<CalcResult> {
        Ok(CalcResult::new(Measurement::new(Decimal::from(5), Unit::Grams)))
    }
}

register_calculator!(WaterChemistryCalculator);