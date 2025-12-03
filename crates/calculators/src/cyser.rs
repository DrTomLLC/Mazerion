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
        "Calculate ingredients for apple mead (cyser)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_abv = input.get_param("target_abv").unwrap_or("12");
        let juice_percentage = input.get_param("juice_percentage").unwrap_or("50");

        let vol: Decimal = volume.parse().map_err(|_| Error::Parse("Invalid volume".into()))?;
        let abv: Decimal = target_abv.parse().map_err(|_| Error::Parse("Invalid target_abv".into()))?;
        let juice_pct: Decimal = juice_percentage.parse().map_err(|_| Error::Parse("Invalid juice_percentage".into()))?;

        let juice_ratio = juice_pct / Decimal::from(100);
        let juice_volume = vol * juice_ratio;

        // Apple juice contributes ~10.4% sugar (SG ~1.045)
        let juice_sugar_kg = juice_volume * Decimal::new(104, 3); // 0.104 kg/L

        let honey_g_per_l_per_abv = Decimal::from(135);
        let total_sugar_needed = vol * abv * honey_g_per_l_per_abv / Decimal::from(1000);
        let honey_needed = (total_sugar_needed - juice_sugar_kg).max(Decimal::ZERO) * Decimal::from(1000);

        let mut result = CalcResult::new(Measurement::new(honey_needed / Decimal::from(1000), Unit::Grams));

        result = result
            .with_meta("honey", format!("{:.2} kg", honey_needed / Decimal::from(1000)))
            .with_meta("apple_juice", format!("{:.2} L", juice_volume))
            .with_meta("target_abv", format!("{}%", abv));

        Ok(result)
    }
}

register_calculator!(CyserCalculator);