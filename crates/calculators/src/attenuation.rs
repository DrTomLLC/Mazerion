use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct AttenuationCalculator;

impl AttenuationCalculator {
    pub const ID: &'static str = "attenuation";
}

impl Calculator for AttenuationCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Attenuation Calculator"
    }

    fn category(&self) -> &'static str {
        "Advanced"
    }

    fn description(&self) -> &'static str {
        "Calculate fermentation attenuation percentage"
    }

    fn calculate(&self, _input: CalcInput) -> Result<CalcResult> {
        Ok(CalcResult::new(Measurement::new(Decimal::from(75), Unit::Percent)))
    }
}

register_calculator!(AttenuationCalculator);