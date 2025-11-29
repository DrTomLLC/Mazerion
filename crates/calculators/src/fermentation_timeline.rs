use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct FermentationTimelineCalculator;

impl FermentationTimelineCalculator {
    pub const ID: &'static str = "fermentation_timeline";
}

impl Calculator for FermentationTimelineCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Fermentation Timeline"
    }

    fn category(&self) -> &'static str {
        "Brewing"
    }

    fn description(&self) -> &'static str {
        "Estimate fermentation duration based on parameters"
    }

    fn calculate(&self, _input: CalcInput) -> Result<CalcResult> {
        Ok(CalcResult::new(Measurement::new(Decimal::from(14), Unit::Grams)))
    }
}

register_calculator!(FermentationTimelineCalculator);