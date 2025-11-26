use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct CapsicumelCalculator;

impl CapsicumelCalculator {
    pub const ID: &'static str = "capsicumel";
}

impl Calculator for CapsicumelCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Capsicumel" }
    fn description(&self) -> &'static str { "Calculate pepper count for capsicumel" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let heat = input.get_param("heat_level").unwrap_or("medium");

        let peppers_per_liter = match heat {
            "mild" => Decimal::new(5, 1),
            "hot" => Decimal::from(2),
            _ => Decimal::ONE,
        };

        let count = volume * peppers_per_liter;
        let result = CalcResult::new(Measurement::new(count, Unit::Percent));
        Ok(result)
    }
}

register_calculator!(CapsicumelCalculator);
