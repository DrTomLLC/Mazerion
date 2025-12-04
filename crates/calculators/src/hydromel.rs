use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct HydromelCalculator;

impl HydromelCalculator {
    pub const ID: &'static str = "hydromel";
}

impl Calculator for HydromelCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Hydromel Calculator"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for session mead (hydromel, 3.5-7.5% ABV)"
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

        let honey_per_liter_per_abv = Decimal::new(135, 0);
        let honey_needed = vol * abv * honey_per_liter_per_abv;

        let mut result = CalcResult::new(Measurement::new(honey_needed, Unit::Grams));

        result = result
            .with_meta("volume", format!("{} L", vol))
            .with_meta("target_abv", format!("{}%", abv))
            .with_meta("style", "Hydromel (Session Mead)")
            .with_meta("honey_kg", format!("{:.2} kg", honey_needed / Decimal::from(1000)));

        if abv < Decimal::new(35, 1) {
            result = result.with_warning("ABV below typical hydromel range (3.5-7.5%)");
        }
        if abv > Decimal::new(75, 1) {
            result = result.with_warning("ABV above hydromel range - consider standard mead");
        }

        Ok(result)
    }
}

register_calculator!(HydromelCalculator);