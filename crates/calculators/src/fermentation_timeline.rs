use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct FermentationTimelineCalculator;

impl FermentationTimelineCalculator {
    pub const ID: &'static str = "fermentation_timeline";
}

impl Calculator for FermentationTimelineCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Fermentation Timeline" }
    fn description(&self) -> &'static str { "Estimate fermentation duration" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let og = input.get_param("og")
            .ok_or_else(|| Error::MissingInput("og required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid og: {}", e)))?;
        let temp = input.get_param("temperature")
            .ok_or_else(|| Error::MissingInput("temperature required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid temperature: {}", e)))?;
        let bev_type = input.get_param("beverage_type").unwrap_or("mead");

        let base_days = match bev_type {
            "ale" => Decimal::from(7),
            "lager" => Decimal::from(14),
            "wine" => Decimal::from(21),
            "cider" => Decimal::from(14),
            _ => Decimal::from(21),
        };

        let gravity_factor = (og - Decimal::ONE) * Decimal::from(10);
        let temp_factor = if temp < Decimal::from(18) { Decimal::new(12, 1) } else { Decimal::ONE };
        let days = base_days + gravity_factor * temp_factor;

        let result = CalcResult::new(Measurement::new(days, Unit::Percent));
        Ok(result)
    }
}

register_calculator!(FermentationTimelineCalculator);
