//! Backsweetening calculator - SAFETY CRITICAL
//! UNIT-AGNOSTIC metadata, all units handled in GUI layer

use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
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

    fn description(&self) -> &'static str {
        "Calculate sweetener additions to reach target gravity"
    }

    fn category(&self) -> &'static str {
        "Finishing"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let sg_meas = input.get_measurement(Unit::SpecificGravity)?;
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required (in liters)".into()))?;
        let target_sg = input
            .get_param("target_sg")
            .ok_or_else(|| Error::MissingInput("target_sg required".into()))?;
        let sweetener_type = input.get_param("sweetener").unwrap_or("honey");

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let curr_sg = sg_meas.value;
        let targ_sg: Decimal = target_sg
            .parse()
            .map_err(|_| Error::Parse("Invalid target_sg".into()))?;

        if targ_sg <= curr_sg {
            return Err(Error::Validation(
                "Target SG must be higher than current SG".into(),
            ));
        }

        let sg_diff = targ_sg - curr_sg;

        // Sugar content by sweetener type
        let sugar_content = match sweetener_type {
            "honey" => Decimal::new(82, 2),       // 0.82
            "table_sugar" => Decimal::ONE,        // 1.00
            "agave" => Decimal::new(76, 2),       // 0.76
            "maple_syrup" => Decimal::new(67, 2), // 0.67
            _ => Decimal::new(82, 2),             // default to honey
        };

        // Density of sweetener (kg/L)
        let sweetener_density = match sweetener_type {
            "honey" => Decimal::new(142, 2),       // 1.42 kg/L
            "table_sugar" => Decimal::new(159, 2), // 1.59 kg/L (granulated)
            "agave" => Decimal::new(138, 2),       // 1.38 kg/L
            "maple_syrup" => Decimal::new(132, 2), // 1.32 kg/L
            _ => Decimal::new(142, 2),
        };

        // Gravity points per kg per liter
        let points_per_kg_per_l = Decimal::from(383); // For pure sugar
        let adjusted_points = points_per_kg_per_l * sugar_content;

        // Concentration needed: (kg/L) = gravity_points / points_per_kg_per_L
        let concentration_kg_per_l = (sg_diff * Decimal::from(1000)) / adjusted_points;

        // Total sweetener needed in kg
        let sweetener_needed_kg = concentration_kg_per_l * vol;
        let sweetener_needed_g = sweetener_needed_kg * Decimal::from(1000);

        // Volume of sweetener added (L)
        let sweetener_volume_l = sweetener_needed_kg / sweetener_density;

        // Final volume after adding sweetener
        let final_volume = vol + sweetener_volume_l;

        let mut result = CalcResult::new(Measurement::new(sweetener_needed_g, Unit::Grams));

        if sg_diff > Decimal::new(40, 3) {
            result = result.with_warning(
                "Large gravity increase (>0.040) - consider backsweetening in stages",
            );
        }

        if sg_diff > Decimal::new(80, 3) {
            result = result
                .with_warning("Very large gravity increase (>0.080) - MUST backsweeten in stages");
        }

        result = result
            .with_warning("CRITICAL: Stabilize with K-meta + K-sorbate BEFORE backsweetening");

        // UNIT-AGNOSTIC METADATA - no hardcoded units
        result = result
            .with_meta("sweetener_type", sweetener_type)
            .with_meta("sweetener_needed_kg", format!("{:.2}", sweetener_needed_kg))
            .with_meta("sweetener_needed_g", format!("{:.0}", sweetener_needed_g))
            .with_meta("sugar_content_percent", format!("{:.0}", sugar_content * Decimal::from(100)))
            .with_meta("current_gravity", format!("{:.3}", curr_sg))
            .with_meta("target_gravity", format!("{:.3}", targ_sg))
            .with_meta("gravity_increase", format!("{:.3}", sg_diff))
            .with_meta("volume_liters", format!("{:.2}", vol))
            .with_meta("sweetener_volume_liters", format!("{:.2}", sweetener_volume_l))
            .with_meta("final_volume_liters", format!("{:.2}", final_volume))
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
