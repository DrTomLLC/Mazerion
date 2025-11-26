use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct AcerglynCalculator;

impl AcerglynCalculator {
    pub const ID: &'static str = "acerglyn";
}

impl Calculator for AcerglynCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Acerglyn" }
    fn description(&self) -> &'static str { "Calculate honey and maple syrup for acerglyn" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let maple_pct = input.get_param("maple_percent").unwrap_or("30")
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid maple_percent: {}", e)))?;

        let maple_vol = volume * (maple_pct / Decimal::from(100));
        let honey_vol = volume - maple_vol;
        let honey_kg = honey_vol * Decimal::new(14, 1);
        let maple_kg = maple_vol * Decimal::new(13, 1);

        let mut result = CalcResult::new(Measurement::new(honey_kg, Unit::Grams));
        result = result.with_meta("maple_syrup_kg", format!("{:.2}", maple_kg));
        Ok(result)
    }
}

register_calculator!(AcerglynCalculator);
