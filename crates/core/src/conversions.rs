//! Unit conversion functions for Imperial/Metric

use rust_decimal::Decimal;

/// Convert gallons (US) to liters
pub fn gallons_to_liters(gal: Decimal) -> Decimal {
    gal * Decimal::new(378541, 5) // 3.78541
}

/// Convert liters to gallons (US)
pub fn liters_to_gallons(l: Decimal) -> Decimal {
    l * Decimal::new(264172, 6) // 0.264172
}

/// Convert Fahrenheit to Celsius
pub fn fahrenheit_to_celsius(f: Decimal) -> Decimal {
    (f - Decimal::from(32)) * Decimal::new(5, 0) / Decimal::new(9, 0)
}

/// Convert Celsius to Fahrenheit
pub fn celsius_to_fahrenheit(c: Decimal) -> Decimal {
    (c * Decimal::new(9, 0) / Decimal::new(5, 0)) + Decimal::from(32)
}

/// Convert ounces to grams
pub fn ounces_to_grams(oz: Decimal) -> Decimal {
    oz * Decimal::new(28349523125, 9) // 28.349523125
}

/// Convert grams to ounces
pub fn grams_to_ounces(g: Decimal) -> Decimal {
    g * Decimal::new(35273962, 9) // 0.035273962
}

/// Convert pounds to kilograms
pub fn pounds_to_kilograms(lb: Decimal) -> Decimal {
    lb * Decimal::new(45359237, 8) // 0.45359237
}

/// Convert kilograms to pounds
pub fn kilograms_to_pounds(kg: Decimal) -> Decimal {
    kg * Decimal::new(220462262, 8) // 2.20462262
}

/// Normalize volume to liters based on unit system
pub fn normalize_volume_to_liters(value: Decimal, is_metric: bool) -> Decimal {
    if is_metric {
        value
    } else {
        gallons_to_liters(value)
    }
}

/// Normalize temperature to Celsius based on unit system
pub fn normalize_temp_to_celsius(value: Decimal, is_metric: bool) -> Decimal {
    if is_metric {
        value
    } else {
        fahrenheit_to_celsius(value)
    }
}

/// Normalize weight to grams based on unit system
pub fn normalize_weight_to_grams(value: Decimal, is_metric: bool) -> Decimal {
    if is_metric {
        value
    } else {
        ounces_to_grams(value)
    }
}

/// Convert result volume from liters to user's preferred unit
pub fn display_volume(liters: Decimal, is_metric: bool) -> (Decimal, &'static str) {
    if is_metric {
        (liters, "L")
    } else {
        (liters_to_gallons(liters), "gal")
    }
}