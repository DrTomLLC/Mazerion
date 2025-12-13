//! Settings tab with theme and unit selection

use crate::MazerionApp;
use crate::state::{Theme, UnitSystem};
use eframe::egui::{self, RichText, CornerRadius};

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    egui::Frame::default()
        .fill(crate::state::colors::LIGHT_CREAM)
        .stroke(egui::Stroke::new(1.5, crate::state::colors::HONEY_GOLD))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.heading(RichText::new("⚙️ Settings").color(crate::state::colors::SADDLE_BROWN));
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Color Theme:");
                egui::ComboBox::from_id_salt("theme")
                    .selected_text(app.state.theme.name())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.state.theme, Theme::HoneyGold, "🍯 Honey & Gold");
                        ui.selectable_value(&mut app.state.theme, Theme::ForestGreen, "🌲 Forest Green");
                        ui.selectable_value(&mut app.state.theme, Theme::OceanBlue, "🌊 Ocean Blue");
                        ui.selectable_value(&mut app.state.theme, Theme::SunsetOrange, "🌅 Sunset Orange");
                        ui.selectable_value(&mut app.state.theme, Theme::LavenderPurple, "💜 Lavender Purple");
                    });
            });
            app.state.custom_colors = app.state.get_theme_colors();

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Unit System:");
                egui::ComboBox::from_id_salt("units")
                    .selected_text(app.state.unit_system.name())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.state.unit_system, UnitSystem::Metric, "Metric");
                        ui.selectable_value(&mut app.state.unit_system, UnitSystem::Imperial, "Imperial/US");
                    });
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("SG Precision:");
                egui::ComboBox::from_id_salt("sg_precision")
                    .selected_text(format!("{} decimals", app.state.sg_precision))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.state.sg_precision, 3, "3 decimals");
                        ui.selectable_value(&mut app.state.sg_precision, 4, "4 decimals");
                        ui.selectable_value(&mut app.state.sg_precision, 5, "5 decimals");
                    });
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("pH Precision:");
                egui::ComboBox::from_id_salt("ph_precision")
                    .selected_text(format!("{} decimals", app.state.ph_precision))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.state.ph_precision, 2, "2 decimals");
                        ui.selectable_value(&mut app.state.ph_precision, 3, "3 decimals");
                        ui.selectable_value(&mut app.state.ph_precision, 4, "4 decimals");
                    });
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            ui.heading(RichText::new("🔧 Unit Converter").color(crate::state::colors::SADDLE_BROWN));
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Value:");
                ui.text_edit_singleline(&mut app.conv_value);
            });

            ui.horizontal(|ui| {
                ui.label("From:");
                egui::ComboBox::from_id_salt("conv_from")
                    .selected_text(short_unit(&app.conv_from_unit))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.conv_from_unit, "liters".to_string(), "Liters (L)");
                        ui.selectable_value(&mut app.conv_from_unit, "gallons".to_string(), "Gallons (gal)");
                        ui.selectable_value(&mut app.conv_from_unit, "kilograms".to_string(), "Kilograms (kg)");
                        ui.selectable_value(&mut app.conv_from_unit, "pounds".to_string(), "Pounds (lb)");
                        ui.selectable_value(&mut app.conv_from_unit, "grams".to_string(), "Grams (g)");
                        ui.selectable_value(&mut app.conv_from_unit, "ounces".to_string(), "Ounces (oz)");
                        ui.selectable_value(&mut app.conv_from_unit, "celsius".to_string(), "Celsius (°C)");
                        ui.selectable_value(&mut app.conv_from_unit, "fahrenheit".to_string(), "Fahrenheit (°F)");
                    });
            });

            ui.horizontal(|ui| {
                ui.label("To:");
                egui::ComboBox::from_id_salt("conv_to")
                    .selected_text(short_unit(&app.conv_to_unit))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.conv_to_unit, "liters".to_string(), "Liters (L)");
                        ui.selectable_value(&mut app.conv_to_unit, "gallons".to_string(), "Gallons (gal)");
                        ui.selectable_value(&mut app.conv_to_unit, "kilograms".to_string(), "Kilograms (kg)");
                        ui.selectable_value(&mut app.conv_to_unit, "pounds".to_string(), "Pounds (lb)");
                        ui.selectable_value(&mut app.conv_to_unit, "grams".to_string(), "Grams (g)");
                        ui.selectable_value(&mut app.conv_to_unit, "ounces".to_string(), "Ounces (oz)");
                        ui.selectable_value(&mut app.conv_to_unit, "celsius".to_string(), "Celsius (°C)");
                        ui.selectable_value(&mut app.conv_to_unit, "fahrenheit".to_string(), "Fahrenheit (°F)");
                    });
            });

            if crate::calculate_button(ui, "Convert") {
                app.conv_result = Some(perform_conversion(&app.conv_value, &app.conv_from_unit, &app.conv_to_unit));
            }

            if let Some(result) = &app.conv_result {
                ui.add_space(10.0);
                ui.label(RichText::new(result).size(18.0));
            }
        });
}

fn short_unit(unit: &str) -> &str {
    match unit {
        "liters" => "L",
        "gallons" => "gal",
        "kilograms" => "kg",
        "pounds" => "lb",
        "grams" => "g",
        "ounces" => "oz",
        "celsius" => "°C",
        "fahrenheit" => "°F",
        _ => unit,
    }
}

fn perform_conversion(value: &str, from: &str, to: &str) -> String {
    let val: f64 = match value.parse() {
        Ok(v) => v,
        Err(_) => return "❌ Invalid number".to_string(),
    };

    let result = match (from, to) {
        ("liters", "gallons") => val * 0.264172,
        ("gallons", "liters") => val * 3.78541,
        ("kilograms", "pounds") => val * 2.20462,
        ("pounds", "kilograms") => val * 0.453592,
        ("grams", "ounces") => val * 0.035274,
        ("ounces", "grams") => val * 28.3495,
        ("celsius", "fahrenheit") => (val * 9.0 / 5.0) + 32.0,
        ("fahrenheit", "celsius") => (val - 32.0) * 5.0 / 9.0,
        _ if from == to => val,
        _ => return "❌ Incompatible units".to_string(),
    };

    format!("{:.4} {} = {:.4} {}", val, short_unit(from), result, short_unit(to))
}