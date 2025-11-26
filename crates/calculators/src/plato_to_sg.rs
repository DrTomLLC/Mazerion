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
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Plato to SG" }
    fn description(&self) -> &'static str { "Convert degrees Plato to specific gravity" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let plato_meas = input.get_measurement(Unit::Plato)?;
        let plato = plato_meas.value;
        Validator::plato(plato)?;

        let sg = Decimal::ONE + (plato * Decimal::new(4, 3));
        let mut result = CalcResult::new(Measurement::sg(sg)?);

        if let Some(warning) = Validator::plato_warning(plato) {
            result = result.with_warning(warning);
        }

        result = result
            .with_meta("plato", plato.to_string())
            .with_meta("formula", "SG ≈ 1.0 + (Plato × 0.004)");
        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        input.get_measurement(Unit::Plato)?;
        Ok(())
    }
}

register_calculator!(PlatoToSgCalculator);
