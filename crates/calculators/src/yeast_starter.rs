use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct YeastStarterCalculator;

impl YeastStarterCalculator {
    pub const ID: &'static str = "yeast_starter";
}

impl Calculator for YeastStarterCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Yeast Starter"
    }

    fn category(&self) -> &'static str {
        "Brewing"
    }

    fn description(&self) -> &'static str {
        "Calculate yeast starter requirements"
    }

    fn calculate(&self, _input: CalcInput) -> Result<CalcResult> {
        Ok(CalcResult::new(Measurement::new(Decimal::from(500), Unit::Milliliters)))
    }
}

register_calculator!(YeastStarterCalculator);