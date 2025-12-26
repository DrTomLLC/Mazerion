use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, register_calculator,
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
        "Hydrometer Temperature Correction"
    }

    fn description(&self) -> &'static str {
        "Correct hydrometer readings for temperature (general polynomial formula)"
    }

    fn category(&self) -> &'static str {
        "Basic"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let measured_sg = input
            .get_param("measured_sg")
            .ok_or_else(|| Error::MissingInput("measured_sg required".into()))?;
        let sample_temp = input
            .get_param("sample_temp")
            .ok_or_else(|| Error::MissingInput("sample_temp required".into()))?;
        let calibration_temp = input.get_param("calibration_temp").unwrap_or("68");

        let mg: Decimal = measured_sg
            .parse()
            .map_err(|_| Error::Parse("Invalid measured_sg".into()))?;
        let t_r: Decimal = sample_temp
            .parse()
            .map_err(|_| Error::Parse("Invalid sample_temp".into()))?;
        let t_c: Decimal = calibration_temp
            .parse()
            .map_err(|_| Error::Parse("Invalid calibration_temp".into()))?;

        // CORRECT FORMULA (general polynomial for any calibration temperature):
        // cg = mg × [1.00130346 − 0.000134722124·T_r + 0.00000204052596·T_r² − 0.00000000232820948·T_r³] /
        //           [1.00130346 − 0.000134722124·T_c + 0.00000204052596·T_c² − 0.00000000232820948·T_c³]

        // Convert to f64 for polynomial calculation
        let tr_f64 = t_r.to_string().parse::<f64>().unwrap_or(68.0);
        let tc_f64 = t_c.to_string().parse::<f64>().unwrap_or(68.0);

        // Calculate numerator (for reading temperature)
        let numerator = 1.00130346 - (0.000134722124 * tr_f64)
            + (0.00000204052596 * tr_f64 * tr_f64)
            - (0.00000000232820948 * tr_f64 * tr_f64 * tr_f64);

        // Calculate denominator (for calibration temperature)
        let denominator = 1.00130346 - (0.000134722124 * tc_f64)
            + (0.00000204052596 * tc_f64 * tc_f64)
            - (0.00000000232820948 * tc_f64 * tc_f64 * tc_f64);

        // Apply correction
        let mg_f64 = mg.to_string().parse::<f64>().unwrap_or(1.0);
        let cg_f64 = mg_f64 * (numerator / denominator);
        let cg = Decimal::from_f64_retain(cg_f64).unwrap_or(mg);

        let correction = cg - mg;

        let mut result = CalcResult::new(Measurement::sg(cg)?);

        if (t_r - t_c).abs() > Decimal::from(15) {
            result = result.with_warning("Large temperature difference - ensure accurate reading");
        }

        result = result
            .with_meta("corrected_sg", format!("{:.4}", cg))
            .with_meta("measured_sg", measured_sg)
            .with_meta("sample_temp", format!("{}°F", t_r))
            .with_meta("calibration_temp", format!("{}°F", t_c))
            .with_meta("correction", format!("{:+.5}", correction))
            .with_meta(
                "formula",
                "General polynomial (accurate for any calibration temp)",
            );

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("measured_sg").is_none() {
            return Err(Error::MissingInput("measured_sg required".into()));
        }
        if input.get_param("sample_temp").is_none() {
            return Err(Error::MissingInput("sample_temp required".into()));
        }
        Ok(())
    }
}

register_calculator!(HydrometerCorrectionCalculator);
