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
        "Calculate ingredients for pepper mead (capsicumel) with heat dosage"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;
        let heat_level = input.get_param("heat_level").unwrap_or("medium");

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse()
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;

        let honey_needed = vol * abv * Decimal::new(135, 0);

        // Pepper dosage per liter
        let pepper_per_liter = match heat_level {
            "mild" => Decimal::new(5, 1),      // 0.5 g/L
            "medium" => Decimal::new(10, 1),   // 1.0 g/L
            "hot" => Decimal::new(15, 1),      // 1.5 g/L
            _ => Decimal::new(10, 1),
        };

        let pepper_needed = vol * pepper_per_liter;

        let mut result = CalcResult::new(Measurement::new(honey_needed, Unit::Grams));

        result = result
            .with_meta("honey_kg", format!("{:.2} kg", honey_needed / Decimal::from(1000)))
            .with_meta("pepper_g", format!("{:.1} g", pepper_needed))
            .with_meta("heat_level", heat_level)
            .with_meta("dosage", format!("{:.1} g/L", pepper_per_liter));

        result = result.with_warning("Add peppers in secondary, taste frequently - heat develops over time");

        Ok(result)
    }
}

register_calculator!(CapsicumelCalculator);