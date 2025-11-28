//! Backsweetening calculator for post-fermentation sweetening.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

/// Calculate sweetener additions to reach target gravity.
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
        "Calculate sweetener additions to reach target sweetness"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let current_sg_meas = input.get_measurement(Unit::SpecificGravity)?;
        let current_sg = current_sg_meas.value;

        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_sg = input
            .get_param("target_sg")
            .ok_or_else(|| Error::MissingInput("target_sg required".into()))?;
        let sweetener = input.get_param("sweetener").unwrap_or("honey");

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid volume: {}", volume)))?;
        let tgt_sg: Decimal = target_sg
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid target SG: {}", target_sg)))?;

        if tgt_sg <= current_sg {
            return Err(Error::Validation(
                "Target SG must be higher than current SG".into(),
            ));
        }

        let sg_increase = tgt_sg - current_sg;

        // Gravity points per kg/L of sweetener
        let grav_per_kg = match sweetener {
            "honey" => Decimal::new(35, 0),         // 35 gravity points per kg/L
            "table_sugar" => Decimal::new(46, 0),   // 46 gravity points per kg/L
            "agave" => Decimal::new(32, 0),         // 32 gravity points per kg/L
            "maple_syrup" => Decimal::new(28, 0),   // 28 gravity points per kg/L
            _ => Decimal::new(35, 0),
        };

        // Convert SG increase to gravity points (multiply by 1000)
        let points_needed = sg_increase * Decimal::from(1000);

        // Sweetener needed (kg) = (points × volume) / gravity_per_kg
        let sweetener_kg = (points_needed * vol) / grav_per_kg;
        let sweetener_g = sweetener_kg * Decimal::from(1000);

        let mut result = CalcResult::new(Measurement::new(sweetener_g, Unit::Grams));
        result = result
            .with_meta("current_sg", format!("{:.4}", current_sg))
            .with_meta("target_sg", format!("{:.4}", tgt_sg))
            .with_meta("sweetener_type", sweetener)
            .with_meta("volume", format!("{} L", vol));

        result = result.with_warning("MUST stabilize (sorbate + sulfite) before backsweetening!");

        Ok(result)
    }
}

register_calculator!(BacksweeteningCalculator);