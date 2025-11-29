use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct AcidAdditionCalculator;

impl AcidAdditionCalculator {
    pub const ID: &'static str = "acid_addition";
}

impl Calculator for AcidAdditionCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Acid Addition"
    }

    fn category(&self) -> &'static str {
        "Finishing"
    }

    fn description(&self) -> &'static str {
        "Calculate acid additions to adjust pH"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let ph_meas = input.get_measurement(Unit::Ph)?;
        let volume = input.get_param("volume").ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_ph = input.get_param("target_ph").ok_or_else(|| Error::MissingInput("target_ph required".into()))?;

        let vol: Decimal = volume.parse().map_err(|_| Error::Parse("Invalid volume".into()))?;
        let curr_ph = ph_meas.value;
        let targ_ph: Decimal = target_ph.parse().map_err(|_| Error::Parse("Invalid target_ph".into()))?;

        let ph_diff = curr_ph - targ_ph;
        let acid_needed = vol * ph_diff * Decimal::new(15, 1);

        Ok(CalcResult::new(Measurement::new(acid_needed, Unit::Grams))
            .with_meta("current_ph", curr_ph.to_string())
            .with_meta("target_ph", target_ph))
    }
}

register_calculator!(AcidAdditionCalculator);