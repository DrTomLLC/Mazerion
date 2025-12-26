//! Batch cost calculator for ingredient costing

use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct BatchCostCalculator;

impl BatchCostCalculator {
    pub const ID: &'static str = "batch_cost";
}

impl Calculator for BatchCostCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Batch Cost Calculator"
    }

    fn category(&self) -> &'static str {
        "Utilities"
    }

    fn description(&self) -> &'static str {
        "Calculate total cost per batch and per bottle"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let honey = input.get_param("honey_cost").unwrap_or("0");
        let fruit = input.get_param("fruit_cost").unwrap_or("0");
        let yeast = input.get_param("yeast_cost").unwrap_or("0");
        let nutrients = input.get_param("nutrients_cost").unwrap_or("0");
        let other = input.get_param("other_cost").unwrap_or("0");
        let bottles = input.get_param("bottles_count").unwrap_or("30");

        let honey_cost: Decimal = honey.parse().unwrap_or(Decimal::ZERO);
        let fruit_cost: Decimal = fruit.parse().unwrap_or(Decimal::ZERO);
        let yeast_cost: Decimal = yeast.parse().unwrap_or(Decimal::ZERO);
        let nutrients_cost: Decimal = nutrients.parse().unwrap_or(Decimal::ZERO);
        let other_cost: Decimal = other.parse().unwrap_or(Decimal::ZERO);
        let bottle_count: Decimal = bottles
            .parse()
            .map_err(|_| Error::Parse("Invalid bottle count".into()))?;

        if bottle_count <= Decimal::ZERO {
            return Err(Error::Validation("Bottle count must be positive".into()));
        }

        let total_cost = honey_cost + fruit_cost + yeast_cost + nutrients_cost + other_cost;
        let cost_per_bottle = total_cost / bottle_count;

        let mut result = CalcResult::new(Measurement::new(total_cost, Unit::Percent))
            .with_meta("total_batch_cost", format!("${:.2}", total_cost))
            .with_meta("cost_per_bottle", format!("${:.2}", cost_per_bottle))
            .with_meta("bottle_count", bottles)
            .with_meta("honey_cost", format!("${:.2}", honey_cost))
            .with_meta("fruit_cost", format!("${:.2}", fruit_cost))
            .with_meta("yeast_cost", format!("${:.2}", yeast_cost))
            .with_meta("nutrients_cost", format!("${:.2}", nutrients_cost))
            .with_meta("other_cost", format!("${:.2}", other_cost));

        if cost_per_bottle > Decimal::from(10) {
            result = result.with_warning("Cost >$10/bottle - expensive batch");
        }
        if total_cost > Decimal::from(200) {
            result = result.with_warning("Total cost >$200 - verify ingredient prices");
        }

        Ok(result)
    }

    fn validate(&self, _input: &CalcInput) -> Result<()> {
        Ok(())
    }
}

register_calculator!(BatchCostCalculator);
