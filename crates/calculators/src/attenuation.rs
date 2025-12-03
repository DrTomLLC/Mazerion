use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
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
        "Calculate fermentation attenuation percentage"
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

        // Apparent Attenuation (AA)
        let apparent_attenuation = ((og_val - fg_val) / (og_val - Decimal::ONE)) * Decimal::from(100);

        // Real Extract (RE) - accounts for alcohol's lower density
        let re = (Decimal::new(1881, 3) * og_val)
            - (Decimal::new(1113, 3) * fg_val)
            - Decimal::new(463, 3);

        // Real Attenuation (RA)
        let oe = (og_val - Decimal::ONE) * Decimal::from(1000); // Original Extract in Plato
        let real_attenuation = ((oe - re) / oe) * Decimal::from(100);

        let mut result = CalcResult::new(Measurement::new(apparent_attenuation, Unit::Percent));

        if apparent_attenuation < Decimal::from(65) {
            result = result.with_warning("Low attenuation - may be under-attenuated or stuck");
        }

        if apparent_attenuation > Decimal::from(85) {
            result = result.with_warning("Very high attenuation - check for contamination");
        }

        result = result
            .with_meta("apparent_attenuation", format!("{:.1}%", apparent_attenuation))
            .with_meta("real_attenuation", format!("{:.1}%", real_attenuation))
            .with_meta("original_gravity", og)
            .with_meta("final_gravity", fg);

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