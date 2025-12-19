use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Default)]
pub struct CyserCalculator;

impl CyserCalculator {
    pub const ID: &'static str = "cyser";
}

impl Calculator for CyserCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Cyser Calculator"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for apple mead with juice sugar contribution"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;
        let juice_percent = input.get_param("juice_percent")
            .ok_or_else(|| Error::MissingInput("juice_percent required".into()))?;

        let vol: Decimal = Decimal::from_str(volume)
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = Decimal::from_str(target_abv)
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;
        let juice_pct: Decimal = Decimal::from_str(juice_percent)
            .map_err(|_| Error::Parse("Invalid juice_percent".into()))?;

        let juice_volume = vol * juice_pct / Decimal::from(100);
        let juice_sugar_g = juice_volume * Decimal::from(104); // Apple juice ~104 g/L sugar

        // FIXED: 33 g per L per % ABV
        let total_sugar_g = vol * abv * Decimal::from(33);
        let honey_sugar_g = if total_sugar_g > juice_sugar_g {
            total_sugar_g - juice_sugar_g
        } else {
            Decimal::ZERO
        };

        let mut result = CalcResult::new(Measurement::new(honey_sugar_g, Unit::Grams));

        result = result
            .with_meta("juice_volume_L", format!("{:.2} L", juice_volume))
            .with_meta("juice_sugar_g", format!("{:.0} g", juice_sugar_g))
            .with_meta("honey_kg", format!("{:.2} kg", honey_sugar_g / Decimal::from(1000)));

        Ok(result)
    }
}

register_calculator!(CyserCalculator);