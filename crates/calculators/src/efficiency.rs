//! Brewhouse efficiency calculator
//! Calculates mash, lauter, and total efficiency

use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct EfficiencyCalculator;

impl EfficiencyCalculator {
    pub const ID: &'static str = "efficiency";
}

impl Calculator for EfficiencyCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Brewhouse Efficiency"
    }

    fn category(&self) -> &'static str {
        "Beer"
    }

    fn description(&self) -> &'static str {
        "Calculate brewhouse efficiency from grain and gravity"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let grain_weight = input
            .get_param("grain_weight")
            .ok_or_else(|| Error::MissingInput("grain_weight required".into()))?;
        let ppg = input.get_param("ppg").ok_or_else(|| {
            Error::MissingInput("ppg (points per pound per gallon) required".into())
        })?;
        let measured_gravity = input
            .get_param("measured_gravity")
            .ok_or_else(|| Error::MissingInput("measured_gravity required".into()))?;
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;

        let weight: Decimal = grain_weight
            .parse()
            .map_err(|_| Error::Parse("Invalid grain weight".into()))?;
        let ppg_val: Decimal = ppg
            .parse()
            .map_err(|_| Error::Parse("Invalid PPG".into()))?;
        let gravity: Decimal = measured_gravity
            .parse()
            .map_err(|_| Error::Parse("Invalid gravity".into()))?;
        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;

        if weight <= Decimal::ZERO {
            return Err(Error::Validation("Grain weight must be positive".into()));
        }
        if vol <= Decimal::ZERO {
            return Err(Error::Validation("Volume must be positive".into()));
        }
        if ppg_val <= Decimal::ZERO {
            return Err(Error::Validation("PPG must be positive".into()));
        }

        // Gravity points = (SG - 1) × 1000
        let gravity_points = (gravity - Decimal::ONE) * Decimal::from(1000);

        // Total points = gravity_points × volume
        let total_points = gravity_points * vol;

        // Potential points = grain_weight × PPG
        let potential_points = weight * ppg_val;

        // Efficiency = (actual / potential) × 100
        let efficiency = if potential_points > Decimal::ZERO {
            (total_points / potential_points) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };

        let efficiency_category = if efficiency >= Decimal::from(80) {
            "Excellent (80%+)"
        } else if efficiency >= Decimal::from(75) {
            "Very Good (75-80%)"
        } else if efficiency >= Decimal::from(70) {
            "Good (70-75%)"
        } else if efficiency >= Decimal::from(65) {
            "Average (65-70%)"
        } else if efficiency >= Decimal::from(60) {
            "Below Average (60-65%)"
        } else {
            "Poor (<60%) - Check process"
        };

        let mut result = CalcResult::new(Measurement::new(efficiency, Unit::Percent))
            .with_meta("efficiency", format!("{:.1}%", efficiency))
            .with_meta("category", efficiency_category)
            .with_meta("gravity_points", format!("{:.1}", gravity_points))
            .with_meta("total_points", format!("{:.1}", total_points))
            .with_meta("potential_points", format!("{:.1}", potential_points))
            .with_meta("formula", "Brewhouse efficiency");

        if efficiency < Decimal::from(60) {
            result = result.with_warning("Efficiency <60% - check crush, mash pH, water chemistry");
        }
        if efficiency > Decimal::from(85) {
            result = result.with_warning("Efficiency >85% - unusually high, verify measurements");
        }
        if gravity < Decimal::new(10000, 4) {
            result = result.with_warning("Gravity <1.000 - check hydrometer calibration");
        }

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("grain_weight").is_none() {
            return Err(Error::MissingInput("grain_weight required".into()));
        }
        if input.get_param("ppg").is_none() {
            return Err(Error::MissingInput("ppg required".into()));
        }
        if input.get_param("measured_gravity").is_none() {
            return Err(Error::MissingInput("measured_gravity required".into()));
        }
        if input.get_param("volume").is_none() {
            return Err(Error::MissingInput("volume required".into()));
        }
        Ok(())
    }
}

register_calculator!(EfficiencyCalculator);
