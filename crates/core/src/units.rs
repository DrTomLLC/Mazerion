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
    Ounces,
    Pounds,
    Liters,
    Milliliters,
    Gallons,
    Quarts,
    Pints,
    FluidOunces,
    Abv,
    Ppm,
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
            Self::Grams | Self::Ounces | Self::Pounds => 2,
            Self::Liters | Self::Milliliters | Self::Gallons | Self::Quarts | Self::Pints | Self::FluidOunces => 2,
            Self::Ppm => 1,
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
            Self::Ounces => "oz",
            Self::Pounds => "lb",
            Self::Liters => "L",
            Self::Milliliters => "mL",
            Self::Gallons => "gal",
            Self::Quarts => "qt",
            Self::Pints => "pt",
            Self::FluidOunces => "fl oz",
            Self::Ppm => "ppm",
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}