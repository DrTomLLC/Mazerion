//! Priming sugar alternatives calculator

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct PrimingAlternativesCalculator;

impl PrimingAlternativesCalculator {
    pub const ID: &'static str = "priming_alternatives";
}

impl Calculator for PrimingAlternativesCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Priming Sugar Alternatives"
    }

    fn category(&self) -> &'static str {
        "Utilities"
    }

    fn description(&self) -> &'static str {
        "Calculate equivalent amounts for different priming sugars"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let sugar_type = input.get_param("sugar_type").unwrap_or("corn_sugar");
        let amount = input.get_param("amount").unwrap_or("100");

        let amt: Decimal = amount.parse()
            .map_err(|_| Error::Parse("Invalid amount".into()))?;

        if amt <= Decimal::ZERO {
            return Err(Error::Validation("Amount must be positive".into()));
        }

        // Conversion factors relative to corn sugar (dextrose = 1.0)
        let from_factor = match sugar_type {
            "table_sugar" => Decimal::new(91, 2),  // 0.91x (sucrose more fermentable)
            "dme" => Decimal::new(135, 2),          // 1.35x (less fermentable)
            "honey" => Decimal::new(125, 2),        // 1.25x
            _ => Decimal::ONE,                      // corn_sugar default
        };

        // Convert input to corn sugar equivalent
        let corn_sugar_equiv = amt / from_factor;

        // Calculate all alternatives
        let table_sugar = corn_sugar_equiv * Decimal::new(91, 2);
        let dme = corn_sugar_equiv * Decimal::new(135, 2);
        let honey = corn_sugar_equiv * Decimal::new(125, 2);

        let mut result = CalcResult::new(Measurement::new(corn_sugar_equiv, Unit::Grams))
            .with_meta("corn_sugar_g", format!("{:.1}", corn_sugar_equiv))
            .with_meta("table_sugar_g", format!("{:.1}", table_sugar))
            .with_meta("dme_g", format!("{:.1}", dme))
            .with_meta("honey_g", format!("{:.1}", honey))
            .with_meta("input_type", sugar_type)
            .with_meta("input_amount", amount);

        if corn_sugar_equiv > Decimal::from(200) {
            result = result.with_warning("High sugar amount - risk of overcarbonation");
        }

        Ok(result)
    }

    fn validate(&self, _input: &CalcInput) -> Result<()> {
        Ok(())
    }
}

register_calculator!(PrimingAlternativesCalculator);