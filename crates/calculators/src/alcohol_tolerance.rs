use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct AlcoholToleranceCalculator;

impl AlcoholToleranceCalculator {
    pub const ID: &'static str = "alcohol_tolerance";
}

impl Calculator for AlcoholToleranceCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Alcohol Tolerance Check" }
    fn description(&self) -> &'static str { "Calculate if yeast can reach target ABV" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid target_abv: {}", e)))?;
        let yeast_tolerance = input.get_param("yeast_tolerance")
            .ok_or_else(|| Error::MissingInput("yeast_tolerance required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid yeast_tolerance: {}", e)))?;

        let headroom = yeast_tolerance - target_abv;
        let safe = headroom >= Decimal::from(2);

        let mut result = CalcResult::new(Measurement::new(headroom, Unit::Abv));
        if !safe {
            result = result.with_warning("Less than 2% headroom - yeast may struggle");
        }
        result = result.with_meta("recommendation", if safe { "Safe" } else { "Risky" });
        Ok(result)
    }
}

register_calculator!(AlcoholToleranceCalculator);
