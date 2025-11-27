//! TOSNA nutrition calculator for mead fermentation.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;
use std::str::FromStr;

/// Calculate Fermaid-O additions using TOSNA 2.0 protocol.
#[derive(Default)]
pub struct NutritionCalculator;

impl NutritionCalculator {
    pub const ID: &'static str = "nutrition";
}

impl Calculator for NutritionCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "TOSNA Nutrition Calculator"
    }

    fn description(&self) -> &'static str {
        "Calculate Fermaid-O schedule using TOSNA 2.0 protocol"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input
            .get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;
        let yn_req = input.get_param("yn_requirement").unwrap_or("medium");

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid volume: {}", volume)))?;
        let abv: Decimal = target_abv
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid ABV: {}", target_abv)))?;

        // YAN requirement based on yeast strain
        let yan_factor = match yn_req {
            "low" => Decimal::new(20, 0),     // 20 ppm per % ABV
            "medium" => Decimal::new(25, 0),  // 25 ppm per % ABV
            "high" => Decimal::new(30, 0),    // 30 ppm per % ABV
            _ => Decimal::new(25, 0),
        };

        // Total YAN needed (ppm)
        let total_yan = abv * yan_factor;

        // Fermaid-O provides ~24 ppm YAN per g/L
        let fermaid_per_liter = total_yan / Decimal::new(24, 0);
        let total_fermaid = fermaid_per_liter * vol;

        let mut result = CalcResult::new(Measurement::new(total_fermaid, Unit::Grams));

        // TOSNA 2.0 schedule: split into 4 additions
        let per_addition = total_fermaid / Decimal::from(4);

        result = result
            .with_meta("schedule", "TOSNA 2.0 (4 additions)")
            .with_meta("pitch", format!("{:.1}g at pitch", per_addition))
            .with_meta("24h", format!("{:.1}g at 24h", per_addition))
            .with_meta("48h", format!("{:.1}g at 48h", per_addition))
            .with_meta("72h", format!("{:.1}g at 72h", per_addition))
            .with_meta("total_yan", format!("{} ppm", total_yan));

        Ok(result)
    }
}

register_calculator!(NutritionCalculator);