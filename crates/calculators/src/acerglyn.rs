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
        "Calculate ingredients for maple syrup mead (acerglyn)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;
        let maple_percent = input.get_param("maple_percent").unwrap_or("30");

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse()
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;
        let maple_pct: Decimal = maple_percent.parse()
            .map_err(|_| Error::Parse("Invalid maple_percent".into()))?;

        // Maple syrup: ~165 g per liter per % ABV (more sugar than honey)
        let maple_contribution_abv = abv * maple_pct / Decimal::from(100);
        let honey_contribution_abv = abv - maple_contribution_abv;

        let maple_needed = vol * maple_contribution_abv * Decimal::new(165, 0);
        let honey_needed = vol * honey_contribution_abv * Decimal::new(135, 0);

        let mut result = CalcResult::new(Measurement::new(honey_needed, Unit::Grams));

        result = result
            .with_meta("maple_syrup_g", format!("{:.0} g ({:.2} kg)", maple_needed, maple_needed / Decimal::from(1000)))
            .with_meta("honey_g", format!("{:.0} g ({:.2} kg)", honey_needed, honey_needed / Decimal::from(1000)))
            .with_meta("maple_abv", format!("{:.1}%", maple_contribution_abv))
            .with_meta("honey_abv", format!("{:.1}%", honey_contribution_abv));

        if maple_pct < Decimal::from(20) {
            result = result.with_warning("Low maple percentage - may have weak maple character");
        }

        Ok(result)
    }
}

register_calculator!(AcerglynCalculator);