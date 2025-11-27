//! Basic calculations tab - COMPLETE IMPLEMENTATION

use crate::state::AppState;
use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};
use mazerion_core::{CalcInput, Measurement, traits::get_calculator};
use rust_decimal::Decimal;
use std::str::FromStr;

const BG_PANEL: Color32 = Color32::from_rgb(255, 255, 255);
const TEXT_MAIN: Color32 = Color32::from_rgb(30, 30, 30);
const TEXT_LABEL: Color32 = Color32::from_rgb(60, 60, 60);
const BORDER: Color32 = Color32::from_rgb(218, 165, 32);
const BUTTON: Color32 = Color32::from_rgb(34, 139, 34);
const BUTTON_TEXT: Color32 = Color32::WHITE;

pub fn render(ui: &mut egui::Ui, state: &mut AppState) {
    // ABV Calculator
    section(ui, "🍺 ABV Calculator", |ui| {
        field(ui, "Original Gravity:", &mut state.og);
        field(ui, "Final Gravity:", &mut state.fg);
        if button(ui, "Calculate ABV") {
            calc_abv(state);
        }
    });

    ui.add_space(10.0);

    // Brix → SG
    section(ui, "📐 Brix → SG Converter", |ui| {
        field(ui, "Brix (°Bx):", &mut state.brix);
        if button(ui, "Convert to SG") {
            calc_brix_to_sg(state);
        }
    });

    ui.add_space(10.0);

    // SG → Brix
    section(ui, "📐 SG → Brix Converter", |ui| {
        field(ui, "Specific Gravity:", &mut state.sg);
        if button(ui, "Convert to Brix") {
            calc_sg_to_brix(state);
        }
    });

    ui.add_space(10.0);

    // SG Temperature Correction
    section(ui, "🌡️ SG Temperature Correction", |ui| {
        field(ui, "Measured SG:", &mut state.sg);
        field(ui, "Temperature (°C):", &mut state.temp);
        if button(ui, "Correct SG") {
            calc_sg_correction(state);
        }
    });

    ui.add_space(10.0);

    // Dilution Calculator
    section(ui, "💧 Dilution Calculator", |ui| {
        field(ui, "Current Volume (L):", &mut state.current_vol);
        field(ui, "Current ABV (%):", &mut state.current_abv);
        field(ui, "Target ABV (%):", &mut state.target_abv);
        if button(ui, "Calculate Water Needed") {
            calc_dilution(state);
        }
    });
}

fn section(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(BG_PANEL)
        .stroke(Stroke::new(1.5, BORDER))
        .rounding(Rounding::same(8.0))
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.heading(RichText::new(title).color(TEXT_MAIN).size(18.0));
            ui.add_space(8.0);
            content(ui);
        });
}

fn field(ui: &mut egui::Ui, label: &str, value: &mut String) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(TEXT_LABEL).strong());
        ui.text_edit_singleline(value);
    });
}

fn button(ui: &mut egui::Ui, text: &str) -> bool {
    ui.add(
        egui::Button::new(RichText::new(text).color(BUTTON_TEXT).size(16.0).strong())
            .fill(BUTTON)
            .rounding(Rounding::same(6.0))
            .min_size(Vec2::new(200.0, 36.0))
    ).clicked()
}

// Calculator implementations

fn calc_abv(state: &mut AppState) {
    let calc = match get_calculator("abv") {
        Some(c) => c,
        None => {
            state.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let input = CalcInput::new()
        .add_param("og", &state.og)
        .add_param("fg", &state.fg);

    match calc.calculate(input) {
        Ok(res) => {
            state.result = Some(format!("✓ ABV: {:.2}%", res.output.value));
            state.warnings = res.warnings.clone();
            state.metadata = res.metadata.clone();
        }
        Err(e) => {
            state.result = Some(format!("❌ {}", e));
            state.warnings.clear();
            state.metadata.clear();
        }
    }
}

fn calc_brix_to_sg(state: &mut AppState) {
    let calc = match get_calculator("brix_to_sg") {
        Some(c) => c,
        None => {
            state.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let brix = match Decimal::from_str(&state.brix) {
        Ok(v) => v,
        Err(_) => {
            state.result = Some("❌ Invalid Brix value".to_string());
            return;
        }
    };

    let measurement = match Measurement::brix(brix) {
        Ok(m) => m,
        Err(e) => {
            state.result = Some(format!("❌ {}", e));
            return;
        }
    };

    let input = CalcInput::new().add_measurement(measurement);

    match calc.calculate(input) {
        Ok(res) => {
            state.result = Some(format!("✓ SG: {:.4}", res.output.value));
            state.warnings = res.warnings.clone();
            state.metadata = res.metadata.clone();
        }
        Err(e) => {
            state.result = Some(format!("❌ {}", e));
            state.warnings.clear();
            state.metadata.clear();
        }
    }
}

fn calc_sg_to_brix(state: &mut AppState) {
    let sg = match Decimal::from_str(&state.sg) {
        Ok(v) => v,
        Err(_) => {
            state.result = Some("❌ Invalid SG value".to_string());
            return;
        }
    };

    let brix = (sg - Decimal::ONE) * Decimal::from(250);

    state.result = Some(format!("✓ Brix: {:.2}°Bx", brix));
    state.warnings.clear();
    state.metadata.clear();
}

fn calc_sg_correction(state: &mut AppState) {
    let calc = match get_calculator("sg_correction") {
        Some(c) => c,
        None => {
            state.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let sg = match Decimal::from_str(&state.sg) {
        Ok(v) => v,
        Err(_) => {
            state.result = Some("❌ Invalid SG".to_string());
            return;
        }
    };

    let temp = match Decimal::from_str(&state.temp) {
        Ok(v) => v,
        Err(_) => {
            state.result = Some("❌ Invalid temperature".to_string());
            return;
        }
    };

    let sg_m = match Measurement::sg(sg) {
        Ok(m) => m,
        Err(e) => {
            state.result = Some(format!("❌ {}", e));
            return;
        }
    };

    let temp_m = match Measurement::celsius(temp) {
        Ok(m) => m,
        Err(e) => {
            state.result = Some(format!("❌ {}", e));
            return;
        }
    };

    let input = CalcInput::new()
        .add_measurement(sg_m)
        .add_measurement(temp_m);

    match calc.calculate(input) {
        Ok(res) => {
            state.result = Some(format!("✓ Corrected SG: {:.4}", res.output.value));
            state.warnings = res.warnings.clone();
            state.metadata = res.metadata.clone();
        }
        Err(e) => {
            state.result = Some(format!("❌ {}", e));
            state.warnings.clear();
            state.metadata.clear();
        }
    }
}

fn calc_dilution(state: &mut AppState) {
    let calc = match get_calculator("dilution") {
        Some(c) => c,
        None => {
            state.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let input = CalcInput::new()
        .add_param("current_volume", &state.current_vol)
        .add_param("current_abv", &state.current_abv)
        .add_param("target_abv", &state.target_abv);

    match calc.calculate(input) {
        Ok(res) => {
            state.result = Some(format!("✓ Water to Add: {:.2} L", res.output.value));
            state.warnings = res.warnings.clone();
            state.metadata = res.metadata.clone();
        }
        Err(e) => {
            state.result = Some(format!("❌ {}", e));
            state.warnings.clear();
            state.metadata.clear();
        }
    }
}