//! Core types for Mazerion MCL - Production Ready
//! SAFETY-CRITICAL: Zero panics, comprehensive error handling

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

pub mod error;
pub mod traits;
pub mod units;
pub mod validation;

#[cfg(test)]
mod calc_input_tests;
#[cfg(test)]
mod calc_result_tests;
#[cfg(test)]
mod units_tests;
#[cfg(test)]
mod validation_tests;

pub use error::{Error, Result};
pub use traits::{
    Calculator, calculator_count, get_all_calculators, get_calculator, list_calculator_ids,
};
pub use units::*;
pub use validation::*;

pub use linkme;
pub use traits::CALCULATORS;

/// Valid calculator categories (enforced at runtime)
pub const VALID_CATEGORIES: &[&str] = &[
    "Basic",
    "Advanced",
    "Brewing",
    "Beer",
    "Finishing",
    "Mead Styles",
    "Utilities",
];

/// Validate that a category string is valid
pub fn validate_category(category: &str) -> Result<()> {
    if VALID_CATEGORIES.contains(&category) {
        Ok(())
    } else {
        Err(Error::Validation(format!(
            "Invalid category '{}'. Must be one of: {}",
            category,
            VALID_CATEGORIES.join(", ")
        )))
    }
}

/// Get all calculators organized by category
pub fn get_calculators_by_category() -> HashMap<String, Vec<Box<dyn Calculator>>> {
    let mut by_category: HashMap<String, Vec<Box<dyn Calculator>>> = HashMap::new();

    for calc in get_all_calculators() {
        let category = calc.category().to_string();
        by_category.entry(category).or_default().push(calc);
    }

    by_category
}

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
            .ok_or(Error::MissingInput(format!(
                "No measurement with unit {}",
                unit
            )))
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

// ═══════════════════════════════════════════════════════════════════════
// UNIT CONVERSION FUNCTIONS (GUI SUPPORT)
// ═══════════════════════════════════════════════════════════════════════

/// Convert US gallons to liters
pub fn gallons_to_liters(gallons: Decimal) -> Decimal {
    gallons * Decimal::new(378541, 5) // 3.78541
}

/// Convert liters to US gallons
pub fn liters_to_gallons(liters: Decimal) -> Decimal {
    liters * Decimal::new(264172, 6) // 0.264172
}

/// Convert pounds to kilograms
pub fn pounds_to_kilograms(pounds: Decimal) -> Decimal {
    pounds * Decimal::new(453592, 6) // 0.453592
}

/// Convert kilograms to pounds
pub fn kilograms_to_pounds(kilograms: Decimal) -> Decimal {
    kilograms * Decimal::new(2204622, 6) // 2.204622
}

/// Convert ounces to grams
pub fn ounces_to_grams(ounces: Decimal) -> Decimal {
    ounces * Decimal::new(2835, 2) // 28.35
}

/// Convert grams to ounces
pub fn grams_to_ounces(grams: Decimal) -> Decimal {
    grams * Decimal::new(353, 5) // 0.0353
}

/// Convert Celsius to Fahrenheit
pub fn celsius_to_fahrenheit(celsius: Decimal) -> Decimal {
    (celsius * Decimal::new(9, 0) / Decimal::new(5, 0)) + Decimal::from(32)
}

/// Convert Fahrenheit to Celsius
pub fn fahrenheit_to_celsius(fahrenheit: Decimal) -> Decimal {
    (fahrenheit - Decimal::from(32)) * Decimal::new(5, 0) / Decimal::new(9, 0)
}
