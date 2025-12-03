use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct YeastPitchCalculator;

impl YeastPitchCalculator {
    pub const ID: &'static str = "yeast_pitch";
}

impl Calculator for YeastPitchCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Yeast Pitch Rate"
    }

    fn category(&self) -> &'static str {
        "Brewing"
    }

    fn description(&self) -> &'static str {
        "Calculate yeast pitch rate for optimal fermentation"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let gravity = input
            .get_param("gravity")
            .ok_or_else(|| Error::MissingInput("gravity required".into()))?;
        let beer_type = input.get_param("beer_type").unwrap_or("ale");

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let grav: Decimal = gravity
            .parse()
            .map_err(|_| Error::Parse("Invalid gravity".into()))?;

        // Convert gravity to Plato
        let plato = (grav - Decimal::ONE) * Decimal::from(250);

        // Pitch rate: 0.75 million cells/mL/°P for ales, 1.5 for lagers
        let rate = if beer_type == "lager" {
            Decimal::new(15, 1) // 1.5
        } else {
            Decimal::new(75, 2) // 0.75
        };

        // Total cells needed (in billions)
        let cells_needed = vol * Decimal::from(1000) * plato * rate;

        // Standard dry yeast packet = 200 billion cells
        // Standard liquid yeast packet = 100 billion cells
        let yeast_type = input.get_param("yeast_type").unwrap_or("dry");

        let cells_per_packet = if yeast_type == "liquid" {
            Decimal::from(100)
        } else {
            Decimal::from(200)
        };

        let packets_needed = cells_needed / cells_per_packet;

        let mut result = CalcResult::new(Measurement::new(packets_needed, Unit::Grams));

        if packets_needed > Decimal::from(2) && yeast_type == "liquid" {
            result = result.with_warning("Consider making a yeast starter for >2 liquid packets");
        }

        result = result
            .with_meta("cells_needed", format!("{:.0} billion", cells_needed))
            .with_meta("packets_needed", format!("{:.1}", packets_needed))
            .with_meta("yeast_type", yeast_type)
            .with_meta("beer_type", beer_type)
            .with_meta("pitch_rate", format!("{} M cells/mL/°P", rate));

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("volume").is_none() {
            return Err(Error::MissingInput("volume required".into()));
        }
        if input.get_param("gravity").is_none() {
            return Err(Error::MissingInput("gravity required".into()));
        }
        Ok(())
    }
}

register_calculator!(YeastPitchCalculator);