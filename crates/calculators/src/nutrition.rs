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
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "TOSNA Nutrition"
    }

    fn category(&self) -> &'static str {
        "Brewing"
    }

    fn description(&self) -> &'static str {
        "Calculate TOSNA yeast nutrition schedule"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume").ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv").ok_or_else(|| Error::MissingInput("target_abv required".into()))?;

        let vol: Decimal = volume.parse().map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse().map_err(|_| Error::Parse("Invalid target_abv".into()))?;

        let fermaid_o = vol * abv * Decimal::new(1, 1);

        Ok(CalcResult::new(Measurement::new(fermaid_o, Unit::Grams))
            .with_meta("volume", volume)
            .with_meta("target_abv", target_abv))
    }
}

register_calculator!(NutritionCalculator);