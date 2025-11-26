use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct CarbonationCalculator;

impl CarbonationCalculator {
    pub const ID: &'static str = "carbonation";
}

impl Calculator for CarbonationCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Carbonation Calculator" }
    fn description(&self) -> &'static str { "Calculate priming sugar or keg PSI" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let temp = input.get_param("temperature")
            .ok_or_else(|| Error::MissingInput("temperature required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid temperature: {}", e)))?;
        let target_co2 = input.get_param("target_co2")
            .ok_or_else(|| Error::MissingInput("target_co2 required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid target_co2: {}", e)))?;
        let method = input.get_param("method").unwrap_or("priming");

        let dissolved_co2 = Decimal::new(56, 2) - (Decimal::new(9, 3) * temp);
        let needed_co2 = target_co2 - dissolved_co2;

        let result_val = if method == "keg" {
            let psi = (needed_co2 - Decimal::new(14, 2)) / Decimal::new(19, 2) * (temp + Decimal::from(460));
            psi
        } else {
            let sugar_type = input.get_param("sugar_type").unwrap_or("table_sugar");
            let factor = match sugar_type {
                "corn_sugar" => Decimal::new(91, 2),
                "honey" => Decimal::new(125, 2),
                "dme" => Decimal::new(135, 2),
                _ => Decimal::ONE,
            };
            volume * needed_co2 * Decimal::from(2) * factor
        };

        let result = CalcResult::new(Measurement::new(result_val, if method == "keg" { Unit::Percent } else { Unit::Grams }));
        Ok(result)
    }
}

register_calculator!(CarbonationCalculator);
