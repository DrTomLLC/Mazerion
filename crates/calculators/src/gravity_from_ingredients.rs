use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct GravityFromIngredientsCalculator;

impl GravityFromIngredientsCalculator {
    pub const ID: &'static str = "gravity_from_ingredients";
}

impl Calculator for GravityFromIngredientsCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Gravity from Ingredients" }
    fn description(&self) -> &'static str { "Calculate expected OG from ingredient amounts" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let honey_kg = input.get_param("honey_kg").unwrap_or("0")
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid honey_kg: {}", e)))?;
        let sugar_kg = input.get_param("sugar_kg").unwrap_or("0")
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid sugar_kg: {}", e)))?;
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;

        let honey_points = honey_kg * Decimal::from(35);
        let sugar_points = sugar_kg * Decimal::from(46);
        let total_points = (honey_points + sugar_points) / volume;
        let og = Decimal::ONE + (total_points / Decimal::from(1000));

        let mut result = CalcResult::new(Measurement::sg(og)?);
        result = result.with_meta("total_gravity_points", format!("{:.1}", total_points));
        Ok(result)
    }
}

register_calculator!(GravityFromIngredientsCalculator);
