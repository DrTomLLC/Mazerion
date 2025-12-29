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

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv
            .parse()
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;

        if abv < Decimal::from(5) || abv > Decimal::from(20) {
            return Err(Error::Validation(
                "ABV should be between 5-20%".into(),
            ));
        }

        let og = if let Some(sg_str) = input.get_param("starting_gravity") {
            sg_str
                .parse::<Decimal>()
                .map_err(|_| Error::Parse("Invalid starting_gravity".into()))?
        } else {
            // ABV = (OG - 1.000) × 131.25, so OG = 1.000 + (ABV / 131.25)
            Decimal::ONE + (abv / Decimal::new(13125, 2))
        };

        let protocol = input.get_param("protocol").unwrap_or("tosna_2");

        let gravity_points = (og - Decimal::ONE) * Decimal::from(1000);

        let break_1_3 =
            og - (gravity_points * Decimal::new(333, 3)) / Decimal::from(1000);
        let break_2_3 =
            og - (gravity_points * Decimal::new(667, 3)) / Decimal::from(1000);

        let break_1_3_str = format!("1/3 sugar break (~{:.3})", break_1_3);
        let break_2_3_str = format!("2/3 sugar break (~{:.3})", break_2_3);

        // TOSNA YAN calculation based on gravity points
        // Formula: Base 200 ppm + (gravity_points - 70) * 2
        // This gives ~200 ppm for 1.070, ~300 ppm for 1.120, ~360 ppm for 1.150
        let base_yan = Decimal::from(200);
        let gravity_factor = if gravity_points > Decimal::from(70) {
            (gravity_points - Decimal::from(70)) * Decimal::from(2)
        } else {
            Decimal::ZERO
        };
        let yan_ppm = base_yan + gravity_factor;

        // YAN calculation: ppm * liters = mg (since ppm = mg/L)
        let yan_mg = yan_ppm * vol;
        let fermaid_o_yan_per_g = Decimal::new(210, 0);
        let total_fermaid_o = yan_mg / fermaid_o_yan_per_g;

        let (add1, add2, add3, add4, meta_keys) = match protocol {
            "tosna_1" => {
                let per_addition = total_fermaid_o / Decimal::from(4);
                (
                    per_addition,
                    per_addition,
                    per_addition,
                    per_addition,
                    [
                        ("addition_1_24hrs", "25%", "24 hours after pitch"),
                        ("addition_2_48hrs", "25%", "48 hours after pitch"),
                        ("addition_3_72hrs", "25%", "72 hours after pitch"),
                        ("addition_4_1/3_break", "25%", break_1_3_str.as_str()),
                    ],
                )
            }
            "tosna_3" => {
                let add1 = total_fermaid_o * Decimal::new(33, 2);
                let add2 = total_fermaid_o * Decimal::new(33, 2);
                let add3 = total_fermaid_o * Decimal::new(34, 2);
                (
                    add1,
                    add2,
                    add3,
                    Decimal::ZERO,
                    [
                        ("addition_1_12hrs", "33%", "12 hours after pitch"),
                        ("addition_2_1/3_break", "33%", break_1_3_str.as_str()),
                        ("addition_3_2/3_break", "34%", break_2_3_str.as_str()),
                        ("", "", ""),
                    ],
                )
            }
            _ => {
                let add1 = total_fermaid_o * Decimal::new(25, 2);
                let add2 = total_fermaid_o * Decimal::new(50, 2);
                let add3 = total_fermaid_o * Decimal::new(25, 2);
                (
                    add1,
                    add2,
                    add3,
                    Decimal::ZERO,
                    [
                        ("addition_1_24hrs", "25%", "24 hours after pitch"),
                        ("addition_2_1/3_break", "50%", break_1_3_str.as_str()),
                        ("addition_3_2/3_break", "25%", break_2_3_str.as_str()),
                        ("", "", ""),
                    ],
                )
            }
        };

        let protocol_name = match protocol {
            "tosna_1" => "TOSNA 1.0",
            "tosna_3" => "TOSNA 3.0",
            _ => "TOSNA 2.0",
        };

        let mut result = CalcResult::new(Measurement::new(total_fermaid_o, Unit::Grams))
            .with_meta("protocol", protocol_name)
            .with_meta("estimated_og", format!("{:.3}", og))
            .with_meta("target_yan_ppm", format!("{:.0}", yan_ppm))
            .with_meta("break_1_3_gravity", format!("{:.3}", break_1_3))
            .with_meta("break_2_3_gravity", format!("{:.3}", break_2_3));

        for (i, (key, pct, timing)) in meta_keys.iter().enumerate() {
            if key.is_empty() {
                continue;
            }

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

        if protocol == "tosna_3" && og < Decimal::new(1100, 3) {
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
        Ok(())
    }
}

register_calculator!(NutritionCalculator);