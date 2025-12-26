use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct TanninCalculator;

impl TanninCalculator {
    pub const ID: &'static str = "tannin";
}

impl Calculator for TanninCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Tannin Calculator"
    }

    fn category(&self) -> &'static str {
        "Finishing"
    }

    fn description(&self) -> &'static str {
        "Calculate tannin additions for body and mouthfeel"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let tannin_level = input.get_param("tannin_level").unwrap_or("medium");
        let tannin_type = input.get_param("tannin_type").unwrap_or("wine_tannin");

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;

        // Tannin dosage per liter
        let dosage_per_liter = match tannin_level {
            "low" => Decimal::new(5, 2),     // 0.05 g/L
            "medium" => Decimal::new(10, 2), // 0.10 g/L
            "high" => Decimal::new(15, 2),   // 0.15 g/L
            _ => Decimal::new(10, 2),
        };

        let tannin_needed = vol * dosage_per_liter;

        let tannin_description = match tannin_type {
            "wine_tannin" => "Wine Tannin (grape-derived, general purpose)",
            "ft_blanc" => "FT Blanc (oak, for white wines/meads)",
            "tannin_riche" => "Tannin Riche (adds body without astringency)",
            "tannin_complex" => "Tannin Complex (mouthfeel enhancement)",
            _ => "Wine Tannin (grape-derived)",
        };

        let mut result = CalcResult::new(Measurement::new(tannin_needed, Unit::Grams));

        result = result
            .with_meta("tannin_g", format!("{:.2} g", tannin_needed))
            .with_meta(
                "tannin_tsp",
                format!("{:.3} tsp", tannin_needed / Decimal::new(5, 0)),
            )
            .with_meta("tannin_type", tannin_description)
            .with_meta("tannin_level", tannin_level)
            .with_meta("dosage", format!("{:.2} g/L", dosage_per_liter));

        result = result.with_warning("Add gradually, taste after 24 hours - easy to over-tannin");
        result =
            result.with_warning("Tannin adds astringency/dryness - use sparingly in sweet meads");

        Ok(result)
    }
}

register_calculator!(TanninCalculator);
