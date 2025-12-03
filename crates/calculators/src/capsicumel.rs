use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

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
        "Calculate ingredients for pepper mead (capsicumel)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv").unwrap_or("12");
        let heat_level = input.get_param("heat_level").unwrap_or("medium");

        let vol: Decimal = volume.parse().map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse().map_err(|_| Error::Parse("Invalid target_abv".into()))?;

        let honey_g_per_l_per_abv = Decimal::from(135);
        let honey_needed = vol * abv * honey_g_per_l_per_abv;

        // Pepper recommendations (grams per liter)
        let pepper_amount = match heat_level {
            "mild" => vol * Decimal::new(5, 1),    // 0.5 g/L
            "hot" => vol * Decimal::new(15, 1),    // 1.5 g/L
            _ => vol * Decimal::ONE,                // 1.0 g/L (medium)
        };

        let mut result = CalcResult::new(Measurement::new(honey_needed / Decimal::from(1000), Unit::Grams));

        result = result
            .with_meta("honey", format!("{:.2} kg", honey_needed / Decimal::from(1000)))
            .with_meta("peppers", format!("{:.1} g", pepper_amount))
            .with_meta("heat_level", heat_level)
            .with_meta("tip", "Add peppers in secondary, taste test regularly");

        Ok(result)
    }
}

register_calculator!(CapsicumelCalculator);