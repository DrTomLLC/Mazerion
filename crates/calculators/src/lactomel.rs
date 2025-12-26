//! Lactomel calculator - milk/lactose mead

use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct LactomelCalculator;

impl LactomelCalculator {
    pub const ID: &'static str = "lactomel";
}

impl Calculator for LactomelCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Lactomel (Milk Mead)"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for lactomel (milk/lactose mead)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input
            .get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;
        let lactose_level = input.get_param("lactose_level").unwrap_or("medium");

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv
            .parse()
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;

        // Formula: 33 g honey per L per % ABV
        let g_per_l_per_abv = Decimal::from(33);

        // Lactose is non-fermentable, adds sweetness and body without raising ABV
        // Dosage: Light 50 g/L, Medium 100 g/L, Heavy 150 g/L
        let lactose_gpl = match lactose_level {
            "light" => Decimal::from(50),
            "medium" => Decimal::from(100),
            "heavy" => Decimal::from(150),
            _ => Decimal::from(100),
        };

        let lactose_g = vol * lactose_gpl;
        let honey_g = vol * abv * g_per_l_per_abv;

        let mut result = CalcResult::new(Measurement::new(honey_g, Unit::Grams));

        if abv < Decimal::from(8) {
            result = result.with_warning("Low ABV (<8%) - lactomel typically 10-14% for balance");
        }

        if abv > Decimal::from(16) {
            result = result.with_warning("High ABV (>16%) - may require strong yeast strain");
        }

        if lactose_gpl > Decimal::from(120) {
            result =
                result.with_warning("High lactose (>120 g/L) - may be overly sweet and creamy");
        }

        let honey_kg = honey_g / Decimal::from(1000);
        let lactose_kg = lactose_g / Decimal::from(1000);

        result = result
            .with_meta("honey_g", format!("{:.0} g", honey_g))
            .with_meta("honey_kg", format!("{:.2} kg", honey_kg))
            .with_meta("lactose_g", format!("{:.0} g", lactose_g))
            .with_meta("lactose_kg", format!("{:.2} kg", lactose_kg))
            .with_meta("lactose_level", lactose_level)
            .with_meta("lactose_gpl", format!("{} g/L", lactose_gpl))
            .with_meta("volume", format!("{:.2} L", vol))
            .with_meta("target_abv", format!("{:.1}%", abv))
            .with_meta("tip", "Lactose is non-fermentable. Add after fermentation or during boil. Creates creamy mouthfeel.");

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("volume").is_none() {
            return Err(Error::MissingInput("volume required".into()));
        }
        if input.get_param("target_abv").is_none() {
            return Err(Error::MissingInput("target_abv required".into()));
        }
        Ok(())
    }
}

register_calculator!(LactomelCalculator);
