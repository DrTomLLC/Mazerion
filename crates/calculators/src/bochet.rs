use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Default)]
pub struct BochetCalculator;

impl BochetCalculator {
    pub const ID: &'static str = "bochet";
}

impl Calculator for BochetCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Bochet Calculator"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for caramelized honey mead with sugar loss"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;
        let bochet_level = input.get_param("bochet_level").unwrap_or("medium");

        let vol: Decimal = Decimal::from_str(volume)
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = Decimal::from_str(target_abv)
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;

        let loss_pct = match bochet_level {
            "light" => Decimal::new(5, 2),
            "medium" => Decimal::new(10, 2),
            "dark" => Decimal::new(15, 2),
            _ => Decimal::new(10, 2),
        };

        // FIXED: 33 g per L per % ABV
        let honey_needed_g = vol * abv * Decimal::from(33);
        let honey_before_caramel = honey_needed_g / (Decimal::ONE - loss_pct);

        let mut result = CalcResult::new(Measurement::new(honey_before_caramel, Unit::Grams));

        result = result
            .with_meta("caramel_level", bochet_level)
            .with_meta("sugar_loss", format!("{:.0}%", loss_pct * Decimal::from(100)));

        Ok(result)
    }
}

register_calculator!(BochetCalculator);