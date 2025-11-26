use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct CyserCalculator;

impl CyserCalculator {
    pub const ID: &'static str = "cyser";
}

impl Calculator for CyserCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Cyser" }
    fn description(&self) -> &'static str { "Calculate honey and apple juice for cyser" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let ratio = input.get_param("ratio").unwrap_or("50")
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid ratio: {}", e)))?;

        let honey_vol = volume * (ratio / Decimal::from(100));
        let juice_vol = volume - honey_vol;
        let honey_kg = honey_vol * Decimal::new(14, 1);

        let mut result = CalcResult::new(Measurement::new(honey_kg, Unit::Grams));
        result = result.with_meta("apple_juice_liters", format!("{:.2}", juice_vol));
        Ok(result)
    }
}

register_calculator!(CyserCalculator);
