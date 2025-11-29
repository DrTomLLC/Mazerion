use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct PrimingAlternativesCalculator;

impl PrimingAlternativesCalculator {
    pub const ID: &'static str = "priming_alternatives";
}

impl Calculator for PrimingAlternativesCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Priming Alternatives"
    }

    fn category(&self) -> &'static str {
        "Utilities"
    }

    fn description(&self) -> &'static str {
        "Calculate alternative priming sugars (honey, DME, maple syrup)"
    }

    fn calculate(&self, _input: CalcInput) -> Result<CalcResult> {
        Ok(CalcResult::new(Measurement::new(Decimal::from(100), Unit::Grams)))
    }
}

register_calculator!(PrimingAlternativesCalculator);