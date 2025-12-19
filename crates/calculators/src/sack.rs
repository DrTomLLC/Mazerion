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
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Sack Mead Calculator"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for high-gravity dessert mead (14-18% ABV)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse()
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;

        // FIXED: 33 g honey per liter per % ABV
        let honey_needed = vol * abv * Decimal::from(33);

        let mut result = CalcResult::new(Measurement::new(honey_needed, Unit::Grams));

        result = result.with_meta("honey_kg", format!("{:.2} kg", honey_needed / Decimal::from(1000)));

        if abv < Decimal::from(14) {
            result = result.with_warning("Below typical sack mead range (14-18%)");
        }
        if abv > Decimal::from(18) {
            result = result.with_warning("Very high ABV - ensure yeast tolerance");
        }

        Ok(result)
    }
}

register_calculator!(SackCalculator);