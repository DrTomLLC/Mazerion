use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct MelomelsCalculator;

impl MelomelsCalculator {
    pub const ID: &'static str = "melomel";
}

impl Calculator for MelomelsCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Melomel (Fruit Mead)" }
    fn description(&self) -> &'static str { "Calculate fruit additions for melomel" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let fruit_type = input.get_param("fruit_type").unwrap_or("strawberry");
        let intensity = input.get_param("intensity").unwrap_or("medium");

        let base_rate = match fruit_type {
            "strawberry" => Decimal::from(3),
            "blueberry" => Decimal::new(25, 1),
            "raspberry" => Decimal::from(2),
            "cherry" => Decimal::from(3),
            "peach" => Decimal::new(35, 1),
            _ => Decimal::from(3),
        };

        let multiplier = match intensity {
            "light" => Decimal::new(75, 2),
            "strong" => Decimal::new(15, 1),
            _ => Decimal::ONE,
        };

        let lbs = volume * base_rate * multiplier;
        let result = CalcResult::new(Measurement::new(lbs, Unit::Grams));
        Ok(result)
    }
}

register_calculator!(MelomelsCalculator);
