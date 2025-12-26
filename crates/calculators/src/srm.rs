//! SRM (Standard Reference Method) color calculator
//! Morey equation: SRM = 1.4922 × (MCU^0.6859)

use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct SrmCalculator;

impl SrmCalculator {
    pub const ID: &'static str = "srm";
}

impl Calculator for SrmCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "SRM Color Calculator"
    }

    fn category(&self) -> &'static str {
        "Beer"
    }

    fn description(&self) -> &'static str {
        "Calculate beer color using Morey equation"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let grain_weight = input
            .get_param("grain_weight")
            .ok_or_else(|| Error::MissingInput("grain_weight required".into()))?;
        let lovibond = input
            .get_param("lovibond")
            .ok_or_else(|| Error::MissingInput("lovibond required".into()))?;
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;

        let weight: Decimal = grain_weight
            .parse()
            .map_err(|_| Error::Parse("Invalid grain weight".into()))?;
        let color: Decimal = lovibond
            .parse()
            .map_err(|_| Error::Parse("Invalid lovibond".into()))?;
        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;

        if weight <= Decimal::ZERO {
            return Err(Error::Validation("Grain weight must be positive".into()));
        }
        if vol <= Decimal::ZERO {
            return Err(Error::Validation("Volume must be positive".into()));
        }
        if color < Decimal::ZERO {
            return Err(Error::Validation("Lovibond cannot be negative".into()));
        }

        // MCU = (grain_lbs × lovibond) / volume_gal
        let mcu = (weight * color) / vol;

        // Morey: SRM = 1.4922 × (MCU^0.6859)
        let mcu_f64 = mcu.to_string().parse::<f64>().unwrap_or(0.0);
        let srm_f64 = 1.4922 * mcu_f64.powf(0.6859);
        let srm = Decimal::from_f64_retain(srm_f64).unwrap_or(Decimal::ZERO);

        let color_desc = match srm_f64 as i32 {
            0..=3 => "Pale Straw",
            4..=6 => "Straw to Pale Gold",
            7..=9 => "Deep Gold to Pale Amber",
            10..=13 => "Amber",
            14..=17 => "Deep Amber to Copper",
            18..=20 => "Copper to Light Brown",
            21..=24 => "Brown",
            25..=30 => "Dark Brown",
            31..=40 => "Very Dark Brown",
            _ => "Black",
        };

        let mut result = CalcResult::new(Measurement::new(srm, Unit::Percent))
            .with_meta("srm", format!("{:.1}", srm))
            .with_meta("mcu", format!("{:.2}", mcu))
            .with_meta("color_description", color_desc)
            .with_meta("formula", "Morey equation");

        if srm > Decimal::from(40) {
            result = result.with_warning("SRM >40 - beer will appear black");
        }

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("grain_weight").is_none() {
            return Err(Error::MissingInput("grain_weight required".into()));
        }
        if input.get_param("lovibond").is_none() {
            return Err(Error::MissingInput("lovibond required".into()));
        }
        if input.get_param("volume").is_none() {
            return Err(Error::MissingInput("volume required".into()));
        }
        Ok(())
    }
}

register_calculator!(SrmCalculator);
