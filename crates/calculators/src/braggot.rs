use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct BraggotCalculator;

impl BraggotCalculator {
    pub const ID: &'static str = "braggot";
}

impl Calculator for BraggotCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Braggot" }
    fn description(&self) -> &'static str { "Calculate honey and malt for braggot" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let target_og = input.get_param("target_og")
            .ok_or_else(|| Error::MissingInput("target_og required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid target_og: {}", e)))?;
        let honey_pct = input.get_param("honey_percent").unwrap_or("50")
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid honey_percent: {}", e)))?;

        let total_points = (target_og - Decimal::ONE) * Decimal::from(1000) * volume;
        let honey_points = total_points * (honey_pct / Decimal::from(100));
        let malt_points = total_points - honey_points;

        let honey_kg = honey_points / Decimal::from(35);
        let malt_kg = malt_points / Decimal::from(37);

        let mut result = CalcResult::new(Measurement::new(honey_kg, Unit::Grams));
        result = result.with_meta("malt_kg", format!("{:.2}", malt_kg));
        Ok(result)
    }
}

register_calculator!(BraggotCalculator);
