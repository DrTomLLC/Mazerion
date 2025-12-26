//! Pyment calculator - grape-honey wine (wine-mead hybrid)

use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct PymentCalculator;

impl PymentCalculator {
    pub const ID: &'static str = "pyment";
}

impl Calculator for PymentCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Pyment (Grape-Honey Wine)"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for pyment (grape-honey wine)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input
            .get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;
        let juice_percent = input.get_param("juice_percent").unwrap_or("40"); // Default 40% grape juice

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv
            .parse()
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;
        let juice_pct: Decimal = juice_percent
            .parse()
            .map_err(|_| Error::Parse("Invalid juice_percent".into()))?;

        // Formula: 33 g honey per L per % ABV (corrected from 135)
        let g_per_l_per_abv = Decimal::from(33);

        // Calculate juice volume
        let juice_vol_l = vol * (juice_pct / Decimal::from(100));

        // Grape juice typically provides ~10-13% potential ABV per liter
        // Using 12% as average
        let juice_sg_contribution = Decimal::new(12, 0);
        let abv_from_juice = juice_sg_contribution;

        // Remaining ABV must come from honey
        let abv_from_honey = if abv > abv_from_juice {
            abv - abv_from_juice
        } else {
            Decimal::ZERO
        };

        // Calculate honey needed
        let honey_needed_g = vol * abv_from_honey * g_per_l_per_abv;

        let mut result = CalcResult::new(Measurement::new(honey_needed_g, Unit::Grams));

        if abv < Decimal::from(8) {
            result = result
                .with_warning("Low ABV (<8%) - consider increasing target or juice percentage");
        }

        if abv > Decimal::from(18) {
            result = result.with_warning("Very high ABV (>18%) - may require strong yeast strain");
        }

        if juice_pct < Decimal::from(30) {
            result = result.with_warning("Low juice percentage (<30%) - may lack grape character");
        }

        if juice_pct > Decimal::from(60) {
            result = result.with_warning("High juice percentage (>60%) - may lack honey character");
        }

        let honey_kg = honey_needed_g / Decimal::from(1000);

        result = result
            .with_meta("honey_g", format!("{:.0} g", honey_needed_g))
            .with_meta("honey_kg", format!("{:.2} kg", honey_kg))
            .with_meta("juice_volume_L", format!("{:.2} L", juice_vol_l))
            .with_meta("juice_percent", format!("{:.0}%", juice_pct))
            .with_meta("abv_from_juice", format!("{:.1}%", abv_from_juice))
            .with_meta("abv_from_honey", format!("{:.1}%", abv_from_honey))
            .with_meta("total_abv", format!("{:.1}%", abv))
            .with_meta("volume", format!("{:.2} L", vol))
            .with_meta(
                "tip",
                "Use quality grape juice or wine must. Red or white grapes both work.",
            );

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("volume").is_none() {
            return Err(Error::MissingInput("volume required".into()));
        }
        if input.get_param("target_abv").is_none() {
            return Err(Error::MissingInput("target_abv required".into()));
        }
        Ok(())
    }
}

register_calculator!(PymentCalculator);
