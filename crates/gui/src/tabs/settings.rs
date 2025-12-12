//! Settings tab - Theme, Units, Precision, and Font Colors

use crate::MazerionApp;
use eframe::egui::{self, RichText, Color32};
use crate::state::{Theme, UnitSystem};

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("⚙️ Settings").size(24.0));
    ui.add_space(15.0);

    // Theme Selection
    ui.heading(RichText::new("🎨 Theme").size(18.0));
    ui.horizontal(|ui| {
        for theme in [Theme::HoneyGold, Theme::ForestGreen, Theme::OceanBlue, Theme::SunsetOrange, Theme::LavenderPurple] {
            if ui.selectable_label(app.state.theme == theme, theme.name()).clicked() {
                app.state.theme = theme;
            }
        }
    });

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(20.0);

    // Unit System Selection
    ui.heading(RichText::new("📏 Unit System").size(18.0));
    ui.horizontal(|ui| {
        if ui.selectable_label(app.state.unit_system == UnitSystem::Metric, "Metric").clicked() {
            app.state.unit_system = UnitSystem::Metric;
        }
        if ui.selectable_label(app.state.unit_system == UnitSystem::Imperial, "Imperial/US").clicked() {
            app.state.unit_system = UnitSystem::Imperial;
        }
    });

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(20.0);

    // Precision Settings
    ui.heading(RichText::new("🔢 Decimal Precision").size(18.0));
    ui.label("Number of decimal places for calculations");
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        ui.label("Specific Gravity:");
        ui.add(egui::Slider::new(&mut app.state.sg_precision, 2..=6).text("decimals"));
    });

    ui.horizontal(|ui| {
        ui.label("pH:");
        ui.add(egui::Slider::new(&mut app.state.ph_precision, 1..=4).text("decimals"));
    });

    ui.horizontal(|ui| {
        ui.label("Brix:");
        ui.add(egui::Slider::new(&mut app.state.brix_precision, 1..=4).text("decimals"));
    });

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(20.0);

    // Unit Converter
    ui.heading(RichText::new("🔄 Unit Converter").size(18.0));
    ui.label("Quick conversions between metric and imperial units");
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

    if ui.button(RichText::new("🔄 Convert").size(14.0)).clicked() {
        app.conv_result = Some(perform_conversion(&app.conv_value, &app.conv_from_unit, &app.conv_to_unit));
    }

    if let Some(result) = &app.conv_result {
        ui.add_space(10.0);
        ui.label(RichText::new(result).size(16.0).color(crate::state::colors::FOREST_GREEN).strong());
    }

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(20.0);

    // Font Color Customization
    ui.heading(RichText::new("🎨 Font Colors").size(18.0));
    ui.label("Customize text colors throughout the application");
    ui.add_space(10.0);

    egui::Grid::new("color_settings")
        .num_columns(3)
        .spacing([15.0, 10.0])
        .show(ui, |ui| {
            // Header
            ui.label(RichText::new("Color Name").strong());
            ui.label(RichText::new("Preview").strong());
            ui.label(RichText::new("Customize").strong());
            ui.end_row();

            // Honey Gold
            ui.label("Honey Gold");
            ui.label(RichText::new("Sample Text").color(app.state.custom_colors.honey_gold));
            ui.color_edit_button_srgba(&mut app.state.custom_colors.honey_gold);
            ui.end_row();

            // Light Cream (Background)
            ui.label("Light Cream (Background)");
            ui.colored_label(app.state.custom_colors.light_cream, "■■■■■");
            ui.color_edit_button_srgba(&mut app.state.custom_colors.light_cream);
            ui.end_row();

            // Dark Text
            ui.label("Dark Text");
            ui.label(RichText::new("Sample Text").color(app.state.custom_colors.dark_text));
            ui.color_edit_button_srgba(&mut app.state.custom_colors.dark_text);
            ui.end_row();

            // Saddle Brown
            ui.label("Saddle Brown");
            ui.label(RichText::new("Sample Text").color(app.state.custom_colors.saddle_brown));
            ui.color_edit_button_srgba(&mut app.state.custom_colors.saddle_brown);
            ui.end_row();

            // Dark Orange
            ui.label("Dark Orange (Warnings)");
            ui.label(RichText::new("Sample Text").color(app.state.custom_colors.dark_orange));
            ui.color_edit_button_srgba(&mut app.state.custom_colors.dark_orange);
            ui.end_row();

            // Forest Green
            ui.label("Forest Green");
            ui.label(RichText::new("Sample Text").color(app.state.custom_colors.forest_green));
            ui.color_edit_button_srgba(&mut app.state.custom_colors.forest_green);
            ui.end_row();

            // Dark Red
            ui.label("Dark Red (Errors)");
            ui.label(RichText::new("Sample Text").color(app.state.custom_colors.dark_red));
            ui.color_edit_button_srgba(&mut app.state.custom_colors.dark_red);
            ui.end_row();
        });

    ui.add_space(15.0);

    if ui.button(RichText::new("🔄 Reset to Defaults").size(14.0)).clicked() {
        app.state.custom_colors = crate::state::CustomColors::default();
    }

    ui.add_space(20.0);
    ui.label(RichText::new("💡 Tip: Color changes apply immediately throughout the app").weak());
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

fn perform_conversion(value_str: &str, from: &str, to: &str) -> String {
    let value: f64 = match value_str.parse() {
        Ok(v) => v,
        Err(_) => return "Invalid number".to_string(),
    };

    // Convert to base unit first, then to target unit
    let result = match (from, to) {
        // Volume conversions (base: liters)
        ("liters", "gallons") => value / 3.78541,
        ("gallons", "liters") => value * 3.78541,
        ("liters", "liters") => value,
        ("gallons", "gallons") => value,

        // Mass conversions (base: grams)
        ("kilograms", "pounds") => value * 2.20462,
        ("pounds", "kilograms") => value / 2.20462,
        ("grams", "ounces") => value / 28.3495,
        ("ounces", "grams") => value * 28.3495,
        ("kilograms", "grams") => value * 1000.0,
        ("grams", "kilograms") => value / 1000.0,
        ("pounds", "ounces") => value * 16.0,
        ("ounces", "pounds") => value / 16.0,
        ("kilograms", "ounces") => value * 35.274,
        ("ounces", "kilograms") => value / 35.274,
        ("pounds", "grams") => value * 453.592,
        ("grams", "pounds") => value / 453.592,
        ("kilograms", "kilograms") => value,
        ("pounds", "pounds") => value,
        ("grams", "grams") => value,
        ("ounces", "ounces") => value,

        // Temperature conversions
        ("celsius", "fahrenheit") => (value * 9.0 / 5.0) + 32.0,
        ("fahrenheit", "celsius") => (value - 32.0) * 5.0 / 9.0,
        ("celsius", "celsius") => value,
        ("fahrenheit", "fahrenheit") => value,

        _ => return "Cannot convert between these units".to_string(),
    };

    format!("{:.3} {} = {:.3} {}", value, short_unit(from), result, short_unit(to))
}