//! Temperature correction for specific gravity readings.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

/// Correct SG reading for temperature (calibrated at 20°C/68°F).
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

    fn description(&self) -> &'static str {
        "Correct specific gravity reading for temperature (calibrated at 20°C/68°F)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let sg_meas = input.get_measurement(Unit::SpecificGravity)?;
        let temp_meas = input.get_measurement(Unit::Celsius)
            .or_else(|_| input.get_measurement(Unit::Fahrenheit))?;

        let sg = sg_meas.value;
        let temp_c = if temp_meas.unit == Unit::Fahrenheit {
            (temp_meas.value - Decimal::from(32)) * Decimal::new(5, 1) / Decimal::from(9)
        } else {
            temp_meas.value
        };

        let cal_temp = Decimal::from(20);
        let correction_factor = Decimal::new(13, 5);
        let temp_diff = temp_c - cal_temp;
        let correction = correction_factor * temp_diff;

        let corrected_sg = sg + correction;

        let mut result = CalcResult::new(Measurement::sg(corrected_sg)?);

        if (temp_c - cal_temp).abs() > Decimal::from(10) {
            result = result.with_warning("Large temperature deviation from calibration (20°C/68°F)");
        }

        result = result
            .with_meta("measured_sg", sg.to_string())
            .with_meta("temperature", format!("{} {}", temp_c, Unit::Celsius))
            .with_meta("correction", correction.to_string())
            .with_meta("calibration", "20°C / 68°F");

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        input.get_measurement(Unit::SpecificGravity)?;
        if input.get_measurement(Unit::Celsius).is_err() && input.get_measurement(Unit::Fahrenheit).is_err() {
            return Err(mazerion_core::Error::MissingInput("Temperature required".into()));
        }
        Ok(())
    }
}

register_calculator!(SgCorrectionCalculator);