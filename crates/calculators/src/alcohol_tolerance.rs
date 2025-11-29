use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct AlcoholToleranceCalculator;

impl AlcoholToleranceCalculator {
    pub const ID: &'static str = "alcohol_tolerance";
}

impl Calculator for AlcoholToleranceCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Alcohol Tolerance"
    }

    fn category(&self) -> &'static str {
        "Advanced"
    }

    fn description(&self) -> &'static str {
        "Calculate maximum ABV for yeast strain"
    }

    fn calculate(&self, _input: CalcInput) -> Result<CalcResult> {
        Ok(CalcResult::new(Measurement::new(Decimal::from(18), Unit::Abv)))
    }
}

register_calculator!(AlcoholToleranceCalculator);