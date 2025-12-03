use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

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
        "Calculate ingredients for caramelized honey mead (bochet)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv").unwrap_or("14");
        let caramelization_level = input.get_param("caramelization").unwrap_or("medium");

        let vol: Decimal = volume.parse().map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse().map_err(|_| Error::Parse("Invalid target_abv".into()))?;

        // Account for sugar loss during caramelization
        let loss_factor = match caramelization_level {
            "light" => Decimal::new(105, 2),  // 5% loss
            "dark" => Decimal::new(115, 2),   // 15% loss
            _ => Decimal::new(110, 2),        // 10% loss (medium)
        };

        let honey_g_per_l_per_abv = Decimal::from(135);
        let honey_needed = vol * abv * honey_g_per_l_per_abv * loss_factor;

        let mut result = CalcResult::new(Measurement::new(honey_needed / Decimal::from(1000), Unit::Grams));

        result = result
            .with_meta("honey_to_caramelize", format!("{:.2} kg", honey_needed / Decimal::from(1000)))
            .with_meta("caramelization_level", caramelization_level)
            .with_meta("target_abv", format!("{}%", abv))
            .with_meta("tip", "Caramelize honey slowly, stop when desired color reached");

        Ok(result)
    }
}

register_calculator!(BochetCalculator);