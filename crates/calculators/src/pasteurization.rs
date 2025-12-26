//! Pasteurization calculator - PU-based bottle pasteurization

use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

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
        "Pasteurization (PU-based)"
    }

    fn category(&self) -> &'static str {
        "Finishing"
    }

    fn description(&self) -> &'static str {
        "Calculate in-bottle pasteurization time using Pasteurization Units"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let temp_c = input
            .get_param("temperature")
            .ok_or_else(|| Error::MissingInput("temperature required (Celsius)".into()))?;
        let target_pu = input.get_param("target_pu").unwrap_or("50");

        let temp: Decimal = temp_c
            .parse()
            .map_err(|_| Error::Parse("Invalid temperature".into()))?;
        let pu_target: Decimal = target_pu
            .parse()
            .map_err(|_| Error::Parse("Invalid target_pu".into()))?;

        // Validate temperature range
        if temp < Decimal::from(60) {
            return Err(Error::Validation(
                "Temperature too low for pasteurization (min 60°C / 140°F)".into(),
            ));
        }
        if temp > Decimal::from(75) {
            return Err(Error::Validation(
                "Temperature too high - risk of flavor damage and bottle bombs (max 75°C / 167°F)"
                    .into(),
            ));
        }

        // Calculate lethal rate: PU/min = 1.393^(T_C - 60)
        // Using ln for decimal exponentiation: a^b = e^(b * ln(a))
        let base = Decimal::new(1393, 3); // 1.393
        let exponent = temp - Decimal::from(60);

        // Convert to f64 for calculation, then back to Decimal
        let base_f64 = base.to_f64().unwrap_or(1.393);
        let exp_f64 = exponent.to_f64().unwrap_or(0.0);
        let lethal_rate_f64 = base_f64.powf(exp_f64);
        let lethal_rate = Decimal::from_f64_retain(lethal_rate_f64).unwrap_or(Decimal::ONE);

        // Calculate hold time: t = target_PU / lethal_rate
        let hold_time_min = pu_target / lethal_rate;

        // Convert to F for display
        let temp_f = temp * Decimal::new(9, 0) / Decimal::from(5) + Decimal::from(32);

        let mut result = CalcResult::new(Measurement::new(hold_time_min, Unit::Grams));

        result = result
            .with_meta("hold_time_min", format!("{:.2} minutes", hold_time_min))
            .with_meta(
                "hold_time_sec",
                format!("{:.0} seconds", hold_time_min * Decimal::from(60)),
            )
            .with_meta("temperature_c", format!("{:.1}°C", temp))
            .with_meta("temperature_f", format!("{:.1}°F", temp_f))
            .with_meta("target_pu", format!("{} PU", pu_target))
            .with_meta("lethal_rate", format!("{:.2} PU/min", lethal_rate))
            .with_meta(
                "method",
                "In-bottle pasteurization (PU-based hot water bath)",
            )
            .with_meta(
                "calculation",
                format!(
                    "PU = t × 1.393^(T-60) = {:.1} × {:.2} = {:.1}",
                    hold_time_min,
                    lethal_rate,
                    hold_time_min * lethal_rate
                ),
            );

        // Critical warnings
        result = result.with_warning("⚠️ CRITICAL: Hold time starts when INTERNAL LIQUID reaches target temp (use sacrificial/probed bottle)");
        result = result.with_warning("Use champagne bottles or bottles rated for pasteurization");
        result = result.with_warning("Monitor temperature closely - exceeding temp risks flavor damage and bottle explosions");

        // Purpose clarification
        result = result.with_meta("purpose", "Beverage stabilization: primarily yeast control to prevent refermentation/over-carbonation");
        result = result.with_meta(
            "safety_note",
            "Reduces spoilage microbes (NOT sterilization)",
        );

        // Common PU targets
        if pu_target == Decimal::from(30) {
            result = result.with_meta(
                "pu_level",
                "30 PU: Light pasteurization (minimal yeast control)",
            );
        } else if pu_target == Decimal::from(50) {
            result = result.with_meta(
                "pu_level",
                "50 PU: Standard pasteurization (good yeast control)",
            );
        } else if pu_target >= Decimal::from(76) {
            result = result.with_meta(
                "pu_level",
                "76+ PU: Heavy pasteurization (maximum yeast kill)",
            );
        }

        // Temperature-specific warnings
        if temp >= Decimal::from(70) {
            result = result.with_warning(
                "High temperature (≥70°C) - watch for caramelization and off-flavors",
            );
        }

        if hold_time_min > Decimal::from(30) {
            result = result
                .with_warning("Long hold time - consider higher temperature for shorter duration");
        } else if hold_time_min < Decimal::from(5) {
            result =
                result.with_warning("Very short hold time - ensure accurate temperature control");
        }

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("temperature").is_none() {
            return Err(Error::MissingInput(
                "temperature required (in Celsius)".into(),
            ));
        }
        Ok(())
    }
}

register_calculator!(PasteurizationCalculator);
