//! Input validation with range checks.

use crate::{Error, Result};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Range validator with warnings.
pub struct Validator;

impl Validator {
    /// Validate specific gravity (0.6000–2.0000).
    pub fn sg(value: Decimal) -> Result<()> {
        if value < dec!(0.6000) || value > dec!(2.0000) {
            return Err(Error::OutOfRange(format!(
                "SG {} outside range 0.6000–2.0000",
                value
            )));
        }
        Ok(())
    }

    /// Validate pH (1.50–8.50).
    pub fn ph(value: Decimal) -> Result<()> {
        if value < dec!(1.50) || value > dec!(8.50) {
            return Err(Error::OutOfRange(format!(
                "pH {} outside range 1.50–8.50",
                value
            )));
        }
        Ok(())
    }

    /// Validate Brix (0–70, warn >45).
    pub fn brix(value: Decimal) -> Result<()> {
        if value < dec!(0) || value > dec!(70) {
            return Err(Error::OutOfRange(format!(
                "Brix {} outside range 0–70",
                value
            )));
        }
        Ok(())
    }

    /// Check if Brix needs warning (>45).
    pub fn brix_warning(value: Decimal) -> Option<String> {
        if value > dec!(45) {
            Some(format!("Brix {} above typical range (0–45)", value))
        } else {
            None
        }
    }

    /// Validate Plato (0–70, warn >45).
    pub fn plato(value: Decimal) -> Result<()> {
        if value < dec!(0) || value > dec!(70) {
            return Err(Error::OutOfRange(format!(
                "Plato {} outside range 0–70",
                value
            )));
        }
        Ok(())
    }

    /// Check if Plato needs warning (>45).
    pub fn plato_warning(value: Decimal) -> Option<String> {
        if value > dec!(45) {
            Some(format!("Plato {} above typical range (0–45)", value))
        } else {
            None
        }
    }

    /// Validate temperature Celsius (−5–100).
    pub fn temp_c(value: Decimal) -> Result<()> {
        if value < dec!(-5) || value > dec!(100) {
            return Err(Error::OutOfRange(format!(
                "Temperature {} °C outside range −5–100",
                value
            )));
        }
        Ok(())
    }

    /// Validate temperature Fahrenheit.
    pub fn temp_f(value: Decimal) -> Result<()> {
        if value < dec!(23) || value > dec!(212) {
            return Err(Error::OutOfRange(format!(
                "Temperature {} °F outside range 23–212",
                value
            )));
        }
        Ok(())
    }

    /// Validate percentage (0–100).
    pub fn percent(value: Decimal) -> Result<()> {
        if value < dec!(0) || value > dec!(100) {
            return Err(Error::OutOfRange(format!(
                "Percentage {} outside range 0–100",
                value
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "validation_tests.rs"]
mod tests;
