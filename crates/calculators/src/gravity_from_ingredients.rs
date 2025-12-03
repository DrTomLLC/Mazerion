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
        "Calculate expected gravity from ingredients and volumes"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let honey_kg = input.get_param("honey_kg").unwrap_or("0");
        let sugar_kg = input.get_param("sugar_kg").unwrap_or("0");
        let fruit_kg = input.get_param("fruit_kg").unwrap_or("0");
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;

        let honey: Decimal = honey_kg
            .parse()
            .map_err(|_| Error::Parse("Invalid honey_kg".into()))?;
        let sugar: Decimal = sugar_kg
            .parse()
            .map_err(|_| Error::Parse("Invalid sugar_kg".into()))?;
        let fruit: Decimal = fruit_kg
            .parse()
            .map_err(|_| Error::Parse("Invalid fruit_kg".into()))?;
        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;

        // Sugar content percentages
        let honey_sugar_content = Decimal::new(82, 2); // 0.82 (82%)
        let table_sugar_content = Decimal::ONE;         // 1.0 (100%)
        let fruit_sugar_content = Decimal::new(12, 2); // 0.12 (12% average)

        // Calculate total fermentable sugars (in kg)
        let total_sugar = (honey * honey_sugar_content)
            + (sugar * table_sugar_content)
            + (fruit * fruit_sugar_content);

        // Gravity points per kg per liter
        // 1 kg of sugar in 1 L raises gravity by approximately 0.046
        let gravity_points_per_kg_per_l = Decimal::new(46, 3); // 0.046

        // Calculate gravity contribution
        let gravity_contribution = (total_sugar / vol) * gravity_points_per_kg_per_l;

        // Final gravity = 1.000 + contribution
        let estimated_og = Decimal::ONE + gravity_contribution;

        let mut result = CalcResult::new(Measurement::sg(estimated_og)?);

        if estimated_og > Decimal::new(115, 2) {
            result = result.with_warning("High OG (>1.150) - may need nutrient additions and strong yeast");
        }

        if total_sugar < Decimal::new(1, 1) {
            result = result.with_warning("Very low sugar content - fermentation may not start");
        }

        result = result
            .with_meta("estimated_og", format!("{:.3}", estimated_og))
            .with_meta("total_fermentable_sugar", format!("{:.2} kg", total_sugar))
            .with_meta("honey", format!("{} kg", honey))
            .with_meta("sugar", format!("{} kg", sugar))
            .with_meta("fruit", format!("{} kg", fruit))
            .with_meta("volume", format!("{} L", vol));

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("volume").is_none() {
            return Err(Error::MissingInput("volume required".into()));
        }
        Ok(())
    }
}

register_calculator!(GravityFromIngredientsCalculator);