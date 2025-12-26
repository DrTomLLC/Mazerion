use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Default)]
pub struct AcerglynCalculator;

impl AcerglynCalculator {
    pub const ID: &'static str = "acerglyn";
}

impl Calculator for AcerglynCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Acerglyn Calculator"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for maple mead"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input
            .get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;
        let maple_percent = input
            .get_param("maple_percent")
            .ok_or_else(|| Error::MissingInput("maple_percent required".into()))?;

        let vol: Decimal =
            Decimal::from_str(volume).map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal =
            Decimal::from_str(target_abv).map_err(|_| Error::Parse("Invalid target_abv".into()))?;
        let maple_pct: Decimal = Decimal::from_str(maple_percent)
            .map_err(|_| Error::Parse("Invalid maple_percent".into()))?;

        // FIXED: 33 g per L per % ABV
        let total_sugar_g = vol * abv * Decimal::from(33);
        let maple_sugar_g = total_sugar_g * maple_pct / Decimal::from(100);
        let honey_sugar_g = total_sugar_g - maple_sugar_g;

        let maple_syrup_g = maple_sugar_g / Decimal::new(672, 3); // Maple syrup ~67.2% sugar

        let mut result = CalcResult::new(Measurement::new(honey_sugar_g, Unit::Grams));

        result = result
            .with_meta("honey_g", format!("{:.0} g", honey_sugar_g))
            .with_meta("maple_syrup_g", format!("{:.0} g", maple_syrup_g));

        Ok(result)
    }
}

register_calculator!(AcerglynCalculator);
