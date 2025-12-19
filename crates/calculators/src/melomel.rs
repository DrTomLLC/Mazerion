use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;
use std::str::FromStr;

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

        let vol: Decimal = Decimal::from_str(volume)
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = Decimal::from_str(target_abv)
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;
        let fruit_kg: Decimal = Decimal::from_str(fruit_weight)
            .map_err(|_| Error::Parse("Invalid fruit_weight".into()))?;

        // Fruit sugar as decimal (0.06 = 6%)
        let fruit_sugar_pct = match fruit_type {
            "strawberry" => Decimal::new(6, 2),
            "blueberry" => Decimal::new(10, 2),
            "raspberry" => Decimal::new(5, 2),
            "cherry" => Decimal::new(12, 2),
            "blackberry" => Decimal::new(9, 2),
            _ => Decimal::new(8, 2),
        };

        let fruit_g = fruit_kg * Decimal::from(1000);
        let fruit_sugar_g = fruit_g * fruit_sugar_pct;

        // FIXED: 33 g per L per % ABV
        let total_sugar_g = vol * abv * Decimal::from(33);
        let honey_sugar_g = if total_sugar_g > fruit_sugar_g {
            total_sugar_g - fruit_sugar_g
        } else {
            Decimal::ZERO
        };

        let fruit_abv = fruit_sugar_g / (vol * Decimal::from(33));

        let mut result = CalcResult::new(Measurement::new(honey_sugar_g, Unit::Grams));

        result = result
            .with_meta("fruit_type", fruit_type)
            .with_meta("fruit_weight_kg", format!("{:.2} kg", fruit_kg))
            .with_meta("fruit_sugar_g", format!("{:.0} g", fruit_sugar_g))
            .with_meta("fruit_abv", format!("{:.1}%", fruit_abv))
            .with_meta("honey_kg", format!("{:.2} kg", honey_sugar_g / Decimal::from(1000)));

        if fruit_kg / vol < Decimal::new(5, 1) {
            result = result.with_warning("Low fruit ratio - may have weak fruit character");
        }

        Ok(result)
    }
}

register_calculator!(MelomelCalculator);