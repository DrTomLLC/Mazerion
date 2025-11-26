use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct SulfiteCalculator;

impl SulfiteCalculator {
    pub const ID: &'static str = "sulfite";
}

impl Calculator for SulfiteCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Sulfite Calculator" }
    fn description(&self) -> &'static str { "Calculate K-meta with pH-dependent effectiveness" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let ph = input.get_measurement(Unit::Ph)?.value;
        let target_so2 = input.get_param("target_free_so2")
            .ok_or_else(|| Error::MissingInput("target_free_so2 required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid target_free_so2: {}", e)))?;

        let ph_factor = if ph < Decimal::from(3) {
            Decimal::ONE
        } else if ph > Decimal::from(4) {
            Decimal::new(15, 1)
        } else {
            Decimal::new(12, 1)
        };

        let kmeta_grams = (volume * target_so2 * ph_factor) / Decimal::from(570);

        let mut result = CalcResult::new(Measurement::new(kmeta_grams, Unit::Grams));
        result = result
            .with_meta("ph_effectiveness", format!("{:.1}x", ph_factor))
            .with_meta("molecular_so2_ppm", format!("{:.1}", target_so2 / ph_factor));
        Ok(result)
    }
}

register_calculator!(SulfiteCalculator);
