//! Water chemistry calculator for mineral adjustments

use mazerion_core::{
    CalcInput, CalcResult, Calculator, Measurement, Result, Unit, register_calculator,
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
        "Calculate water profile and mineral additions"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let ca = input.get_param("calcium").unwrap_or("0");
        let mg = input.get_param("magnesium").unwrap_or("0");
        let so4 = input.get_param("sulfate").unwrap_or("0");
        let cl = input.get_param("chloride").unwrap_or("0");

        let calcium: Decimal = ca.parse().unwrap_or(Decimal::ZERO);
        let magnesium: Decimal = mg.parse().unwrap_or(Decimal::ZERO);
        let sulfate: Decimal = so4.parse().unwrap_or(Decimal::ZERO);
        let chloride: Decimal = cl.parse().unwrap_or(Decimal::ZERO);

        // Calculate sulfate to chloride ratio (key brewing metric)
        let ratio = if chloride > Decimal::ZERO {
            sulfate / chloride
        } else if sulfate > Decimal::ZERO {
            Decimal::from(999) // Very high ratio
        } else {
            Decimal::ONE
        };

        let profile = if ratio > Decimal::from(3) {
            "Highly Bitter (IPA, Pale Ale)"
        } else if ratio > Decimal::new(15, 1) {
            "Moderately Bitter (Amber, Brown)"
        } else if ratio > Decimal::new(5, 1) {
            "Balanced"
        } else if ratio > Decimal::ZERO {
            "Malty (Stout, Porter, Mead)"
        } else {
            "Chloride Dominant (Sweet)"
        };

        // Calculate mineral additions needed
        let mineral = if calcium < Decimal::from(50) {
            "Gypsum (CaSO4) or Calcium Chloride (CaCl2)"
        } else if sulfate < Decimal::from(50) && chloride < Decimal::from(50) {
            "Gypsum for bitter, CaCl2 for malty"
        } else {
            "Water profile adequate"
        };

        let ion_contribution = format!(
            "Ca: {}ppm, Mg: {}ppm, SO4: {}ppm, Cl: {}ppm",
            calcium, magnesium, sulfate, chloride
        );

        let mut result = CalcResult::new(Measurement::new(ratio, Unit::Percent))
            .with_meta("profile", profile)
            .with_meta("so4_cl_ratio", format!("{:.2}:1", ratio))
            .with_meta("calcium", format!("{} ppm", calcium))
            .with_meta("magnesium", format!("{} ppm", magnesium))
            .with_meta("sulfate", format!("{} ppm", sulfate))
            .with_meta("chloride", format!("{} ppm", chloride))
            .with_meta("mineral", mineral)
            .with_meta("ion_contribution", &ion_contribution);

        if calcium < Decimal::from(50) {
            result = result.with_warning("Calcium <50 ppm - may affect mash pH and yeast health");
        }
        if calcium > Decimal::from(150) {
            result = result.with_warning("Calcium >150 ppm - may be excessive");
        }
        if sulfate > Decimal::from(400) {
            result = result.with_warning("Sulfate >400 ppm - may be too bitter/astringent");
        }
        if chloride > Decimal::from(200) {
            result = result.with_warning("Chloride >200 ppm - may be too sweet/minerally");
        }

        Ok(result)
    }

    fn validate(&self, _input: &CalcInput) -> Result<()> {
        Ok(())
    }
}

register_calculator!(WaterChemistryCalculator);
