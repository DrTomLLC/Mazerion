//! Settings tab - Configuration and preferences

use crate::state::AppState;
use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};

const BG_PANEL: Color32 = Color32::from_rgb(255, 255, 255);
const TEXT_MAIN: Color32 = Color32::from_rgb(30, 30, 30);
const TEXT_LABEL: Color32 = Color32::from_rgb(60, 60, 60);
const BORDER: Color32 = Color32::from_rgb(218, 165, 32);

pub fn render(ui: &mut egui::Ui, state: &mut AppState) {
    section(ui, "⚙️ Display Settings", |ui| {
        ui.label(RichText::new("Theme:").color(TEXT_LABEL).strong());
        ui.horizontal(|ui| {
            ui.radio_value(&mut state.theme, "soft_blue".to_string(), "Soft Blue (Current)");
            ui.radio_value(&mut state.theme, "light".to_string(), "Light Gray");
            ui.radio_value(&mut state.theme, "cream".to_string(), "Cream");
        });

        ui.add_space(10.0);

        ui.label(RichText::new("Font Size:").color(TEXT_LABEL).strong());
        ui.horizontal(|ui| {
            ui.radio_value(&mut state.font_size, "small".to_string(), "Small");
            ui.radio_value(&mut state.font_size, "medium".to_string(), "Medium (Current)");
            ui.radio_value(&mut state.font_size, "large".to_string(), "Large");
        });
    });

    ui.add_space(10.0);

    section(ui, "📐 Calculation Settings", |ui| {
        ui.label(RichText::new("Default Volume Unit:").color(TEXT_LABEL).strong());
        ui.horizontal(|ui| {
            ui.radio_value(&mut state.volume_unit, "liters".to_string(), "Liters");
            ui.radio_value(&mut state.volume_unit, "gallons".to_string(), "Gallons (US)");
        });

        ui.add_space(10.0);

        ui.label(RichText::new("Temperature Unit:").color(TEXT_LABEL).strong());
        ui.horizontal(|ui| {
            ui.radio_value(&mut state.temp_unit, "celsius".to_string(), "Celsius");
            ui.radio_value(&mut state.temp_unit, "fahrenheit".to_string(), "Fahrenheit");
        });

        ui.add_space(10.0);

        ui.checkbox(&mut state.show_warnings, "Show calculation warnings");
        ui.checkbox(&mut state.show_metadata, "Show detailed metadata");
        ui.checkbox(&mut state.auto_save, "Auto-save calculations to history");
    });

    ui.add_space(10.0);

    section(ui, "🍯 Brewing Defaults", |ui| {
        ui.label(RichText::new("Default Batch Size:").color(TEXT_LABEL).strong());
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut state.default_batch_size);
            ui.label("L");
        });

        ui.add_space(5.0);

        ui.label(RichText::new("Default Yeast Requirement:").color(TEXT_LABEL).strong());
        ui.horizontal(|ui| {
            ui.radio_value(&mut state.default_yn_req, "low".to_string(), "Low");
            ui.radio_value(&mut state.default_yn_req, "medium".to_string(), "Medium");
            ui.radio_value(&mut state.default_yn_req, "high".to_string(), "High");
        });

        ui.add_space(5.0);

        ui.label(RichText::new("Default Fermentation Temp:").color(TEXT_LABEL).strong());
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut state.default_ferm_temp);
            ui.label("°C");
        });
    });

    ui.add_space(10.0);

    section(ui, "📊 About Mazerion", |ui| {
        ui.label(RichText::new("Version:").strong());
        ui.label("0.2.0");

        ui.add_space(5.0);

        ui.label(RichText::new("Features:").strong());
        ui.label("• 11 Professional Calculators");
        ui.label("• Decimal Precision (rust_decimal)");
        ui.label("• Zero Panics - Production Ready");
        ui.label("• TOSNA 2.0 Nutrition Protocol");
        ui.label("• Terrill Cubic Refractometer Correction");
        ui.label("• pH-Dependent Sulfite Calculations");

        ui.add_space(5.0);

        ui.label(RichText::new("License:").strong());
        ui.label("MIT OR Apache-2.0");
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