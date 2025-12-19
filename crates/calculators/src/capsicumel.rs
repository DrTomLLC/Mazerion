use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Default)]
pub struct CapsicumelCalculator;

impl CapsicumelCalculator {
    pub const ID: &'static str = "capsicumel";
}

impl Calculator for CapsicumelCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Capsicumel Calculator"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for pepper mead"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;

        let vol: Decimal = Decimal::from_str(volume)
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = Decimal::from_str(target_abv)
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;

        // FIXED: 33 g per L per % ABV
        let honey_needed = vol * abv * Decimal::from(33);

        let result = CalcResult::new(Measurement::new(honey_needed, Unit::Grams));

        Ok(result)
    }
}

register_calculator!(CapsicumelCalculator);