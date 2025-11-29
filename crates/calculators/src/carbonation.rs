use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
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
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let temperature = input.get_param("temperature")
            .ok_or_else(|| Error::MissingInput("temperature required".into()))?;
        let target_co2 = input.get_param("target_co2")
            .ok_or_else(|| Error::MissingInput("target_co2 required".into()))?;

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let temp: Decimal = temperature.parse()
            .map_err(|_| Error::Parse("Invalid temperature".into()))?;
        let target: Decimal = target_co2.parse()
            .map_err(|_| Error::Parse("Invalid target_co2".into()))?;

        // Calculate residual CO2 at temperature (simplified formula)
        let residual_co2 = Decimal::new(3, 1) - (temp * Decimal::new(1, 2));

        // CO2 needed = target - residual
        let co2_needed = target - residual_co2;

        if co2_needed < Decimal::ZERO {
            return Err(Error::Validation("Target CO2 already present at this temperature".into()));
        }

        // Check method - priming sugar or force carbonation
        let method = input.get_param("method").unwrap_or("priming");

        let result = if method == "keg" {
            // Calculate PSI for force carbonation
            // PSI = (target_co2 - residual) * temp_factor
            let psi = co2_needed * (Decimal::from(15) - (temp * Decimal::new(2, 1)));

            CalcResult::new(Measurement::new(psi, Unit::Grams))
                .with_meta("method", "Force Carbonation (Keg)")
                .with_meta("psi", format!("{:.1}", psi))
                .with_meta("target_co2", target_co2)
        } else {
            // Calculate priming sugar (table sugar default)
            // Grams = volume_L * co2_needed * 4 (simplified)
            let sugar_type = input.get_param("sugar_type").unwrap_or("table_sugar");

            let factor = match sugar_type {
                "table_sugar" => Decimal::from(4),
                "corn_sugar" => Decimal::new(44, 1),  // 4.4
                "honey" => Decimal::new(35, 1),       // 3.5
                "dme" => Decimal::new(46, 1),         // 4.6
                _ => Decimal::from(4),
            };

            let priming_sugar = vol * co2_needed * factor;

            CalcResult::new(Measurement::new(priming_sugar, Unit::Grams))
                .with_meta("method", "Bottle Priming")
                .with_meta("sugar_type", sugar_type)
                .with_meta("target_co2", target_co2)
                .with_meta("residual_co2", format!("{:.2}", residual_co2))
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