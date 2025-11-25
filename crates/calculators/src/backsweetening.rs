//! Backsweetening calculator.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

/// Calculate sweetener additions to reach target sweetness.
#[derive(Default)]
pub struct BacksweeteningCalculator;

impl BacksweeteningCalculator {
    pub const ID: &'static str = "backsweetening";
}

impl Calculator for BacksweeteningCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Backsweetening Calculator"
    }

    fn description(&self) -> &'static str {
        "Calculate sweetener additions (MUST stabilize first!)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let current_sg_meas = input.get_measurement(Unit::SpecificGravity)?;
        let current_sg = current_sg_meas.value;

        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;

        let target_sg = input.get_param("target_sg")
            .ok_or_else(|| Error::MissingInput("target_sg required".into()))?
            .parse::<Decimal>()
            .map_err(|_| Error::Parse("Invalid target_sg".into()))?;

        let sweetener = input.get_param("sweetener").unwrap_or("honey");

        if target_sg <= current_sg {
            return Err(Error::Validation("Target SG must be greater than current SG".into()));
        }

        let sg_increase = target_sg - current_sg;

        let sweetener_factor = match sweetener {
            "honey" => Decimal::new(35, 0),
            "table_sugar" => Decimal::new(46, 0),
            "agave" => Decimal::new(32, 0),
            "maple_syrup" => Decimal::new(28, 0),
            _ => Decimal::new(35, 0),
        };

        let gravity_points_needed = sg_increase * Decimal::from(1000);

        let sweetener_grams = (gravity_points_needed * volume) / sweetener_factor;

        let mut result = CalcResult::new(Measurement::grams(sweetener_grams)?);

        result = result.with_warning("⚠️ CRITICAL: Stabilize with K-meta and K-sorbate FIRST!");

        if sg_increase > Decimal::new(20, 3) {
            result = result.with_warning("Large gravity increase - add gradually");
        }

        let sweetener_name = match sweetener {
            "honey" => "Honey",
            "table_sugar" => "Table Sugar",
            "agave" => "Agave Nectar",
            "maple_syrup" => "Maple Syrup",
            _ => "Sweetener",
        };

        result = result
            .with_meta("sweetener", sweetener_name)
            .with_meta("current_sg", format!("{:.4}", current_sg))
            .with_meta("target_sg", format!("{:.4}", target_sg))
            .with_meta("sg_increase", format!("{:.4}", sg_increase))
            .with_meta("volume", format!("{} L", volume));

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        input.get_measurement(Unit::SpecificGravity)?;
        let required = ["volume", "target_sg"];
        for param in &required {
            if input.get_param(param).is_none() {
                return Err(Error::MissingInput(format!("{} required", param)));
            }
        }
        Ok(())
    }
}

register_calculator!(BacksweeteningCalculator);