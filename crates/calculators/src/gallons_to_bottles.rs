//! Gallons to Bottles calculator - Simple version without losses

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Default)]
pub struct GallonsToBottles;

impl GallonsToBottles {
    pub const ID: &'static str = "gallons_to_bottles";
}

impl Calculator for GallonsToBottles {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Gallons to Bottles"
    }

    fn description(&self) -> &'static str {
        "Calculate bottle count from volume"
    }

    fn category(&self) -> &'static str {
        "Utilities"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume_str = input.params.iter()
            .find(|(k, _)| k == "volume")
            .map(|(_, v)| v.as_str())
            .ok_or(Error::MissingInput("volume".into()))?;

        let volume_l = Decimal::from_str(volume_str)
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;

        if volume_l <= Decimal::ZERO {
            return Err(Error::Validation("Volume must be positive".into()));
        }

        let vol_ml = volume_l * Decimal::from(1000);

        // Bottle sizes in mL
        let bottle_12oz = Decimal::new(35488, 2);
        let bottle_375ml = Decimal::from(375);
        let bottle_500ml = Decimal::from(500);
        let bottle_750ml = Decimal::from(750);
        let bottle_1l = Decimal::from(1000);
        let bottle_1_5l = Decimal::from(1500);
        let bottle_3l = Decimal::from(3000);
        let bottle_5l = Decimal::from(5000);
        let bottle_6l = Decimal::from(6000);

        // Calculate bottle counts
        let bottles_12oz = (vol_ml / bottle_12oz).round_dp(0);
        let bottles_375ml = (vol_ml / bottle_375ml).round_dp(0);
        let bottles_500ml = (vol_ml / bottle_500ml).round_dp(0);
        let bottles_750ml = (vol_ml / bottle_750ml).round_dp(0);
        let bottles_1l = (vol_ml / bottle_1l).round_dp(0);
        let bottles_1_5l = (vol_ml / bottle_1_5l).round_dp(0);
        let bottles_3l = (vol_ml / bottle_3l).round_dp(0);
        let bottles_5l = (vol_ml / bottle_5l).round_dp(0);
        let bottles_6l = (vol_ml / bottle_6l).round_dp(0);

        // Calculate cases
        let cases_12oz = (bottles_12oz / Decimal::from(24)).ceil();
        let cases_375ml = (bottles_375ml / Decimal::from(12)).ceil();
        let cases_750ml = (bottles_750ml / Decimal::from(12)).ceil();
        let cases_1l = (bottles_1l / Decimal::from(12)).ceil();

        // Primary output: volume in liters
        let output = Measurement::new(volume_l, Unit::Liters);

        let result = CalcResult::new(output)
            .with_meta("bottles_12oz", format!("{} bottles (12 oz / 355 mL)", bottles_12oz))
            .with_meta("bottles_375ml", format!("{} bottles (375 mL / half-bottle)", bottles_375ml))
            .with_meta("bottles_500ml", format!("{} bottles (500 mL)", bottles_500ml))
            .with_meta("bottles_750ml", format!("{} bottles (750 mL / standard wine)", bottles_750ml))
            .with_meta("bottles_1L", format!("{} bottles (1 L / magnum)", bottles_1l))
            .with_meta("bottles_1.5L", format!("{} bottles (1.5 L)", bottles_1_5l))
            .with_meta("bottles_3L", format!("{} bottles (3 L / double magnum)", bottles_3l))
            .with_meta("bottles_5L", format!("{} bottles (5 L / jeroboam)", bottles_5l))
            .with_meta("bottles_6L", format!("{} bottles (6 L / imperial)", bottles_6l))
            .with_meta("cases_12oz", format!("{} cases (24 × 12oz)", cases_12oz))
            .with_meta("cases_375ml", format!("{} cases (12 × 375mL)", cases_375ml))
            .with_meta("cases_750ml", format!("{} cases (12 × 750mL)", cases_750ml))
            .with_meta("cases_1L", format!("{} cases (12 × 1L)", cases_1l))
            .with_meta("volume_gallons", format!("{:.2} gal", volume_l * Decimal::new(264172, 6)))
            .with_meta("volume_liters", format!("{:.2} L", volume_l))
            .with_meta("volume_quarts", format!("{:.2} qt", volume_l * Decimal::new(105669, 5)))
            .with_meta("volume_fluid_ounces", format!("{:.1} fl oz", volume_l * Decimal::new(338140, 4)));

        Ok(result)
    }
}

register_calculator!(GallonsToBottles);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5_gallons() {
        let calc = GallonsToBottles;
        let input = CalcInput::new()
            .add_param("volume", "18.93");

        let result = calc.calculate(input).unwrap();
        assert_eq!(result.output.value, Decimal::new(1893, 2));
    }
}