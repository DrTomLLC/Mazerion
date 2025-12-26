//! Sulfite calculator - K-meta additions with pH effectiveness
//! FIXED: Now returns correct grams value

use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct SulfiteCalculator;

impl SulfiteCalculator {
    pub const ID: &'static str = "sulfite";
}

impl Calculator for SulfiteCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Sulfite Calculator"
    }

    fn category(&self) -> &'static str {
        "Finishing"
    }

    fn description(&self) -> &'static str {
        "Calculate K-meta additions with pH-dependent effectiveness"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let ph_meas = input.get_measurement(Unit::Ph)?;
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target = input
            .get_param("target_free_so2")
            .ok_or_else(|| Error::MissingInput("target_free_so2 required".into()))?;

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let targ: Decimal = target
            .parse()
            .map_err(|_| Error::Parse("Invalid target".into()))?;
        let ph = ph_meas.value;

        // Formula: K-meta (g) = Volume (L) × Target SO₂ (ppm) × 0.2 / 1000 × 1000
        // Simplified: Volume × Target × 0.2
        // FIXED: Was dividing by 1000 and returning kg as grams
        let kmeta_g = vol * targ * Decimal::new(2, 1);

        // pH effectiveness factor (molecular SO₂ increases at lower pH)
        let effectiveness = if ph < Decimal::new(30, 1) {
            "Very High (pH < 3.0)"
        } else if ph < Decimal::new(35, 1) {
            "High (pH 3.0-3.5)"
        } else if ph < Decimal::new(38, 1) {
            "Moderate (pH 3.5-3.8)"
        } else {
            "Low (pH > 3.8) - Consider adding more or lowering pH"
        };

        let mut result = CalcResult::new(Measurement::new(kmeta_g, Unit::Grams));

        result = result
            .with_meta("kmeta_g", format!("{:.2} g", kmeta_g))
            .with_meta(
                "kmeta_tsp",
                format!("{:.2} tsp", kmeta_g / Decimal::from(5)),
            )
            .with_meta("volume_L", format!("{:.2} L", vol))
            .with_meta("target_so2_ppm", format!("{} ppm", targ))
            .with_meta("ph", format!("{:.2}", ph))
            .with_meta("effectiveness", effectiveness)
            .with_meta(
                "tip",
                "Add K-meta 24 hours before sorbate. Dissolve in small amount of water first.",
            );

        if ph > Decimal::new(38, 1) {
            result = result.with_warning(
                "High pH reduces sulfite effectiveness - consider adjusting pH first",
            );
        }

        if targ > Decimal::from(80) {
            result = result.with_warning("High SO₂ target (>80 ppm) - may affect aroma and flavor");
        }

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        input.get_measurement(Unit::Ph)?;
        if input.get_param("volume").is_none() {
            return Err(Error::MissingInput("volume required".into()));
        }
        if input.get_param("target_free_so2").is_none() {
            return Err(Error::MissingInput("target_free_so2 required".into()));
        }
        Ok(())
    }
}

register_calculator!(SulfiteCalculator);
