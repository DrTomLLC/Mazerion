//! Settings tab - COMPLETE WITH ALL DELIMITERS CLOSED

use crate::{MazerionApp, state::{colors, Theme, UnitSystem}};
use eframe::egui::{self, Color32, RichText, CornerRadius, Stroke, Vec2};
use rust_decimal::Decimal;
use std::str::FromStr;

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("⚙️ Settings & Reference").color(colors::SADDLE_BROWN).size(24.0));
    ui.add_space(10.0);

    egui::Frame::default()
        .fill(colors::LIGHT_CREAM)
        .stroke(Stroke::new(1.5, colors::HONEY_GOLD))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.label(RichText::new("🎨 Theme Selection").strong().size(16.0));
            ui.horizontal(|ui| {
                ui.label("Theme:");
                egui::ComboBox::from_id_salt("theme_select")
                    .selected_text(app.state.theme.name())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.state.theme, Theme::HoneyGold, Theme::HoneyGold.name());
                        ui.selectable_value(&mut app.state.theme, Theme::ForestGreen, Theme::ForestGreen.name());
                        ui.selectable_value(&mut app.state.theme, Theme::OceanBlue, Theme::OceanBlue.name());
                        ui.selectable_value(&mut app.state.theme, Theme::SunsetOrange, Theme::SunsetOrange.name());
                        ui.selectable_value(&mut app.state.theme, Theme::LavenderPurple, Theme::LavenderPurple.name());
                    });
            });

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(15.0);

            ui.label(RichText::new("📏 Unit System").strong().size(16.0));
            ui.horizontal(|ui| {
                ui.label("Units:");
                egui::ComboBox::from_id_salt("unit_system")
                    .selected_text(app.state.unit_system.name())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.state.unit_system, UnitSystem::Metric, "Metric (L, kg, °C)");
                        ui.selectable_value(&mut app.state.unit_system, UnitSystem::Imperial, "Imperial/US (gal, lb, °F)");
                    });
            });
            ui.label(RichText::new("⚠️ Changes update ALL calculator labels").color(colors::DARK_ORANGE).size(12.0));

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(15.0);

            ui.label(RichText::new("🔢 Display Precision").strong().size(16.0));

            ui.horizontal(|ui| {
                ui.label("Specific Gravity:");
                ui.add(egui::Slider::new(&mut app.state.sg_precision, 1..=6).text("decimals"));
            });

            ui.horizontal(|ui| {
                ui.label("pH:");
                ui.add(egui::Slider::new(&mut app.state.ph_precision, 1..=4).text("decimals"));
            });

            ui.horizontal(|ui| {
                ui.label("Brix/Plato:");
                ui.add(egui::Slider::new(&mut app.state.brix_precision, 0..=4).text("decimals"));
            });

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(15.0);

            ui.label(RichText::new("🔄 Unit Converter").strong().size(16.0));
            ui.horizontal(|ui| {
                ui.label("Value:");
                ui.text_edit_singleline(&mut app.conv_value);
            });

            ui.horizontal(|ui| {
                ui.label("From:");
                egui::ComboBox::from_id_salt("conv_from")
                    .selected_text(short_unit(&app.conv_from_unit))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.conv_from_unit, "liters".to_string(), "Liters");
                        ui.selectable_value(&mut app.conv_from_unit, "gallons".to_string(), "Gallons");
                        ui.selectable_value(&mut app.conv_from_unit, "kilograms".to_string(), "Kilograms");
                        ui.selectable_value(&mut app.conv_from_unit, "pounds".to_string(), "Pounds");
                        ui.selectable_value(&mut app.conv_from_unit, "celsius".to_string(), "Celsius");
                        ui.selectable_value(&mut app.conv_from_unit, "fahrenheit".to_string(), "Fahrenheit");
                    });

                ui.label("→");

                ui.label("To:");
                egui::ComboBox::from_id_salt("conv_to")
                    .selected_text(short_unit(&app.conv_to_unit))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.conv_to_unit, "liters".to_string(), "Liters");
                        ui.selectable_value(&mut app.conv_to_unit, "gallons".to_string(), "Gallons");
                        ui.selectable_value(&mut app.conv_to_unit, "kilograms".to_string(), "Kilograms");
                        ui.selectable_value(&mut app.conv_to_unit, "pounds".to_string(), "Pounds");
                        ui.selectable_value(&mut app.conv_to_unit, "celsius".to_string(), "Celsius");
                        ui.selectable_value(&mut app.conv_to_unit, "fahrenheit".to_string(), "Fahrenheit");
                    });
            });

            if ui.add(
                egui::Button::new(RichText::new("Convert").color(Color32::WHITE).strong())
                    .fill(colors::FOREST_GREEN)
                    .min_size(Vec2::new(120.0, 28.0))
            ).clicked() {
                perform_conversion(app);
            }

            if let Some(ref result) = app.conv_result {
                ui.label(RichText::new(result).size(15.0).strong());
            }

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(15.0);

            ui.label(RichText::new("📊 Reference Tables").strong().size(16.0));

            ui.columns(2, |columns| {
                columns[0].group(|ui| {
                    ui.label(RichText::new("💧 Volume").strong());
                    ui.label("1 L = 0.264 gal");
                    ui.label("1 gal = 3.785 L");
                    ui.label("5 gal = 18.93 L");
                });

                columns[1].group(|ui| {
                    ui.label(RichText::new("⚖️ Weight").strong());
                    ui.label("1 kg = 2.205 lb");
                    ui.label("1 lb = 0.454 kg");
                    ui.label("1 oz = 28.35 g");
                });
            });

            ui.add_space(10.0);

            ui.columns(2, |columns| {
                columns[0].group(|ui| {
                    ui.label(RichText::new("🌡️ Temperature").strong());
                    ui.label("°F = (°C × 9/5) + 32");
                    ui.label("°C = (°F - 32) × 5/9");
                    ui.label("20°C = 68°F");
                });

                columns[1].group(|ui| {
                    ui.label(RichText::new("📊 Gravity").strong());
                    ui.label("1.040 ≈ 10° Brix");
                    ui.label("1.080 ≈ 19° Brix");
                    ui.label("SG ≈ 1+(Bx×0.004)");
                });
            });

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(10.0);

            ui.label(RichText::new("ℹ️ About Mazerion v0.10.4").strong().size(14.0));
            ui.label("40 calculators • 7 categories • Zero panics");
        });
}

