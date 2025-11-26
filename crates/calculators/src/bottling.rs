use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct BottlingCalculator;

impl BottlingCalculator {
    pub const ID: &'static str = "bottling";
}

impl Calculator for BottlingCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Bottling" }
    fn description(&self) -> &'static str { "Calculate bottles needed and loss" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let bottle_size = input.get_param("bottle_size").unwrap_or("0.75")
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid bottle_size: {}", e)))?;
        let loss_percent = input.get_param("loss_percent").unwrap_or("5")
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid loss_percent: {}", e)))?;

        let usable = volume * (Decimal::ONE - (loss_percent / Decimal::from(100)));
        let bottles = (usable / bottle_size).ceil();

        let mut result = CalcResult::new(Measurement::new(bottles, Unit::Percent));
        result = result.with_meta("usable_volume", format!("{:.2}L", usable));
        Ok(result)
    }
}

register_calculator!(BottlingCalculator);
