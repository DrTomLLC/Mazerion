use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct CarbonationCalculator;

impl CarbonationCalculator {
    pub const ID: &'static str = "carbonation";
}

impl Calculator for CarbonationCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Carbonation Calculator"
    }

    fn category(&self) -> &'static str {
        "Brewing"
    }

    fn description(&self) -> &'static str {
        "Calculate priming sugar or keg PSI for target carbonation"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input
            .get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let temperature = input
            .get_param("temperature")
            .ok_or_else(|| Error::MissingInput("temperature required".into()))?;
        let target_co2 = input
            .get_param("target_co2")
            .ok_or_else(|| Error::MissingInput("target_co2 required".into()))?;

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let temp_c: Decimal = temperature
            .parse()
            .map_err(|_| Error::Parse("Invalid temperature".into()))?;
        let target: Decimal = target_co2
            .parse()
            .map_err(|_| Error::Parse("Invalid target_co2".into()))?;

        let method = input.get_param("method").unwrap_or("priming");

        if method != "priming" && method != "keg" {
            return Err(Error::Validation(
                "Invalid method - must be 'priming' or 'keg'".into(),
            ));
        }

        let temp_f = (temp_c * Decimal::new(9, 0) / Decimal::new(5, 0)) + Decimal::from(32);

        let temp_f_f64 = temp_f.to_string().parse::<f64>().unwrap_or(68.0);
        let residual_co2_f64 =
            3.0378 - (0.050062 * temp_f_f64) + (0.00026555 * temp_f_f64 * temp_f_f64);
        let residual_co2 = Decimal::from_f64_retain(residual_co2_f64).unwrap_or(Decimal::new(8, 1));

        let co2_needed = target - residual_co2;

        if co2_needed < Decimal::ZERO {
            return Err(Error::Validation(
                "Target CO2 already present at this temperature".into(),
            ));
        }

        let result = if method == "keg" {
            let t = temp_f_f64;
            let co2 = target.to_string().parse::<f64>().unwrap_or(2.5);
            let psi_f64 = -16.6999 - (0.0101059 * t)
                + (0.00116512 * t * t)
                + (0.173354 * t * co2)
                + (4.24267 * co2)
                - (0.0684226 * co2 * co2);
            let psi = Decimal::from_f64_retain(psi_f64.max(0.0)).unwrap_or(Decimal::from(10));

            CalcResult::new(Measurement::new(psi, Unit::Grams))
                .with_meta("method", "Force Carbonation (Keg)")
                .with_meta("psi", format!("{:.1}", psi))
                .with_meta("target_co2", target_co2)
                .with_meta("residual_co2", format!("{:.2} volumes", residual_co2))
                .with_meta("temp_c", format!("{:.1}°C", temp_c))
                .with_meta("temp_f", format!("{:.1}°F", temp_f))
        } else {
            let sugar_type = input.get_param("sugar_type").unwrap_or("table_sugar");

            let factor = match sugar_type {
                "table_sugar" => Decimal::new(40, 1),
                "corn_sugar" => Decimal::new(386, 2),
                "honey" => Decimal::new(50, 1),
                "dme" => Decimal::new(460, 2),
                _ => Decimal::new(40, 1),
            };

            let priming_sugar = co2_needed * factor * vol;

            let mut res = CalcResult::new(Measurement::new(priming_sugar, Unit::Grams))
                .with_meta("method", "Bottle Priming")
                .with_meta("sugar_type", sugar_type)
                .with_meta("priming_sugar_g", format!("{:.1} g", priming_sugar))
                .with_meta("target_co2", format!("{:.1} volumes", target))
                .with_meta("residual_co2", format!("{:.2} volumes", residual_co2))
                .with_meta("co2_to_add", format!("{:.2} volumes", co2_needed))
                .with_meta("temp_c", format!("{:.1}°C", temp_c))
                .with_meta("temp_f", format!("{:.1}°F", temp_f));

            if priming_sugar > vol * Decimal::from(10) {
                res = res.with_warning("Very high priming sugar - double-check target CO2");
            }

            if target > Decimal::new(45, 1) {
                res = res.with_warning("High carbonation (>4.5 vol) - risk of bottle bombs");
            }

            res
        };

        Ok(result)
    }

    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.get_param("volume").is_none() {
            return Err(Error::MissingInput("volume required".into()));
        }
        if input.get_param("temperature").is_none() {
            return Err(Error::MissingInput("temperature required".into()));
        }
        if input.get_param("target_co2").is_none() {
            return Err(Error::MissingInput("target_co2 required".into()));
        }
        Ok(())
    }
}

register_calculator!(CarbonationCalculator);