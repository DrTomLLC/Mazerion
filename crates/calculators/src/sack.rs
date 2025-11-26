use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct SackCalculator;

impl SackCalculator {
    pub const ID: &'static str = "sack";
}

impl Calculator for SackCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Sack" }
    fn description(&self) -> &'static str { "Calculate honey for sack mead (14%+ ABV)" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid target_abv: {}", e)))?;

        let honey_kg = (target_abv / Decimal::from(13)) * volume * Decimal::new(14, 1);
        let result = CalcResult::new(Measurement::new(honey_kg, Unit::Grams));
        Ok(result)
    }
}

register_calculator!(SackCalculator);
