// SAFETY-CRITICAL: Calculate expected gravity from honey/sugar and water volumes
// Uses CORRECT metric formula with proper unit conversions

use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, register_calculator,
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

    fn description(&self) -> &'static str {
        "Calculate expected gravity from honey/sugar and water volumes"
    }

    fn category(&self) -> &'static str {
        "Basic"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let water_volume = input
            .get_param("water_volume")
            .ok_or_else(|| Error::MissingInput("water_volume required".into()))?;
        let honey_weight = input
            .get_param("honey_weight")
            .ok_or_else(|| Error::MissingInput("honey_weight required".into()))?;
        let ingredient_type = input.get_param("ingredient_type").unwrap_or("honey");

        let water_l: Decimal = water_volume
            .parse()
            .map_err(|_| Error::Parse("Invalid water_volume".into()))?;
        let honey_kg: Decimal = honey_weight
            .parse()
            .map_err(|_| Error::Parse("Invalid honey_weight".into()))?;

        // CORRECT METRIC FORMULA: gravity_points = (kg/L) × points_per_kg_per_L
        // These are the CORRECT metric conversions from imperial ppg (points per pound per gallon):
        // Conversion factor: 2.20462 lb/kg × 3.78541 L/gal ≈ 8.345
        // Metric points = Imperial ppg × 8.345

        let points_per_kg_per_l = match ingredient_type {
            "honey" => Decimal::from(292),       // 35 ppg × 8.345 = 292
            "table_sugar" => Decimal::from(384), // 46 ppg × 8.345 = 384
            "dme" => Decimal::from(367),         // 44 ppg × 8.345 = 367 (dry malt extract)
            "maple_syrup" => Decimal::from(250), // 30 ppg × 8.345 = 250
            _ => Decimal::from(292),
        };

        // Calculate concentration (kg/L) then multiply by points per concentration unit
        let concentration_kg_per_l = honey_kg / water_l;
        let gravity_points = concentration_kg_per_l * points_per_kg_per_l;

        // Convert to SG (1.000 + points/1000)
        let calculated_sg = Decimal::ONE + (gravity_points / Decimal::from(1000));

        let mut result = CalcResult::new(Measurement::sg(calculated_sg)?);

        // Calculate total volume including honey displacement
        // Honey density ~1.42 kg/L
        let honey_volume_l = honey_kg / Decimal::new(142, 2);
        let total_volume = water_l + honey_volume_l;

        result = result
            .with_meta("ingredient_type", ingredient_type)
            .with_meta("ingredient_kg", format!("{:.2} kg", honey_kg))
            .with_meta("water_L", format!("{:.2} L", water_l))
            .with_meta("honey_volume_L", format!("{:.2} L", honey_volume_l))
            .with_meta("total_volume", format!("{:.2} L", total_volume))
            .with_meta(
                "concentration",
                format!("{:.3} kg/L", concentration_kg_per_l),
            )
            .with_meta("gravity_points", format!("{:.1} points", gravity_points))
            .with_meta("estimated_sg", format!("{:.3}", calculated_sg))
            .with_meta("formula", "Metric: (kg/L) × points_per_kg_per_L");

        if calculated_sg > Decimal::new(1120, 3) {
            result = result.with_warning(
                "Very high gravity (>1.120) - may stress yeast, consider stepped feeding",
            );
        }

        if calculated_sg < Decimal::new(1040, 3) {
            result = result.with_warning("Low gravity (<1.040) - will produce low ABV");
        }

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("water_volume").is_none() {
            return Err(Error::MissingInput("water_volume required".into()));
        }
        if input.get_param("honey_weight").is_none() {
            return Err(Error::MissingInput("honey_weight required".into()));
        }
        Ok(())
    }
}

register_calculator!(GravityFromIngredientsCalculator);
