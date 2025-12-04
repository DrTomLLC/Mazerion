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
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let target_co2 = input.get_param("target_co2")
            .ok_or_else(|| Error::MissingInput("target_co2 required".into()))?;
        let temperature = input.get_param("temperature")
            .ok_or_else(|| Error::MissingInput("temperature required".into()))?;

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let target: Decimal = target_co2.parse()
            .map_err(|_| Error::Parse("Invalid target_co2".into()))?;
        let temp: Decimal = temperature.parse()
            .map_err(|_| Error::Parse("Invalid temperature".into()))?;

        // Calculate residual CO2 at temperature (simplified formula)
        let residual_co2 = Decimal::new(3, 1) - (temp * Decimal::new(1, 2));
        let co2_needed = target - residual_co2;

        if co2_needed < Decimal::ZERO {
            return Err(Error::Validation("Target CO2 already present at this temperature".into()));
        }

        // Priming sugar factors (grams per liter per volume CO2)
        // Base: table sugar = 4.0 g/L/vol
        let table_sugar = vol * co2_needed * Decimal::from(4);
        let corn_sugar = vol * co2_needed * Decimal::new(44, 1);    // 4.4 g/L/vol (dextrose)
        let honey = vol * co2_needed * Decimal::new(35, 1);         // 3.5 g/L/vol
        let dme = vol * co2_needed * Decimal::new(46, 1);           // 4.6 g/L/vol
        let maple_syrup = vol * co2_needed * Decimal::new(33, 1);   // 3.3 g/L/vol
        let agave = vol * co2_needed * Decimal::new(36, 1);         // 3.6 g/L/vol
        let molasses = vol * co2_needed * Decimal::new(37, 1);      // 3.7 g/L/vol

        let mut result = CalcResult::new(Measurement::new(table_sugar, Unit::Grams));

        result = result
            .with_meta("table_sugar_g", format!("{:.0} g ({:.2} oz)", table_sugar, table_sugar / Decimal::new(2835, 2)))
            .with_meta("corn_sugar_g", format!("{:.0} g ({:.2} oz)", corn_sugar, corn_sugar / Decimal::new(2835, 2)))
            .with_meta("honey_g", format!("{:.0} g ({:.2} oz)", honey, honey / Decimal::new(2835, 2)))
            .with_meta("dme_g", format!("{:.0} g ({:.2} oz)", dme, dme / Decimal::new(2835, 2)))
            .with_meta("maple_syrup_g", format!("{:.0} g ({:.2} oz)", maple_syrup, maple_syrup / Decimal::new(2835, 2)))
            .with_meta("agave_g", format!("{:.0} g ({:.2} oz)", agave, agave / Decimal::new(2835, 2)))
            .with_meta("molasses_g", format!("{:.0} g ({:.2} oz)", molasses, molasses / Decimal::new(2835, 2)))
            .with_meta("co2_needed", format!("{:.2} volumes", co2_needed))
            .with_meta("residual_co2", format!("{:.2} volumes", residual_co2));

        if target > Decimal::new(45, 1) {
            result = result.with_warning("High carbonation - risk of bottle bombs, ensure bottles rated for pressure");
        }

        Ok(result)
    }
}

register_calculator!(PrimingAlternativesCalculator);