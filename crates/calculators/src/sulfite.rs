use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct SulfiteCalculator;

impl SulfiteCalculator {
    pub const ID: &'static str = "sulfite";
}

impl Calculator for SulfiteCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Sulfite Calculator"
    }

    fn category(&self) -> &'static str {
        "Finishing"
    }

    fn description(&self) -> &'static str {
        "Calculate K-meta additions with pH-dependent effectiveness"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let _ph_meas = input.get_measurement(Unit::Ph)?;
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target = input.get_param("target_free_so2")
            .ok_or_else(|| Error::MissingInput("target_free_so2 required".into()))?;

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let targ: Decimal = target.parse()
            .map_err(|_| Error::Parse("Invalid target".into()))?;

        let kmeta = vol * targ * Decimal::new(2, 1) / Decimal::from(1000);

        Ok(CalcResult::new(Measurement::new(kmeta, Unit::Grams))
            .with_meta("volume", volume)
            .with_meta("target_so2", target))
    }
}

register_calculator!(SulfiteCalculator);