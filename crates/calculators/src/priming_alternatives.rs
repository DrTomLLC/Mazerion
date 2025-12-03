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
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Priming Alternatives"
    }

    fn category(&self) -> &'static str {
        "Utilities"
    }

    fn description(&self) -> &'static str {
        "Calculate alternative priming sugars (honey, DME, maple syrup)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_co2 = input.get_param("target_co2").unwrap_or("2.5");

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let co2: Decimal = target_co2
            .parse()
            .map_err(|_| Error::Parse("Invalid target_co2".into()))?;

        // Base calculation: table sugar
        // Simplified: ~4g sugar per liter per volume CO2
        let table_sugar = vol * co2 * Decimal::from(4);

        // Alternative sugar conversion factors (relative to table sugar)
        let corn_sugar_factor = Decimal::new(11, 1);    // 1.1x (91% fermentability)
        let honey_factor = Decimal::new(125, 2);        // 1.25x (80% fermentability)
        let dme_factor = Decimal::new(135, 2);          // 1.35x (74% fermentability)
        let maple_factor = Decimal::new(133, 2);        // 1.33x (75% fermentability)

        let corn_sugar = table_sugar * corn_sugar_factor;
        let honey = table_sugar * honey_factor;
        let dme = table_sugar * dme_factor;
        let maple_syrup = table_sugar * maple_factor;

        let mut result = CalcResult::new(Measurement::new(table_sugar, Unit::Grams));

        result = result
            .with_meta("table_sugar", format!("{:.1} g", table_sugar))
            .with_meta("corn_sugar", format!("{:.1} g", corn_sugar))
            .with_meta("honey", format!("{:.1} g", honey))
            .with_meta("dme", format!("{:.1} g", dme))
            .with_meta("maple_syrup", format!("{:.1} g", maple_syrup))
            .with_meta("target_co2", format!("{} volumes", co2))
            .with_meta("volume", format!("{} L", vol));

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("volume").is_none() {
            return Err(Error::MissingInput("volume required".into()));
        }
        Ok(())
    }
}

register_calculator!(PrimingAlternativesCalculator);