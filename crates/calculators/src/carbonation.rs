//! Carbonation calculator for priming sugar and keg PSI.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

/// Calculate priming sugar or keg PSI for target carbonation.
#[derive(Default)]
pub struct CarbonationCalculator;

impl CarbonationCalculator {
    pub const ID: &'static str = "carbonation";
}

impl Calculator for CarbonationCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Carbonation Calculator"
    }

    fn description(&self) -> &'static str {
        "Calculate priming sugar or keg PSI for carbonation"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;

        let temperature = input.get_param("temperature")
            .ok_or_else(|| Error::MissingInput("temperature required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid temperature".into()))?;

        let target_co2 = input.get_param("target_co2")
            .ok_or_else(|| Error::MissingInput("target_co2 required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid target_co2".into()))?;

        let method = input.get_param("method").unwrap_or("priming");

        let t_sq = temperature * temperature;
        let residual_co2 = Decimal::new(30378, 4)
            - (Decimal::new(50062, 6) * temperature)
            + (Decimal::new(26555, 8) * t_sq);

        let co2_needed = target_co2 - residual_co2;

        if method == "priming" {
            let sugar_type = input.get_param("sugar_type").unwrap_or("table_sugar");

            let sugar_factor = match sugar_type {
                "table_sugar" => Decimal::new(4, 0),
                "corn_sugar" => Decimal::new(37, 1),
                "honey" => Decimal::new(47, 1),
                "dme" => Decimal::new(52, 1),
                _ => Decimal::new(4, 0),
            };

            let sugar_needed = co2_needed * sugar_factor * volume;

            let mut result = CalcResult::new(Measurement::grams(sugar_needed)?);

            result = result
                .with_meta("method", "Bottle Priming")
                .with_meta("sugar_type", sugar_type)
                .with_meta("target_co2", format!("{:.2} volumes", target_co2))
                .with_meta("residual_co2", format!("{:.2} volumes", residual_co2));

            Ok(result)
        } else {
            let temp_f = (temperature * Decimal::new(9, 1) / Decimal::from(5)) + Decimal::from(32);
            let psi = ((target_co2 - Decimal::new(5, 1)) * (temp_f + Decimal::from(460))
                / Decimal::new(2992, 2)) - Decimal::new(147, 1);

            let mut result = CalcResult::new(Measurement::new(psi, Unit::Ppm));

            result = result
                .with_meta("method", "Force Carbonation (Keg)")
                .with_meta("target_co2", format!("{:.2} volumes", target_co2))
                .with_meta("temperature", format!("{:.1}°C / {:.1}°F", temperature, temp_f))
                .with_meta("note", "Allow 7-14 days for full carbonation");

            Ok(result)
        }
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        let required = ["volume", "temperature", "target_co2"];
        for param in &required {
            if input.get_param(param).is_none() {
                return Err(Error::MissingInput(format!("{} required", param)));
            }
        }
        Ok(())
    }
}

register_calculator!(CarbonationCalculator);