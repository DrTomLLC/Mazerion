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
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "Yeast Pitch Rate" }
    fn description(&self) -> &'static str { "Calculate yeast cells needed" }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid volume: {}", e)))?;
        let og = input.get_param("og")
            .ok_or_else(|| Error::MissingInput("og required".into()))?
            .parse::<Decimal>()
            .map_err(|e| Error::Parse(format!("Invalid og: {}", e)))?;
        let bev_type = input.get_param("beverage_type").unwrap_or("mead");

        let rate = match bev_type {
            "ale" => Decimal::new(75, 1),
            "lager" => Decimal::from(15),
            "wine" | "mead" => Decimal::from(5),
            "cider" => Decimal::from(3),
            _ => Decimal::from(5),
        };

        let plato = (og - Decimal::ONE) * Decimal::from(1000) / Decimal::from(4);
        let cells_billions = volume * plato * rate;

        let mut result = CalcResult::new(Measurement::new(cells_billions, Unit::Percent));
        result = result.with_meta("pitch_rate", format!("{} billion cells/L/°P", rate));
        Ok(result)
    }
}

register_calculator!(YeastPitchCalculator);
