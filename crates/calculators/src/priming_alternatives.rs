use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct PrimingAlternativesCalculator;

impl PrimingAlternativesCalculator {
    pub const ID: &'static str = "priming_alternatives";
}

impl Calculator for PrimingAlternativesCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Priming Sugar Alternatives" }
    fn description(&self) -> &'static str { "Convert between priming sugar types" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let base_sugar = input.get_param("base_sugar_grams")
            .ok_or_else(|| Error::MissingInput("base_sugar_grams required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid base_sugar_grams: {}", e)))?;
        let from_type = input.get_param("from_type").unwrap_or("table_sugar");
        let to_type = input.get_param("to_type").unwrap_or("corn_sugar");

        let conversion = match (from_type, to_type) {
            ("table_sugar", "corn_sugar") => Decimal::new(11, 1),
            ("table_sugar", "honey") => Decimal::new(125, 2),
            ("corn_sugar", "table_sugar") => Decimal::new(91, 2),
            ("honey", "table_sugar") => Decimal::new(80, 2),
            _ => Decimal::ONE,
        };

        let converted = base_sugar * conversion;
        let result = CalcResult::new(Measurement::new(converted, Unit::Grams));
        Ok(result)
    }
}

register_calculator!(PrimingAlternativesCalculator);
