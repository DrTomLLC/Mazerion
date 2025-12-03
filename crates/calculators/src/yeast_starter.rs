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
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Yeast Starter"
    }

    fn category(&self) -> &'static str {
        "Brewing"
    }

    fn description(&self) -> &'static str {
        "Calculate yeast starter requirements"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let target_cells = input
            .get_param("target_cells")
            .ok_or_else(|| Error::MissingInput("target_cells required".into()))?;
        let starting_cells = input
            .get_param("starting_cells")
            .ok_or_else(|| Error::MissingInput("starting_cells required".into()))?;

        let target: Decimal = target_cells
            .parse()
            .map_err(|_| Error::Parse("Invalid target_cells".into()))?;
        let starting: Decimal = starting_cells
            .parse()
            .map_err(|_| Error::Parse("Invalid starting_cells".into()))?;

        if target <= starting {
            return Err(Error::Validation(
                "Target cells must be greater than starting cells".into(),
            ));
        }

        // Growth rate: ~1.4 billion cells per gram of DME per liter
        // Optimal starter gravity: 1.036-1.040 (~100g DME per liter)

        let growth_needed = target - starting;

        // Calculate starter volume needed
        // Assume 2L starter for most cases, scale if needed
        let mut starter_volume = Decimal::from(2);

        if growth_needed > Decimal::from(300) {
            // Need a larger starter
            starter_volume = Decimal::from(4);
        }

        if growth_needed > Decimal::from(600) {
            // Need step starter
            starter_volume = Decimal::from(2); // First step
        }

        // DME calculation: 100g per liter for 1.040 gravity
        let dme_needed = starter_volume * Decimal::from(100);

        // Growth calculation (simplified Brewer's Friend model)
        // Growth rate = 1.4 billion cells/g DME
        let growth_rate = Decimal::new(14, 1);
        let cells_produced = dme_needed * growth_rate;
        let final_cells = starting + cells_produced;

        let mut result = CalcResult::new(Measurement::new(starter_volume, Unit::Liters));

        if growth_needed > Decimal::from(600) {
            result = result.with_warning("Multi-step starter recommended for this cell count");
        }

        if starter_volume > Decimal::from(3) {
            result = result.with_warning("Large starter - consider using stir plate");
        }

        result = result
            .with_meta("starter_volume", format!("{} L", starter_volume))
            .with_meta("dme_needed", format!("{:.0} g", dme_needed))
            .with_meta("starting_cells", format!("{} billion", starting))
            .with_meta("final_cells", format!("{:.0} billion", final_cells))
            .with_meta("growth", format!("+{:.0} billion", cells_produced))
            .with_meta("target_gravity", "1.040 SG");

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("target_cells").is_none() {
            return Err(Error::MissingInput("target_cells required".into()));
        }
        if input.get_param("starting_cells").is_none() {
            return Err(Error::MissingInput("starting_cells required".into()));
        }
        Ok(())
    }
}

register_calculator!(YeastStarterCalculator);