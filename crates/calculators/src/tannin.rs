use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
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
        "Calculate tannin additions for mouthfeel and structure"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_level = input.get_param("target_level").unwrap_or("medium");

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;

        // Typical tannin additions:
        // Light: 0.25 g/L (subtle structure)
        // Medium: 0.5 g/L (balanced)
        // Heavy: 1.0 g/L (bold, wine-like)

        let g_per_liter = match target_level {
            "light" => Decimal::new(25, 2),   // 0.25
            "heavy" => Decimal::ONE,           // 1.0
            _ => Decimal::new(5, 1),          // 0.5 medium
        };

        let tannin_needed = vol * g_per_liter;

        let mut result = CalcResult::new(Measurement::new(tannin_needed, Unit::Grams));

        if g_per_liter >= Decimal::ONE {
            result = result.with_warning("Heavy tannin - may be astringent if over-added");
        }

        result = result
            .with_meta("tannin_powder", format!("{:.2} g", tannin_needed))
            .with_meta("target_level", target_level)
            .with_meta("concentration", format!("{} g/L", g_per_liter))
            .with_meta("tip", "Add gradually and taste - you can always add more");

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("volume").is_none() {
            return Err(Error::MissingInput("volume required".into()));
        }
        Ok(())
    }
}

register_calculator!(TanninCalculator);