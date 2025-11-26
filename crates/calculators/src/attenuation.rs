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
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Attenuation" }
    fn description(&self) -> &'static str { "Calculate apparent and real attenuation" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let og = input.get_param("og")
            .ok_or_else(|| Error::MissingInput("og required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid og: {}", e)))?;
        let fg = input.get_param("fg")
            .ok_or_else(|| Error::MissingInput("fg required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid fg: {}", e)))?;

        let apparent = ((og - fg) / (og - Decimal::ONE)) * Decimal::from(100);
        let real_extract = (Decimal::new(1948, 3) * og) - (Decimal::new(648, 3) * fg);
        let real = ((og - real_extract) / (og - Decimal::ONE)) * Decimal::from(100);

        let mut result = CalcResult::new(Measurement::new(apparent, Unit::Percent));
        result = result
            .with_meta("apparent_attenuation", format!("{:.1}%", apparent))
            .with_meta("real_attenuation", format!("{:.1}%", real));
        Ok(result)
    }
}

register_calculator!(AttenuationCalculator);
