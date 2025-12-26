use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Unit, register_calculator,
};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

#[derive(Default)]
pub struct GallonsToBottlesCalculator;
impl GallonsToBottlesCalculator {
    pub const ID: &'static str = "gallons_to_bottles";
}
impl Calculator for GallonsToBottlesCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Gallons to Bottles (with Losses)"
    }

    fn description(&self) -> &'static str {
        "Convert volume in gallons to bottle count (750ml bottles)"
    }

    fn category(&self) -> &'static str {
        "Utilities"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult, Error> {
        // input.params is Vec<(String, String)> - iterate to find
        let get_param = |name: &str| -> Result<Decimal, Error> {
            input
                .params
                .iter()
                .find(|(k, _)| k == name)
                .map(|(_, v)| v.as_str())
                .ok_or_else(|| Error::MissingInput(name.to_string()))?
                .parse::<Decimal>()
                .map_err(|_| Error::Calculation(format!("Invalid {}", name)))
        };

        let initial_volume_gal = get_param("initial_volume")?;
        let loss_rate_percent = get_param("loss_rate_percent")?;
        let num_rackings = get_param("num_rackings")?;

        // Validate
        if initial_volume_gal <= Decimal::ZERO {
            return Err(Error::Calculation(
                "Initial volume must be positive".to_string(),
            ));
        }
        if loss_rate_percent < Decimal::ZERO || loss_rate_percent >= Decimal::from(100) {
            return Err(Error::Calculation("Loss rate must be 0-100%".to_string()));
        }
        if num_rackings < Decimal::ONE || num_rackings > Decimal::from(20) {
            return Err(Error::Calculation("Rackings must be 1-20".to_string()));
        }

        let num_rackings_u32 = num_rackings
            .to_u32()
            .ok_or_else(|| Error::Calculation("Invalid rackings".to_string()))?;

        // Constants
        let liters_per_gallon = Decimal::new(3785411784, 9);
        let liters_per_bottle = Decimal::new(75, 2);
        let bottles_per_gallon = liters_per_gallon / liters_per_bottle;

        // CORRECT COMPOUNDING MATH
        let loss_rate_decimal = loss_rate_percent / Decimal::from(100);
        let retention_rate = Decimal::ONE - loss_rate_decimal;

        let mut compounded_retention = Decimal::ONE;
        for _ in 0..num_rackings_u32 {
            compounded_retention *= retention_rate;
        }

        let final_volume_gal = initial_volume_gal * compounded_retention;
        let total_bottles = final_volume_gal * bottles_per_gallon;
        let total_loss_gal = initial_volume_gal - final_volume_gal;
        let total_loss_percent = (total_loss_gal / initial_volume_gal) * Decimal::from(100);

        let mut metadata = vec![
            (
                "Initial".to_string(),
                format!("{:.3} gal", initial_volume_gal),
            ),
            ("Final".to_string(), format!("{:.3} gal", final_volume_gal)),
            (
                "Loss".to_string(),
                format!("{:.3} gal ({:.2}%)", total_loss_gal, total_loss_percent),
            ),
        ];

        let mut current = initial_volume_gal;
        for i in 1..=num_rackings_u32 {
            current *= retention_rate;
            metadata.push((
                format!("After racking {}", i),
                format!(
                    "{:.3} gal ({:.2} bottles)",
                    current,
                    current * bottles_per_gallon
                ),
            ));
        }

        let mut warnings = Vec::new();
        if loss_rate_percent > Decimal::from(10) {
            warnings.push(format!("High loss rate: {:.1}%", loss_rate_percent));
        }

        Ok(CalcResult {
            output: Measurement {
                value: total_bottles.round_dp(2),
                unit: Unit::Grams,
            },
            metadata,
            warnings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compounding() {
        let calc = GallonsToBottlesCalculator;
        let input = CalcInput::new()
            .add_param("initial_volume", "5")
            .add_param("loss_rate_percent", "5")
            .add_param("num_rackings", "3");
        let result = calc.calculate(input).unwrap();
        assert_eq!(result.output.value.round_dp(2), Decimal::new(2164, 2));
    }
}
register_calculator!(GallonsToBottlesCalculator);
