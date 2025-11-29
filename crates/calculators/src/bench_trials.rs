use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct BenchTrialsCalculator;

impl BenchTrialsCalculator {
    pub const ID: &'static str = "bench_trials";
}

impl Calculator for BenchTrialsCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Bench Trials"
    }

    fn description(&self) -> &'static str {
        "Calculate bench trial additions and scaling"
    }

    fn category(&self) -> &'static str {
        "Advanced"
    }

    fn calculate(&self, _input: CalcInput) -> Result<CalcResult> {
        Ok(CalcResult::new(Measurement::new(Decimal::from(100), Unit::Milliliters)))
    }
}

register_calculator!(BenchTrialsCalculator);