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

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let og = input.get_param("og")
            .ok_or_else(|| Error::MissingInput("og required".into()))?;
        let fg = input.get_param("fg")
            .ok_or_else(|| Error::MissingInput("fg required".into()))?;
        let temperature = input.get_param("temperature").unwrap_or("20");
        let beverage_type = input.get_param("beverage_type").unwrap_or("ale");

        let og_val: Decimal = og.parse()
            .map_err(|_| Error::Parse("Invalid og".into()))?;
        let fg_val: Decimal = fg.parse()
            .map_err(|_| Error::Parse("Invalid fg".into()))?;
        let temp: Decimal = temperature.parse()
            .map_err(|_| Error::Parse("Invalid temperature".into()))?;

        // Calculate gravity points to ferment
        let gravity_points = (og_val - fg_val) * Decimal::from(1000);

        // Base fermentation time (days) by beverage type
        let base_days = match beverage_type {
            "ale" => Decimal::from(7),
            "lager" => Decimal::from(14),
            "mead" => Decimal::from(21),
            "wine" => Decimal::from(14),
            _ => Decimal::from(7),
        };

        // Temperature adjustment (slower at cooler temps)
        let temp_factor = if temp < Decimal::from(18) {
            Decimal::new(13, 1)  // 1.3x longer
        } else if temp > Decimal::from(25) {
            Decimal::new(9, 1)   // 0.9x faster
        } else {
            Decimal::ONE
        };

        // Gravity adjustment (more points = longer time)
        let gravity_factor = if gravity_points > Decimal::from(60) {
            Decimal::new(12, 1)  // 1.2x longer for high gravity
        } else if gravity_points < Decimal::from(30) {
            Decimal::new(8, 1)   // 0.8x faster for low gravity
        } else {
            Decimal::ONE
        };

        let estimated_days = base_days * temp_factor * gravity_factor;

        // Add conditioning time
        let conditioning_days = match beverage_type {
            "lager" => Decimal::from(21),
            "mead" => Decimal::from(90),
            "wine" => Decimal::from(30),
            _ => Decimal::from(7),
        };

        let total_days = estimated_days + conditioning_days;

        let mut result = CalcResult::new(Measurement::new(estimated_days, Unit::Grams));

        result = result
            .with_meta("primary_days", format!("{:.0} days", estimated_days))
            .with_meta("conditioning_days", format!("{:.0} days", conditioning_days))
            .with_meta("total_days", format!("{:.0} days ({:.1} weeks)", total_days, total_days / Decimal::from(7)))
            .with_meta("beverage_type", beverage_type)
            .with_meta("gravity_points", format!("{:.0} points", gravity_points))
            .with_meta("temperature", format!("{}°C", temp));

        result = result.with_warning("Timeline is an estimate - always verify completion with stable gravity readings");

        if total_days > Decimal::from(120) {
            result = result.with_warning("Long fermentation expected - patience required!");
        }

        Ok(result)
    }
}

register_calculator!(FermentationTimelineCalculator);