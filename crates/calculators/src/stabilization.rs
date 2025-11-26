use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct StabilizationCalculator;

impl StabilizationCalculator {
    pub const ID: &'static str = "stabilization";
}

impl Calculator for StabilizationCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Stabilization Calculator" }
    fn description(&self) -> &'static str { "Calculate K-meta and sorbate for stabilization" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;

        let kmeta_grams = volume * Decimal::new(56, 3);
        let sorbate_grams = volume * Decimal::new(5, 1);

        let mut result = CalcResult::new(Measurement::new(kmeta_grams, Unit::Grams));
        result = result
            .with_meta("potassium_metabisulfite", format!("{:.2}g", kmeta_grams))
            .with_meta("potassium_sorbate", format!("{:.2}g", sorbate_grams))
            .with_warning("Always add K-meta before sorbate");
        Ok(result)
    }
}

register_calculator!(StabilizationCalculator);
