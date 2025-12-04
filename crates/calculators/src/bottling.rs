use mazerion_core::{
    register_calculator, CalcInput, CalcResult, Calculator, Error, Measurement, Result, Unit,
};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct BottlingCalculator;

impl BottlingCalculator {
    pub const ID: &'static str = "bottling";
}

impl Calculator for BottlingCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "Bottling Calculator"
    }

    fn category(&self) -> &'static str {
        "Finishing"
    }

    fn description(&self) -> &'static str {
        "Calculate bottles needed and headspace for batch volume"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        let volume = input.get_param("volume")
            .ok_or_else(|| Error::MissingInput("volume required".into()))?;
        let bottle_size = input.get_param("bottle_size").unwrap_or("750");
        let loss_percent = input.get_param("loss_percent").unwrap_or("3");

        let vol: Decimal = volume.parse()
            .map_err(|_| Error::Parse("Invalid volume".into()))?;
        let bottle_ml: Decimal = bottle_size.parse()
            .map_err(|_| Error::Parse("Invalid bottle_size".into()))?;
        let loss_pct: Decimal = loss_percent.parse()
            .map_err(|_| Error::Parse("Invalid loss_percent".into()))?;

        // Account for losses (racking, lees, sampling, spillage)
        let usable_volume = vol * (Decimal::ONE - loss_pct / Decimal::from(100));
        let usable_volume_ml = usable_volume * Decimal::from(1000);

        // Calculate bottles needed
        let bottles_exact = usable_volume_ml / bottle_ml;
        let bottles_needed = bottles_exact.floor();

        // Calculate leftover
        let leftover_ml = usable_volume_ml - (bottles_needed * bottle_ml);

        // Calculate case quantities
        let cases_12 = bottles_needed / Decimal::from(12);
        let loose_bottles = bottles_needed % Decimal::from(12);

        let mut result = CalcResult::new(Measurement::new(bottles_needed, Unit::Grams));

        result = result
            .with_meta("bottle_size_ml", format!("{} mL", bottle_ml))
            .with_meta("bottles_needed", format!("{:.0} bottles", bottles_needed))
            .with_meta("cases_12", format!("{:.0} cases + {:.0} loose", cases_12, loose_bottles))
            .with_meta("usable_volume_L", format!("{:.2} L", usable_volume))
            .with_meta("loss_L", format!("{:.2} L ({}%)", vol - usable_volume, loss_pct))
            .with_meta("leftover_ml", format!("{:.0} mL", leftover_ml));

        if leftover_ml > Decimal::from(200) {
            result = result.with_warning("Significant leftover - consider adding a smaller bottle");
        }

        if bottle_ml < Decimal::from(375) {
            result = result.with_warning("Small bottles - consider aging potential and oxidation");
        }

        Ok(result)
    }
}

register_calculator!(BottlingCalculator);