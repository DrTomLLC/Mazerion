use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct NutritionCalculator;

impl NutritionCalculator {
    pub const ID: &'static str = "nutrition";
}

impl Calculator for NutritionCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "TOSNA Nutrition"
    }

    fn category(&self) -> &'static str {
        "Brewing"
    }

    fn description(&self) -> &'static str {
        "Calculate TOSNA yeast nutrition schedule"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;
        let yn_requirement = input.get_param("yn_requirement").unwrap_or("medium");

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse()
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;

        let yeast_factor = match yn_requirement {
            "low" => Decimal::new(8, 1),
            "high" => Decimal::new(12, 1),
            _ => Decimal::ONE,
        };

        let yan_ppm = abv * Decimal::from(50) * yeast_factor;
        let fermaid_o_ppm_per_gl = Decimal::from(24);
        let fermaid_o_gl = yan_ppm / fermaid_o_ppm_per_gl;
        let total_fermaid_o = fermaid_o_gl * vol;

        let addition1 = total_fermaid_o * Decimal::new(33, 2);
        let addition2 = total_fermaid_o * Decimal::new(33, 2);
        let addition3 = total_fermaid_o * Decimal::new(33, 2);
        let addition4 = total_fermaid_o - addition1 - addition2 - addition3;

        let mut result = CalcResult::new(Measurement::new(total_fermaid_o, Unit::Grams))
            .with_meta("protocol", "TOSNA 2.0")
            .with_meta("target_yan_ppm", format!("{:.0}", yan_ppm))
            .with_meta("total_fermaid_o", format!("{:.2} g", total_fermaid_o))
            .with_meta("addition_1_24hrs", format!("{:.2} g", addition1))
            .with_meta("addition_2_at_1067", format!("{:.2} g", addition2))
            .with_meta("addition_3_at_1045", format!("{:.2} g", addition3))
            .with_meta("addition_4_at_1023", format!("{:.2} g", addition4));

        if abv > Decimal::from(18) {
            result = result.with_warning("ABV >18% - consider staggered yeast additions");
        }
        if yan_ppm > Decimal::from(400) {
            result = result.with_warning("YAN >400ppm - may stress yeast");
        }
        if yan_ppm < Decimal::from(150) {
            result = result.with_warning("YAN <150ppm - may result in sluggish fermentation");
        }

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("volume").is_none() {
            return Err(Error::MissingInput("volume required".into()));
        }
        if input.get_param("target_abv").is_none() {
            return Err(Error::MissingInput("target_abv required".into()));
        }

        if let Some(abv_str) = input.get_param("target_abv") {
            if let Ok(abv) = abv_str.parse::<Decimal>() {
                if abv < Decimal::from(5) || abv > Decimal::from(20) {
                    return Err(Error::Validation("ABV should be between 5-20%".into()));
                }
            }
        }

        Ok(())
    }
}

register_calculator!(NutritionCalculator);