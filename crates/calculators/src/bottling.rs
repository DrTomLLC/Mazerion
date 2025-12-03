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
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Bottling Calculator"
    }

    fn category(&self) -> &'static str {
        "Finishing"
    }

    fn description(&self) -> &'static str {
        "Calculate bottles needed for batch volume"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let bottle_size = input.get_param("bottle_size").unwrap_or("750");

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let size: Decimal = bottle_size
            .parse()
            .map_err(|_| Error::Parse("Invalid bottle size".into()))?;

        // Convert bottle size from mL to L
        let bottle_liters = size / Decimal::from(1000);

        // Calculate bottles needed
        let bottles = vol / bottle_liters;

        // Account for 5% loss (trub, racking, etc.)
        let loss_factor = Decimal::new(95, 2); // 0.95
        let effective_bottles = bottles * loss_factor;

        // Common bottle sizes and counts
        let bottle_type = match size.to_string().as_str() {
            "330" => "12 oz / 330 mL",
            "355" => "12 oz / 355 mL (US)",
            "375" => "Split / 375 mL",
            "500" => "500 mL",
            "750" => "750 mL (wine)",
            "1000" => "1 L",
            _ => "custom",
        };

        let mut result = CalcResult::new(Measurement::new(effective_bottles, Unit::Grams));

        result = result
            .with_meta("bottles_needed", format!("{:.0}", effective_bottles.ceil()))
            .with_meta("bottle_size", format!("{} mL", size))
            .with_meta("bottle_type", bottle_type)
            .with_meta("total_volume", format!("{} L", vol))
            .with_meta("loss_assumed", "5%");

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("volume").is_none() {
            return Err(Error::MissingInput("volume required".into()));
        }
        Ok(())
    }
}

register_calculator!(BottlingCalculator);