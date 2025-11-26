use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct TanninCalculator;

impl TanninCalculator {
    pub const ID: &'static str = "tannin";
}

impl Calculator for TanninCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Tannin Addition" }
    fn description(&self) -> &'static str { "Calculate tannin powder additions" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let intensity = input.get_param("intensity").unwrap_or("medium");

        let rate = match intensity {
            "light" => Decimal::new(5, 2),
            "strong" => Decimal::new(15, 2),
            _ => Decimal::new(10, 2),
        };

        let grams = volume * rate;
        let result = CalcResult::new(Measurement::new(grams, Unit::Grams));
        Ok(result)
    }
}

register_calculator!(TanninCalculator);
