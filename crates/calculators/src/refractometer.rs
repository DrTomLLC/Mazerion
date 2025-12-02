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
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Refractometer Correction"
    }

    fn category(&self) -> &'static str {
        "Advanced"
    }

    fn description(&self) -> &'static str {
        "Correct refractometer readings for alcohol presence (Terrill cubic)"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let orig_brix_meas = input.get_measurement(Unit::Brix)?;
        let current_brix = input.get_param("current_brix")
            .ok_or_else(|| Error::MissingInput("current_brix required".into()))?;

        let ob = orig_brix_meas.value;
        let cb: Decimal = current_brix.parse()
            .map_err(|_| Error::Parse("Invalid current_brix".into()))?;

        let ob_f64 = ob.to_string().parse::<f64>().unwrap_or(0.0);
        let cb_f64 = cb.to_string().parse::<f64>().unwrap_or(0.0);

        let fg_f64 = 1.0000
            - (0.0044993 * ob_f64)
            + (0.011774 * cb_f64)
            + (0.00027581 * ob_f64 * ob_f64)
            - (0.0012717 * cb_f64 * cb_f64)
            - (0.0000072800 * ob_f64 * ob_f64 * ob_f64)
            + (0.000063293 * cb_f64 * cb_f64 * cb_f64);

        let fg = Decimal::from_f64_retain(fg_f64).unwrap_or(Decimal::ONE);
        let abv = (ob - cb) * Decimal::new(55, 2);

        let ae_f64 = 1.0000 - 0.00085683 * ob_f64 + 0.0034941 * cb_f64;
        let ae = Decimal::from_f64_retain(ae_f64).unwrap_or(Decimal::ONE);

        let mut result = CalcResult::new(Measurement::sg(fg)?)
            .with_meta("original_brix", format!("{:.2}", ob))
            .with_meta("current_brix", format!("{:.2}", cb))
            .with_meta("corrected_fg", format!("{:.4}", fg))
            .with_meta("estimated_abv", format!("{:.2}%", abv))
            .with_meta("actual_extract_sg", format!("{:.4}", ae))
            .with_meta("formula", "Terrill Cubic");

        if cb > ob {
            result = result.with_warning("Current Brix higher than original - fermentation may not have started");
        }
        if abv > Decimal::from(18) {
            result = result.with_warning("ABV >18% - equation accuracy decreases");
        }
        if fg < Decimal::new(9900, 4) {
            result = result.with_warning("FG <0.990 - unusually low, verify readings");
        }

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        input.get_measurement(Unit::Brix)?;
        if input.get_param("current_brix").is_none() {
            return Err(Error::MissingInput("current_brix required".into()));
        }
        Ok(())
    }
}

register_calculator!(RefractometerCalculator);