//! Backsweetening calculator - supports multiple sweetener types

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
        let curr_sg = current_sg_meas.value;

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

        // Get sweetener type (default to honey if not specified)
        let sweetener_type = input.get_param("sweetener").unwrap_or("honey");

        // Get sweetener properties based on type
        let (sugar_content, density_sg, sweetener_name) = match sweetener_type {
            "honey" => (Decimal::new(82, 2), Decimal::new(1425, 3), "Honey"),
            "table_sugar" => (Decimal::new(100, 2), Decimal::new(1587, 3), "Table Sugar"),
            "agave" => (Decimal::new(76, 2), Decimal::new(1368, 3), "Agave Nectar"),
            "maple_syrup" => (Decimal::new(67, 2), Decimal::new(1325, 3), "Maple Syrup"),
            _ => (Decimal::new(82, 2), Decimal::new(1425, 3), "Honey"), // fallback
        };

        if targ_sg <= curr_sg {
            return Err(Error::Validation(
                "Target gravity must be greater than current gravity".into(),
            ));
        }

        let sg_diff = targ_sg - curr_sg;

        // Formula: 1 kg of pure sugar in 1 L raises SG by ~0.356
        // Adjust for actual sugar content of sweetener
        let sg_per_kg_pure_sugar_per_liter = Decimal::new(356, 3); // 0.356
        let sg_per_kg_sweetener_per_liter = sg_per_kg_pure_sugar_per_liter * sugar_content;

        // Calculate sweetener needed
        let sweetener_needed_kg = (vol * sg_diff) / sg_per_kg_sweetener_per_liter;

        // Account for volume displacement
        let sweetener_volume_l = sweetener_needed_kg / density_sg;
        let final_volume = vol + sweetener_volume_l;

        // Convert to grams for display
        let sweetener_needed_g = sweetener_needed_kg * Decimal::from(1000);

        let mut result = CalcResult::new(Measurement::new(sweetener_needed_g, Unit::Grams));

        if sg_diff > Decimal::new(40, 3) {
            result = result.with_warning(
                "Large gravity increase (>0.040) - consider backsweetening in stages"
            );
        }

        if targ_sg > Decimal::new(1100, 3) {
            result = result.with_warning(
                "Final gravity >1.100 - ensure stabilization before backsweetening"
            );
        }

        result = result
            .with_meta("sweetener_type", sweetener_name)
            .with_meta("sweetener_needed", format!("{:.2} kg ({:.0} g)", sweetener_needed_kg, sweetener_needed_g))
            .with_meta("sugar_content", format!("{:.0}%", sugar_content * Decimal::from(100)))
            .with_meta("current_gravity", format!("{:.3}", curr_sg))
            .with_meta("target_gravity", format!("{:.3}", targ_sg))
            .with_meta("gravity_increase", format!("{:.3}", sg_diff))
            .with_meta("current_volume", format!("{:.2} L", vol))
            .with_meta("sweetener_volume", format!("{:.2} L", sweetener_volume_l))
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