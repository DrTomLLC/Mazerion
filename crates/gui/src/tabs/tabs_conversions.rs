//! Unit Conversions tab - ZERO PANICS, proper error handling

use crate::state::AppState;
use eframe::egui::{self, Color32, RichText, CornerRadius, Stroke};
use rust_decimal::Decimal;
use std::str::FromStr;

pub fn render(state: &mut AppState, ui: &mut egui::Ui) {
    let c = state.custom_colors;

    ui.heading(RichText::new("🔄 Unit Conversions")
        .size(24.0)
        .color(c.honey_gold));
    ui.add_space(15.0);

    egui::Frame::default()
        .fill(c.light_cream)
        .stroke(Stroke::new(2.0, c.honey_gold))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Value:").strong().size(16.0));
                ui.text_edit_singleline(&mut state.conversion_value);
            });
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label(RichText::new("From:").strong().size(16.0));
                egui::ComboBox::from_id_salt("from_unit")
                    .selected_text(get_unit_display(&state.conversion_from_unit))
                    .width(200.0)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut state.conversion_from_unit, "liters".to_string(), "Liters (L)");
                        ui.selectable_value(&mut state.conversion_from_unit, "gallons".to_string(), "Gallons (US)");
                        ui.selectable_value(&mut state.conversion_from_unit, "ml".to_string(), "Milliliters (mL)");
                        ui.selectable_value(&mut state.conversion_from_unit, "fl_oz".to_string(), "Fluid Ounces (fl oz)");
                        ui.separator();
                        ui.selectable_value(&mut state.conversion_from_unit, "kg".to_string(), "Kilograms (kg)");
                        ui.selectable_value(&mut state.conversion_from_unit, "lbs".to_string(), "Pounds (lb)");
                        ui.selectable_value(&mut state.conversion_from_unit, "g".to_string(), "Grams (g)");
                        ui.selectable_value(&mut state.conversion_from_unit, "oz".to_string(), "Ounces (oz)");
                        ui.separator();
                        ui.selectable_value(&mut state.conversion_from_unit, "celsius".to_string(), "Celsius (°C)");
                        ui.selectable_value(&mut state.conversion_from_unit, "fahrenheit".to_string(), "Fahrenheit (°F)");
                        ui.separator();
                        ui.selectable_value(&mut state.conversion_from_unit, "sg".to_string(), "Specific Gravity");
                        ui.selectable_value(&mut state.conversion_from_unit, "brix".to_string(), "Brix (°Bx)");
                        ui.selectable_value(&mut state.conversion_from_unit, "plato".to_string(), "Plato (°P)");
                    });
            });
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label(RichText::new("To:").strong().size(16.0));
                egui::ComboBox::from_id_salt("to_unit")
                    .selected_text(get_unit_display(&state.conversion_to_unit))
                    .width(200.0)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut state.conversion_to_unit, "liters".to_string(), "Liters (L)");
                        ui.selectable_value(&mut state.conversion_to_unit, "gallons".to_string(), "Gallons (US)");
                        ui.selectable_value(&mut state.conversion_to_unit, "ml".to_string(), "Milliliters (mL)");
                        ui.selectable_value(&mut state.conversion_to_unit, "fl_oz".to_string(), "Fluid Ounces (fl oz)");
                        ui.separator();
                        ui.selectable_value(&mut state.conversion_to_unit, "kg".to_string(), "Kilograms (kg)");
                        ui.selectable_value(&mut state.conversion_to_unit, "lbs".to_string(), "Pounds (lb)");
                        ui.selectable_value(&mut state.conversion_to_unit, "g".to_string(), "Grams (g)");
                        ui.selectable_value(&mut state.conversion_to_unit, "oz".to_string(), "Ounces (oz)");
                        ui.separator();
                        ui.selectable_value(&mut state.conversion_to_unit, "celsius".to_string(), "Celsius (°C)");
                        ui.selectable_value(&mut state.conversion_to_unit, "fahrenheit".to_string(), "Fahrenheit (°F)");
                        ui.separator();
                        ui.selectable_value(&mut state.conversion_to_unit, "sg".to_string(), "Specific Gravity");
                        ui.selectable_value(&mut state.conversion_to_unit, "brix".to_string(), "Brix (°Bx)");
                        ui.selectable_value(&mut state.conversion_to_unit, "plato".to_string(), "Plato (°P)");
                    });
            });
            ui.add_space(15.0);

            if ui.add(
                egui::Button::new(RichText::new("🔄 Convert").size(18.0).strong())
                    .fill(c.forest_green)
                    .stroke(Stroke::new(1.0, c.dark_text))
                    .corner_radius(CornerRadius::same(6))
                    .min_size(egui::vec2(200.0, 40.0))
            ).clicked() {
                perform_conversion(state);
            }
        });

    ui.add_space(20.0);

    if let Some(ref result) = state.conversion_result {
        egui::Frame::default()
            .fill(Color32::WHITE)
            .stroke(Stroke::new(2.0, c.forest_green))
            .corner_radius(CornerRadius::same(8))
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.label(RichText::new(result)
                    .size(20.0)
                    .color(if result.starts_with("❌") { c.sunset_orange } else { c.forest_green }));
            });
    }
}

fn get_unit_display(unit: &str) -> &str {
    match unit {
        "liters" => "Liters (L)",
        "gallons" => "Gallons (US)",
        "ml" => "Milliliters (mL)",
        "fl_oz" => "Fluid Ounces (fl oz)",
        "kg" => "Kilograms (kg)",
        "lbs" => "Pounds (lb)",
        "g" => "Grams (g)",
        "oz" => "Ounces (oz)",
        "celsius" => "Celsius (°C)",
        "fahrenheit" => "Fahrenheit (°F)",
        "sg" => "Specific Gravity",
        "brix" => "Brix (°Bx)",
        "plato" => "Plato (°P)",
        _ => unit,
    }
}

