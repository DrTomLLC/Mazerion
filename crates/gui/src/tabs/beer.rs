//! Beer-specific calculators tab

use crate::{MazerionApp, state::colors};
use eframe::egui::{self, RichText, CornerRadius};
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeerCalculator {
    Ibu,
    Srm,
}

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Select Calculator:").strong());
        egui::ComboBox::from_id_salt("beer_calc")
            .selected_text(get_calc_name(app.beer_calc))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.beer_calc, BeerCalculator::Ibu, "IBU Calculator");
                ui.selectable_value(&mut app.beer_calc, BeerCalculator::Srm, "SRM Color Calculator");
            });
    });

    ui.add_space(10.0);

    egui::Frame::new()
        .fill(colors::LIGHT_CREAM)
        .stroke(egui::Stroke::new(1.5, colors::HONEY_GOLD))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            match app.beer_calc {
                BeerCalculator::Ibu => render_ibu(app, ui),
                BeerCalculator::Srm => render_srm(app, ui),
            }
        });
}

fn get_calc_name(calc: BeerCalculator) -> &'static str {
    match calc {
        BeerCalculator::Ibu => "IBU Calculator",
        BeerCalculator::Srm => "SRM Color Calculator",
    }
}

fn render_ibu(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🌿 IBU Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate International Bitterness Units for hop additions");
    ui.add_space(10.0);

    let weight_unit = if app.state.unit_system == crate::state::UnitSystem::Metric { "g" } else { "oz" };
    let volume_unit = if app.state.unit_system == crate::state::UnitSystem::Metric { "L" } else { "gal" };

    crate::input_field(ui, &format!("Hop Weight ({}):", weight_unit), &mut app.hop_weight, "Weight of hops");
    crate::input_field(ui, "Alpha Acid %:", &mut app.hop_alpha, "Alpha acid percentage (e.g., 5.5)");
    crate::input_field(ui, "Boil Time (min):", &mut app.boil_time, "Boil duration in minutes");
    crate::input_field(ui, &format!("Batch Volume ({}):", volume_unit), &mut app.batch_volume, "Final batch volume");
    crate::input_field(ui, "Wort Gravity:", &mut app.wort_gravity, "Wort specific gravity (e.g., 1.050)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate IBU") {
        calc_ibu(app);
    }
}

fn render_srm(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🎨 SRM Color Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate beer color in Standard Reference Method (SRM)");
    ui.add_space(10.0);

    let weight_unit = if app.state.unit_system == crate::state::UnitSystem::Metric { "kg" } else { "lb" };
    let volume_unit = if app.state.unit_system == crate::state::UnitSystem::Metric { "L" } else { "gal" };

    crate::input_field(ui, &format!("Grain Weight ({}):", weight_unit), &mut app.grain_weight, "Weight of grain/malt");
    crate::input_field(ui, "Grain Color (°L):", &mut app.grain_lovibond, "Lovibond rating of grain");
    crate::input_field(ui, &format!("Batch Volume ({}):", volume_unit), &mut app.batch_volume, "Final batch volume");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate SRM") {
        calc_srm(app);
    }
}

fn calc_ibu(app: &mut MazerionApp) {
    let weight: Decimal = match Decimal::from_str(&app.hop_weight) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid hop weight".to_string());
            return;
        }
    };

    let alpha: Decimal = match Decimal::from_str(&app.hop_alpha) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid alpha acid %".to_string());
            return;
        }
    };

    let time: Decimal = match Decimal::from_str(&app.boil_time) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid boil time".to_string());
            return;
        }
    };

    let volume: Decimal = match Decimal::from_str(&app.batch_volume) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid batch volume".to_string());
            return;
        }
    };

    let gravity: Decimal = match Decimal::from_str(&app.wort_gravity) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid wort gravity".to_string());
            return;
        }
    };

    let weight_grams = if app.state.unit_system == crate::state::UnitSystem::Metric {
        weight
    } else {
        mazerion_core::ounces_to_grams(weight)
    };

    let volume_liters = if app.state.unit_system == crate::state::UnitSystem::Metric {
        volume
    } else {
        mazerion_core::gallons_to_liters(volume)
    };

    let time_f64 = time.to_string().parse::<f64>().unwrap_or(60.0);
    let utilization_f64 = 1.65 * 0.000125_f64.powf(gravity.to_string().parse::<f64>().unwrap_or(1.050) - 1.0)
        * (1.0 - (-0.04 * time_f64).exp()) / 4.15;

    let utilization = Decimal::from_f64_retain(utilization_f64).unwrap_or(Decimal::new(20, 2));

    let ibu = (weight_grams * alpha * utilization * Decimal::from(10)) / volume_liters;

    app.result = Some(format!("IBU: {:.1}", ibu));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("Utilization".to_string(), format!("{:.1}%", utilization * Decimal::from(100))));
    app.metadata.push(("Formula".to_string(), "Tinseth".to_string()));
}

fn calc_srm(app: &mut MazerionApp) {
    let weight: Decimal = match Decimal::from_str(&app.grain_weight) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid grain weight".to_string());
            return;
        }
    };

    let lovibond: Decimal = match Decimal::from_str(&app.grain_lovibond) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid Lovibond".to_string());
            return;
        }
    };

    let volume: Decimal = match Decimal::from_str(&app.batch_volume) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid batch volume".to_string());
            return;
        }
    };

    let weight_kg = if app.state.unit_system == crate::state::UnitSystem::Metric {
        weight
    } else {
        mazerion_core::pounds_to_kilograms(weight)
    };

    let volume_liters = if app.state.unit_system == crate::state::UnitSystem::Metric {
        volume
    } else {
        mazerion_core::gallons_to_liters(volume)
    };

    let weight_lbs = weight_kg * Decimal::new(2204622, 6);
    let volume_gal = volume_liters * Decimal::new(264172, 6);

    let mcu = (weight_lbs * lovibond) / volume_gal;

    let mcu_f64 = mcu.to_string().parse::<f64>().unwrap_or(10.0);
    let srm_f64 = 1.4922 * mcu_f64.powf(0.6859);
    let srm = Decimal::from_f64_retain(srm_f64).unwrap_or(Decimal::from(10));

    app.result = Some(format!("SRM: {:.1}", srm));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("MCU".to_string(), format!("{:.1}", mcu)));
    app.metadata.push(("Formula".to_string(), "Morey".to_string()));
}