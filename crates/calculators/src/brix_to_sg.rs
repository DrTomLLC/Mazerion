// Convert degrees Brix to specific gravity using accurate Brew Your Own formula.

use mazerion_core::{
    CalcInput, CalcResult, Calculator, Measurement, Result, Unit, Validator, register_calculator,
};
use rust_decimal::Decimal;

/// Convert Brix to SG using accurate Brew Your Own / Brewer's Friend formula.
#[derive(Default)]
pub struct BrixToSgCalculator;

impl BrixToSgCalculator {
    pub const ID: &'static str = "brix_to_sg";
}

impl Calculator for BrixToSgCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Brix to SG"
    }

    fn category(&self) -> &'static str {
        "Basic"
    }

    fn description(&self) -> &'static str {
        "Convert degrees Brix to specific gravity (Brew Your Own formula)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let brix_meas = input.get_measurement(Unit::Brix)?;
        let brix = brix_meas.value;

        Validator::brix(brix)?;

        // CORRECT FORMULA (Brew Your Own / Brewer's Friend):
        // SG = (Brix / (258.6 - (Brix/258.2)×227.1)) + 1

        let denominator =
            Decimal::new(2586, 1) - ((brix / Decimal::new(2582, 1)) * Decimal::new(2271, 1));
        let sg = (brix / denominator) + Decimal::ONE;

        let mut result = CalcResult::new(Measurement::sg(sg)?);

        if let Some(warning) = Validator::brix_warning(brix) {
            result = result.with_warning(warning);
        }

        result = result
            .with_meta("brix", format!("{:.2}°Bx", brix))
            .with_meta("sg", format!("{:.4}", sg))
            .with_meta("formula", "Brew Your Own (accurate)")
            .with_meta(
                "calculation",
                format!("{}/(258.6 - ({}÷258.2)×227.1) + 1", brix, brix),
            );

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        input.get_measurement(Unit::Brix)?;
        Ok(())
    }
}

register_calculator!(BrixToSgCalculator);
