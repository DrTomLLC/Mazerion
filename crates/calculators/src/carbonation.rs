//! Carbonation calculator - priming sugar and keg PSI.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

/// Calculate carbonation requirements.
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
        "Calculate priming sugar or keg PSI for target carbonation"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let temp = input
            .get_param("temperature")
            .ok_or_else(|| Error::MissingInput("temperature required".into()))?;
        let target_co2 = input
            .get_param("target_co2")
            .ok_or_else(|| Error::MissingInput("target_co2 required".into()))?;
        let method = input.get_param("method").unwrap_or("priming");

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid volume: {}", volume)))?;
        let t: Decimal = temp
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid temperature: {}", temp)))?;
        let co2: Decimal = target_co2
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid CO2: {}", target_co2)))?;

        // CO2 already dissolved at temperature (Henry's Law approximation)
        let dissolved_co2 = Decimal::new(3044, 3) - (t * Decimal::new(19, 3)); // 3.044 - 0.019T

        // Additional CO2 needed
        let needed_co2 = co2 - dissolved_co2;

        if needed_co2 <= Decimal::ZERO {
            return Err(Error::Validation(
                "Already naturally carbonated at this temperature".into(),
            ));
        }

        if method == "priming" {
            let sugar_type = input.get_param("sugar_type").unwrap_or("table_sugar");

            // Sugar factors (g/L per volume CO2)
            let factor = match sugar_type {
                "corn_sugar" => Decimal::new(4, 0),   // 4.0 g/L
                "table_sugar" => Decimal::new(38, 1), // 3.8 g/L
                "honey" => Decimal::new(45, 1),       // 4.5 g/L
                "dme" => Decimal::new(46, 1),         // 4.6 g/L
                _ => Decimal::new(4, 0),
            };

            let sugar_needed = vol * needed_co2 * factor;

            let mut result = CalcResult::new(Measurement::new(sugar_needed, Unit::Grams));
            result = result
                .with_meta("method", "Bottle Priming")
                .with_meta("sugar_type", sugar_type)
                .with_meta("volume", format!("{} L", vol))
                .with_meta("target_co2", format!("{} vol", co2));

            Ok(result)
        } else {
            // Keg PSI calculation
            let psi = (needed_co2 - Decimal::new(5, 1)) * Decimal::new(2, 0) + Decimal::new(5, 0);

            let mut result = CalcResult::new(Measurement::new(psi, Unit::Percent)); // Using Percent as PSI placeholder
            result = result
                .with_meta("method", "Force Carbonation (Keg)")
                .with_meta("temperature", format!("{} °C", t))
                .with_meta("target_co2", format!("{} vol", co2));

            Ok(result)
        }
    }
}

register_calculator!(CarbonationCalculator);