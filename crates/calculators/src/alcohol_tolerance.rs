use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct AlcoholToleranceCalculator;

impl AlcoholToleranceCalculator {
    pub const ID: &'static str = "alcohol_tolerance";
}

impl Calculator for AlcoholToleranceCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Alcohol Tolerance"
    }

    fn category(&self) -> &'static str {
        "Advanced"
    }

    fn description(&self) -> &'static str {
        "Calculate maximum ABV and estimated FG for yeast strain"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let yeast_strain = input
            .get_param("yeast_strain")
            .ok_or_else(|| Error::MissingInput("yeast_strain required".into()))?;

        // Common yeast strain tolerances
        let (tolerance, temp_range, characteristics) = match yeast_strain.to_lowercase().as_str() {
            "ec-1118" | "ec1118" => (
                Decimal::from(18),
                "15-30°C",
                "Champagne yeast, very clean, high tolerance"
            ),
            "k1-v1116" | "k1v1116" => (
                Decimal::from(18),
                "15-30°C",
                "Strong fermenter, good for meads"
            ),
            "71b-1122" | "71b1122" => (
                Decimal::from(14),
                "15-30°C",
                "Fruity, softens acid, good for melomels"
            ),
            "d47" => (
                Decimal::from(15),
                "15-20°C",
                "Tropical fruit notes, temperature sensitive"
            ),
            "us-05" | "us05" => (
                Decimal::from(12),
                "15-24°C",
                "Clean American ale, neutral"
            ),
            "s-04" | "s04" => (
                Decimal::from(11),
                "15-24°C",
                "English ale, slightly fruity"
            ),
            "wy3068" | "wyeast3068" => (
                Decimal::from(10),
                "18-24°C",
                "Hefeweizen, banana and clove"
            ),
            "safale_be-134" => (
                Decimal::from(11),
                "18-28°C",
                "Belgian Saison, peppery"
            ),
            "qa23" => (
                Decimal::from(16),
                "15-30°C",
                "Portuguese wine yeast, neutral"
            ),
            "dv10" => (
                Decimal::from(16),
                "10-35°C",
                "Wide temperature range, champagne-like"
            ),
            _ => (
                Decimal::from(12),
                "18-24°C",
                "Generic strain (estimate)"
            ),
        };

        let mut result = CalcResult::new(Measurement::new(tolerance, Unit::Abv));

        if tolerance < Decimal::from(12) {
            result = result.with_warning("Low tolerance strain - not suitable for high-gravity brews");
        }

        result = result
            .with_meta("max_abv", format!("{}%", tolerance))
            .with_meta("yeast_strain", yeast_strain)
            .with_meta("temperature_range", temp_range)
            .with_meta("characteristics", characteristics);

        // If OG provided, calculate estimated FG
        if let Some(og_str) = input.get_param("og") {
            let og: Decimal = og_str.parse().map_err(|_| Error::Parse("Invalid OG".into()))?;

            // Formula: ABV = (OG - FG) × 131.25
            // Rearranging: FG = OG - (ABV / 131.25)
            let estimated_fg = og - (tolerance / Decimal::new(13125, 2));

            result = result
                .with_meta("original_gravity", format!("{:.3}", og))
                .with_meta("estimated_fg", format!("{:.3}", estimated_fg))
                .with_meta("calculation", format!("FG = {:.3} - ({} / 131.25) = {:.3}", og, tolerance, estimated_fg));
        }

        result = result.with_meta("tip", "Actual tolerance varies with nutrition and fermentation conditions");

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("yeast_strain").is_none() {
            return Err(Error::MissingInput("yeast_strain required".into()));
        }
        Ok(())
    }
}

register_calculator!(AlcoholToleranceCalculator);