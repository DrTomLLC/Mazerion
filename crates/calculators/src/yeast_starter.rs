use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
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
        "Calculate yeast starter size and DME requirements"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let cells_needed = input
            .get_param("cells_needed")
            .ok_or_else(|| Error::MissingInput("cells_needed required (billions)".into()))?;
        let cells_available = input.get_param("cells_available").unwrap_or("100");

        let target_cells: Decimal = cells_needed
            .parse()
            .map_err(|_| Error::Parse("Invalid cells_needed".into()))?;
        let starting_cells: Decimal = cells_available
            .parse()
            .map_err(|_| Error::Parse("Invalid cells_available".into()))?;

        // Growth rate: ~3x in optimal conditions (1.040 SG starter)
        let growth_factor = Decimal::from(3);

        // Calculate starter size needed
        let cells_to_grow = target_cells - starting_cells;

        if cells_to_grow <= Decimal::ZERO {
            return Err(Error::Validation(
                "Already have enough cells, no starter needed".into(),
            ));
        }

        // Starter volume: ~1L per 100 billion cells to grow
        let starter_volume = cells_to_grow / Decimal::from(100);

        // DME: 100g per liter for 1.040 SG
        let dme_needed = starter_volume * Decimal::from(100);

        // Expected cell count after growth
        let final_cells = starting_cells * growth_factor;

        let mut result = CalcResult::new(Measurement::new(
            starter_volume * Decimal::from(1000),
            Unit::Milliliters,
        ));

        result = result
            .with_meta("starter_volume_L", format!("{:.1} L", starter_volume))
            .with_meta(
                "dme_needed_g",
                format!(
                    "{:.0} g ({:.2} oz)",
                    dme_needed,
                    dme_needed / Decimal::new(2835, 2)
                ),
            )
            .with_meta("starting_cells", format!("{} billion", starting_cells))
            .with_meta("target_cells", format!("{} billion", target_cells))
            .with_meta("expected_final", format!("{:.0} billion", final_cells))
            .with_meta("target_sg", "1.040");

        if final_cells < target_cells {
            result =
                result.with_warning("May need multiple starter steps to reach target cell count");
        }

        if starter_volume > Decimal::from(5) {
            result =
                result.with_warning("Large starter volume - consider stepped starter approach");
        }

        Ok(result)
    }
}

register_calculator!(YeastStarterCalculator);
