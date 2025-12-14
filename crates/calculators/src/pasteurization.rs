//! Pasteurization calculator - SAFETY CRITICAL
//! Time/temperature calculations for safe pasteurization without flavor damage

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct PasteurizationCalculator;

impl PasteurizationCalculator {
    pub const ID: &'static str = "pasteurization";
}

impl Calculator for PasteurizationCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Pasteurization"
    }

    fn category(&self) -> &'static str {
        "Finishing"
    }

    fn description(&self) -> &'static str {
        "Calculate time/temperature for safe pasteurization"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let temp_meas = input.get_measurement(Unit::Celsius)?;
        let temp_c = temp_meas.value;

        // Validate temperature range
        if temp_c < Decimal::from(60) {
            return Err(Error::Validation("Temperature too low (<60°C) - insufficient for pasteurization".into()));
        }
        if temp_c > Decimal::from(75) {
            return Err(Error::Validation("Temperature too high (>75°C) - will damage flavor and aroma".into()));
        }

        // Calculate required time using logarithmic relationship
        // Based on D-value reduction (10°C = 10x time reduction)
        // Reference: 65°C = 30 min, each 1°C increase = ~25% time reduction

        // Time calculation: exponential decay from reference point
        let reference_temp = Decimal::from(65);
        let reference_time = Decimal::from(30); // 30 minutes at 65°C

        let temp_diff = temp_c - reference_temp;

        // For each degree above 65°C, reduce time by ~25%
        // Formula: time = reference_time * 0.75^(temp_diff)
        let temp_diff_f64 = temp_diff.to_string().parse::<f64>().unwrap_or(0.0);
        let time_factor = 0.75_f64.powf(temp_diff_f64);
        let time_minutes = reference_time * Decimal::from_f64_retain(time_factor).unwrap_or(Decimal::ONE);

        // Get safe time ranges
        let (min_time, max_time, recommendation) = if temp_c >= Decimal::from(72) {
            (Decimal::new(5, 0), Decimal::from(10), "Short high-temp pasteurization - minimal flavor impact")
        } else if temp_c >= Decimal::from(68) {
            (Decimal::from(12), Decimal::from(20), "Medium-temp pasteurization - good balance")
        } else if temp_c >= Decimal::from(65) {
            (Decimal::from(25), Decimal::from(35), "Standard pasteurization temperature")
        } else if temp_c >= Decimal::from(62) {
            (Decimal::from(40), Decimal::from(60), "Low-temp pasteurization - best for delicate flavors")
        } else {
            (Decimal::from(60), Decimal::from(90), "Very low temp - extended time required")
        };

        let temp_f = (temp_c * Decimal::new(9, 0) / Decimal::new(5, 0)) + Decimal::from(32);

        let mut result = CalcResult::new(Measurement::new(time_minutes, Unit::Grams)); // Using Grams as generic unit for minutes

        result = result
            .with_meta("temperature_c", format!("{:.1}°C", temp_c))
            .with_meta("temperature_f", format!("{:.1}°F", temp_f))
            .with_meta("calculated_time_min", format!("{:.0}", time_minutes))
            .with_meta("safe_time_range_min", format!("{:.0}-{:.0} minutes", min_time, max_time))
            .with_meta("recommendation", recommendation)
            .with_meta("method", "Bottle pasteurization (water bath)")
            .with_meta("tip", "Use thermometer - monitor actual bottle temperature, not water temp");

        // Warnings
        if temp_c > Decimal::from(70) {
            result = result.with_warning("High temp (>70°C) - monitor closely to avoid overcooking flavors");
        }

        if temp_c < Decimal::from(63) {
            result = result.with_warning("Low temp (<63°C) - extended time required, ensure full pasteurization");
        }

        if time_minutes < Decimal::from(5) {
            result = result.with_warning("Very short time (<5 min) - difficult to maintain consistent temperature");
        }

        if time_minutes > Decimal::from(60) {
            result = result.with_warning("Long time (>60 min) - consider higher temperature for efficiency");
        }

        result = result.with_warning("CRITICAL: Bottles must reach target temperature throughout - not just surface");
        result = result.with_warning("Start timing AFTER bottles reach target temperature");

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        input.get_measurement(Unit::Celsius)?;
        Ok(())
    }
}

register_calculator!(PasteurizationCalculator);