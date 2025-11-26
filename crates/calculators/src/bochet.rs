use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct BochetCalculator;

impl BochetCalculator {
    pub const ID: &'static str = "bochet";
}

impl Calculator for BochetCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Bochet" }
    fn description(&self) -> &'static str { "Calculate caramelization time for bochet" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let honey_kg = input.get_param("honey_kg")
            .ok_or_else(|| Error::MissingInput("honey_kg required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid honey_kg: {}", e)))?;
        let darkness = input.get_param("darkness").unwrap_or("medium");

        let minutes = match darkness {
            "light" => honey_kg * Decimal::from(15),
            "dark" => honey_kg * Decimal::from(45),
            _ => honey_kg * Decimal::from(30),
        };

        let result = CalcResult::new(Measurement::new(minutes, Unit::Percent));
        Ok(result)
    }
}

register_calculator!(BochetCalculator);
