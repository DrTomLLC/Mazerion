//! Advanced calculations tab - COMPLETE IMPLEMENTATION

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
    // Blending Calculator
    section(ui, "🔀 Blending Calculator", |ui| {
        ui.label(RichText::new("Batch 1:").strong());
        field(ui, "Volume (L):", &mut state.vol1);
        field(ui, "ABV (%):", &mut state.abv1);
        ui.add_space(5.0);
        ui.label(RichText::new("Batch 2:").strong());
        field(ui, "Volume (L):", &mut state.vol2);
        field(ui, "ABV (%):", &mut state.abv2);
        if button(ui, "Calculate Blend") {
            calc_blending(state);
        }
    });

    ui.add_space(10.0);

    // Refractometer Correction
    section(ui, "🔍 Refractometer Correction", |ui| {
        field(ui, "Original Brix (°Bx):", &mut state.orig_brix);
        field(ui, "Current Brix (°Bx):", &mut state.curr_brix);
        if button(ui, "Calculate True FG") {
            calc_refractometer(state);
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

fn calc_blending(state: &mut AppState) {
    let calc = match get_calculator("blending") {
        Some(c) => c,
        None => {
            state.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let input = CalcInput::new()
        .add_param("volume1", &state.vol1)
        .add_param("abv1", &state.abv1)
        .add_param("volume2", &state.vol2)
        .add_param("abv2", &state.abv2);

    match calc.calculate(input) {
        Ok(res) => {
            state.result = Some(format!("✓ Blended ABV: {:.2}%", res.output.value));
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

fn calc_refractometer(state: &mut AppState) {
    let calc = match get_calculator("refractometer") {
        Some(c) => c,
        None => {
            state.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let orig_brix = match Decimal::from_str(&state.orig_brix) {
        Ok(v) => v,
        Err(_) => {
            state.result = Some("❌ Invalid original Brix".to_string());
            return;
        }
    };

    let measurement = match Measurement::brix(orig_brix) {
        Ok(m) => m,
        Err(e) => {
            state.result = Some(format!("❌ {}", e));
            return;
        }
    };

    let input = CalcInput::new()
        .add_measurement(measurement)
        .add_param("current_brix", &state.curr_brix);

    match calc.calculate(input) {
        Ok(res) => {
            state.result = Some(format!("✓ True FG: {:.4}", res.output.value));
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