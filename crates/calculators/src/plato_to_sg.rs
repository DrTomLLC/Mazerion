use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result, Unit, Validator,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct PlatoToSgCalculator;

impl PlatoToSgCalculator {
    pub const ID: &'static str = "plato_to_sg";
}

impl Calculator for PlatoToSgCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Plato to SG"
    }

    fn category(&self) -> &'static str {
        "Basic"
    }

    fn description(&self) -> &'static str {
        "Convert degrees Plato to specific gravity"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let plato_meas = input.get_measurement(Unit::Plato)?;
        let plato = plato_meas.value;
        Validator::plato(plato)?;
        let sg = Decimal::ONE + (plato * Decimal::new(4, 3));
        Ok(CalcResult::new(Measurement::sg(sg)?))
    }
}

register_calculator!(PlatoToSgCalculator);