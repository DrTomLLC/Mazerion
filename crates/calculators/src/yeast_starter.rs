use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct YeastStarterCalculator;

impl YeastStarterCalculator {
    pub const ID: &'static str = "yeast_starter";
}

impl Calculator for YeastStarterCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Yeast Starter Calculator" }
    fn description(&self) -> &'static str { "Calculate starter volume needed" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let target_cells = input.get_param("target_cells")
            .ok_or_else(|| Error::MissingInput("target_cells required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid target_cells: {}", e)))?;
        let starting_cells = input.get_param("starting_cells")
            .ok_or_else(|| Error::MissingInput("starting_cells required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid starting_cells: {}", e)))?;
        let stir_plate = input.get_param("stir_plate").unwrap_or("no");

        let growth_rate = if stir_plate == "yes" {
            Decimal::new(25, 1)
        } else {
            Decimal::new(16, 1)
        };

        let cells_needed = target_cells - starting_cells;
        let starter_liters = cells_needed / (growth_rate * Decimal::from(100));

        let result = CalcResult::new(Measurement::new(starter_liters, Unit::Liters));
        Ok(result)
    }
}

register_calculator!(YeastStarterCalculator);
