use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct MelomelCalculator;

impl MelomelCalculator {
    pub const ID: &'static str = "melomel";
}

impl Calculator for MelomelCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Melomel Calculator"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for fruit mead with sugar contribution"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;
        let fruit_weight = input.get_param("fruit_weight")
            .ok_or_else(|| Error::MissingInput("fruit_weight required".into()))?;
        let fruit_type = input.get_param("fruit_type").unwrap_or("strawberry");

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse()
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;
        let fruit_kg: Decimal = fruit_weight.parse()
            .map_err(|_| Error::Parse("Invalid fruit_weight".into()))?;

        // Fruit sugar content percentages
        let fruit_sugar_pct = match fruit_type {
            "strawberry" => Decimal::new(8, 0),
            "blueberry" => Decimal::new(10, 0),
            "raspberry" => Decimal::new(5, 0),
            "cherry" => Decimal::new(12, 0),
            "blackberry" => Decimal::new(9, 0),
            _ => Decimal::new(8, 0), // Default
        };

        // Calculate sugar from fruit (kg fruit * % sugar * 1000 = grams sugar)
        let fruit_sugar_g = fruit_kg * fruit_sugar_pct * Decimal::from(10);

        // Sugar contributes ~0.6% ABV per 135g/L
        let fruit_abv_contribution = fruit_sugar_g / (vol * Decimal::new(135, 0));

        // Honey needed for remaining ABV
        let remaining_abv = abv - fruit_abv_contribution;
        let honey_needed = vol * remaining_abv * Decimal::new(135, 0);

        let mut result = CalcResult::new(Measurement::new(honey_needed, Unit::Grams));

        result = result
            .with_meta("fruit_type", fruit_type)
            .with_meta("fruit_weight_kg", format!("{:.2} kg", fruit_kg))
            .with_meta("fruit_sugar_g", format!("{:.0} g", fruit_sugar_g))
            .with_meta("fruit_abv", format!("{:.1}%", fruit_abv_contribution))
            .with_meta("honey_kg", format!("{:.2} kg", honey_needed / Decimal::from(1000)));

        if fruit_kg / vol < Decimal::new(1, 0) {
            result = result.with_warning("Low fruit ratio - may have weak fruit character");
        }

        Ok(result)
    }
}

register_calculator!(MelomelCalculator);