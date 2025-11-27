//! Conversions tab - Reference charts and interactive converter

use crate::state::AppState;
use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};
use rust_decimal::Decimal;
use std::str::FromStr;

const BG_PANEL: Color32 = Color32::from_rgb(245, 250, 255);
const TEXT_MAIN: Color32 = Color32::from_rgb(30, 30, 30);
const TEXT_ACCENT: Color32 = Color32::from_rgb(70, 130, 180);
const BORDER: Color32 = Color32::from_rgb(70, 130, 180);
const BUTTON: Color32 = Color32::from_rgb(70, 130, 180);

pub fn render(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading(RichText::new("📐 Unit Conversions & Reference").color(TEXT_ACCENT).size(20.0));
    ui.add_space(10.0);

    // Interactive Converter
    section(ui, "🔄 Interactive Converter", |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Value:").strong());
            ui.text_edit_singleline(&mut state.conv_value);
        });

        ui.horizontal(|ui| {
            ui.label(RichText::new("From:").strong());
            egui::ComboBox::new("conv_from", "")
                .selected_text(&state.conv_from_unit)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.conv_from_unit, "liters".to_string(), "Liters (L)");
                    ui.selectable_value(&mut state.conv_from_unit, "gallons".to_string(), "Gallons (US)");
                    ui.selectable_value(&mut state.conv_from_unit, "kilograms".to_string(), "Kilograms (kg)");
                    ui.selectable_value(&mut state.conv_from_unit, "pounds".to_string(), "Pounds (lb)");
                    ui.selectable_value(&mut state.conv_from_unit, "celsius".to_string(), "Celsius (°C)");
                    ui.selectable_value(&mut state.conv_from_unit, "fahrenheit".to_string(), "Fahrenheit (°F)");
                });

            ui.label(RichText::new("→").size(20.0));

            ui.label(RichText::new("To:").strong());
            egui::ComboBox::new("conv_to", "")
                .selected_text(&state.conv_to_unit)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.conv_to_unit, "liters".to_string(), "Liters (L)");
                    ui.selectable_value(&mut state.conv_to_unit, "gallons".to_string(), "Gallons (US)");
                    ui.selectable_value(&mut state.conv_to_unit, "kilograms".to_string(), "Kilograms (kg)");
                    ui.selectable_value(&mut state.conv_to_unit, "pounds".to_string(), "Pounds (lb)");
                    ui.selectable_value(&mut state.conv_to_unit, "celsius".to_string(), "Celsius (°C)");
                    ui.selectable_value(&mut state.conv_to_unit, "fahrenheit".to_string(), "Fahrenheit (°F)");
                });
        });

        if button(ui, "Convert") {
            perform_conversion(state);
        }

        if let Some(ref result) = state.conv_result {
            ui.add_space(5.0);
            ui.label(RichText::new(result).color(Color32::from_rgb(34, 139, 34)).size(18.0).strong());
        }
    });

    ui.add_space(15.0);

    // Reference Charts
    ui.columns(2, |columns| {
        // Volume conversions
        section(&mut columns[0], "💧 Volume Conversions", |ui| {
            table(ui, &[
                ("1 Liter (L)", "0.2642 Gallons (US)"),
                ("1 Liter", "1.0567 Quarts (US)"),
                ("1 Liter", "2.1134 Pints (US)"),
                ("1 Liter", "4.2268 Cups (US)"),
                ("1 Liter", "33.814 Fluid Ounces (US)"),
                ("1 Liter", "1000 Milliliters (mL)"),
                ("", ""),
                ("1 Gallon (US)", "3.7854 Liters"),
                ("1 Gallon", "4 Quarts"),
                ("1 Gallon", "8 Pints"),
                ("1 Gallon", "16 Cups"),
                ("1 Gallon", "128 Fluid Ounces"),
                ("", ""),
                ("1 Quart (US)", "0.9464 Liters"),
                ("1 Pint (US)", "0.4732 Liters"),
                ("1 Cup (US)", "236.6 mL"),
                ("1 Fl Oz (US)", "29.574 mL"),
            ]);
        });

        // Weight conversions
        section(&mut columns[1], "⚖️ Weight/Mass Conversions", |ui| {
            table(ui, &[
                ("1 Kilogram (kg)", "2.2046 Pounds (lb)"),
                ("1 Kilogram", "35.274 Ounces (oz)"),
                ("1 Kilogram", "1000 Grams (g)"),
                ("", ""),
                ("1 Pound (lb)", "0.4536 Kilograms"),
                ("1 Pound", "16 Ounces (oz)"),
                ("1 Pound", "453.6 Grams (g)"),
                ("", ""),
                ("1 Ounce (oz)", "28.35 Grams"),
                ("1 Gram (g)", "0.0353 Ounces"),
                ("", ""),
                ("1 Metric Ton", "1000 Kilograms"),
                ("1 US Ton", "2000 Pounds"),
                ("1 US Ton", "907.2 Kilograms"),
            ]);
        });
    });

    ui.add_space(10.0);

    ui.columns(2, |columns| {
        // Temperature conversions
        section(&mut columns[0], "🌡️ Temperature Conversions", |ui| {
            table(ui, &[
                ("Formula", "Result"),
                ("°C → °F", "°F = (°C × 9/5) + 32"),
                ("°F → °C", "°C = (°F - 32) × 5/9"),
                ("°C → K", "K = °C + 273.15"),
                ("", ""),
                ("Common Temps:", ""),
                ("0°C", "32°F (Water freezes)"),
                ("20°C", "68°F (Room temp)"),
                ("100°C", "212°F (Water boils)"),
                ("", ""),
                ("Brewing Temps:", ""),
                ("10-15°C", "50-59°F (Lager)"),
                ("18-22°C", "64-72°F (Ale)"),
                ("22-30°C", "72-86°F (Wine/Mead)"),
            ]);
        });

        // Gravity/Sugar conversions
        section(&mut columns[1], "📊 Gravity & Sugar Conversions", |ui| {
            table(ui, &[
                ("Specific Gravity", "Brix / Plato"),
                ("1.000 SG", "0.0° Bx"),
                ("1.020 SG", "~5.1° Bx"),
                ("1.040 SG", "~10.0° Bx"),
                ("1.060 SG", "~14.7° Bx"),
                ("1.080 SG", "~19.3° Bx"),
                ("1.100 SG", "~23.7° Bx"),
                ("1.120 SG", "~28.0° Bx"),
                ("", ""),
                ("Formula (approx):", ""),
                ("Brix → SG", "SG ≈ 1 + (Bx × 0.004)"),
                ("SG → Brix", "Bx ≈ (SG - 1) × 250"),
                ("", ""),
                ("Note:", "Brix ≈ Plato"),
            ]);
        });
    });

    ui.add_space(10.0);

    // Brewing-specific conversions
    section(ui, "🍺 Brewing-Specific Conversions", |ui| {
        ui.columns(2, |columns| {
            columns[0].group(|ui| {
                ui.label(RichText::new("Common Batch Sizes:").strong());
                ui.label("5 gallons (US) = 18.93 L");
                ui.label("6 gallons (US) = 22.71 L");
                ui.label("10 gallons (US) = 37.85 L");
                ui.label("20 gallons (US) = 75.71 L");
                ui.label("1 barrel (US) = 31 gallons = 117.3 L");
            });

            columns[1].group(|ui| {
                ui.label(RichText::new("Sugar/Honey Conversions:").strong());
                ui.label("1 cup honey ≈ 340g ≈ 12 oz");
                ui.label("1 cup sugar ≈ 200g ≈ 7 oz");
                ui.label("1 tbsp honey ≈ 21g");
                ui.label("1 tsp honey ≈ 7g");
                ui.label("Honey density ≈ 1.42 g/mL");
            });
        });
    });
}

