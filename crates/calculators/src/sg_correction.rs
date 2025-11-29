// Temperature correction for specific gravity readings.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

/// Correct SG reading for temperature (calibrated at 20°C).
#[derive(Default)]
pub struct SgCorrectionCalculator;

impl SgCorrectionCalculator {
    pub const ID: &'static str = "sg_correction";
}

impl Calculator for SgCorrectionCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "SG Temperature Correction"
    }

    fn category(&self) -> &'static str {
        "Basic"
    }

    fn description(&self) -> &'static str {
        "Correct specific gravity reading for temperature (calibrated at 20°C)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let sg_meas = input.get_measurement(Unit::SpecificGravity)?;
        let temp_meas = input.get_measurement(Unit::Celsius)?;

        let sg = sg_meas.value;
        let temp = temp_meas.value;

        let cal_temp = Decimal::from(20);
        let correction_factor = Decimal::new(13, 5);
        let temp_diff = temp - cal_temp;
        let correction = correction_factor * temp_diff;

        let corrected_sg = sg + correction;

        let mut result = CalcResult::new(Measurement::sg(corrected_sg)?);

        if (temp - cal_temp).abs() > Decimal::from(10) {
            result = result.with_warning("Large temperature deviation from calibration (20°C)");
        }

        result = result
            .with_meta("measured_sg", sg.to_string())
            .with_meta("temperature", format!("{} °C", temp))
            .with_meta("correction", correction.to_string())
            .with_meta("calibration", "20°C");

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        input.get_measurement(Unit::SpecificGravity)?;
        input.get_measurement(Unit::Celsius)?;
        Ok(())
    }
}

register_calculator!(SgCorrectionCalculator);