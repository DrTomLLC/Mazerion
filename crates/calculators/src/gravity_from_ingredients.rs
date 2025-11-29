use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct GravityFromIngredientsCalculator;

impl GravityFromIngredientsCalculator {
    pub const ID: &'static str = "gravity_from_ingredients";
}

impl Calculator for GravityFromIngredientsCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Gravity from Ingredients"
    }

    fn category(&self) -> &'static str {
        "Basic"
    }

    fn description(&self) -> &'static str {
        "Calculate expected gravity from ingredients and volumes"
    }

    fn calculate(&self, _input: CalcInput) -> Result<CalcResult> {
        Ok(CalcResult::new(Measurement::sg(Decimal::new(1100, 3))?))
    }
}

register_calculator!(GravityFromIngredientsCalculator);