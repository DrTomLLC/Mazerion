//! Basic calculators tab with specific labeled inputs

use crate::{MazerionApp, state::{BasicCalculator, colors}};
use eframe::egui::{self, RichText, CornerRadius};
use mazerion_core::CalcInput;
use std::str::FromStr;
use rust_decimal::Decimal;

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Select Calculator:").strong());
        egui::ComboBox::from_id_salt("basic_calc")
            .selected_text(get_calc_name(app.state.basic_calc))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.state.basic_calc, BasicCalculator::Abv, "ABV Calculator");
                ui.selectable_value(&mut app.state.basic_calc, BasicCalculator::BrixSgConverter, "Brix ↔ SG Converter");
                ui.selectable_value(&mut app.state.basic_calc, BasicCalculator::Dilution, "Dilution Calculator");
            });
    });

    ui.add_space(10.0);

    egui::Frame::new()
        .fill(colors::LIGHT_CREAM)
        .stroke(egui::Stroke::new(1.5, colors::HONEY_GOLD))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            match app.state.basic_calc {
                BasicCalculator::Abv => render_abv(app, ui),
                BasicCalculator::BrixSgConverter => render_brix_converter(app, ui),
                BasicCalculator::Dilution => render_dilution(app, ui),
            }
        });
}

fn get_calc_name(calc: BasicCalculator) -> &'static str {
    match calc {
        BasicCalculator::Abv => "ABV Calculator",
        BasicCalculator::BrixSgConverter => "Brix ↔ SG Converter",
        BasicCalculator::Dilution => "Dilution Calculator",
    }
}

fn render_abv(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍺 ABV Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate alcohol by volume from gravity readings");
    ui.add_space(10.0);

    crate::input_field(ui, "Original Gravity (OG):", &mut app.og, "Starting specific gravity (e.g., 1.090)");
    crate::input_field(ui, "Final Gravity (FG):", &mut app.fg, "Ending specific gravity (e.g., 1.010)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate ABV") {
        calc_abv(app);
    }
}

fn render_brix_converter(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("📐 Brix to SG Converter").color(colors::SADDLE_BROWN));
    ui.label("Convert degrees Brix to Specific Gravity");
    ui.add_space(10.0);

    crate::input_field(ui, "Brix (°Bx):", &mut app.brix, "Sugar content in degrees Brix (e.g., 15.0)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Convert to SG") {
        calc_brix_to_sg(app);
    }
}

fn render_dilution(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("💧 Dilution Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate water needed to reduce ABV");
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };
    crate::input_field(ui, &format!("Current Volume ({}):", vol_unit), &mut app.current_vol, "Current volume");
    crate::input_field(ui, "Current ABV (%):", &mut app.current_abv, "Current alcohol percentage");
    crate::input_field(ui, "Target ABV (%):", &mut app.target_abv, "Desired alcohol percentage");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Dilution") {
        calc_dilution(app);
    }
}

fn calc_abv(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("abv") {
        Some(c) => c,
        None => {
            app.result = Some("Error: ABV calculator not found".to_string());
            return;
        }
    };

    let input = CalcInput::new()
        .add_param("og", &app.og)
        .add_param("fg", &app.fg);

    match calc.calculate(input) {
        Ok(res) => {
            app.result = Some(format!("ABV: {:.2}%", res.output.value));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => {
            app.result = Some(format!("Error: {}", e));
            app.warnings.clear();
            app.metadata.clear();
        }
    }
}

fn calc_brix_to_sg(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("brix_to_sg") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Brix converter not found".to_string());
            return;
        }
    };

    let brix_val = match Decimal::from_str(&app.brix) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid Brix value".to_string());
            return;
        }
    };

    let measurement = match mazerion_core::Measurement::brix(brix_val) {
        Ok(m) => m,
        Err(e) => {
            app.result = Some(format!("Error: {}", e));
            return;
        }
    };

    let input = CalcInput::new().add_measurement(measurement);

    match calc.calculate(input) {
        Ok(res) => {
            app.result = Some(format!("Specific Gravity: {:.4}", res.output.value));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => {
            app.result = Some(format!("Error: {}", e));
            app.warnings.clear();
            app.metadata.clear();
        }
    }
}

fn calc_dilution(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("dilution") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Dilution calculator not found".to_string());
            return;
        }
    };

    let input = CalcInput::new()
        .add_param("current_volume", &app.current_vol)
        .add_param("current_abv", &app.current_abv)
        .add_param("target_abv", &app.target_abv);

    match calc.calculate(input) {
        Ok(res) => {
            let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };
            app.result = Some(format!("Water to Add: {:.2} {}", res.output.value, vol_unit));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => {
            app.result = Some(format!("Error: {}", e));
            app.warnings.clear();
            app.metadata.clear();
        }
    }
}