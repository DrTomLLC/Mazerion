//! Oxymel calculator - vinegar/honey beverage

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct OxymelCalculator;

impl OxymelCalculator {
    pub const ID: &'static str = "oxymel";
}

impl Calculator for OxymelCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Oxymel (Vinegar-Honey)"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for oxymel (vinegar-honey beverage)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let vinegar_percent = input.get_param("vinegar_percent")
            .ok_or_else(|| Error::MissingInput("vinegar_percent required".into()))?;
        let honey_percent = input.get_param("honey_percent")
            .ok_or_else(|| Error::MissingInput("honey_percent required".into()))?;

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let vin_pct: Decimal = vinegar_percent.parse()
            .map_err(|_| Error::Parse("Invalid vinegar_percent".into()))?;
        let hon_pct: Decimal = honey_percent.parse()
            .map_err(|_| Error::Parse("Invalid honey_percent".into()))?;

        // Traditional oxymel: 1 part honey to 4 parts vinegar (20% honey, 80% vinegar)
        // Modern variations: 10-30% honey, 20-50% vinegar, rest water

        let vinegar_l = vol * (vin_pct / Decimal::from(100));
        let honey_volume = vol * (hon_pct / Decimal::from(100));

        // Honey density ~1.42 kg/L, 82% sugar
        let honey_kg = honey_volume * Decimal::new(142, 2);
        let honey_g = honey_kg * Decimal::from(1000);

        let water_l = vol - vinegar_l - honey_volume;

        let mut result = CalcResult::new(Measurement::new(honey_g, Unit::Grams));

        // Validate ratios
        if vin_pct + hon_pct > Decimal::from(100) {
            return Err(Error::Validation(
                "Vinegar + honey percentages cannot exceed 100%".into()
            ));
        }

        if vin_pct < Decimal::from(10) {
            result = result.with_warning("Low vinegar (<10%) - may lack characteristic tang");
        }

        if vin_pct > Decimal::from(60) {
            result = result.with_warning("High vinegar (>60%) - may be too acidic");
        }

        if hon_pct < Decimal::from(10) {
            result = result.with_warning("Low honey (<10%) - may lack sweetness and body");
        }

        if hon_pct > Decimal::from(40) {
            result = result.with_warning("High honey (>40%) - may be overly sweet");
        }

        let ratio = if hon_pct > Decimal::ZERO {
            vin_pct / hon_pct
        } else {
            Decimal::from(999)
        };

        let balance = if ratio > Decimal::from(5) {
            "Very Tart (Digestive/Medicinal)"
        } else if ratio > Decimal::from(3) {
            "Tart (Traditional Oxymel)"
        } else if ratio > Decimal::from(2) {
            "Balanced Tart-Sweet"
        } else if ratio > Decimal::ONE {
            "Mildly Tart"
        } else {
            "Sweet-Tart (Modern Style)"
        };

        result = result
            .with_meta("honey_g", format!("{:.0} g", honey_g))
            .with_meta("honey_kg", format!("{:.2} kg", honey_kg))
            .with_meta("vinegar_L", format!("{:.2} L", vinegar_l))
            .with_meta("water_L", format!("{:.2} L", water_l))
            .with_meta("vinegar_percent", format!("{:.0}%", vin_pct))
            .with_meta("honey_percent", format!("{:.0}%", hon_pct))
            .with_meta("ratio", format!("{:.1}:1 (vinegar:honey)", ratio))
            .with_meta("balance", balance)
            .with_meta("tip", "Use quality vinegar (apple cider, wine, or balsamic). Mix honey with water first, then add vinegar.");

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("volume").is_none() {
            return Err(Error::MissingInput("volume required".into()));
        }
        if input.get_param("vinegar_percent").is_none() {
            return Err(Error::MissingInput("vinegar_percent required".into()));
        }
        if input.get_param("honey_percent").is_none() {
            return Err(Error::MissingInput("honey_percent required".into()));
        }
        Ok(())
    }
}

register_calculator!(OxymelCalculator);