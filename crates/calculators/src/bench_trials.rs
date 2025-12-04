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
        "Calculate bench trial additions and scaling to full batch"
    }

    fn category(&self) -> &'static str {
        "Advanced"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let trial_volume = input.get_param("trial_volume")
            .ok_or_else(|| Error::MissingInput("trial_volume required".into()))?;
        let trial_addition = input.get_param("trial_addition")
            .ok_or_else(|| Error::MissingInput("trial_addition required".into()))?;
        let batch_volume = input.get_param("batch_volume")
            .ok_or_else(|| Error::MissingInput("batch_volume required".into()))?;

        let trial_vol: Decimal = trial_volume.parse()
            .map_err(|_| Error::Parse("Invalid trial_volume".into()))?;
        let trial_add: Decimal = trial_addition.parse()
            .map_err(|_| Error::Parse("Invalid trial_addition".into()))?;
        let batch_vol: Decimal = batch_volume.parse()
            .map_err(|_| Error::Parse("Invalid batch_volume".into()))?;

        if trial_vol <= Decimal::ZERO || batch_vol <= Decimal::ZERO {
            return Err(Error::Validation("Volumes must be positive".into()));
        }

        // Calculate dosage rate
        let dosage_rate = trial_add / trial_vol;

        // Scale to full batch
        let batch_addition = dosage_rate * batch_vol;

        // Calculate scaling factor
        let scale_factor = batch_vol / trial_vol;

        let mut result = CalcResult::new(Measurement::new(batch_addition, Unit::Grams));

        result = result
            .with_meta("trial_volume_mL", format!("{} mL", trial_vol))
            .with_meta("trial_addition_g", format!("{:.2} g", trial_add))
            .with_meta("dosage_rate", format!("{:.3} g/mL", dosage_rate))
            .with_meta("batch_volume_L", format!("{} L", batch_vol))
            .with_meta("batch_addition_g", format!("{:.1} g ({:.2} kg)", batch_addition, batch_addition / Decimal::from(1000)))
            .with_meta("scale_factor", format!("{}x", scale_factor));

        if scale_factor > Decimal::from(100) {
            result = result.with_warning("Large scale factor - consider intermediate trials to verify dosage");
        }

        if trial_vol < Decimal::from(50) {
            result = result.with_warning("Very small trial volume - measurement errors will be amplified");
        }

        Ok(result)
    }
}

register_calculator!(BenchTrialsCalculator);