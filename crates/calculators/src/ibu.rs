use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct IbuCalculator;

impl IbuCalculator {
    pub const ID: &'static str = "ibu";
}

impl Calculator for IbuCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "IBU Calculator" }
    fn description(&self) -> &'static str { "Calculate International Bitterness Units" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let hop_grams = input.get_param("hop_grams")
            .ok_or_else(|| Error::MissingInput("hop_grams required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid hop_grams: {}", e)))?;
        let alpha_acid = input.get_param("alpha_acid")
            .ok_or_else(|| Error::MissingInput("alpha_acid required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid alpha_acid: {}", e)))?;
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let boil_time = input.get_param("boil_time")
            .ok_or_else(|| Error::MissingInput("boil_time required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid boil_time: {}", e)))?;

        let utilization = Decimal::new(30, 2);
        let ibu = (hop_grams * alpha_acid * utilization * Decimal::from(10)) / volume;

        let result = CalcResult::new(Measurement::new(ibu, Unit::Percent));
        Ok(result)
    }
}

register_calculator!(IbuCalculator);
