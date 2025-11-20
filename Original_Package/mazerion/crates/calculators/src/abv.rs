//! ABV calculator from original and final gravity.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Calculate alcohol by volume from gravity readings.
#[derive(Default)]
pub struct AbvCalculator;

impl AbvCalculator {
    pub const ID: &'static str = "abv";
}

impl Calculator for AbvCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "ABV Calculator"
    }

    fn description(&self) -> &'static str {
        "Calculate alcohol by volume from original and final specific gravity"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        self.validate(&input)?;

        let og = input
            .get_param("og")
            .ok_or_else(|| Error::MissingInput("og parameter required".into()))?;
        let fg = input
            .get_param("fg")
            .ok_or_else(|| Error::MissingInput("fg parameter required".into()))?;

        let og_val: Decimal = og
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid OG: {}", og)))?;
        let fg_val: Decimal = fg
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid FG: {}", fg)))?;

        if og_val < fg_val {
            return Err(Error::Validation("OG must be >= FG".into()));
        }

        let abv = (og_val - fg_val) * dec!(131.25);

        let mut result = CalcResult::new(Measurement::new(abv, Unit::Abv));

        if abv > dec!(20) {
            result = result.with_warning("ABV > 20% is unusually high");
        }

        result = result
            .with_meta("og", og)
            .with_meta("fg", fg)
            .with_meta("formula", "Standard ABV = (OG - FG) × 131.25");

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

register_calculator!(AbvCalculator);

#[cfg(test)]
#[path = "abv_tests.rs"]
mod tests;