fn section(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(BG_PANEL)
        .stroke(Stroke::new(1.5, BORDER))
        .rounding(Rounding::same(8.0))
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.label(RichText::new(title).color(TEXT_ACCENT).size(16.0).strong());
            ui.add_space(6.0);
            content(ui);
        });
}

fn table(ui: &mut egui::Ui, rows: &[(&str, &str)]) {
    for (left, right) in rows {
        if left.is_empty() {
            ui.add_space(3.0);
        } else {
            ui.horizontal(|ui| {
                ui.label(RichText::new(*left).color(TEXT_MAIN));
                ui.label("=");
                ui.label(RichText::new(*right).color(Color32::from_rgb(100, 100, 100)));
            });
        }
    }
}

fn button(ui: &mut egui::Ui, text: &str) -> bool {
    ui.add(
        egui::Button::new(RichText::new(text).color(Color32::WHITE).size(16.0).strong())
            .fill(BUTTON)
            .rounding(Rounding::same(6.0))
            .min_size(Vec2::new(150.0, 36.0))
    ).clicked()
}

fn perform_conversion(state: &mut AppState) {
    let value = match Decimal::from_str(&state.conv_value) {
        Ok(v) => v,
        Err(_) => {
            state.conv_result = Some("❌ Invalid number".to_string());
            return;
        }
    };

    let result = match (state.conv_from_unit.as_str(), state.conv_to_unit.as_str()) {
        ("liters", "gallons") => value * Decimal::from_str("0.264172").unwrap(),
        ("gallons", "liters") => value * Decimal::from_str("3.78541").unwrap(),
        ("liters", "liters") => value,
        ("gallons", "gallons") => value,
        ("kilograms", "pounds") => value * Decimal::from_str("2.20462").unwrap(),
        ("pounds", "kilograms") => value * Decimal::from_str("0.453592").unwrap(),
        ("kilograms", "kilograms") => value,
        ("pounds", "pounds") => value,
        ("celsius", "fahrenheit") => (value * Decimal::from_str("1.8").unwrap()) + Decimal::from(32),
        ("fahrenheit", "celsius") => (value - Decimal::from(32)) * Decimal::from_str("0.555556").unwrap(),
        ("celsius", "celsius") => value,
        ("fahrenheit", "fahrenheit") => value,
        _ => {
            state.conv_result = Some("❌ Cannot convert between these units".to_string());
            return;
        }
    };

    state.conv_result = Some(format!(
        "✓ {} {} = {:.4} {}",
        state.conv_value,
        unit_display(&state.conv_from_unit),
        result,
        unit_display(&state.conv_to_unit)
    ));
}

fn unit_display(unit: &str) -> &str {
    match unit {
        "liters" => "L",
        "gallons" => "gal (US)",
        "kilograms" => "kg",
        "pounds" => "lb",
        "celsius" => "°C",
        "fahrenheit" => "°F",
        _ => unit,
    }
}