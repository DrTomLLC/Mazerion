use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct BraggotCalculator;

impl BraggotCalculator {
    pub const ID: &'static str = "braggot";
}

impl mazerion_core::Calculator for BraggotCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Braggot Calculator"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for honey-malt hybrid (braggot)"
    }

    fn calculate(&self, _input: CalcInput) -> Result<CalcResult> {
        Ok(CalcResult::new(Measurement::new(Decimal::from(19), Unit::Liters)))
    }
}

register_calculator!(BraggotCalculator);