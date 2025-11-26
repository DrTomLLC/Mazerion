use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct DilutionCalculator;

impl DilutionCalculator {
    pub const ID: &'static str = "dilution";
}

impl Calculator for DilutionCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Dilution Calculator" }
    fn description(&self) -> &'static str { "Calculate water needed to reduce ABV" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let current_vol = input.get_param("current_volume")
            .ok_or_else(|| Error::MissingInput("current_volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid current_volume: {}", e)))?;
        let current_abv = input.get_param("current_abv")
            .ok_or_else(|| Error::MissingInput("current_abv required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid current_abv: {}", e)))?;
        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid target_abv: {}", e)))?;

        if target_abv >= current_abv {
            return Err(Error::Validation("Target ABV must be less than current ABV".into()));
        }

        let water_needed = current_vol * ((current_abv / target_abv) - Decimal::ONE);
        let final_volume = current_vol + water_needed;

        let mut result = CalcResult::new(Measurement::new(water_needed, Unit::Liters));
        result = result
            .with_meta("final_volume", format!("{:.2}L", final_volume))
            .with_meta("dilution_ratio", format!("1:{:.2}", water_needed / current_vol));
        Ok(result)
    }
}

register_calculator!(DilutionCalculator);
