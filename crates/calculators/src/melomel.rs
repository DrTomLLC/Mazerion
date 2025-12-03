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
        "Calculate ingredients for fruit mead (melomel)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv").unwrap_or("12");
        let fruit_ratio = input.get_param("fruit_ratio").unwrap_or("0.2");

        let vol: Decimal = volume.parse().map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse().map_err(|_| Error::Parse("Invalid target_abv".into()))?;
        let fruit_kg_per_l: Decimal = fruit_ratio.parse().map_err(|_| Error::Parse("Invalid fruit_ratio".into()))?;

        let fruit_kg = vol * fruit_kg_per_l;

        // Fruit contributes ~12% sugar on average
        let fruit_sugar_kg = fruit_kg * Decimal::new(12, 2);

        let honey_g_per_l_per_abv = Decimal::from(135);
        let total_sugar_needed = vol * abv * honey_g_per_l_per_abv / Decimal::from(1000);
        let honey_needed = (total_sugar_needed - fruit_sugar_kg).max(Decimal::ZERO) * Decimal::from(1000);

        let mut result = CalcResult::new(Measurement::new(honey_needed / Decimal::from(1000), Unit::Grams));

        result = result
            .with_meta("honey", format!("{:.2} kg", honey_needed / Decimal::from(1000)))
            .with_meta("fruit", format!("{:.2} kg", fruit_kg))
            .with_meta("fruit_ratio", format!("{} kg/L", fruit_kg_per_l))
            .with_meta("target_abv", format!("{}%", abv));

        Ok(result)
    }
}

register_calculator!(MelomelCalculator);