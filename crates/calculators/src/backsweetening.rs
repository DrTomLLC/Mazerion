use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct BacksweeteningCalculator;

impl BacksweeteningCalculator {
    pub const ID: &'static str = "backsweetening";
}

impl Calculator for BacksweeteningCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Backsweetening"
    }

    fn category(&self) -> &'static str {
        "Finishing"
    }

    fn description(&self) -> &'static str {
        "Calculate sweetener needed to reach target gravity"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let current_sg_meas = input.get_measurement(Unit::SpecificGravity)?;
        let volume = input.get_param("volume").ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_sg = input.get_param("target_sg").ok_or_else(|| Error::MissingInput("target_sg required".into()))?;

        let vol: Decimal = volume.parse().map_err(|_| Error::Parse("Invalid volume".into()))?;
        let curr_sg = current_sg_meas.value;
        let targ_sg: Decimal = target_sg.parse().map_err(|_| Error::Parse("Invalid target_sg".into()))?;

        let sg_diff = targ_sg - curr_sg;
        let honey_needed = vol * sg_diff * Decimal::from(1000);

        Ok(CalcResult::new(Measurement::new(honey_needed, Unit::Grams))
            .with_meta("current_sg", curr_sg.to_string())
            .with_meta("target_sg", target_sg))
    }
}

register_calculator!(BacksweeteningCalculator);