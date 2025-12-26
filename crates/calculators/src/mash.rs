//! Mash water calculator for strike water temperature and volume
//! Uses standard brewing equations

use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct MashCalculator;

impl MashCalculator {
    pub const ID: &'static str = "mash";
}

impl Calculator for MashCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Mash Water Calculator"
    }

    fn category(&self) -> &'static str {
        "Beer"
    }

    fn description(&self) -> &'static str {
        "Calculate strike water temperature and volume for mash"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let target_temp = input
            .get_param("target_temp")
            .ok_or_else(|| Error::MissingInput("target_temp required".into()))?;
        let grain_temp = input
            .get_param("grain_temp")
            .ok_or_else(|| Error::MissingInput("grain_temp required".into()))?;
        let grain_weight = input
            .get_param("grain_weight")
            .ok_or_else(|| Error::MissingInput("grain_weight required".into()))?;
        let ratio = input
            .get_param("ratio")
            .ok_or_else(|| Error::MissingInput("ratio required".into()))?;

        let target: Decimal = target_temp
            .parse()
            .map_err(|_| Error::Parse("Invalid target temp".into()))?;
        let grain_t: Decimal = grain_temp
            .parse()
            .map_err(|_| Error::Parse("Invalid grain temp".into()))?;
        let weight: Decimal = grain_weight
            .parse()
            .map_err(|_| Error::Parse("Invalid grain weight".into()))?;
        let ratio_val: Decimal = ratio
            .parse()
            .map_err(|_| Error::Parse("Invalid ratio".into()))?;

        if weight <= Decimal::ZERO {
            return Err(Error::Validation("Grain weight must be positive".into()));
        }
        if ratio_val <= Decimal::ZERO {
            return Err(Error::Validation("Ratio must be positive".into()));
        }

        // Water volume = grain_weight × ratio
        let water_volume = weight * ratio_val;

        // Strike temp = (0.2/ratio) × (target - grain) + target
        // Simplified: strike = target + (0.2/ratio) × (target - grain)
        let temp_diff = target - grain_t;
        let thermal_constant = Decimal::new(2, 1) / ratio_val; // 0.2 / ratio
        let strike_temp = target + (thermal_constant * temp_diff);

        let mut result = CalcResult::new(Measurement::new(strike_temp, Unit::Celsius))
            .with_meta(
                "strike_temperature",
                format!(
                    "{:.1}°C / {:.1}°F",
                    strike_temp,
                    strike_temp * Decimal::new(9, 0) / Decimal::new(5, 0) + Decimal::from(32)
                ),
            )
            .with_meta(
                "water_volume",
                format!(
                    "{:.2} L / {:.2} gal",
                    water_volume,
                    water_volume * Decimal::new(264172, 6)
                ),
            )
            .with_meta("mash_ratio", format!("{:.2} L/kg", ratio_val))
            .with_meta("target_mash_temp", format!("{:.1}°C", target))
            .with_meta("grain_temperature", format!("{:.1}°C", grain_t));

        if strike_temp > Decimal::from(80) {
            result = result.with_warning("Strike temp >80°C may extract tannins");
        }
        if strike_temp < Decimal::from(50) {
            result = result.with_warning("Strike temp <50°C may be too cold");
        }
        if ratio_val < Decimal::new(15, 1) {
            result = result.with_warning("Ratio <1.5 L/kg - very thick mash");
        }
        if ratio_val > Decimal::from(4) {
            result = result.with_warning("Ratio >4 L/kg - very thin mash");
        }

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("target_temp").is_none() {
            return Err(Error::MissingInput("target_temp required".into()));
        }
        if input.get_param("grain_temp").is_none() {
            return Err(Error::MissingInput("grain_temp required".into()));
        }
        if input.get_param("grain_weight").is_none() {
            return Err(Error::MissingInput("grain_weight required".into()));
        }
        if input.get_param("ratio").is_none() {
            return Err(Error::MissingInput("ratio required".into()));
        }
        Ok(())
    }
}

register_calculator!(MashCalculator);