fn perform_conversion(state: &mut AppState) {
    let value = match Decimal::from_str(&state.conversion_value) {
        Ok(v) => v,
        Err(_) => {
            state.conversion_result = Some("❌ Invalid number".to_string());
            return;
        }
    };

    let result = convert_units(value, &state.conversion_from_unit, &state.conversion_to_unit);

    match result {
        Ok(converted) => {
            state.conversion_result = Some(format!(
                "✓ {} {} = {:.4} {}",
                value,
                get_unit_display(&state.conversion_from_unit),
                converted,
                get_unit_display(&state.conversion_to_unit)
            ));
        }
        Err(e) => {
            state.conversion_result = Some(format!("❌ {}", e));
        }
    }
}

fn convert_units(value: Decimal, from: &str, to: &str) -> Result<Decimal, String> {
    match (from, to) {
        // Same unit
        (f, t) if f == t => Ok(value),

        // Volume: Liters
        ("liters", "gallons") => {
            let factor = Decimal::new(378541, 5); // 3.78541
            Ok(value / factor)
        }
        ("liters", "ml") => Ok(value * Decimal::from(1000)),
        ("liters", "fl_oz") => {
            let factor = Decimal::new(33814, 3); // 33.814
            Ok(value * factor)
        }

        // Volume: Gallons
        ("gallons", "liters") => {
            let factor = Decimal::new(378541, 5); // 3.78541
            Ok(value * factor)
        }
        ("gallons", "ml") => {
            let factor = Decimal::new(378541, 2); // 3785.41
            Ok(value * factor)
        }
        ("gallons", "fl_oz") => Ok(value * Decimal::from(128)),

        // Volume: Milliliters
        ("ml", "liters") => Ok(value / Decimal::from(1000)),
        ("ml", "gallons") => {
            let factor = Decimal::new(378541, 2); // 3785.41
            Ok(value / factor)
        }
        ("ml", "fl_oz") => {
            let factor = Decimal::new(295735, 4); // 29.5735
            Ok(value / factor)
        }

        // Volume: Fluid Ounces
        ("fl_oz", "liters") => {
            let factor = Decimal::new(33814, 3); // 33.814
            Ok(value / factor)
        }
        ("fl_oz", "gallons") => Ok(value / Decimal::from(128)),
        ("fl_oz", "ml") => {
            let factor = Decimal::new(295735, 4); // 29.5735
            Ok(value * factor)
        }

        // Weight: Kilograms
        ("kg", "lbs") => {
            let factor = Decimal::new(220462, 5); // 2.20462
            Ok(value * factor)
        }
        ("kg", "g") => Ok(value * Decimal::from(1000)),
        ("kg", "oz") => {
            let factor = Decimal::new(35274, 3); // 35.274
            Ok(value * factor)
        }

        // Weight: Pounds
        ("lbs", "kg") => {
            let factor = Decimal::new(220462, 5); // 2.20462
            Ok(value / factor)
        }
        ("lbs", "g") => {
            let factor = Decimal::new(453592, 3); // 453.592
            Ok(value * factor)
        }
        ("lbs", "oz") => Ok(value * Decimal::from(16)),

        // Weight: Grams
        ("g", "kg") => Ok(value / Decimal::from(1000)),
        ("g", "lbs") => {
            let factor = Decimal::new(453592, 3); // 453.592
            Ok(value / factor)
        }
        ("g", "oz") => {
            let factor = Decimal::new(283495, 4); // 28.3495
            Ok(value / factor)
        }

        // Weight: Ounces
        ("oz", "kg") => {
            let factor = Decimal::new(35274, 3); // 35.274
            Ok(value / factor)
        }
        ("oz", "lbs") => Ok(value / Decimal::from(16)),
        ("oz", "g") => {
            let factor = Decimal::new(283495, 4); // 28.3495
            Ok(value * factor)
        }

        // Temperature
        ("celsius", "fahrenheit") => {
            let nine_fifths = Decimal::new(18, 1); // 1.8
            Ok(value * nine_fifths + Decimal::from(32))
        }
        ("fahrenheit", "celsius") => {
            let nine_fifths = Decimal::new(18, 1); // 1.8
            Ok((value - Decimal::from(32)) / nine_fifths)
        }

        // Gravity/Sugar: SG to others
        ("sg", "brix") | ("sg", "plato") => {
            let c1 = Decimal::new(1824601, 4); // 182.4601
            let c2 = Decimal::new(7756821, 4); // 775.6821
            let c3 = Decimal::new(12627794, 4); // 1262.7794
            let c4 = Decimal::new(6695622, 4); // 669.5622

            let result = ((c1 * value - c2) * value + c3) * value - c4;
            Ok(result)
        }

        // Gravity/Sugar: Brix to others
        ("brix", "sg") | ("plato", "sg") => {
            let c1 = Decimal::new(2586, 1); // 258.6
            let c2 = Decimal::new(2582, 1); // 258.2
            let c3 = Decimal::new(2271, 1); // 227.1

            let denom = c1 - ((value / c2) * c3);
            if denom == Decimal::ZERO {
                return Err("Division by zero in conversion".to_string());
            }

            let result = Decimal::from(1) + (value / denom);
            Ok(result)
        }

        // Brix ≈ Plato
        ("brix", "plato") | ("plato", "brix") => Ok(value),

        _ => Err("Cannot convert between these unit types".to_string())
    }
}