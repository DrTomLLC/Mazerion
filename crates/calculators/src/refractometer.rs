//! Refractometer correction using Terrill cubic equation.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

/// Correct refractometer readings for alcohol presence.
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
        "Correct refractometer readings for alcohol (Terrill cubic)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let orig_brix_meas = input.get_measurement(Unit::Brix)?;
        let orig_brix = orig_brix_meas.value;

        let curr_brix = input.get_param("current_brix")
            .ok_or_else(|| Error::MissingInput("current_brix required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid current_brix".into()))?;

        let ob_squared = orig_brix * orig_brix;
        let cb_squared = curr_brix * curr_brix;

        let sg = Decimal::ONE
            - (Decimal::new(85683, 8) * orig_brix)
            + (Decimal::new(34941, 7) * curr_brix)
            + (Decimal::new(11687, 8) * ob_squared)
            + (Decimal::new(29, 6) * cb_squared);

        let mut result = CalcResult::new(Measurement::sg(sg)?);

        let apparent_attenuation = ((orig_brix - curr_brix) / orig_brix) * Decimal::from(100);

        if apparent_attenuation > Decimal::from(90) {
            result = result.with_warning("Very high attenuation - verify fermentation complete");
        } else if apparent_attenuation < Decimal::from(30) {
            result = result.with_warning("Low attenuation - fermentation may still be active");
        }

        result = result
            .with_meta("original_brix", format!("{:.2}°Bx", orig_brix))
            .with_meta("current_brix", format!("{:.2}°Bx", curr_brix))
            .with_meta("apparent_attenuation", format!("{:.1}%", apparent_attenuation))
            .with_meta("formula", "Terrill cubic equation");

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        input.get_measurement(Unit::Brix)?;
        if input.get_param("current_brix").is_none() {
            return Err(Error::MissingInput("current_brix required".into()));
        }
        Ok(())
    }
}

register_calculator!(RefractometerCalculator);