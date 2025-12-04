use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct BraggotCalculator;

impl BraggotCalculator {
    pub const ID: &'static str = "braggot";
}

impl Calculator for BraggotCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Braggot Calculator"
    }

    fn category(&self) -> &'static str {
        "Mead Styles"
    }

    fn description(&self) -> &'static str {
        "Calculate ingredients for honey-malt hybrid (braggot)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;
        let honey_percent = input.get_param("honey_percent").unwrap_or("50");
        let malt_weight = input.get_param("malt_weight").unwrap_or("0");

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse()
            .map_err(|_| Error::Parse("Invalid target_abv".into()))?;
        let honey_pct: Decimal = honey_percent.parse()
            .map_err(|_| Error::Parse("Invalid honey_percent".into()))?;
        let malt_kg: Decimal = malt_weight.parse()
            .map_err(|_| Error::Parse("Invalid malt_weight".into()))?;

        // Honey: 135 g/L/%ABV, Malt: ~140 g/L/%ABV (varies by efficiency)
        let honey_abv = abv * honey_pct / Decimal::from(100);
        let malt_abv = abv - honey_abv;

        let honey_needed = vol * honey_abv * Decimal::new(135, 0);
        let calculated_malt = vol * malt_abv * Decimal::new(140, 0) / Decimal::from(1000); // kg

        let malt_to_use = if malt_kg > Decimal::ZERO {
            malt_kg
        } else {
            calculated_malt
        };

        let mut result = CalcResult::new(Measurement::new(honey_needed, Unit::Grams));

        result = result
            .with_meta("honey_kg", format!("{:.2} kg", honey_needed / Decimal::from(1000)))
            .with_meta("malt_kg", format!("{:.2} kg", malt_to_use))
            .with_meta("honey_abv", format!("{:.1}%", honey_abv))
            .with_meta("malt_abv", format!("{:.1}%", malt_abv));

        if honey_pct < Decimal::from(30) {
            result = result.with_warning("Low honey percentage - may be more beer than braggot");
        }

        Ok(result)
    }
}

register_calculator!(BraggotCalculator);