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
        "Calculate ingredients for caramelized honey mead (bochet) with sugar loss"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;
        let caramel_level = input.get_param("bochet_level").unwrap_or("medium");

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse()
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;

        // Sugar loss during caramelization
        let sugar_loss_pct = match caramel_level {
            "light" => Decimal::new(5, 0),
            "medium" => Decimal::new(10, 0),
            "dark" => Decimal::new(15, 0),
            _ => Decimal::new(10, 0),
        };

        let base_honey = vol * abv * Decimal::new(135, 0);
        let loss_factor = Decimal::ONE + (sugar_loss_pct / Decimal::from(100));
        let honey_needed = base_honey * loss_factor;

        let mut result = CalcResult::new(Measurement::new(honey_needed, Unit::Grams));

        result = result
            .with_meta("caramel_level", caramel_level)
            .with_meta("sugar_loss", format!("{}%", sugar_loss_pct))
            .with_meta("honey_before_caramel_kg", format!("{:.2} kg", honey_needed / Decimal::from(1000)))
            .with_meta("expected_abv", format!("{}%", abv));

        result = result.with_warning(
            "Caramelize honey BEFORE measuring - sugar loss accounted for in calculation"
        );

        Ok(result)
    }
}

register_calculator!(BochetCalculator);