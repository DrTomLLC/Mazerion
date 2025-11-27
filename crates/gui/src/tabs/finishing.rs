//! Finishing tab - COMPLETE IMPLEMENTATION

use crate::state::AppState;
use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};
use mazerion_core::{CalcInput, Measurement, traits::get_calculator};
use rust_decimal::Decimal;
use std::str::FromStr;

const BG_PANEL: Color32 = Color32::from_rgb(255, 255, 255);
const TEXT_MAIN: Color32 = Color32::from_rgb(30, 30, 30);
const TEXT_LABEL: Color32 = Color32::from_rgb(60, 60, 60);
const TEXT_WARN: Color32 = Color32::from_rgb(255, 140, 0);
const BORDER: Color32 = Color32::from_rgb(218, 165, 32);
const BUTTON: Color32 = Color32::from_rgb(34, 139, 34);
const BUTTON_TEXT: Color32 = Color32::WHITE;

pub fn render(ui: &mut egui::Ui, state: &mut AppState) {
    // Backsweetening Calculator
    section(ui, "🍯 Backsweetening Calculator", |ui| {
        ui.label(RichText::new("⚠️ MUST stabilize before backsweetening!").color(TEXT_WARN).strong());
        ui.add_space(5.0);

        field(ui, "Volume (L):", &mut state.sweet_vol);
        field(ui, "Current SG:", &mut state.current_sg);
        field(ui, "Target SG:", &mut state.target_sg);

        ui.horizontal(|ui| {
            ui.label(RichText::new("Sweetener:").color(TEXT_LABEL).strong());
            egui::ComboBox::from_id_source("sweetener")
                .selected_text(&state.sweetener)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.sweetener, "honey".to_string(), "Honey");
                    ui.selectable_value(&mut state.sweetener, "table_sugar".to_string(), "Table Sugar");
                    ui.selectable_value(&mut state.sweetener, "agave".to_string(), "Agave Nectar");
                    ui.selectable_value(&mut state.sweetener, "maple_syrup".to_string(), "Maple Syrup");
                });
        });

        if button(ui, "Calculate Sweetener Amount") {
            calc_backsweetening(state);
        }
    });

    ui.add_space(10.0);

    // Sulfite Calculator
    section(ui, "🛡️ Sulfite Calculator", |ui| {
        field(ui, "Volume (L):", &mut state.sulfite_vol);
        field(ui, "pH:", &mut state.ph);
        field(ui, "Target Free SO₂ (ppm):", &mut state.target_so2);

        if button(ui, "Calculate K-meta Addition") {
            calc_sulfite(state);
        }
    });

    ui.add_space(10.0);

    // Acid Addition Calculator
    section(ui, "🍋 Acid Addition Calculator", |ui| {
        field(ui, "Volume (L):", &mut state.acid_vol);
        field(ui, "Current pH:", &mut state.current_ph);
        field(ui, "Target pH:", &mut state.target_ph_acid);

        ui.horizontal(|ui| {
            ui.label(RichText::new("Acid Type:").color(TEXT_LABEL).strong());
            egui::ComboBox::from_id_source("acid_type")
                .selected_text(&state.acid_type)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.acid_type, "tartaric".to_string(), "Tartaric (strongest)");
                    ui.selectable_value(&mut state.acid_type, "citric".to_string(), "Citric (bright)");
                    ui.selectable_value(&mut state.acid_type, "malic".to_string(), "Malic (soft)");
                    ui.selectable_value(&mut state.acid_type, "lactic".to_string(), "Lactic (smooth)");
                });
        });

        if button(ui, "Calculate Acid Addition") {
            calc_acid_addition(state);
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
            .min_size(Vec2::new(220.0, 36.0))
    ).clicked()
}

fn calc_backsweetening(state: &mut AppState) {
    let calc = match get_calculator("backsweetening") {
        Some(c) => c,
        None => {
            state.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let current_sg = match Decimal::from_str(&state.current_sg) {
        Ok(v) => v,
        Err(_) => {
            state.result = Some("❌ Invalid current SG".to_string());
            return;
        }
    };

    let measurement = match Measurement::sg(current_sg) {
        Ok(m) => m,
        Err(e) => {
            state.result = Some(format!("❌ {}", e));
            return;
        }
    };

    let input = CalcInput::new()
        .add_measurement(measurement)
        .add_param("volume", &state.sweet_vol)
        .add_param("target_sg", &state.target_sg)
        .add_param("sweetener", &state.sweetener);

    match calc.calculate(input) {
        Ok(res) => {
            state.result = Some(format!("✓ {}: {:.0}g ({:.2}kg)",
                                        match state.sweetener.as_str() {
                                            "honey" => "Honey",
                                            "table_sugar" => "Table Sugar",
                                            "agave" => "Agave",
                                            "maple_syrup" => "Maple Syrup",
                                            _ => "Sweetener"
                                        },
                                        res.output.value,
                                        res.output.value / Decimal::from(1000)
            ));
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

fn calc_sulfite(state: &mut AppState) {
    let calc = match get_calculator("sulfite") {
        Some(c) => c,
        None => {
            state.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let ph = match Decimal::from_str(&state.ph) {
        Ok(v) => v,
        Err(_) => {
            state.result = Some("❌ Invalid pH".to_string());
            return;
        }
    };

    let measurement = match Measurement::ph(ph) {
        Ok(m) => m,
        Err(e) => {
            state.result = Some(format!("❌ {}", e));
            return;
        }
    };

    let input = CalcInput::new()
        .add_measurement(measurement)
        .add_param("volume", &state.sulfite_vol)
        .add_param("target_free_so2", &state.target_so2);

    match calc.calculate(input) {
        Ok(res) => {
            state.result = Some(format!("✓ K-meta: {:.2}g", res.output.value));
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

fn calc_acid_addition(state: &mut AppState) {
    let calc = match get_calculator("acid_addition") {
        Some(c) => c,
        None => {
            state.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let current_ph = match Decimal::from_str(&state.current_ph) {
        Ok(v) => v,
        Err(_) => {
            state.result = Some("❌ Invalid current pH".to_string());
            return;
        }
    };

    let measurement = match Measurement::ph(current_ph) {
        Ok(m) => m,
        Err(e) => {
            state.result = Some(format!("❌ {}", e));
            return;
        }
    };

    let input = CalcInput::new()
        .add_measurement(measurement)
        .add_param("volume", &state.acid_vol)
        .add_param("target_ph", &state.target_ph_acid)
        .add_param("acid_type", &state.acid_type);

    match calc.calculate(input) {
        Ok(res) => {
            state.result = Some(format!("✓ {} Acid: {:.2}g",
                                        match state.acid_type.as_str() {
                                            "tartaric" => "Tartaric",
                                            "citric" => "Citric",
                                            "malic" => "Malic",
                                            "lactic" => "Lactic",
                                            _ => "Acid"
                                        },
                                        res.output.value
            ));
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