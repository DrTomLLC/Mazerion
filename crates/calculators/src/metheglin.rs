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
        "Calculate ingredients for spiced mead (metheglin)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv").unwrap_or("12");
        let spice_intensity = input.get_param("spice_intensity").unwrap_or("medium");

        let vol: Decimal = volume.parse().map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse().map_err(|_| Error::Parse("Invalid target_abv".into()))?;

        let honey_g_per_l_per_abv = Decimal::from(135);
        let honey_needed = vol * abv * honey_g_per_l_per_abv;

        // Spice recommendations (grams per liter total)
        let spice_amount = match spice_intensity {
            "light" => vol * Decimal::new(5, 1),    // 0.5 g/L
            "heavy" => vol * Decimal::new(2, 0),    // 2.0 g/L
            _ => vol * Decimal::ONE,                 // 1.0 g/L (medium)
        };

        let mut result = CalcResult::new(Measurement::new(honey_needed / Decimal::from(1000), Unit::Grams));

        result = result
            .with_meta("honey", format!("{:.2} kg", honey_needed / Decimal::from(1000)))
            .with_meta("spices_total", format!("{:.1} g", spice_amount))
            .with_meta("spice_intensity", spice_intensity)
            .with_meta("tip", "Add spices in secondary, taste test regularly");

        Ok(result)
    }
}

register_calculator!(MetheglinCalculator);