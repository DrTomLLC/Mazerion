use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct AcerglynCalculator;

impl AcerglynCalculator {
    pub const ID: &'static str = "acerglyn";
}

impl Calculator for AcerglynCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Acerglyn Calculator"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for maple mead (acerglyn)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv").unwrap_or("12");
        let maple_percentage = input.get_param("maple_percentage").unwrap_or("30");

        let vol: Decimal = volume.parse().map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse().map_err(|_| Error::Parse("Invalid target_abv".into()))?;
        let maple_pct: Decimal = maple_percentage.parse().map_err(|_| Error::Parse("Invalid maple_percentage".into()))?;

        let maple_ratio = maple_pct / Decimal::from(100);
        let honey_ratio = Decimal::ONE - maple_ratio;

        // Maple syrup: 67% sugar, Honey: 82% sugar
        let honey_g_per_l_per_abv = Decimal::from(135);
        let maple_g_per_l_per_abv = Decimal::from(165); // More maple needed due to lower sugar

        let honey_needed = vol * abv * honey_g_per_l_per_abv * honey_ratio;
        let maple_needed = vol * abv * maple_g_per_l_per_abv * maple_ratio;

        let mut result = CalcResult::new(Measurement::new(honey_needed / Decimal::from(1000), Unit::Grams));

        result = result
            .with_meta("honey", format!("{:.2} kg", honey_needed / Decimal::from(1000)))
            .with_meta("maple_syrup", format!("{:.2} kg", maple_needed / Decimal::from(1000)))
            .with_meta("target_abv", format!("{}%", abv))
            .with_meta("maple_percentage", format!("{}%", maple_pct));

        Ok(result)
    }
}

register_calculator!(AcerglynCalculator);