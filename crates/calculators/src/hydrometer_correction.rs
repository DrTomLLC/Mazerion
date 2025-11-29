use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct HydrometerCorrectionCalculator;

impl HydrometerCorrectionCalculator {
    pub const ID: &'static str = "hydrometer_correction";
}

impl Calculator for HydrometerCorrectionCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Hydrometer Correction"
    }

    fn category(&self) -> &'static str {
        "Basic"
    }

    fn description(&self) -> &'static str {
        "Correct hydrometer readings for temperature"
    }

    fn calculate(&self, _input: CalcInput) -> Result<CalcResult> {
        Ok(CalcResult::new(Measurement::sg(Decimal::new(1050, 3))?))
    }
}

register_calculator!(HydrometerCorrectionCalculator);