fn short_unit(unit: &str) -> &str {
    match unit {
        "liters" => "L",
        "gallons" => "gal",
        "kilograms" => "kg",
        "pounds" => "lb",
        "celsius" => "°C",
        "fahrenheit" => "°F",
        _ => unit,
    }
}

fn perform_conversion(app: &mut MazerionApp) {
    let value = match Decimal::from_str(&app.conv_value) {
        Ok(v) => v,
        Err(_) => {
            app.conv_result = Some("❌ Invalid number".to_string());
            return;
        }
    };

    let result = match (app.conv_from_unit.as_str(), app.conv_to_unit.as_str()) {
        ("liters", "gallons") => mazerion_core::liters_to_gallons(value),
        ("gallons", "liters") => mazerion_core::gallons_to_liters(value),
        ("kilograms", "pounds") => mazerion_core::kilograms_to_pounds(value),
        ("pounds", "kilograms") => mazerion_core::pounds_to_kilograms(value),
        ("celsius", "fahrenheit") => mazerion_core::celsius_to_fahrenheit(value),
        ("fahrenheit", "celsius") => mazerion_core::fahrenheit_to_celsius(value),
        (from, to) if from == to => value,
        _ => {
            app.conv_result = Some("❌ Cannot convert between unit types".to_string());
            return;
        }
    };

    app.conv_result = Some(format!(
        "✓ {} {} = {:.4} {}",
        app.conv_value,
        short_unit(&app.conv_from_unit),
        result,
        short_unit(&app.conv_to_unit)
    ));
}