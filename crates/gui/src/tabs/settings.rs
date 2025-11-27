//! Settings tab - FIXED borrow checker issues

use crate::state::AppState;
use eframe::egui::{self, Color32, RichText, Rounding, Stroke};

const BG_PANEL: Color32 = Color32::from_rgb(245, 250, 255);
const TEXT_MAIN: Color32 = Color32::from_rgb(30, 30, 30);
const TEXT_ACCENT: Color32 = Color32::from_rgb(70, 130, 180);
const SELECTED: Color32 = Color32::from_rgb(70, 130, 180);
const SELECTED_TEXT: Color32 = Color32::WHITE;
const UNSELECTED: Color32 = Color32::from_rgb(220, 230, 240);
const BORDER: Color32 = Color32::from_rgb(70, 130, 180);

pub fn render(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading(RichText::new("⚙️ Settings & Preferences").color(TEXT_ACCENT).size(20.0));
    ui.add_space(10.0);

    section(ui, "🎨 Display Settings", |ui| {
        ui.label(RichText::new("Theme:").strong());
        ui.horizontal(|ui| {
            let current_theme = state.theme.clone();
            if radio_button_styled(ui, "Soft Blue", current_theme == "soft_blue").clicked() {
                state.theme = "soft_blue".to_string();
            }
            if radio_button_styled(ui, "Light Gray", current_theme == "light").clicked() {
                state.theme = "light".to_string();
            }
            if radio_button_styled(ui, "Cream", current_theme == "cream").clicked() {
                state.theme = "cream".to_string();
            }
        });

        ui.add_space(8.0);
        ui.label(RichText::new("Font Size:").strong());
        ui.horizontal(|ui| {
            let current_size = state.font_size.clone();
            if radio_button_styled(ui, "Small", current_size == "small").clicked() {
                state.font_size = "small".to_string();
            }
            if radio_button_styled(ui, "Medium", current_size == "medium").clicked() {
                state.font_size = "medium".to_string();
            }
            if radio_button_styled(ui, "Large", current_size == "large").clicked() {
                state.font_size = "large".to_string();
            }
        });
    });

    ui.add_space(10.0);

    section(ui, "📐 Calculation Settings", |ui| {
        ui.label(RichText::new("Default Volume Unit:").strong());
        ui.horizontal(|ui| {
            let current_vol = state.volume_unit.clone();
            if radio_button_styled(ui, "Liters (L)", current_vol == "liters").clicked() {
                state.volume_unit = "liters".to_string();
            }
            if radio_button_styled(ui, "Gallons (US)", current_vol == "gallons").clicked() {
                state.volume_unit = "gallons".to_string();
            }
        });

        ui.add_space(8.0);
        ui.label(RichText::new("Default Temperature Unit:").strong());
        ui.horizontal(|ui| {
            let current_temp = state.temp_unit.clone();
            if radio_button_styled(ui, "Celsius (°C)", current_temp == "celsius").clicked() {
                state.temp_unit = "celsius".to_string();
            }
            if radio_button_styled(ui, "Fahrenheit (°F)", current_temp == "fahrenheit").clicked() {
                state.temp_unit = "fahrenheit".to_string();
            }
        });

        ui.add_space(8.0);
        ui.checkbox(&mut state.show_warnings, "Show calculation warnings");
        ui.checkbox(&mut state.show_metadata, "Show detailed metadata");
        ui.checkbox(&mut state.auto_save, "Auto-save calculations to history");
    });

    ui.add_space(10.0);

    section(ui, "🍺 Brewing Defaults", |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Default Batch Size:").strong());
            ui.text_edit_singleline(&mut state.default_batch_size);
            ui.label("L");
        });

        ui.add_space(8.0);
        ui.label(RichText::new("Default Yeast Nitrogen Requirement:").strong());
        ui.horizontal(|ui| {
            let current_yn = state.default_yn_req.clone();
            if radio_button_styled(ui, "Low", current_yn == "low").clicked() {
                state.default_yn_req = "low".to_string();
            }
            if radio_button_styled(ui, "Medium", current_yn == "medium").clicked() {
                state.default_yn_req = "medium".to_string();
            }
            if radio_button_styled(ui, "High", current_yn == "high").clicked() {
                state.default_yn_req = "high".to_string();
            }
        });

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new("Default Fermentation Temp:").strong());
            ui.text_edit_singleline(&mut state.default_ferm_temp);
            ui.label("°C");
        });
    });

    ui.add_space(10.0);

    section(ui, "ℹ️ About Mazerion", |ui| {
        ui.label(RichText::new("Version:").strong());
        ui.label("0.2.0");

        ui.add_space(8.0);
        ui.label(RichText::new("Features:").strong());
        ui.label("• 11 Professional Calculators");
        ui.label("• Decimal Precision (rust_decimal)");
        ui.label("• Zero Panics Guarantee");
        ui.label("• TOSNA 2.0 Nutrition Protocol");
        ui.label("• Terrill Cubic Refractometer Correction");
        ui.label("• pH-Dependent Sulfite Calculations");
        ui.label("• Interactive Unit Conversions");

        ui.add_space(8.0);
        ui.label(RichText::new("License:").strong());
        ui.label("MIT OR Apache-2.0");

        ui.add_space(8.0);
        ui.label(RichText::new("Calculator Categories:").strong());
        ui.label("📊 Basic: ABV, Brix↔SG, Dilution");
        ui.label("🔬 Advanced: Blending, Refractometer, SG Correction");
        ui.label("🍺 Brewing: TOSNA Nutrition, Carbonation");
        ui.label("✨ Finishing: Backsweetening, Sulfite, Acid Addition");
        ui.label("📐 Conversions: Volume, Weight, Temperature, Gravity");
    });
}

fn section(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(BG_PANEL)
        .stroke(Stroke::new(1.5, BORDER))
        .rounding(Rounding::same(8.0))
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.label(RichText::new(title).color(TEXT_ACCENT).size(18.0).strong());
            ui.add_space(8.0);
            content(ui);
        });
}

fn radio_button_styled(ui: &mut egui::Ui, label: &str, is_selected: bool) -> egui::Response {
    let button_color = if is_selected { SELECTED } else { UNSELECTED };
    let text_color = if is_selected { SELECTED_TEXT } else { TEXT_MAIN };

    let button = egui::Button::new(
        RichText::new(label)
            .color(text_color)
            .size(14.0)
            .strong()
    )
        .fill(button_color)
        .rounding(Rounding::same(6.0))
        .min_size(egui::Vec2::new(100.0, 32.0));

    ui.add(button)
}