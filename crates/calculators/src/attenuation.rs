use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct AttenuationCalculator;

impl AttenuationCalculator {
    pub const ID: &'static str = "attenuation";
}

impl Calculator for AttenuationCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Attenuation Calculator"
    }

    fn category(&self) -> &'static str {
        "Advanced"
    }

    fn description(&self) -> &'static str {
        "Calculate apparent and real attenuation (ASBC formulas)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let og = input
            .get_param("og")
            .ok_or_else(|| Error::MissingInput("og required".into()))?;
        let fg = input
            .get_param("fg")
            .ok_or_else(|| Error::MissingInput("fg required".into()))?;

        let og_val: Decimal = og.parse().map_err(|_| Error::Parse("Invalid OG".into()))?;
        let fg_val: Decimal = fg.parse().map_err(|_| Error::Parse("Invalid FG".into()))?;

        if fg_val > og_val {
            return Err(Error::Validation("FG cannot be greater than OG".into()));
        }

        // Apparent Attenuation (AA%)
        // AA% = (OG - FG) / (OG - 1.000) × 100
        let apparent_attenuation =
            ((og_val - fg_val) / (og_val - Decimal::ONE)) * Decimal::from(100);

        // Convert SG to Plato (rough approximation)
        // P ≈ 250 × SG - 250
        let p0 = (og_val - Decimal::ONE) * Decimal::from(250); // Original extract (°P)
        let pf = (fg_val - Decimal::ONE) * Decimal::from(250); // Apparent extract (°P)

        // Real Extract (RE, °P) - ASBC formula
        // RE = 0.1808 × P_0 + 0.8192 × P_f
        let re = (Decimal::new(1808, 4) * p0) + (Decimal::new(8192, 4) * pf);

        // Real Attenuation (TA%) - ASBC formula
        // TA% = (P_0 - RE) / P_0 × 100
        let real_attenuation = if p0 > Decimal::ZERO {
            ((p0 - re) / p0) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };

        let mut result = CalcResult::new(Measurement::new(apparent_attenuation, Unit::Percent));

        if apparent_attenuation < Decimal::from(65) {
            result =
                result.with_warning("Low attenuation (<65%) - may be under-attenuated or stuck");
        }

        if apparent_attenuation > Decimal::from(85) {
            result = result.with_warning("Very high attenuation (>85%) - check for contamination");
        }

        result = result
            .with_meta(
                "apparent_attenuation",
                format!("{:.1}%", apparent_attenuation),
            )
            .with_meta("real_attenuation", format!("{:.1}%", real_attenuation))
            .with_meta("real_extract", format!("{:.2}°P", re))
            .with_meta("original_extract", format!("{:.2}°P", p0))
            .with_meta("original_gravity", og)
            .with_meta("final_gravity", fg)
            .with_meta("formula", "ASBC standard");

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("og").is_none() {
            return Err(Error::MissingInput("OG required".into()));
        }
        if input.get_param("fg").is_none() {
            return Err(Error::MissingInput("FG required".into()));
        }
        Ok(())
    }
}

register_calculator!(AttenuationCalculator);
