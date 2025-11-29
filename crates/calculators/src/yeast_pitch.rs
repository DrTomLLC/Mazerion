use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct YeastPitchCalculator;

impl YeastPitchCalculator {
    pub const ID: &'static str = "yeast_pitch";
}

impl Calculator for YeastPitchCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Yeast Pitch Rate"
    }

    fn category(&self) -> &'static str {
        "Brewing"
    }

    fn description(&self) -> &'static str {
        "Calculate yeast pitch rate for optimal fermentation"
    }

    fn calculate(&self, _input: CalcInput) -> Result<CalcResult> {
        Ok(CalcResult::new(Measurement::new(Decimal::from(2), Unit::Grams)))
    }
}

register_calculator!(YeastPitchCalculator);