use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct BraggotCalculator;

impl BraggotCalculator {
    pub const ID: &'static str = "braggot";
}

impl Calculator for BraggotCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Braggot Calculator"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for honey-malt hybrid (braggot)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv").unwrap_or("10");
        let honey_percentage = input.get_param("honey_percentage").unwrap_or("50");

        let vol: Decimal = volume.parse().map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse().map_err(|_| Error::Parse("Invalid target_abv".into()))?;
        let honey_pct: Decimal = honey_percentage.parse().map_err(|_| Error::Parse("Invalid honey_percentage".into()))?;

        let honey_ratio = honey_pct / Decimal::from(100);
        let malt_ratio = Decimal::ONE - honey_ratio;

        let honey_g_per_l_per_abv = Decimal::from(135);
        let honey_needed = vol * abv * honey_g_per_l_per_abv * honey_ratio;

        // Malt contribution: ~37 PPG (points per pound per gallon)
        // For simplicity, use similar calculation
        let malt_g_per_l_per_abv = Decimal::from(140);
        let malt_needed = vol * abv * malt_g_per_l_per_abv * malt_ratio;

        let mut result = CalcResult::new(Measurement::new(honey_needed / Decimal::from(1000), Unit::Grams));

        result = result
            .with_meta("honey", format!("{:.2} kg", honey_needed / Decimal::from(1000)))
            .with_meta("malt", format!("{:.2} kg", malt_needed / Decimal::from(1000)))
            .with_meta("honey_percentage", format!("{}%", honey_pct))
            .with_meta("target_abv", format!("{}%", abv));

        Ok(result)
    }
}

register_calculator!(BraggotCalculator);