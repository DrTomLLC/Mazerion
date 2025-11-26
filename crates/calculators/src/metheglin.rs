use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct MetheglinCalculator;

impl MetheglinCalculator {
    pub const ID: &'static str = "metheglin";
}

impl Calculator for MetheglinCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Metheglin" }
    fn description(&self) -> &'static str { "Calculate spice additions for metheglin" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let spice_type = input.get_param("spice_type").unwrap_or("cinnamon");
        let intensity = input.get_param("intensity").unwrap_or("medium");

        let base_rate = match spice_type {
            "cinnamon" => Decimal::from(2),
            "ginger" => Decimal::from(5),
            "vanilla" => Decimal::new(5, 1),
            _ => Decimal::from(2),
        };

        let multiplier = match intensity {
            "light" => Decimal::new(75, 2),
            "strong" => Decimal::new(15, 1),
            _ => Decimal::ONE,
        };

        let grams = volume * base_rate * multiplier;
        let result = CalcResult::new(Measurement::new(grams, Unit::Grams));
        Ok(result)
    }
}

register_calculator!(MetheglinCalculator);
