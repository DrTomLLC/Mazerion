use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct MetheglinCalculator;

impl MetheglinCalculator {
    pub const ID: &'static str = "metheglin";
}

impl Calculator for MetheglinCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Metheglin Calculator"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for spiced mead (metheglin) with spice dosage"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;
        let spice_level = input.get_param("spice_level").unwrap_or("medium");

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse()
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;

        let honey_needed = vol * abv * Decimal::new(135, 0);

        // Spice dosage per liter (varies by spice type)
        let spice_per_liter = match spice_level {
            "light" => Decimal::new(5, 1),      // 0.5 g/L
            "medium" => Decimal::new(10, 1),    // 1.0 g/L
            "heavy" => Decimal::new(20, 1),     // 2.0 g/L
            _ => Decimal::new(10, 1),
        };

        let spice_needed = vol * spice_per_liter;

        let mut result = CalcResult::new(Measurement::new(honey_needed, Unit::Grams));

        result = result
            .with_meta("honey_kg", format!("{:.2} kg", honey_needed / Decimal::from(1000)))
            .with_meta("spice_g", format!("{:.1} g", spice_needed))
            .with_meta("spice_level", spice_level)
            .with_meta("dosage", format!("{:.1} g/L", spice_per_liter));

        result = result.with_warning("Dosage varies by spice - start conservative, can always add more");

        Ok(result)
    }
}

register_calculator!(MetheglinCalculator);