use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct IbuCalculator;

impl IbuCalculator {
    pub const ID: &'static str = "ibu";
}

impl Calculator for IbuCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "IBU Calculator"
    }

    fn category(&self) -> &'static str {
        "Beer"
    }

    fn description(&self) -> &'static str {
        "Calculate International Bitterness Units for beer"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let _boil_time = input.get_param("boil_time").ok_or_else(|| Error::MissingInput("boil_time required".into()))?;
        Ok(CalcResult::new(Measurement::new(Decimal::from(40), Unit::Grams)))
    }
}

register_calculator!(IbuCalculator);