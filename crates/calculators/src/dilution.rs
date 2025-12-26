use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct DilutionCalculator;

impl DilutionCalculator {
    pub const ID: &'static str = "dilution";
}

impl Calculator for DilutionCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Dilution Calculator"
    }

    fn category(&self) -> &'static str {
        "Advanced"
    }

    fn description(&self) -> &'static str {
        "Calculate water needed to dilute to target ABV"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("current_volume")
            .ok_or_else(|| Error::MissingInput("current_volume required".into()))?;
        let current_abv = input
            .get_param("current_abv")
            .ok_or_else(|| Error::MissingInput("current_abv required".into()))?;
        let target_abv = input
            .get_param("target_abv")
            .ok_or_else(|| Error::MissingInput("target_abv required".into()))?;

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let curr_abv: Decimal = current_abv
            .parse()
            .map_err(|_| Error::Parse("Invalid current ABV".into()))?;
        let targ_abv: Decimal = target_abv
            .parse()
            .map_err(|_| Error::Parse("Invalid target ABV".into()))?;

        if vol <= Decimal::ZERO {
            return Err(Error::Validation(
                "Current volume must be positive".into(),
            ));
        }

        if targ_abv >= curr_abv {
            return Err(Error::Validation(
                "Target ABV must be less than current ABV".into(),
            ));
        }

        if targ_abv == Decimal::ZERO {
            return Err(Error::Validation(
                "Cannot dilute to 0% ABV - mathematically impossible".into(),
            ));
        }

        let water_needed = vol * ((curr_abv - targ_abv) / targ_abv);

        Ok(
            CalcResult::new(Measurement::new(water_needed, Unit::Liters))
                .with_meta("current_volume", volume)
                .with_meta("current_abv", current_abv)
                .with_meta("target_abv", target_abv),
        )
    }
}

register_calculator!(DilutionCalculator);