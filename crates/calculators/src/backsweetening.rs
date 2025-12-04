// Calculate sweetener additions for backsweetening to target gravity.

use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

/// Calculate honey or other sweetener needed to reach target gravity.
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
        // Get current SG from measurement
        let current_sg_meas = input.get_measurement(Unit::SpecificGravity)?;
        let curr_sg = current_sg_meas.value;

        // Get parameters - GUI sends "volume" (in LITERS) and "target_sg"
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_sg = input
            .get_param("target_sg")
            .ok_or_else(|| Error::MissingInput("target_sg required".into()))?;

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let targ_sg: Decimal = target_sg
            .parse()
            .map_err(|_| Error::Parse("Invalid target_sg".into()))?;

        // Honey density (kg/L) - default to typical honey at 1.425
        let hon_sg = Decimal::new(1425, 3); // 1.425 kg/L

        if targ_sg <= curr_sg {
            return Err(Error::Validation(
                "Target gravity must be greater than current gravity".into(),
            ));
        }

        let sg_diff = targ_sg - curr_sg;

        // CORRECT FORMULA (empirically verified):
        // 1 lb (0.454 kg) honey in 1 gal (3.785 L) raises SG by 0.035
        // Therefore: SG_increase_per_kg_per_L = 0.035 / (0.454 / 3.785) = 0.292
        // honey_kg = volume_L × delta_SG / 0.292

        let sg_per_kg_per_liter = Decimal::new(292, 3); // 0.292
        let honey_needed_kg = (vol * sg_diff) / sg_per_kg_per_liter;

        // Account for volume displacement from honey (honey @ 1.425 kg/L)
        let honey_volume_l = honey_needed_kg / hon_sg;
        let final_volume = vol + honey_volume_l;

        // Return in grams (Unit::Grams exists, not Kilograms)
        let honey_needed_g = honey_needed_kg * Decimal::from(1000);
        let mut result = CalcResult::new(Measurement::new(honey_needed_g, Unit::Grams));

        if sg_diff > Decimal::new(40, 3) {
            result = result.with_warning("Large gravity increase (>0.040) - consider backsweetening in stages");
        }

        if targ_sg > Decimal::new(1100, 3) {
            result = result.with_warning("Final gravity >1.100 - ensure stabilization before backsweetening");
        }

        result = result
            .with_meta("honey_needed", format!("{:.2} kg ({:.0} g)", honey_needed_kg, honey_needed_g))
            .with_meta("current_gravity", format!("{:.3}", curr_sg))
            .with_meta("target_gravity", format!("{:.3}", targ_sg))
            .with_meta("gravity_increase", format!("{:.3}", sg_diff))
            .with_meta("current_volume", format!("{:.2} L", vol))
            .with_meta("honey_volume", format!("{:.2} L", honey_volume_l))
            .with_meta("final_volume", format!("{:.2} L", final_volume))
            .with_meta("tip", "Always stabilize (sorbate + sulfite) before backsweetening to prevent refermentation");

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        input.get_measurement(Unit::SpecificGravity)?;
        if input.get_param("volume").is_none() {
            return Err(Error::MissingInput("volume required".into()));
        }
        if input.get_param("target_sg").is_none() {
            return Err(Error::MissingInput("target_sg required".into()));
        }
        Ok(())
    }
}

register_calculator!(BacksweeteningCalculator);