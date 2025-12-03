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

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let measured_sg = input
            .get_param("measured_sg")
            .ok_or_else(|| Error::MissingInput("measured_sg required".into()))?;
        let measured_temp = input
            .get_param("measured_temp")
            .ok_or_else(|| Error::MissingInput("measured_temp required".into()))?;
        let calibration_temp = input.get_param("calibration_temp").unwrap_or("20");

        let sg: Decimal = measured_sg
            .parse()
            .map_err(|_| Error::Parse("Invalid measured_sg".into()))?;
        let temp: Decimal = measured_temp
            .parse()
            .map_err(|_| Error::Parse("Invalid measured_temp".into()))?;
        let cal_temp: Decimal = calibration_temp
            .parse()
            .map_err(|_| Error::Parse("Invalid calibration_temp".into()))?;

        // Temperature correction formula (simplified)
        // Correction = 1.313454 × (T - Cal_T) / 1000
        let temp_diff = temp - cal_temp;
        let correction = Decimal::new(1313454, 6) * temp_diff / Decimal::from(1000);

        let corrected_sg = sg + correction;

        let mut result = CalcResult::new(Measurement::sg(corrected_sg)?);

        if (temp - cal_temp).abs() > Decimal::from(10) {
            result = result.with_warning("Large temperature difference - correction may be less accurate");
        }

        result = result
            .with_meta("corrected_sg", format!("{:.4}", corrected_sg))
            .with_meta("measured_sg", measured_sg)
            .with_meta("temperature", format!("{}°C", temp))
            .with_meta("calibration_temp", format!("{}°C", cal_temp))
            .with_meta("correction", format!("{:+.4}", correction));

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("measured_sg").is_none() {
            return Err(Error::MissingInput("measured_sg required".into()));
        }
        if input.get_param("measured_temp").is_none() {
            return Err(Error::MissingInput("measured_temp required".into()));
        }
        Ok(())
    }
}

register_calculator!(HydrometerCorrectionCalculator);