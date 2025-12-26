//! Input validation with range checks.

use crate::{Error, Result};
use rust_decimal::Decimal;

/// Range validator with warnings.
pub struct Validator;

impl Validator {
    pub fn sg(value: Decimal) -> Result<()> {
        let min = Decimal::new(6000, 4);
        let max = Decimal::new(20000, 4);
        if value < min || value > max {
            return Err(Error::OutOfRange(format!(
                "SG {} outside range 0.6000–2.0000",
                value
            )));
        }
        Ok(())
    }

    pub fn ph(value: Decimal) -> Result<()> {
        let min = Decimal::new(150, 2);
        let max = Decimal::new(850, 2);
        if value < min || value > max {
            return Err(Error::OutOfRange(format!(
                "pH {} outside range 1.50–8.50",
                value
            )));
        }
        Ok(())
    }

    pub fn brix(value: Decimal) -> Result<()> {
        if value < Decimal::ZERO || value > Decimal::from(70) {
            return Err(Error::OutOfRange(format!(
                "Brix {} outside range 0–70",
                value
            )));
        }
        Ok(())
    }

    pub fn brix_warning(value: Decimal) -> Option<String> {
        if value > Decimal::from(45) {
            Some(format!("Brix {} above typical range (0–45)", value))
        } else {
            None
        }
    }

    pub fn plato(value: Decimal) -> Result<()> {
        if value < Decimal::ZERO || value > Decimal::from(70) {
            return Err(Error::OutOfRange(format!(
                "Plato {} outside range 0–70",
                value
            )));
        }
        Ok(())
    }

    pub fn plato_warning(value: Decimal) -> Option<String> {
        if value > Decimal::from(45) {
            Some(format!("Plato {} above typical range (0–45)", value))
        } else {
            None
        }
    }

    pub fn temp_c(value: Decimal) -> Result<()> {
        if value < Decimal::from(-5) || value > Decimal::from(100) {
            return Err(Error::OutOfRange(format!(
                "Temperature {} °C outside range −5–100",
                value
            )));
        }
        Ok(())
    }

    pub fn temp_f(value: Decimal) -> Result<()> {
        if value < Decimal::from(23) || value > Decimal::from(212) {
            return Err(Error::OutOfRange(format!(
                "Temperature {} °F outside range 23–212",
                value
            )));
        }
        Ok(())
    }

    pub fn percent(value: Decimal) -> Result<()> {
        if value < Decimal::ZERO || value > Decimal::from(100) {
            return Err(Error::OutOfRange(format!(
                "Percentage {} outside range 0–100",
                value
            )));
        }
        Ok(())
    }
}
