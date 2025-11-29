use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

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

    fn category(&self) -> &'static str {
        "Advanced"
    }

    fn description(&self) -> &'static str {
        "Correct refractometer readings for alcohol presence (Terrill cubic)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let orig_brix_meas = input.get_measurement(Unit::Brix)?;
        let current_brix = input.get_param("current_brix").ok_or_else(|| Error::MissingInput("current_brix required".into()))?;

        let ob = orig_brix_meas.value;
        let cb: Decimal = current_brix.parse().map_err(|_| Error::Parse("Invalid current_brix".into()))?;

        let fg = Decimal::ONE + (ob * Decimal::new(4, 3)) - (cb * Decimal::new(4, 3));

        Ok(CalcResult::new(Measurement::sg(fg)?)
            .with_meta("original_brix", ob.to_string())
            .with_meta("current_brix", current_brix))
    }
}

register_calculator!(RefractometerCalculator);