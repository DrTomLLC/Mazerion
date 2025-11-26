use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct GreatMeadCalculator;

impl GreatMeadCalculator {
    pub const ID: &'static str = "great_mead";
}

impl Calculator for GreatMeadCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Great Mead" }
    fn description(&self) -> &'static str { "Calculate honey for traditional mead" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let style = input.get_param("style").unwrap_or("standard");

        let ratio = match style {
            "dry" => Decimal::new(12, 1),
            "sweet" => Decimal::new(16, 1),
            _ => Decimal::new(14, 1),
        };

        let honey_kg = volume * ratio;
        let result = CalcResult::new(Measurement::new(honey_kg, Unit::Grams));
        Ok(result)
    }
}

register_calculator!(GreatMeadCalculator);
