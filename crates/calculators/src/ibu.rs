use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct IbuCalculator;

impl IbuCalculator {
    pub const ID: &'static str = "ibu";
}

impl Calculator for IbuCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "IBU Calculator (Tinseth)"
    }

    fn category(&self) -> &'static str {
        "Beer"
    }

    fn description(&self) -> &'static str {
        "Calculate International Bitterness Units using Tinseth formula"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let hop_weight_g = input
            .get_param("hop_weight_g")
            .ok_or_else(|| Error::MissingInput("hop_weight_g required (in grams)".into()))?;
        let alpha_acid = input
            .get_param("alpha_acid")
            .ok_or_else(|| Error::MissingInput("alpha_acid required (%)".into()))?;
        let boil_time = input
            .get_param("boil_time")
            .ok_or_else(|| Error::MissingInput("boil_time required (minutes)".into()))?;
        let volume_l = input
            .get_param("volume_l")
            .ok_or_else(|| Error::MissingInput("volume_l required (liters)".into()))?;
        let boil_gravity = input
            .get_param("boil_gravity")
            .ok_or_else(|| Error::MissingInput("boil_gravity required (SG)".into()))?;

        let weight: Decimal = hop_weight_g
            .parse()
            .map_err(|_| Error::Parse("Invalid hop_weight_g".into()))?;
        let aa: Decimal = alpha_acid
            .parse()
            .map_err(|_| Error::Parse("Invalid alpha_acid".into()))?;
        let time_min: Decimal = boil_time
            .parse()
            .map_err(|_| Error::Parse("Invalid boil_time".into()))?;
        let volume: Decimal = volume_l
            .parse()
            .map_err(|_| Error::Parse("Invalid volume_l".into()))?;
        let sg: Decimal = boil_gravity
            .parse()
            .map_err(|_| Error::Parse("Invalid boil_gravity".into()))?;

        // Tinseth formula:
        // B = 1.65 × 0.000125^(SG_boil - 1.0)
        // T = (1 - e^(-0.04 × t)) / 4.15
        // U = B × T
        // IBU = (W_g × AA% × U × 1000) / V_L

        // Convert to f64 for exponential calculations
        let sg_f64 = (sg - Decimal::ONE)
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.05);
        let time_f64 = time_min.to_string().parse::<f64>().unwrap_or(60.0);

        // Bigness factor (gravity correction)
        let bigness = 1.65 * 0.000125_f64.powf(sg_f64);

        // Boil time factor (utilization from boil time) - NO DIVISION BY 60!
        let boil_factor = (1.0 - (-0.04 * time_f64).exp()) / 4.15;

        // Total utilization
        let utilization = bigness * boil_factor;
        let util_decimal = Decimal::from_f64_retain(utilization).unwrap_or(Decimal::ZERO);

        // IBU calculation (metric)
        let aa_decimal = aa / Decimal::from(100);
        let ibu = (weight * aa_decimal * util_decimal * Decimal::from(1000)) / volume;

        let mut result = CalcResult::new(Measurement::new(ibu, Unit::Grams));

        if ibu > Decimal::from(100) {
            result = result.with_warning("IBU > 100 is extremely bitter");
        }

        if aa > Decimal::from(20) {
            result = result.with_warning("Alpha acid > 20% is unusually high - verify value");
        }

        result = result
            .with_meta("ibu", format!("{:.1}", ibu))
            .with_meta(
                "utilization",
                format!("{:.1}%", util_decimal * Decimal::from(100)),
            )
            .with_meta("bigness_factor", format!("{:.4}", bigness))
            .with_meta("boil_factor", format!("{:.4}", boil_factor))
            .with_meta("formula", "Tinseth")
            .with_meta("hop_weight", format!("{:.1} g", weight))
            .with_meta("alpha_acid", format!("{:.1}%", aa))
            .with_meta("boil_time", format!("{:.0} min", time_min));

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("hop_weight_g").is_none() {
            return Err(Error::MissingInput("hop_weight_g required".into()));
        }
        if input.get_param("alpha_acid").is_none() {
            return Err(Error::MissingInput("alpha_acid required".into()));
        }
        if input.get_param("boil_time").is_none() {
            return Err(Error::MissingInput("boil_time required".into()));
        }
        if input.get_param("volume_l").is_none() {
            return Err(Error::MissingInput("volume_l required".into()));
        }
        if input.get_param("boil_gravity").is_none() {
            return Err(Error::MissingInput("boil_gravity required".into()));
        }
        Ok(())
    }
}

register_calculator!(IbuCalculator);
