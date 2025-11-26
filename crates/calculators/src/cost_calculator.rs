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

    fn description(&self) -> &'static str {
        "Calculate total batch cost and cost per bottle"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let honey_lbs = input.get_param("honey_lbs")
            .ok_or_else(|| Error::MissingInput("honey_lbs required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid honey_lbs: {}", e)))?;
        
        let honey_cost_per_lb = input.get_param("honey_cost_per_lb")
            .ok_or_else(|| Error::MissingInput("honey_cost_per_lb required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid honey_cost_per_lb: {}", e)))?;
        
        let yeast_cost = input.get_param("yeast_cost")
            .ok_or_else(|| Error::MissingInput("yeast_cost required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid yeast_cost: {}", e)))?;
        
        let nutrients_cost = input.get_param("nutrients_cost")
            .unwrap_or("5.00")
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid nutrients_cost: {}", e)))?;
        
        let other_costs = input.get_param("other_costs")
            .unwrap_or("0.00")
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid other_costs: {}", e)))?;
        
        let bottles = input.get_param("bottles")
            .unwrap_or("30")
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid bottles: {}", e)))?;

        // Calculate total cost
        let honey_cost = honey_lbs * honey_cost_per_lb;
        let total_cost = honey_cost + yeast_cost + nutrients_cost + other_costs;
        let cost_per_bottle = total_cost / bottles;

        let mut result = CalcResult::new(Measurement::new(total_cost, Unit::Percent));
        result = result
            .with_meta("honey_cost", format!("${:.2}", honey_cost))
            .with_meta("yeast_cost", format!("${:.2}", yeast_cost))
            .with_meta("nutrients_cost", format!("${:.2}", nutrients_cost))
            .with_meta("other_costs", format!("${:.2}", other_costs))
            .with_meta("total_cost", format!("${:.2}", total_cost))
            .with_meta("cost_per_bottle", format!("${:.2}", cost_per_bottle))
            .with_meta("bottles", format!("{}", bottles));

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("honey_lbs").is_none() {
            return Err(Error::MissingInput("honey_lbs required".into()));
        }
        if input.get_param("honey_cost_per_lb").is_none() {
            return Err(Error::MissingInput("honey_cost_per_lb required".into()));
        }
        if input.get_param("yeast_cost").is_none() {
            return Err(Error::MissingInput("yeast_cost required".into()));
        }
        Ok(())
    }
}

register_calculator!(CostCalculator);
