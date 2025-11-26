use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct RefractometerCalculator;

impl RefractometerCalculator {
    pub const ID: &'static str = "refractometer";
}

impl Calculator for RefractometerCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Refractometer Correction" }
    fn description(&self) -> &'static str { "Correct refractometer readings for alcohol (Terrill cubic)" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let og_brix_meas = input.get_measurement(Unit::Brix)?;
        let og_brix = og_brix_meas.value;
        
        let fg_brix = input.get_param("current_brix")
            .ok_or_else(|| Error::MissingInput("current_brix required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid current_brix: {}", e)))?;

        // Terrill cubic equation approximation
        let og_sg = Decimal::ONE + (og_brix * Decimal::new(4, 3));
        let fg_sg = Decimal::ONE - (Decimal::new(616868, 9) * fg_brix)
            + (Decimal::new(1313, 9) * fg_brix * fg_brix)
            - (Decimal::new(247, 12) * fg_brix * fg_brix * fg_brix);

        let abv = (og_sg - fg_sg) * Decimal::new(13125, 2);

        let mut result = CalcResult::new(Measurement::sg(fg_sg)?);
        result = result
            .with_meta("corrected_fg", format!("{:.4}", fg_sg))
            .with_meta("calculated_abv", format!("{:.2}%", abv))
            .with_meta("formula", "Terrill cubic equation");
        Ok(result)
    }
}

register_calculator!(RefractometerCalculator);
