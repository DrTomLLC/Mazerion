use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct CostCalculator;

impl CostCalculator {
    pub const ID: &'static str = "cost_calculator";
}

impl Calculator for CostCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Cost Calculator"
    }

    fn category(&self) -> &'static str {
        "Utilities"
    }

    fn description(&self) -> &'static str {
        "Calculate batch cost breakdown and per-bottle pricing"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let honey_cost = input.get_param("honey_cost")
            .ok_or_else(|| Error::MissingInput("honey_cost required".into()))?;
        let honey_kg = input.get_param("honey_kg")
            .ok_or_else(|| Error::MissingInput("honey_kg required".into()))?;

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let h_cost: Decimal = honey_cost.parse()
            .map_err(|_| Error::Parse("Invalid honey_cost".into()))?;
        let h_kg: Decimal = honey_kg.parse()
            .map_err(|_| Error::Parse("Invalid honey_kg".into()))?;

        // Optional costs
        let yeast_cost: Decimal = input.get_param("yeast_cost")
            .and_then(|v| v.parse().ok())
            .unwrap_or(Decimal::new(5, 0));
        let nutrient_cost: Decimal = input.get_param("nutrient_cost")
            .and_then(|v| v.parse().ok())
            .unwrap_or(Decimal::new(3, 0));
        let additive_cost: Decimal = input.get_param("additive_cost")
            .and_then(|v| v.parse().ok())
            .unwrap_or(Decimal::ZERO);
        let bottle_cost: Decimal = input.get_param("bottle_cost")
            .and_then(|v| v.parse().ok())
            .unwrap_or(Decimal::new(15, 1)); // $1.50/bottle default

        // Calculate total honey cost
        let total_honey_cost = h_kg * h_cost;

        // Calculate total batch cost
        let total_batch_cost = total_honey_cost + yeast_cost + nutrient_cost + additive_cost;

        // Calculate number of bottles (750mL standard)
        let bottles_750ml = (vol * Decimal::from(1000)) / Decimal::new(750, 0);
        let bottles_count = bottles_750ml.floor();

        // Calculate per-bottle cost (ingredients only)
        let cost_per_bottle_ingredients = if bottles_count > Decimal::ZERO {
            total_batch_cost / bottles_count
        } else {
            Decimal::ZERO
        };

        // Calculate per-bottle cost (including bottle)
        let cost_per_bottle_total = cost_per_bottle_ingredients + bottle_cost;

        // Total cost including bottles
        let total_cost_with_bottles = total_batch_cost + (bottles_count * bottle_cost);

        let mut result = CalcResult::new(Measurement::new(total_batch_cost, Unit::Grams));

        result = result
            .with_meta("total_batch_cost", format!("${:.2}", total_batch_cost))
            .with_meta("honey_cost", format!("${:.2}", total_honey_cost))
            .with_meta("yeast_cost", format!("${:.2}", yeast_cost))
            .with_meta("nutrient_cost", format!("${:.2}", nutrient_cost))
            .with_meta("additive_cost", format!("${:.2}", additive_cost))
            .with_meta("bottles_750ml", format!("{:.0}", bottles_count))
            .with_meta("cost_per_bottle_ingredients", format!("${:.2}", cost_per_bottle_ingredients))
            .with_meta("cost_per_bottle_with_bottle", format!("${:.2}", cost_per_bottle_total))
            .with_meta("total_with_bottles", format!("${:.2}", total_cost_with_bottles));

        if cost_per_bottle_total > Decimal::from(15) {
            result = result.with_warning("High per-bottle cost - consider bulk ingredient purchases");
        }

        Ok(result)
    }
}

register_calculator!(CostCalculator);