//! Sulfite calculator with pH-dependent effectiveness.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;
use std::str::FromStr;

/// Calculate K-meta additions for target free SO2.
#[derive(Default)]
pub struct SulfiteCalculator;

impl SulfiteCalculator {
    pub const ID: &'static str = "sulfite";
}

impl Calculator for SulfiteCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Sulfite Calculator"
    }

    fn description(&self) -> &'static str {
        "Calculate K-meta additions with pH-dependent effectiveness"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let ph_meas = input.get_measurement(Unit::Ph)?;
        let ph = ph_meas.value;

        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_so2 = input
            .get_param("target_free_so2")
            .ok_or_else(|| Error::MissingInput("target_free_so2 required".into()))?;

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid volume: {}", volume)))?;
        let so2: Decimal = target_so2
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid SO2: {}", target_so2)))?;

        // pH effectiveness factor (molecular SO2 percentage)
        // Lower pH = more effective = less K-meta needed
        let effectiveness = if ph < Decimal::new(30, 1) {
            Decimal::new(6, 0) // 6% at pH < 3.0
        } else if ph < Decimal::new(35, 1) {
            Decimal::new(5, 0) // 5% at pH 3.0-3.5
        } else if ph < Decimal::new(40, 1) {
            Decimal::new(3, 0) // 3% at pH 3.5-4.0
        } else {
            Decimal::new(2, 0) // 2% at pH > 4.0
        };

        // K-meta provides ~57% SO2 by weight
        // Total SO2 needed = (target ppm × volume L) / 1000
        // K-meta needed = (total SO2) / (0.57 × effectiveness/100)
        let total_so2_mg = so2 * vol;
        let kmeta_mg = (total_so2_mg * Decimal::from(100))
            / (Decimal::new(57, 0) * effectiveness);
        let kmeta_g = kmeta_mg / Decimal::from(1000);

        let mut result = CalcResult::new(Measurement::new(kmeta_g, Unit::Grams));
        result = result
            .with_meta("ph", format!("{}", ph))
            .with_meta("effectiveness", format!("{}% molecular SO2", effectiveness))
            .with_meta("target_free_so2", format!("{} ppm", so2))
            .with_meta("volume", format!("{} L", vol));

        if ph > Decimal::new(38, 1) {
            result = result.with_warning("High pH reduces SO2 effectiveness - consider acidifying");
        }

        Ok(result)
    }
}

register_calculator!(SulfiteCalculator);