use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct NutritionCalculator;

impl NutritionCalculator {
    pub const ID: &'static str = "nutrition";
}

impl Calculator for NutritionCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "TOSNA Nutrition Calculator" }
    fn description(&self) -> &'static str { "Calculate Fermaid-O schedule using TOSNA 2.0" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid target_abv: {}", e)))?;
        let yn_req = input.get_param("yn_requirement").unwrap_or("medium");

        let base_rate = match yn_req {
            "low" => Decimal::new(8, 1),
            "high" => Decimal::new(12, 1),
            _ => Decimal::from(10),
        };

        let abv_factor = target_abv / Decimal::from(13);
        let total_grams = volume * base_rate * abv_factor;

        let mut result = CalcResult::new(Measurement::new(total_grams, Unit::Grams));
        result = result
            .with_meta("schedule", "24h: 25%, 48h: 25%, 72h: 25%, 1/3 break: 25%")
            .with_meta("per_addition", format!("{:.1}g", total_grams / Decimal::from(4)));
        Ok(result)
    }
}

register_calculator!(NutritionCalculator);
