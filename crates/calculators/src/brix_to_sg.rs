// Convert degrees Brix to specific gravity.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result, Unit,
    Validator,
};
use rust_decimal::Decimal;
use std::str::FromStr;

/// Convert Brix to SG using polynomial approximation.
#[derive(Default)]
pub struct BrixToSgCalculator;

impl BrixToSgCalculator {
    pub const ID: &'static str = "brix_to_sg";
}

impl Calculator for BrixToSgCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Brix to SG"
    }

    fn description(&self) -> &'static str {
        "Convert degrees Brix to specific gravity"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let brix_meas = input.get_measurement(Unit::Brix)?;
        let brix = brix_meas.value;

        Validator::brix(brix)?;

        let sg = Decimal::ONE + (brix * Decimal::new(4, 3)); // 0.004

        let mut result = CalcResult::new(Measurement::sg(sg)?);

        if let Some(warning) = Validator::brix_warning(brix) {
            result = result.with_warning(warning);
        }

        result = result
            .with_meta("brix", brix.to_string())
            .with_meta("formula", "SG ≈ 1.0 + (Brix × 0.004)");

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        input.get_measurement(Unit::Brix)?;
        Ok(())
    }
}

register_calculator!(BrixToSgCalculator);
