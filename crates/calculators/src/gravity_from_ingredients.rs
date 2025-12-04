use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct GravityFromIngredientsCalculator;

impl GravityFromIngredientsCalculator {
    pub const ID: &'static str = "gravity_from_ingredients";
}

impl Calculator for GravityFromIngredientsCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Gravity from Ingredients"
    }

    fn category(&self) -> &'static str {
        "Basic"
    }

    fn description(&self) -> &'static str {
        "Calculate expected gravity from honey/sugar and water volumes"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let water_volume = input.get_param("water_volume")
            .ok_or_else(|| Error::MissingInput("water_volume required".into()))?;
        let honey_weight = input.get_param("honey_weight")
            .ok_or_else(|| Error::MissingInput("honey_weight required".into()))?;
        let ingredient_type = input.get_param("ingredient_type").unwrap_or("honey");

        let water_l: Decimal = water_volume.parse()
            .map_err(|_| Error::Parse("Invalid water_volume".into()))?;
        let honey_kg: Decimal = honey_weight.parse()
            .map_err(|_| Error::Parse("Invalid honey_weight".into()))?;

        // Gravity points per kg per liter for different ingredients
        let ppg = match ingredient_type {
            "honey" => Decimal::from(35),           // 35 points/kg/L
            "table_sugar" => Decimal::from(46),      // 46 points/kg/L
            "dme" => Decimal::from(44),              // 44 points/kg/L (dry malt extract)
            "maple_syrup" => Decimal::from(30),      // 30 points/kg/L
            _ => Decimal::from(35),
        };

        // Calculate gravity points
        let gravity_points = (honey_kg * ppg) / water_l;

        // Convert to SG (1.000 + points/1000)
        let calculated_sg = Decimal::ONE + (gravity_points / Decimal::from(1000));

        let mut result = CalcResult::new(Measurement::sg(calculated_sg)?);

        result = result
            .with_meta("ingredient_type", ingredient_type)
            .with_meta("ingredient_kg", format!("{:.2} kg", honey_kg))
            .with_meta("water_L", format!("{:.2} L", water_l))
            .with_meta("total_volume", format!("{:.2} L", water_l + (honey_kg / Decimal::new(142, 2))))
            .with_meta("gravity_points", format!("{:.0} points", gravity_points))
            .with_meta("estimated_sg", format!("{:.3}", calculated_sg));

        if calculated_sg > Decimal::new(1120, 3) {
            result = result.with_warning("Very high gravity - may stress yeast, consider stepped feeding");
        }

        Ok(result)
    }
}

register_calculator!(GravityFromIngredientsCalculator);