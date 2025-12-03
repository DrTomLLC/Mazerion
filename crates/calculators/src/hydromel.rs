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
        "Calculate ingredients for session mead (hydromel)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv").unwrap_or("6");

        let vol: Decimal = volume.parse().map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse().map_err(|_| Error::Parse("Invalid target_abv".into()))?;

        if abv > Decimal::from(8) {
            return Err(Error::Validation("Hydromel typically 5-8% ABV".into()));
        }

        let honey_g_per_l_per_abv = Decimal::from(135);
        let honey_needed = vol * abv * honey_g_per_l_per_abv;

        let mut result = CalcResult::new(Measurement::new(honey_needed / Decimal::from(1000), Unit::Grams));

        result = result
            .with_meta("honey", format!("{:.2} kg", honey_needed / Decimal::from(1000)))
            .with_meta("water", format!("{:.2} L", vol - (honey_needed / Decimal::from(1420))))
            .with_meta("target_abv", format!("{}%", abv))
            .with_meta("style", "Session Mead (Hydromel)");

        Ok(result)
    }
}

register_calculator!(HydromelCalculator);