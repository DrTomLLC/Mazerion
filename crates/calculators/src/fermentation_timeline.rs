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
        let og = input
            .get_param("og")
            .ok_or_else(|| Error::MissingInput("og required".into()))?;
        let fg = input
            .get_param("fg")
            .ok_or_else(|| Error::MissingInput("fg required".into()))?;
        let temp = input.get_param("temperature").unwrap_or("20");
        let yeast_speed = input.get_param("yeast_speed").unwrap_or("medium");

        let og_val: Decimal = og.parse().map_err(|_| Error::Parse("Invalid OG".into()))?;
        let fg_val: Decimal = fg.parse().map_err(|_| Error::Parse("Invalid FG".into()))?;
        let temp_val: Decimal = temp
            .parse()
            .map_err(|_| Error::Parse("Invalid temperature".into()))?;

        // Calculate gravity points to ferment
        let points = (og_val - fg_val) * Decimal::from(1000);

        // Base timeline: 1 day per 10 gravity points
        let base_days = points / Decimal::from(10);

        // Temperature adjustment (optimal 18-22°C for ales)
        let temp_factor = if temp_val < Decimal::from(18) {
            Decimal::new(14, 1) // 1.4x slower
        } else if temp_val > Decimal::from(22) {
            Decimal::new(8, 1) // 0.8x faster
        } else {
            Decimal::ONE
        };

        // Yeast strain speed factor
        let yeast_factor = match yeast_speed {
            "fast" => Decimal::new(8, 1),     // 0.8x
            "slow" => Decimal::new(12, 1),    // 1.2x
            _ => Decimal::ONE,                 // medium
        };

        let estimated_days = base_days * temp_factor * yeast_factor;

        // Add conditioning time (typically 3-7 days)
        let conditioning_days = Decimal::from(5);
        let total_days = estimated_days + conditioning_days;

        let mut result = CalcResult::new(Measurement::new(estimated_days, Unit::Grams));

        if estimated_days > Decimal::from(21) {
            result = result.with_warning("Long fermentation - check for stuck fermentation");
        }

        if temp_val < Decimal::from(15) {
            result = result.with_warning("Temperature is low - fermentation may be very slow");
        }

        result = result
            .with_meta("primary_fermentation", format!("{:.0} days", estimated_days))
            .with_meta("conditioning", format!("{} days", conditioning_days))
            .with_meta("total_time", format!("{:.0} days", total_days))
            .with_meta("gravity_points", format!("{:.0}", points))
            .with_meta("temperature", format!("{}°C", temp_val));

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("og").is_none() {
            return Err(Error::MissingInput("og required".into()));
        }
        if input.get_param("fg").is_none() {
            return Err(Error::MissingInput("fg required".into()));
        }
        Ok(())
    }
}

register_calculator!(FermentationTimelineCalculator);