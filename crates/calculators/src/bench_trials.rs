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
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Bench Trials" }
    fn description(&self) -> &'static str { "Scale test additions to batch size" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let sample_vol = input.get_param("sample_volume")
            .ok_or_else(|| Error::MissingInput("sample_volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid sample_volume: {}", e)))?;
        let sample_amt = input.get_param("sample_amount")
            .ok_or_else(|| Error::MissingInput("sample_amount required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid sample_amount: {}", e)))?;
        let batch_vol = input.get_param("batch_volume")
            .ok_or_else(|| Error::MissingInput("batch_volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid batch_volume: {}", e)))?;

        let scale = batch_vol / sample_vol;
        let batch_amt = sample_amt * scale;

        let result = CalcResult::new(Measurement::new(batch_amt, Unit::Grams));
        Ok(result)
    }
}

register_calculator!(BenchTrialsCalculator);
