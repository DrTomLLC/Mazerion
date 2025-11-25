//! TOSNA 2.0 nutrition calculator.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

/// Calculate Fermaid-O schedule using TOSNA 2.0 protocol.
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
        "TOSNA Nutrition"
    }

    fn description(&self) -> &'static str {
        "Calculate Fermaid-O schedule using TOSNA 2.0 protocol"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;

        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;

        let yn_req = input.get_param("yn_requirement").unwrap_or("medium");

        if volume <= Decimal::ZERO {
            return Err(Error::OutOfRange("Volume must be positive".into()));
        }

        let base_yan = target_abv * Decimal::from(18);

        let yan_per_liter = match yn_req {
            "low" => base_yan * Decimal::new(8, 1),
            "high" => base_yan * Decimal::new(12, 1),
            _ => base_yan,
        };

        let total_yan_needed = yan_per_liter * volume;
        let fermaid_o_grams = total_yan_needed / Decimal::from(10);

        let per_addition = fermaid_o_grams / Decimal::from(4);

        let mut result = CalcResult::new(Measurement::grams(fermaid_o_grams)?);

        if fermaid_o_grams > Decimal::from(100) {
            result = result.with_warning("Large nutrient addition - consider more doses");
        }

        result = result
            .with_meta("volume", format!("{} L", volume))
            .with_meta("target_abv", format!("{}%", target_abv))
            .with_meta("yn_requirement", yn_req)
            .with_meta("total_yan", format!("{:.1} mg/L", yan_per_liter))
            .with_meta("schedule", "TOSNA 2.0")
            .with_meta("addition_1", format!("{:.2} g at 24h", per_addition))
            .with_meta("addition_2", format!("{:.2} g at 48h", per_addition))
            .with_meta("addition_3", format!("{:.2} g at 72h", per_addition))
            .with_meta("addition_4", format!("{:.2} g at 1/3 sugar break", per_addition));

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("volume").is_none() {
            return Err(Error::MissingInput("volume required".into()));
        }
        if input.get_param("target_abv").is_none() {
            return Err(Error::MissingInput("target_abv required".into()));
        }
        Ok(())
    }
}

register_calculator!(NutritionCalculator);