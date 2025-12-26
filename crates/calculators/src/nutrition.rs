// TOSNA Nutrition Calculator - FIXED with calculated sugar breaks

use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
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
        "Calculate TOSNA yeast nutrition schedule (1.0, 2.0, or 3.0 protocol)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input
            .get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;
        let yn_requirement = input.get_param("yn_requirement").unwrap_or("medium");
        let protocol = input.get_param("protocol").unwrap_or("tosna_2");

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv
            .parse()
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;

        // Estimate OG from target ABV (assuming ~85% attenuation)
        let estimated_og = Decimal::ONE + (abv / Decimal::new(1115625, 4));
        let gravity_points = (estimated_og - Decimal::ONE) * Decimal::from(1000);

        // CALCULATE ACTUAL SUGAR BREAK GRAVITIES
        let break_1_3 =
            estimated_og - (gravity_points * Decimal::new(333, 3)) / Decimal::from(1000);
        let break_2_3 =
            estimated_og - (gravity_points * Decimal::new(667, 3)) / Decimal::from(1000);

        // Pre-format strings for meta_keys (avoids lifetime issues)
        let break_1_3_str = format!("1/3 sugar break (~{:.3})", break_1_3);
        let break_2_3_str = format!("2/3 sugar break (~{:.3})", break_2_3);

        // YAN targets based on yeast nitrogen requirements
        let yan_factor = match yn_requirement {
            "low" => Decimal::new(90, 2),
            "high" => Decimal::new(150, 2),
            _ => Decimal::new(120, 2),
        };

        let yan_ppm = gravity_points * yan_factor;
        let fermaid_o_ppm_per_gl = Decimal::from(24);
        let fermaid_o_gl = yan_ppm / fermaid_o_ppm_per_gl;
        let total_fermaid_o = fermaid_o_gl * vol;

        // Protocol-specific splits
        let (add1, add2, add3, add4, protocol_name, meta_keys) = match protocol {
            "tosna_1" => {
                let a1 = total_fermaid_o * Decimal::new(333, 3);
                let a2 = total_fermaid_o * Decimal::new(333, 3);
                let a3 = total_fermaid_o - a1 - a2;
                (
                    a1,
                    a2,
                    a3,
                    Decimal::ZERO,
                    "TOSNA 1.0",
                    vec![
                        ("addition_1_24hrs", "33%", "At pitch + 24 hours"),
                        ("addition_2_day_3", "33%", "Day 3"),
                        ("addition_3_day_7", "34%", "Day 7"),
                    ],
                )
            }
            "tosna_3" => {
                let a1 = total_fermaid_o * Decimal::new(5, 2);
                let a2 = total_fermaid_o * Decimal::new(20, 2);
                let a3 = total_fermaid_o * Decimal::new(50, 2);
                let a4 = total_fermaid_o * Decimal::new(25, 2);
                (
                    a1,
                    a2,
                    a3,
                    a4,
                    "TOSNA 3.0",
                    vec![
                        ("addition_1_24hrs", "5%", "24 hours after pitch"),
                        ("addition_2_48hrs", "20%", "48 hours after pitch"),
                        ("addition_3_1/3_break", "50%", break_1_3_str.as_str()),
                        ("addition_4_2/3_break", "25%", break_2_3_str.as_str()),
                    ],
                )
            }
            _ => {
                // TOSNA 2.0 (default): 25%-50%-25%
                let a1 = total_fermaid_o * Decimal::new(25, 2);
                let a2 = total_fermaid_o * Decimal::new(50, 2);
                let a3 = total_fermaid_o * Decimal::new(25, 2);
                (
                    a1,
                    a2,
                    a3,
                    Decimal::ZERO,
                    "TOSNA 2.0",
                    vec![
                        ("addition_1_24hrs", "25%", "24 hours after pitch"),
                        ("addition_2_1/3_break", "50%", break_1_3_str.as_str()),
                        ("addition_3_2/3_break", "25%", break_2_3_str.as_str()),
                    ],
                )
            }
        };

        let mut result = CalcResult::new(Measurement::new(total_fermaid_o, Unit::Grams))
            .with_meta("protocol", protocol_name)
            .with_meta("estimated_og", format!("{:.3}", estimated_og))
            .with_meta("target_yan_ppm", format!("{:.0}", yan_ppm))
            .with_meta("break_1_3_gravity", format!("{:.3}", break_1_3))
            .with_meta("break_2_3_gravity", format!("{:.3}", break_2_3));

        // Add additions with labels
        for (i, (key, pct, timing)) in meta_keys.iter().enumerate() {
            let amount = match i {
                0 => add1,
                1 => add2,
                2 => add3,
                3 => add4,
                _ => Decimal::ZERO,
            };

            if amount > Decimal::ZERO {
                result = result.with_meta(*key, format!("{:.2} g ({}) - {}", amount, pct, timing));
            }
        }

        result = result.with_meta("total_fermaid_o", format!("{:.2} g", total_fermaid_o));

        // Warnings
        if protocol == "tosna_3" && estimated_og < Decimal::new(1100, 3) {
            result = result.with_warning(
                "TOSNA 3.0 designed for high-gravity (OG >1.100) - consider TOSNA 2.0",
            );
        }
        if protocol == "tosna_1" {
            result = result.with_warning(
                "TOSNA 1.0 is older protocol - TOSNA 2.0 or 3.0 recommended for better results",
            );
        }
        if abv > Decimal::from(18) {
            result = result.with_warning("ABV >18% - consider staggered yeast pitch or TOSNA 3.0");
        }
        if yan_ppm > Decimal::from(400) {
            result = result.with_warning("YAN >400ppm - high nitrogen may cause off-flavors");
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

        if let Some(abv_str) = input.get_param("target_abv")
            && let Ok(abv) = abv_str.parse::<Decimal>()
            && (abv < Decimal::from(5) || abv > Decimal::from(20))
        {
            return Err(Error::Validation("ABV should be between 5-20%".into()));
        }

        Ok(())
    }
}

register_calculator!(NutritionCalculator);
