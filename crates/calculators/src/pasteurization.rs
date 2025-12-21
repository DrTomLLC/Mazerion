//! Pasteurization calculator - time/temperature for safe bottling

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct PasteurizationCalculator;

impl PasteurizationCalculator {
    pub const ID: &'static str = "pasteurization";
}

impl Calculator for PasteurizationCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Pasteurization"
    }

    fn category(&self) -> &'static str {
        "Finishing"
    }

    fn description(&self) -> &'static str {
        "Calculate time/temperature for pasteurization"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let temp_c = input.get_param("temperature")
            .ok_or_else(|| Error::MissingInput("temperature required (Celsius)".into()))?;
        let beverage_type = input.get_param("beverage_type").unwrap_or("mead");

        let temp: Decimal = temp_c.parse()
            .map_err(|_| Error::Parse("Invalid temperature".into()))?;

        // Pasteurization Unit (PU) calculation
        // 1 PU = 1 minute at 60°C (140°F)
        // Time required decreases exponentially with temperature
        let time_minutes = if temp >= Decimal::from(75) {
            Decimal::new(5, 1) // 0.5 minutes (30 seconds)
        } else if temp >= Decimal::from(71) {
            Decimal::ONE // 1 minute
        } else if temp >= Decimal::from(68) {
            Decimal::from(2) // 2 minutes
        } else if temp >= Decimal::from(65) {
            Decimal::from(5) // 5 minutes
        } else if temp >= Decimal::from(63) {
            Decimal::from(10) // 10 minutes
        } else if temp >= Decimal::from(60) {
            Decimal::from(20) // 20 minutes
        } else {
            return Err(Error::Validation(
                "Temperature too low for effective pasteurization (min 60°C / 140°F)".into()
            ));
        };

        let temp_f = temp * Decimal::new(18, 1) / Decimal::from(10) + Decimal::from(32);

        let method = if temp >= Decimal::from(71) {
            "Flash Pasteurization"
        } else if temp >= Decimal::from(63) {
            "HTST (High Temperature Short Time)"
        } else {
            "LTLT (Low Temperature Long Time)"
        };

        let safety_level = if beverage_type == "mead" || beverage_type == "wine" {
            "Wine/Mead: Targets spoilage organisms (Brettanomyces, Acetobacter)"
        } else if beverage_type == "beer" {
            "Beer: Targets spoilage yeast and bacteria"
        } else {
            "General: Targets common spoilage organisms"
        };

        let mut result = CalcResult::new(Measurement::new(time_minutes, Unit::Grams));

        result = result
            .with_meta("time_minutes", format!("{:.1} minutes", time_minutes))
            .with_meta("time_seconds", format!("{:.0} seconds", time_minutes * Decimal::from(60)))
            .with_meta("temperature_c", format!("{:.1}°C", temp))
            .with_meta("temperature_f", format!("{:.1}°F", temp_f))
            .with_meta("method", method)
            .with_meta("safety_level", safety_level)
            .with_meta("tip", "Heat bottles in water bath. Monitor temperature closely. Cool quickly after pasteurization.");

        result = result.with_warning("CRITICAL: Do not exceed temperature - risk of flavor damage and bottle explosion");
        result = result.with_warning("Use proper bottles rated for pasteurization (champagne bottles recommended)");

        if temp > Decimal::from(75) {
            result = result.with_warning("High temperature (>75°C) - risk of caramelization and off-flavors");
        }

        if time_minutes > Decimal::from(30) {
            result = result.with_warning("Long pasteurization time - consider higher temperature for shorter duration");
        }

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("temperature").is_none() {
            return Err(Error::MissingInput("temperature required".into()));
        }
        Ok(())
    }
}

register_calculator!(PasteurizationCalculator);