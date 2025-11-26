use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct SrmCalculator;

impl SrmCalculator {
    pub const ID: &'static str = "srm";
}

impl Calculator for SrmCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "SRM Color" }
    fn description(&self) -> &'static str { "Calculate beer color in SRM" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let malt_kg = input.get_param("malt_kg")
            .ok_or_else(|| Error::MissingInput("malt_kg required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid malt_kg: {}", e)))?;
        let lovibond = input.get_param("lovibond")
            .ok_or_else(|| Error::MissingInput("lovibond required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid lovibond: {}", e)))?;
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;

        let malt_lbs = malt_kg * Decimal::new(2204622, 6);
        let volume_gal = volume * Decimal::new(264172, 6);
        let mcu = (malt_lbs * lovibond) / volume_gal;
        
        let mcu_f64 = mcu.to_string().parse::<f64>().unwrap_or(0.0);
        let srm_f64 = 1.4922 * mcu_f64.powf(0.6859);
        let srm = Decimal::from_f64_retain(srm_f64).unwrap_or(Decimal::ZERO);

        let mut result = CalcResult::new(Measurement::new(srm, Unit::Percent));
        result = result.with_meta("MCU", format!("{:.1}", mcu));
        Ok(result)
    }
}

register_calculator!(SrmCalculator);
