use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct AcidAdditionCalculator;

impl AcidAdditionCalculator {
    pub const ID: &'static str = "acid_addition";
}

impl Calculator for AcidAdditionCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Acid Addition" }
    fn description(&self) -> &'static str { "Calculate acid needed to adjust pH" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let current_ph = input.get_measurement(Unit::Ph)?.value;
        let target_ph = input.get_param("target_ph")
            .ok_or_else(|| Error::MissingInput("target_ph required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid target_ph: {}", e)))?;
        let acid_type = input.get_param("acid_type").unwrap_or("tartaric");

        if target_ph >= current_ph {
            return Err(Error::Validation("Target pH must be lower than current pH".into()));
        }

        let ph_drop = current_ph - target_ph;
        let base_rate = match acid_type {
            "tartaric" => Decimal::new(15, 1),
            "citric" => Decimal::new(17, 1),
            "malic" => Decimal::new(20, 1),
            "lactic" => Decimal::new(25, 1),
            _ => Decimal::new(15, 1),
        };

        let grams = volume * ph_drop * base_rate;
        let mut result = CalcResult::new(Measurement::new(grams, Unit::Grams));
        result = result.with_warning("Add incrementally and test pH between additions");
        Ok(result)
    }
}

register_calculator!(AcidAdditionCalculator);
