use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct WaterChemistryCalculator;

impl WaterChemistryCalculator {
    pub const ID: &'static str = "water_chemistry";
}

impl Calculator for WaterChemistryCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Water Chemistry" }
    fn description(&self) -> &'static str { "Calculate gypsum for calcium adjustment" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let target_ca = input.get_param("target_calcium")
            .ok_or_else(|| Error::MissingInput("target_calcium required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid target_calcium: {}", e)))?;
        let current_ca = input.get_param("current_calcium").unwrap_or("0")
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid current_calcium: {}", e)))?;

        let delta = target_ca - current_ca;
        let gypsum = (delta * volume) / Decimal::from(61);

        let result = CalcResult::new(Measurement::new(gypsum, Unit::Grams));
        Ok(result)
    }
}

register_calculator!(WaterChemistryCalculator);
