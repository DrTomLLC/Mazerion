use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct BacksweeteningCalculator;

impl BacksweeteningCalculator {
    pub const ID: &'static str = "backsweetening";
}

impl Calculator for BacksweeteningCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Backsweetening" }
    fn description(&self) -> &'static str { "Calculate sweetener to reach target FG" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let current_sg = input.get_measurement(Unit::SpecificGravity)?.value;
        let target_sg = input.get_param("target_sg")
            .ok_or_else(|| Error::MissingInput("target_sg required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid target_sg: {}", e)))?;
        let sweetener = input.get_param("sweetener").unwrap_or("honey");

        if target_sg <= current_sg {
            return Err(Error::Validation("Target SG must be higher than current SG".into()));
        }

        let points_needed = (target_sg - current_sg) * Decimal::from(1000);
        let factor = match sweetener {
            "table_sugar" => Decimal::from(46),
            "agave" => Decimal::from(40),
            "maple_syrup" => Decimal::from(36),
            _ => Decimal::from(35),
        };

        let grams = (points_needed * volume) / factor;
        let mut result = CalcResult::new(Measurement::new(grams, Unit::Grams));
        result = result.with_warning("MUST stabilize with K-meta and sorbate first!");
        Ok(result)
    }
}

register_calculator!(BacksweeteningCalculator);
