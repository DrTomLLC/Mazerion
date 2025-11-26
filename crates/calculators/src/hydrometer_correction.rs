use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct HydrometerCorrectionCalculator;

impl HydrometerCorrectionCalculator {
    pub const ID: &'static str = "hydrometer_correction";
}

impl Calculator for HydrometerCorrectionCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Hydrometer Temperature Correction" }
    fn description(&self) -> &'static str { "Correct hydrometer reading for temperature" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let sg_meas = input.get_measurement(Unit::SpecificGravity)?;
        let temp_meas = input.get_measurement(Unit::Celsius)?;
        let sg = sg_meas.value;
        let temp = temp_meas.value;
        
        let cal_temp = input.get_param("calibration_temp")
            .unwrap_or("20")
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid calibration_temp: {}", e)))?;

        let correction = Decimal::new(13, 5) * (temp - cal_temp);
        let corrected_sg = sg + correction;

        let mut result = CalcResult::new(Measurement::sg(corrected_sg)?);
        result = result
            .with_meta("uncorrected_sg", sg.to_string())
            .with_meta("temperature", format!("{}°C", temp))
            .with_meta("calibration_temp", format!("{}°C", cal_temp))
            .with_meta("correction", format!("{:+.4}", correction));
        Ok(result)
    }
}

register_calculator!(HydrometerCorrectionCalculator);
