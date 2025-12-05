use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

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
        "Calculate ingredients for apple juice mead (cyser)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;
        let juice_percent = input.get_param("juice_percent").unwrap_or("50");

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse()
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;
        let juice_pct: Decimal = juice_percent.parse()
            .map_err(|_| Error::Parse("Invalid juice_percent".into()))?;

        // Apple juice ~10.4% sugar, contributes ~0.6% ABV per 135g/L
        let juice_volume = vol * juice_pct / Decimal::from(100);
        let juice_sugar_per_liter = Decimal::new(104, 0); // 104 g/L
        let total_juice_sugar = juice_volume * juice_sugar_per_liter;

        let juice_abv = total_juice_sugar / (vol * Decimal::from(135));

        let remaining_abv = abv - juice_abv;

        let honey_needed = vol * remaining_abv * Decimal::new(135, 0);
        let water_volume = vol - juice_volume;

        let mut result = CalcResult::new(Measurement::new(honey_needed, Unit::Grams));

        result = result
            .with_meta("juice_volume_L", format!("{:.2} L", juice_volume))
            .with_meta("water_volume_L", format!("{:.2} L", water_volume))
            .with_meta("juice_abv", format!("{:.1}%", juice_abv))
            .with_meta("honey_kg", format!("{:.2} kg", honey_needed / Decimal::from(1000)));

        if juice_pct < Decimal::from(30) {
            result = result.with_warning("Low juice ratio - may lack apple character");
        }
        if juice_pct > Decimal::from(70) {
            result = result.with_warning("High juice ratio - may be more cider than mead");
        }

        Ok(result)
    }
}

register_calculator!(CyserCalculator);