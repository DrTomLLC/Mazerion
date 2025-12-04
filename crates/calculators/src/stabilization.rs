use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct StabilizationCalculator;

impl StabilizationCalculator {
    pub const ID: &'static str = "stabilization";
}

impl Calculator for StabilizationCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Stabilization"
    }

    fn category(&self) -> &'static str {
        "Finishing"
    }

    fn description(&self) -> &'static str {
        "Calculate K-meta + sorbate for chemical stabilization"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let ph_meas = input.get_measurement(Unit::Ph);

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;

        // K-meta (potassium metabisulfite): 0.5 g/L standard dose
        let kmeta = vol * Decimal::new(5, 1);

        // Potassium sorbate: 0.75 g/L standard dose
        let sorbate = vol * Decimal::new(75, 2);

        let mut result = CalcResult::new(Measurement::new(kmeta, Unit::Grams));

        result = result
            .with_meta("kmeta_g", format!("{:.1} g", kmeta))
            .with_meta("kmeta_tsp", format!("{:.2} tsp", kmeta / Decimal::new(5, 0)))
            .with_meta("sorbate_g", format!("{:.1} g", sorbate))
            .with_meta("sorbate_tsp", format!("{:.2} tsp", sorbate / Decimal::new(5, 0)))
            .with_meta("volume_L", format!("{} L", vol));

        // pH-dependent warning
        if let Ok(ph) = ph_meas {
            if ph.value > Decimal::new(36, 1) {
                result = result.with_warning("pH > 3.6 - sorbate less effective, may produce geranium off-flavor");
            }
        }

        result = result.with_warning("CRITICAL: Add K-meta 24 hours before sorbate to kill remaining yeast");
        result = result.with_warning("Stabilization prevents re-fermentation for backsweetening");

        Ok(result)
    }
}

register_calculator!(StabilizationCalculator);