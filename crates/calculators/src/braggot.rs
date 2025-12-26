use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Default)]
pub struct BraggotCalculator;

impl BraggotCalculator {
    pub const ID: &'static str = "braggot";
}

impl Calculator for BraggotCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Braggot Calculator"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for honey-malt hybrid mead"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input
            .get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;
        let honey_percent = input
            .get_param("honey_percent")
            .ok_or_else(|| Error::MissingInput("honey_percent required".into()))?;
        let _malt_weight = input.get_param("malt_weight");

        let vol: Decimal =
            Decimal::from_str(volume).map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal =
            Decimal::from_str(target_abv).map_err(|_| Error::Parse("Invalid target_abv".into()))?;
        let honey_pct: Decimal = Decimal::from_str(honey_percent)
            .map_err(|_| Error::Parse("Invalid honey_percent".into()))?;

        // FIXED: 33 g per L per % ABV
        let total_sugar_g = vol * abv * Decimal::from(33);
        let honey_sugar_g = total_sugar_g * honey_pct / Decimal::from(100);

        let mut result = CalcResult::new(Measurement::new(honey_sugar_g, Unit::Grams));

        result = result
            .with_meta(
                "honey_kg",
                format!("{:.2} kg", honey_sugar_g / Decimal::from(1000)),
            )
            .with_meta("honey_g", format!("{:.0} g", honey_sugar_g));

        Ok(result)
    }
}

register_calculator!(BraggotCalculator);
