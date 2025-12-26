use mazerion_core::{
    CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit, register_calculator,
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
        let og = input
            .get_param("og")
            .ok_or_else(|| Error::MissingInput("og required".into()))?;
        let yeast_type = input.get_param("yeast_type").unwrap_or("ale");

        let vol: Decimal = volume
            .parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let gravity: Decimal = og.parse().map_err(|_| Error::Parse("Invalid og".into()))?;

        // Convert volume to mL
        let vol_ml = vol * Decimal::from(1000);

        // Calculate degrees Plato from SG
        let plato = (gravity - Decimal::ONE) * Decimal::from(250);

        // Pitch rate (million cells per mL per degree Plato)
        let pitch_rate = match yeast_type {
            "ale" => Decimal::new(75, 2),   // 0.75 million cells/mL/°P
            "lager" => Decimal::new(15, 1), // 1.5 million cells/mL/°P
            "mead" => Decimal::new(5, 1),   // 0.5 million cells/mL/°P (lower)
            _ => Decimal::new(75, 2),
        };

        // Total cells needed (in billions)
        let cells_needed = (vol_ml * plato * pitch_rate) / Decimal::from(1000);

        // Typical dry yeast packet: 200 billion cells
        let packets_needed = cells_needed / Decimal::from(200);

        let mut result = CalcResult::new(Measurement::new(packets_needed, Unit::Grams));

        result = result
            .with_meta("yeast_type", yeast_type)
            .with_meta(
                "cells_billion",
                format!("{:.0} billion cells", cells_needed),
            )
            .with_meta(
                "packets_5g",
                format!("{:.1} packets (5g ea)", packets_needed),
            )
            .with_meta(
                "grams_dry",
                format!("{:.1} g", packets_needed * Decimal::from(5)),
            )
            .with_meta("pitch_rate", format!("{:.2} M cells/mL/°P", pitch_rate))
            .with_meta("plato", format!("{:.1}°P", plato));

        if packets_needed > Decimal::from(5) {
            result = result.with_warning("High cell count needed - consider making a starter");
        }

        Ok(result)
    }
}

register_calculator!(YeastPitchCalculator);
