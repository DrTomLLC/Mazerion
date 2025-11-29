use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct MelomelCalculator;

impl MelomelCalculator {
    pub const ID: &'static str = "melomel";
}

impl Calculator for MelomelCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Melomel Calculator"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for fruit mead (melomel)"
    }

    fn calculate(&self, _input: CalcInput) -> Result<CalcResult> {
        Ok(CalcResult::new(Measurement::new(Decimal::from(19), Unit::Liters)))
    }
}

register_calculator!(MelomelCalculator);