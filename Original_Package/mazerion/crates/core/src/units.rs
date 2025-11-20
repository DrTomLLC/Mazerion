//! Units of measurement.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Unit {
    SpecificGravity,
    Ph,
    Brix,
    Plato,
    Celsius,
    Fahrenheit,
    Percent,
    Grams,
    Liters,
    Milliliters,
    Abv,
}

impl Unit {
    /// Decimal precision for display.
    pub fn precision(self) -> u32 {
        match self {
            Self::SpecificGravity => 4,
            Self::Ph => 3,
            Self::Brix | Self::Plato => 2,
            Self::Celsius | Self::Fahrenheit => 1,
            Self::Percent | Self::Abv => 2,
            Self::Grams | Self::Liters | Self::Milliliters => 2,
        }
    }

    /// Unit symbol.
    pub fn symbol(self) -> &'static str {
        match self {
            Self::SpecificGravity => "SG",
            Self::Ph => "pH",
            Self::Brix => "°Bx",
            Self::Plato => "°P",
            Self::Celsius => "°C",
            Self::Fahrenheit => "°F",
            Self::Percent => "%",
            Self::Abv => "% ABV",
            Self::Grams => "g",
            Self::Liters => "L",
            Self::Milliliters => "mL",
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}
