use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct StabilizationCalculator;

impl StabilizationCalculator {
    pub const ID: &'static str = "stabilization";
}

impl Calculator for StabilizationCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Stabilization"
    }

    fn category(&self) -> &'static str {
        "Finishing"
    }

    fn description(&self) -> &'static str {
        "Calculate stabilization agents (K-meta + sorbate)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume").ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let vol: Decimal = volume.parse().map_err(|_| Error::Parse("Invalid volume".into()))?;
        let kmeta = vol * Decimal::new(5, 2);
        Ok(CalcResult::new(Measurement::new(kmeta, Unit::Grams)))
    }
}

register_calculator!(StabilizationCalculator);