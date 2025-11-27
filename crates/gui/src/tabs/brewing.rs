//! Brewing tab - COMPLETE IMPLEMENTATION

use crate::state::AppState;
use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};
use mazerion_core::{CalcInput, traits::get_calculator};

const BG_PANEL: Color32 = Color32::from_rgb(255, 255, 255);
const TEXT_MAIN: Color32 = Color32::from_rgb(30, 30, 30);
const TEXT_LABEL: Color32 = Color32::from_rgb(60, 60, 60);
const BORDER: Color32 = Color32::from_rgb(218, 165, 32);
const BUTTON: Color32 = Color32::from_rgb(34, 139, 34);
const BUTTON_TEXT: Color32 = Color32::WHITE;

pub fn render(ui: &mut egui::Ui, state: &mut AppState) {
    // TOSNA Nutrition Calculator
    section(ui, "🧪 TOSNA Nutrition Calculator", |ui| {
        field(ui, "Volume (L):", &mut state.volume);
        field(ui, "Target ABV (%):", &mut state.target_abv_brew);

        ui.horizontal(|ui| {
            ui.label(RichText::new("Yeast N Requirements:").color(TEXT_LABEL).strong());
            egui::ComboBox::from_id_source("yn_req")
                .selected_text(&state.yn_requirement)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.yn_requirement, "low".to_string(), "Low (DV10, QA23)");
                    ui.selectable_value(&mut state.yn_requirement, "medium".to_string(), "Medium (most yeasts)");
                    ui.selectable_value(&mut state.yn_requirement, "high".to_string(), "High (EC-1118, K1-V1116)");
                });
        });

        if button(ui, "Calculate Fermaid-O Schedule") {
            calc_nutrition(state);
        }
    });

    ui.add_space(10.0);

    // Carbonation Calculator
    section(ui, "🫧 Carbonation Calculator", |ui| {
        field(ui, "Volume (L):", &mut state.volume);
        field(ui, "Temperature (°C):", &mut state.carb_temp);
        field(ui, "Target CO₂ (volumes):", &mut state.target_co2);

        ui.horizontal(|ui| {
            ui.label(RichText::new("Method:").color(TEXT_LABEL).strong());
            egui::ComboBox::from_id_source("carb_method")
                .selected_text(if state.carb_method == "priming" { "Bottle Priming" } else { "Force Carbonation" })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.carb_method, "priming".to_string(), "Bottle Priming");
                    ui.selectable_value(&mut state.carb_method, "keg".to_string(), "Force Carbonation (Keg)");
                });
        });

        if state.carb_method == "priming" {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Sugar Type:").color(TEXT_LABEL).strong());
                egui::ComboBox::from_id_source("sugar_type")
                    .selected_text(&state.sugar_type)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut state.sugar_type, "table_sugar".to_string(), "Table Sugar");
                        ui.selectable_value(&mut state.sugar_type, "corn_sugar".to_string(), "Corn Sugar");
                        ui.selectable_value(&mut state.sugar_type, "honey".to_string(), "Honey");
                        ui.selectable_value(&mut state.sugar_type, "dme".to_string(), "DME");
                    });
            });
        }

        if button(ui, "Calculate Carbonation") {
            calc_carbonation(state);
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

fn calc_nutrition(state: &mut AppState) {
    let calc = match get_calculator("nutrition") {
        Some(c) => c,
        None => {
            state.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let input = CalcInput::new()
        .add_param("volume", &state.volume)
        .add_param("target_abv", &state.target_abv_brew)
        .add_param("yn_requirement", &state.yn_requirement);

    match calc.calculate(input) {
        Ok(res) => {
            state.result = Some(format!("✓ Total Fermaid-O: {:.2}g", res.output.value));
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

fn calc_carbonation(state: &mut AppState) {
    let calc = match get_calculator("carbonation") {
        Some(c) => c,
        None => {
            state.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let input = CalcInput::new()
        .add_param("volume", &state.volume)
        .add_param("temperature", &state.carb_temp)
        .add_param("target_co2", &state.target_co2)
        .add_param("method", &state.carb_method)
        .add_param("sugar_type", &state.sugar_type);

    match calc.calculate(input) {
        Ok(res) => {
            if state.carb_method == "priming" {
                state.result = Some(format!("✓ Priming Sugar: {:.1}g", res.output.value));
            } else {
                state.result = Some(format!("✓ Keg PSI: {:.1}", res.output.value));
            }
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