//! Core types for Mazerion beverage calculations.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

pub mod error;
pub mod traits;
pub mod units;
pub mod validation;

pub use error::{Error, Result};
pub use traits::{Calculator, get_calculator, list_calculators, get_all_calculators, CALCULATORS};
pub use units::*;
pub use validation::*;

// Re-export the register_calculator macro

/// Measurement with unit and precision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Measurement {
    pub value: Decimal,
    pub unit: Unit,
}

impl Measurement {
    pub fn new(value: Decimal, unit: Unit) -> Self {
        Self { value, unit }
    }

    pub fn sg(value: Decimal) -> Result<Self> {
        Validator::sg(value)?;
        Ok(Self::new(value, Unit::SpecificGravity))
    }

    pub fn ph(value: Decimal) -> Result<Self> {
        Validator::ph(value)?;
        Ok(Self::new(value, Unit::Ph))
    }

    pub fn brix(value: Decimal) -> Result<Self> {
        Validator::brix(value)?;
        Ok(Self::new(value, Unit::Brix))
    }

    pub fn plato(value: Decimal) -> Result<Self> {
        Validator::plato(value)?;
        Ok(Self::new(value, Unit::Plato))
    }

    pub fn celsius(value: Decimal) -> Result<Self> {
        Validator::temp_c(value)?;
        Ok(Self::new(value, Unit::Celsius))
    }

    pub fn fahrenheit(value: Decimal) -> Result<Self> {
        Validator::temp_f(value)?;
        Ok(Self::new(value, Unit::Fahrenheit))
    }

    pub fn liters(value: Decimal) -> Result<Self> {
        if value < Decimal::ZERO {
            return Err(Error::OutOfRange("Volume cannot be negative".into()));
        }
        Ok(Self::new(value, Unit::Liters))
    }

    pub fn grams(value: Decimal) -> Result<Self> {
        if value < Decimal::ZERO {
            return Err(Error::OutOfRange("Mass cannot be negative".into()));
        }
        Ok(Self::new(value, Unit::Grams))
    }

    pub fn percent(value: Decimal) -> Result<Self> {
        Validator::percent(value)?;
        Ok(Self::new(value, Unit::Percent))
    }
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

/// Calculation result with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalcResult {
    pub output: Measurement,
    pub warnings: Vec<String>,
    pub metadata: Vec<(String, String)>,
}

impl CalcResult {
    pub fn new(output: Measurement) -> Self {
        Self {
            output,
            warnings: Vec::new(),
            metadata: Vec::new(),
        }
    }

    pub fn with_warning(mut self, msg: impl Into<String>) -> Self {
        self.warnings.push(msg.into());
        self
    }

    pub fn with_meta(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.metadata.push((key.into(), val.into()));
        self
    }
}

/// Input parameters for calculations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalcInput {
    pub measurements: Vec<Measurement>,
    pub params: Vec<(String, String)>,
}

impl CalcInput {
    pub fn new() -> Self {
        Self {
            measurements: Vec::new(),
            params: Vec::new(),
        }
    }

    pub fn add_measurement(mut self, m: Measurement) -> Self {
        self.measurements.push(m);
        self
    }

    pub fn add_param(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.params.push((k.into(), v.into()));
        self
    }

    pub fn get_measurement(&self, unit: Unit) -> Result<&Measurement> {
        self.measurements
            .iter()
            .find(|m| m.unit == unit)
            .ok_or(Error::MissingInput(format!("No measurement with unit {}", unit)))
    }

    pub fn get_param(&self, key: &str) -> Option<&str> {
        self.params
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}

impl Default for CalcInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro to register a calculator.
#[macro_export]
macro_rules! register_calculator {
    ($calc:ty) => {
        #[::linkme::distributed_slice($crate::CALCULATORS)]
        static ENTRY: $crate::traits::CalculatorEntry = $crate::traits::CalculatorEntry::new(
            <$calc>::ID,
            || Box::new(<$calc>::default()),
        );
    };
}