use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct BenchTrialsCalculator;

impl BenchTrialsCalculator {
    pub const ID: &'static str = "bench_trials";
}

impl Calculator for BenchTrialsCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Bench Trials"
    }

    fn description(&self) -> &'static str {
        "Calculate bench trial additions and scaling"
    }

    fn category(&self) -> &'static str {
        "Advanced"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let trial_volume = input
            .get_param("trial_volume")
            .ok_or_else(|| Error::MissingInput("trial_volume required".into()))?;
        let trial_addition = input
            .get_param("trial_addition")
            .ok_or_else(|| Error::MissingInput("trial_addition required".into()))?;
        let batch_volume = input
            .get_param("batch_volume")
            .ok_or_else(|| Error::MissingInput("batch_volume required".into()))?;

        let trial_vol: Decimal = trial_volume
            .parse()
            .map_err(|_| Error::Parse("Invalid trial_volume".into()))?;
        let trial_add: Decimal = trial_addition
            .parse()
            .map_err(|_| Error::Parse("Invalid trial_addition".into()))?;
        let batch_vol: Decimal = batch_volume
            .parse()
            .map_err(|_| Error::Parse("Invalid batch_volume".into()))?;

        if trial_vol <= Decimal::ZERO || batch_vol <= Decimal::ZERO {
            return Err(Error::Validation("Volumes must be positive".into()));
        }

        // Calculate concentration in trial
        let trial_concentration = trial_add / trial_vol;

        // Scale to batch
        let batch_addition = trial_concentration * batch_vol;

        // Calculate scale factor
        let scale_factor = batch_vol / trial_vol;

        let mut result = CalcResult::new(Measurement::new(batch_addition, Unit::Grams));

        if scale_factor > Decimal::from(100) {
            result = result.with_warning("Large scale factor - consider multiple bench trials");
        }

        result = result
            .with_meta("batch_addition", format!("{:.2} g or mL", batch_addition))
            .with_meta("scale_factor", format!("{:.1}x", scale_factor))
            .with_meta("trial_concentration", format!("{:.3} g/mL per L", trial_concentration))
            .with_meta("tip", "Always double-check calculations before adding to full batch");

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("trial_volume").is_none() {
            return Err(Error::MissingInput("trial_volume required".into()));
        }
        if input.get_param("trial_addition").is_none() {
            return Err(Error::MissingInput("trial_addition required".into()));
        }
        if input.get_param("batch_volume").is_none() {
            return Err(Error::MissingInput("batch_volume required".into()));
        }
        Ok(())
    }
}

register_calculator!(BenchTrialsCalculator);