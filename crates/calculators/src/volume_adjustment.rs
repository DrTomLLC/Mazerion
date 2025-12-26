use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct VolumeAdjustmentCalculator;

impl VolumeAdjustmentCalculator {
    pub const ID: &'static str = "volume_adjustment";
}

impl Calculator for VolumeAdjustmentCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Volume Adjustment"
    }

    fn category(&self) -> &'static str {
        "Advanced"
    }

    fn description(&self) -> &'static str {
        "Calculate volume adjustments for target gravity (dilution or concentration)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let current_volume = input
            .get_param("current_volume")
            .ok_or_else(|| Error::MissingInput("current_volume required".into()))?;
        let current_gravity = input
            .get_param("current_gravity")
            .ok_or_else(|| Error::MissingInput("current_gravity required".into()))?;
        let target_gravity = input
            .get_param("target_gravity")
            .ok_or_else(|| Error::MissingInput("target_gravity required".into()))?;

        let vol: Decimal = current_volume
            .parse()
            .map_err(|_| Error::Parse("Invalid current_volume".into()))?;
        let curr_grav: Decimal = current_gravity
            .parse()
            .map_err(|_| Error::Parse("Invalid current_gravity".into()))?;
        let targ_grav: Decimal = target_gravity
            .parse()
            .map_err(|_| Error::Parse("Invalid target_gravity".into()))?;

        // Only support DILUTION (adding water to reduce gravity)
        // Concentration (boiling off water) is a different process
        if targ_grav >= curr_grav {
            return Err(Error::Validation(
                "Target gravity must be less than current gravity. Use boiling to concentrate wort, not this calculator.".into(),
            ));
        }

        // Dilution formula: V1 × G1 = V2 × G2
        // V2 = (V1 × G1) / G2
        let final_volume = (vol * curr_grav) / targ_grav;
        let water_to_add = final_volume - vol;

        let mut result = CalcResult::new(Measurement::new(water_to_add, Unit::Liters));

        if water_to_add > vol {
            result =
                result.with_warning("Adding more water than original volume - double-check target");
        }

        result = result
            .with_meta("water_to_add", format!("{:.2} L", water_to_add))
            .with_meta("final_volume", format!("{:.2} L", final_volume))
            .with_meta("current_gravity", current_gravity)
            .with_meta("target_gravity", target_gravity);

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("current_volume").is_none() {
            return Err(Error::MissingInput("current_volume required".into()));
        }
        if input.get_param("current_gravity").is_none() {
            return Err(Error::MissingInput("current_gravity required".into()));
        }
        if input.get_param("target_gravity").is_none() {
            return Err(Error::MissingInput("target_gravity required".into()));
        }
        Ok(())
    }
}

register_calculator!(VolumeAdjustmentCalculator);
