use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct WaterChemistryCalculator;

impl WaterChemistryCalculator {
    pub const ID: &'static str = "water_chemistry";
}

impl Calculator for WaterChemistryCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Water Chemistry"
    }

    fn category(&self) -> &'static str {
        "Utilities"
    }

    fn description(&self) -> &'static str {
        "Calculate water chemistry adjustments (mineral additions)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let adjustment = input.get_param("adjustment").unwrap_or("gypsum");
        let target_ppm = input.get_param("target_ppm").unwrap_or("50");

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let target: Decimal = target_ppm.parse()
            .map_err(|_| Error::Parse("Invalid target_ppm".into()))?;

        // Mineral addition rates (grams per liter to raise ppm by 1)
        let (mineral_name, rate_per_ppm, ion_name) = match adjustment {
            "gypsum" => ("Gypsum (CaSO₄)", Decimal::new(15, 1), "Calcium & Sulfate"),        // 1.5 g/L per 100 ppm
            "calcium_chloride" => ("Calcium Chloride (CaCl₂)", Decimal::new(14, 1), "Calcium & Chloride"), // 1.4 g/L per 100 ppm
            "epsom" => ("Epsom Salt (MgSO₄)", Decimal::new(16, 1), "Magnesium & Sulfate"),   // 1.6 g/L per 100 ppm
            "baking_soda" => ("Baking Soda (NaHCO₃)", Decimal::new(12, 1), "Sodium & Bicarbonate"), // 1.2 g/L per 100 ppm
            "chalk" => ("Chalk (CaCO₃)", Decimal::new(18, 1), "Calcium & Carbonate"),        // 1.8 g/L per 100 ppm
            _ => ("Gypsum (CaSO₄)", Decimal::new(15, 1), "Calcium & Sulfate"),
        };

        // Calculate grams needed
        let grams_needed = vol * target * rate_per_ppm / Decimal::from(100);

        let mut result = CalcResult::new(Measurement::new(grams_needed, Unit::Grams));

        result = result
            .with_meta("mineral", mineral_name)
            .with_meta("ion_contribution", ion_name)
            .with_meta("grams_needed", format!("{:.1} g", grams_needed))
            .with_meta("teaspoons", format!("{:.1} tsp", grams_needed / Decimal::new(5, 0)))
            .with_meta("target_ppm", format!("{} ppm", target))
            .with_meta("volume_L", format!("{} L", vol));

        if target > Decimal::from(150) {
            result = result.with_warning("High mineral addition - may affect flavor profile significantly");
        }

        if adjustment == "chalk" {
            result = result.with_warning("Chalk has poor solubility - dissolve in acid or use slaked lime instead");
        }

        Ok(result)
    }
}

register_calculator!(WaterChemistryCalculator);