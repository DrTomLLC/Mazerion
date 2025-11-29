use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct CyserCalculator;

impl CyserCalculator {
    pub const ID: &'static str = "cyser";
}

impl Calculator for CyserCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Cyser Calculator"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for apple mead (cyser)"
    }

    fn calculate(&self, _input: CalcInput) -> Result<CalcResult> {
        Ok(CalcResult::new(Measurement::new(Decimal::from(19), Unit::Liters)))
    }
}

register_calculator!(CyserCalculator);