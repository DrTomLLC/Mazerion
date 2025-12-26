//! Recipe Upscaling Calculator
//! Scale recipes up or down while maintaining proportions

use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct UpscalingCalculator;

impl UpscalingCalculator {
    pub const ID: &'static str = "upscaling";
}

impl Calculator for UpscalingCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Recipe Upscaling"
    }

    fn description(&self) -> &'static str {
        "Scale recipes up or down - maintains perfect proportions"
    }

    fn category(&self) -> &'static str {
        "Utilities"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        // Required parameters
        let current_vol = input
            .get_param("current_volume")
            .ok_or_else(|| Error::MissingInput("current_volume required".into()))?;
        let target_vol = input
            .get_param("target_volume")
            .ok_or_else(|| Error::MissingInput("target_volume required".into()))?;

        let current: Decimal = current_vol
            .parse()
            .map_err(|_| Error::Parse("Invalid current_volume".into()))?;
        let target: Decimal = target_vol
            .parse()
            .map_err(|_| Error::Parse("Invalid target_volume".into()))?;

        if current <= Decimal::ZERO || target <= Decimal::ZERO {
            return Err(Error::Validation("Volumes must be positive".into()));
        }

        // Calculate scale factor
        let scale_factor = target / current;

        // Convert scale factor to percentage (5x = 500%)
        let scale_percentage = scale_factor * Decimal::from(100);

        // Optional ingredients to scale
        let mut scaled_ingredients = Vec::new();

        // Honey
        if let Some(honey) = input.get_param("honey")
            && let Ok(amount) = honey.parse::<Decimal>()
        {
            let scaled = amount * scale_factor;
            scaled_ingredients.push(("honey", amount, scaled));
        }

        // Water
        if let Some(water) = input.get_param("water")
            && let Ok(amount) = water.parse::<Decimal>()
        {
            let scaled = amount * scale_factor;
            scaled_ingredients.push(("water", amount, scaled));
        }

        // Fruit
        if let Some(fruit) = input.get_param("fruit")
            && let Ok(amount) = fruit.parse::<Decimal>()
        {
            let scaled = amount * scale_factor;
            scaled_ingredients.push(("fruit", amount, scaled));
        }

        // Nutrients
        if let Some(nutrients) = input.get_param("nutrients")
            && let Ok(amount) = nutrients.parse::<Decimal>()
        {
            let scaled = amount * scale_factor;
            scaled_ingredients.push(("nutrients", amount, scaled));
        }

        // Spices
        if let Some(spices) = input.get_param("spices")
            && let Ok(amount) = spices.parse::<Decimal>()
        {
            let scaled = amount * scale_factor;
            scaled_ingredients.push(("spices", amount, scaled));
        }

        // Yeast (optional - may not scale linearly)
        if let Some(yeast) = input.get_param("yeast")
            && let Ok(amount) = yeast.parse::<Decimal>()
        {
            let scaled = amount * scale_factor;
            scaled_ingredients.push(("yeast", amount, scaled));
        }

        // Build result - use scale percentage as main output
        let mut result = CalcResult::new(Measurement::new(scale_percentage, Unit::Percent));

        result = result
            .with_meta("original_volume", format!("{} L", current))
            .with_meta("target_volume", format!("{} L", target))
            .with_meta("scale_factor", format!("{:.2}x", scale_factor));

        // Add scaled ingredients to metadata
        for (name, original, scaled) in &scaled_ingredients {
            result = result
                .with_meta(format!("{}_original", name), format!("{:.2}", original))
                .with_meta(format!("{}_scaled", name), format!("{:.2}", scaled));
        }

        // Warnings
        if scale_factor > Decimal::from(10) {
            result = result.with_warning("Large scale factor - verify equipment capacity");
        }

        if scale_factor < Decimal::new(1, 1) {
            result = result.with_warning("Scaling down - small measurements may be difficult");
        }

        if scaled_ingredients.is_empty() {
            result = result.with_warning("No ingredients provided - showing scale factor only");
        }

        Ok(result)
    }
}

register_calculator!(UpscalingCalculator);
