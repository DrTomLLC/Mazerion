//! Refractometer correction using Terrill cubic equations.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

/// Correct refractometer readings for alcohol.
#[derive(Default)]
pub struct RefractometerCalculator;

impl RefractometerCalculator {
    pub const ID: &'static str = "refractometer";
}

impl Calculator for RefractometerCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Refractometer Correction"
    }

    fn description(&self) -> &'static str {
        "Correct refractometer readings for alcohol presence (Terrill cubic)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let orig_brix_meas = input.get_measurement(Unit::Brix)?;
        let orig_brix = orig_brix_meas.value;

        let current_brix = input
            .get_param("current_brix")
            .ok_or_else(|| Error::MissingInput("current_brix required".into()))?;

        let curr_brix: Decimal = current_brix
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid current_brix: {}", current_brix)))?;

        // Terrill cubic equation approximation
        // FG = 1.0000 - 0.00085683 × OB + 0.0034941 × CB
        let fg = Decimal::ONE
            - (Decimal::new(85683, 8) * orig_brix)
            + (Decimal::new(34941, 7) * curr_brix);

        let mut result = CalcResult::new(Measurement::sg(fg)?);
        result = result
            .with_meta("original_brix", format!("{} °Bx", orig_brix))
            .with_meta("current_brix", format!("{} °Bx", curr_brix))
            .with_meta("formula", "Terrill Cubic Equation");

        if curr_brix > orig_brix {
            result = result.with_warning("Current Brix higher than original - check readings");
        }

        Ok(result)
    }
}

register_calculator!(RefractometerCalculator);