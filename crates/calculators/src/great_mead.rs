use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct GreatMeadCalculator;

impl GreatMeadCalculator {
    pub const ID: &'static str = "great_mead";
}

impl Calculator for GreatMeadCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Great Mead Calculator"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for traditional mead (honey, water, yeast)"
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

        if abv < Decimal::from(5) || abv > Decimal::from(20) {
            return Err(Error::OutOfRange("ABV should be 5-20% for traditional mead".into()));
        }

        // Traditional mead: ~135 g honey per liter per % ABV
        let honey_per_liter_per_abv = Decimal::new(135, 0);
        let honey_needed = vol * abv * honey_per_liter_per_abv;

        let mut result = CalcResult::new(Measurement::new(honey_needed, Unit::Grams));

        result = result
            .with_meta("volume", format!("{} L", vol))
            .with_meta("target_abv", format!("{}%", abv))
            .with_meta("honey_kg", format!("{:.2} kg", honey_needed / Decimal::from(1000)))
            .with_meta("honey_lbs", format!("{:.2} lbs", honey_needed / Decimal::new(45359, 2)))
            .with_meta("formula", "135 g honey/L/%ABV");

        if abv < Decimal::from(8) {
            result = result.with_warning("Low ABV for traditional mead - consider hydromel style");
        }
        if abv > Decimal::from(14) {
            result = result.with_warning("High ABV - ensure yeast tolerance and proper nutrients");
        }

        Ok(result)
    }
}

register_calculator!(GreatMeadCalculator);