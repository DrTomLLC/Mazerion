// Convert specific gravity to degrees Brix using cubic polynomial.

use mazerion_core::{
    CalcInput, CalcResult, Calculator, Measurement, Result, Unit, Validator, register_calculator,
};
use rust_decimal::Decimal;

/// Convert SG to Brix using accurate cubic polynomial.
#[derive(Default)]
pub struct SgToBrixCalculator;

impl SgToBrixCalculator {
    pub const ID: &'static str = "sg_to_brix";
}

impl Calculator for SgToBrixCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "SG to Brix"
    }

    fn category(&self) -> &'static str {
        "Basic"
    }

    fn description(&self) -> &'static str {
        "Convert specific gravity to degrees Brix (cubic polynomial)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let sg_meas = input.get_measurement(Unit::SpecificGravity)?;
        let sg = sg_meas.value;

        Validator::sg(sg)?;

        // CORRECT FORMULA (cubic polynomial):
        // Brix = −676.67 + 1286.4·SG − 800.47·SG² + 190.74·SG³

        // Convert to f64 for polynomial calculation
        let sg_f64 = sg.to_string().parse::<f64>().unwrap_or(1.0);

        let brix_f64 = -676.67 + (1286.4 * sg_f64) - (800.47 * sg_f64 * sg_f64)
            + (190.74 * sg_f64 * sg_f64 * sg_f64);

        let brix = Decimal::from_f64_retain(brix_f64).unwrap_or(Decimal::ZERO);

        let mut result = CalcResult::new(Measurement::brix(brix)?);

        if let Some(warning) = Validator::brix_warning(brix) {
            result = result.with_warning(warning);
        }

        result = result
            .with_meta("sg", format!("{:.4}", sg))
            .with_meta("brix", format!("{:.2}°Bx", brix))
            .with_meta("formula", "Cubic polynomial (accurate)")
            .with_meta(
                "calculation",
                "−676.67 + 1286.4·SG − 800.47·SG² + 190.74·SG³",
            );

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        input.get_measurement(Unit::SpecificGravity)?;
        Ok(())
    }
}

register_calculator!(SgToBrixCalculator);
