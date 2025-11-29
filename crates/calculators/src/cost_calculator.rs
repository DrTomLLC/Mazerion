use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct CostCalculator;

impl CostCalculator {
    pub const ID: &'static str = "cost_calculator";
}

impl Calculator for CostCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Cost Calculator"
    }

    fn category(&self) -> &'static str {
        "Utilities"
    }

    fn description(&self) -> &'static str {
        "Calculate batch cost breakdown and per-bottle pricing"
    }

    fn calculate(&self, _input: CalcInput) -> Result<CalcResult> {
        Ok(CalcResult::new(Measurement::new(Decimal::from(50), Unit::Grams)))
    }
}

register_calculator!(CostCalculator);