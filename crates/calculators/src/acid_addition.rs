//! Acid addition calculator for pH adjustment.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;
use std::str::FromStr;

/// Calculate acid additions to adjust pH.
#[derive(Default)]
pub struct AcidAdditionCalculator;

impl AcidAdditionCalculator {
    pub const ID: &'static str = "acid_addition";
}

impl Calculator for AcidAdditionCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Acid Addition Calculator"
    }

    fn description(&self) -> &'static str {
        "Calculate acid additions to adjust pH"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let ph_meas = input.get_measurement(Unit::Ph)?;
        let current_ph = ph_meas.value;

        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_ph = input
            .get_param("target_ph")
            .ok_or_else(|| Error::MissingInput("target_ph required".into()))?;
        let acid_type = input.get_param("acid_type").unwrap_or("tartaric");

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid volume: {}", volume)))?;
        let tgt_ph: Decimal = target_ph
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid target pH: {}", target_ph)))?;

        if tgt_ph >= current_ph {
            return Err(Error::Validation(
                "Target pH must be lower than current pH".into(),
            ));
        }

        let ph_drop = current_ph - tgt_ph;

        // Acid strength (g/L per 0.1 pH drop in typical must)
        let acid_factor = match acid_type {
            "tartaric" => Decimal::new(15, 1),  // 1.5 g/L per 0.1 pH
            "citric" => Decimal::new(17, 1),    // 1.7 g/L per 0.1 pH
            "malic" => Decimal::new(20, 1),     // 2.0 g/L per 0.1 pH
            "lactic" => Decimal::new(25, 1),    // 2.5 g/L per 0.1 pH
            _ => Decimal::new(15, 1),
        };

        let ph_units = ph_drop * Decimal::from(10); // Convert to 0.1 pH units
        let acid_needed = vol * ph_units * acid_factor / Decimal::from(10);

        let mut result = CalcResult::new(Measurement::new(acid_needed, Unit::Grams));
        result = result
            .with_meta("current_ph", format!("{}", current_ph))
            .with_meta("target_ph", format!("{}", tgt_ph))
            .with_meta("ph_drop", format!("{:.2}", ph_drop))
            .with_meta("acid_type", acid_type);

        result = result.with_warning("Add acid slowly and retest - this is an estimate");

        Ok(result)
    }
}

register_calculator!(AcidAdditionCalculator);