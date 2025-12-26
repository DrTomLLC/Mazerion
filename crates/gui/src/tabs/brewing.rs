//! Brewing calculators tab with TOSNA protocol support

use crate::{
    MazerionApp,
    state::{BrewingCalculator, colors},
};
use eframe::egui::{self, Color32, CornerRadius, RichText};
use mazerion_core::CalcInput;
use rust_decimal::Decimal;
use std::str::FromStr;

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Select Calculator:").strong());
        egui::ComboBox::from_id_salt("brewing_calc")
            .selected_text(get_calc_name(app.state.brewing_calc))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut app.state.brewing_calc,
                    BrewingCalculator::Nutrition,
                    "TOSNA Nutrition Calculator",
                );
                ui.selectable_value(
                    &mut app.state.brewing_calc,
                    BrewingCalculator::Carbonation,
                    "Carbonation Calculator",
                );
            });
    });

    ui.add_space(10.0);

    egui::Frame::new()
        .fill(colors::LIGHT_CREAM)
        .stroke(egui::Stroke::new(1.5, colors::HONEY_GOLD))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| match app.state.brewing_calc {
            BrewingCalculator::Nutrition => render_nutrition(app, ui),
            BrewingCalculator::Carbonation => render_carbonation(app, ui),
        });
}

fn get_calc_name(calc: BrewingCalculator) -> &'static str {
    match calc {
        BrewingCalculator::Nutrition => "TOSNA Nutrition Calculator",
        BrewingCalculator::Carbonation => "Carbonation Calculator",
    }
}

fn render_nutrition(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🧪 TOSNA Nutrition Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate Fermaid-O schedule using TOSNA protocols");
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) {
        "L"
    } else {
        "gal"
    };

    crate::input_field(
        ui,
        &format!("Volume ({}):", vol_unit),
        &mut app.volume,
        "Total must volume",
    );
    crate::input_field(
        ui,
        "Target ABV (%):",
        &mut app.target_abv_brew,
        "Expected final ABV",
    );

    // TOSNA Protocol selector - using yn_requirement field temporarily as protocol selector
    ui.add_space(5.0);
    ui.horizontal(|ui| {
        ui.label(RichText::new("TOSNA Protocol:").strong());

        // Store protocol in first char of yn_requirement: l=TOSNA1, m=TOSNA2, h=TOSNA3
        let current_protocol = match app.yn_requirement.chars().next() {
            Some('1') => "TOSNA 1.0",
            Some('3') => "TOSNA 3.0",
            _ => "TOSNA 2.0",
        };

        egui::ComboBox::from_id_salt("tosna_protocol")
            .selected_text(current_protocol)
            .width(200.0)
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(current_protocol == "TOSNA 1.0", "TOSNA 1.0 (Original)")
                    .clicked()
                {
                    app.yn_requirement = "1_medium".to_string();
                }
                if ui
                    .selectable_label(current_protocol == "TOSNA 2.0", "TOSNA 2.0 (Recommended)")
                    .clicked()
                {
                    app.yn_requirement = "medium".to_string();
                }
                if ui
                    .selectable_label(current_protocol == "TOSNA 3.0", "TOSNA 3.0 (High Gravity)")
                    .clicked()
                {
                    app.yn_requirement = "3_medium".to_string();
                }
            });
    });

    // Protocol description box
    ui.add_space(5.0);
    egui::Frame::new()
        .fill(Color32::from_rgb(255, 250, 240))
        .stroke(egui::Stroke::new(1.0, colors::HONEY_GOLD))
        .corner_radius(CornerRadius::same(4))
        .inner_margin(8.0)
        .show(ui, |ui| match app.yn_requirement.chars().next() {
            Some('1') => {
                ui.label(
                    RichText::new("📜 TOSNA 1.0 - Original Protocol")
                        .strong()
                        .color(colors::SADDLE_BROWN),
                );
                ui.label("• Schedule: 33% - 33% - 33% over 7 days");
                ui.label("• Timing: Day 1, Day 3, Day 7");
                ui.label("• When to use: Simple low-gravity meads (OG <1.100)");
                ui.label("• Note: Older protocol - TOSNA 2.0 generally preferred");
            }
            Some('3') => {
                ui.label(
                    RichText::new("🚀 TOSNA 3.0 - Advanced High-Gravity Protocol")
                        .strong()
                        .color(colors::SADDLE_BROWN),
                );
                ui.label("• Schedule: 5% - 20% - 50% - 25% (4 additions)");
                ui.label("• Timing: 24hr, 48hr, 1/3 break (~1.070), 2/3 break (~1.040)");
                ui.label("• When to use: HIGH GRAVITY meads (OG >1.100, ABV >14%)");
                ui.label("• Why: More gradual feeding prevents stuck ferments");
                ui.label("• Best for: Sack meads, dessert meads, high-ABV traditional");
            }
            _ => {
                ui.label(
                    RichText::new("⭐ TOSNA 2.0 - Recommended Standard")
                        .strong()
                        .color(colors::SADDLE_BROWN),
                );
                ui.label("• Schedule: 25% - 50% - 25% (3 additions)");
                ui.label("• Timing: 24hr, 1/3 break (~1.070), 2/3 break (~1.040)");
                ui.label("• When to use: MOST MEADS (OG 1.080-1.120, ABV 10-14%)");
                ui.label("• Why: Balanced nutrition matches yeast growth curve");
                ui.label("• Best for: Traditional meads, melomels, session meads");
            }
        });

    ui.add_space(10.0);

    // Yeast nitrogen requirement selector
    ui.horizontal(|ui| {
        ui.label(RichText::new("Yeast Nitrogen Needs:").strong());

        let yeast_level = if app.yn_requirement.contains("low") {
            "low"
        } else if app.yn_requirement.contains("high") {
            "high"
        } else {
            "medium"
        };

        egui::ComboBox::from_id_salt("yn_req")
            .selected_text(match yeast_level {
                "low" => "Low (DV10, QA23, D254)",
                "high" => "High (EC-1118, K1-V1116, RC212)",
                _ => "Medium (71B, D47, most yeasts)",
            })
            .show_ui(ui, |ui| {
                let protocol_prefix = match app.yn_requirement.chars().next() {
                    Some('1') => "1_",
                    Some('3') => "3_",
                    _ => "",
                };

                if ui
                    .selectable_label(yeast_level == "low", "Low (DV10, QA23, D254)")
                    .clicked()
                {
                    app.yn_requirement = format!("{}low", protocol_prefix);
                }
                if ui
                    .selectable_label(yeast_level == "medium", "Medium (71B, D47, most yeasts)")
                    .clicked()
                {
                    app.yn_requirement = format!("{}medium", protocol_prefix);
                }
                if ui
                    .selectable_label(yeast_level == "high", "High (EC-1118, K1-V1116, RC212)")
                    .clicked()
                {
                    app.yn_requirement = format!("{}high", protocol_prefix);
                }
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate TOSNA Schedule") {
        calc_nutrition(app);
    }
}

fn calc_nutrition(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("nutrition") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Nutrition calculator not found".to_string());
            return;
        }
    };

    // Convert volume to liters if in Imperial mode
    let volume_val = match Decimal::from_str(&app.volume) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid volume".to_string());
            return;
        }
    };

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);
    let volume_liters = if is_metric {
        volume_val
    } else {
        mazerion_core::gallons_to_liters(volume_val)
    };

    // Parse protocol and yeast requirement from yn_requirement field
    let (protocol, yeast_req) = match app.yn_requirement.chars().next() {
        Some('1') => (
            "tosna_1",
            app.yn_requirement.strip_prefix("1_").unwrap_or("medium"),
        ),
        Some('3') => (
            "tosna_3",
            app.yn_requirement.strip_prefix("3_").unwrap_or("medium"),
        ),
        _ => ("tosna_2", app.yn_requirement.as_str()),
    };

    let input = CalcInput::new()
        .add_param("volume", volume_liters.to_string())
        .add_param("target_abv", &app.target_abv_brew)
        .add_param("yn_requirement", yeast_req)
        .add_param("protocol", protocol);

    match calc.calculate(input) {
        Ok(res) => {
            let total_grams = res.output.value;

            // Convert to ounces if in Imperial mode
            let (display_amount, weight_unit) = if is_metric {
                (total_grams, "g")
            } else {
                let ounces = total_grams * Decimal::new(35273962, 9);
                (ounces, "oz")
            };

            app.result = Some(format!(
                "Total Fermaid-O: {:.2} {}",
                display_amount, weight_unit
            ));
            app.warnings = res.warnings;

            // Convert metadata weights if in Imperial
            if is_metric {
                app.metadata = res.metadata;
            } else {
                app.metadata = convert_nutrition_metadata(&res.metadata);
            }
        }
        Err(e) => {
            app.result = Some(format!("Error: {}", e));
            app.warnings.clear();
            app.metadata.clear();
        }
    }
}

fn convert_nutrition_metadata(metadata: &[(String, String)]) -> Vec<(String, String)> {
    metadata
        .iter()
        .map(|(key, value)| {
            // Convert grams in metadata to ounces
            if key.starts_with("addition_") && value.contains(" g ") {
                if let Some(g_pos) = value.find(" g ")
                    && let Ok(grams) = Decimal::from_str(value[..g_pos].trim())
                {
                    let ounces = grams * Decimal::new(35273962, 9);
                    let rest = &value[g_pos + 3..]; // Keep percentage and timing
                    return (key.clone(), format!("{:.2} oz {}", ounces, rest));
                }
            } else if key == "total_fermaid_o"
                && value.ends_with(" g")
                && let Some(g_pos) = value.find(" g")
                && let Ok(grams) = Decimal::from_str(value[..g_pos].trim())
            {
                let ounces = grams * Decimal::new(35273962, 9);
                return (key.clone(), format!("{:.2} oz", ounces));
            }
            (key.clone(), value.clone())
        })
        .collect()
}

fn render_carbonation(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🫧 Carbonation Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate priming sugar or keg PSI for target carbonation");
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) {
        "L"
    } else {
        "gal"
    };
    let temp_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) {
        "°C"
    } else {
        "°F"
    };

    crate::input_field(
        ui,
        &format!("Volume ({}):", vol_unit),
        &mut app.volume,
        "Total volume to carbonate",
    );
    crate::input_field(
        ui,
        &format!("Temperature ({}):", temp_unit),
        &mut app.carb_temp,
        "Current temperature",
    );
    crate::input_field(
        ui,
        "Target CO₂ (volumes):",
        &mut app.target_co2,
        "Desired carbonation level (1.5-4.5)",
    );

    ui.horizontal(|ui| {
        ui.label(RichText::new("Method:").strong());
        egui::ComboBox::from_id_salt("carb_method")
            .selected_text(&app.carb_method)
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut app.carb_method,
                    "priming".to_string(),
                    "Bottle Priming",
                );
                ui.selectable_value(
                    &mut app.carb_method,
                    "keg".to_string(),
                    "Force Carbonation (Keg)",
                );
            });
    });

    if app.carb_method == "priming" {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Sugar Type:").strong());
            egui::ComboBox::from_id_salt("sugar_type")
                .selected_text(&app.sugar_type)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut app.sugar_type,
                        "table_sugar".to_string(),
                        "Table Sugar (Sucrose)",
                    );
                    ui.selectable_value(
                        &mut app.sugar_type,
                        "corn_sugar".to_string(),
                        "Corn Sugar (Dextrose)",
                    );
                    ui.selectable_value(&mut app.sugar_type, "honey".to_string(), "Honey");
                    ui.selectable_value(&mut app.sugar_type, "dme".to_string(), "Dry Malt Extract");
                });
        });
    }

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Carbonation") {
        calc_carbonation(app);
    }
}

fn calc_carbonation(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("carbonation") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Carbonation calculator not found".to_string());
            return;
        }
    };

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);

    // CRITICAL FIX: Convert temperature to Celsius if in Imperial mode
    let temp_c = if is_metric {
        // Already in Celsius
        app.carb_temp.clone()
    } else {
        // User entered Fahrenheit - convert to Celsius
        match Decimal::from_str(&app.carb_temp) {
            Ok(temp_f) => {
                let temp_c = (temp_f - Decimal::from(32)) * Decimal::new(5, 0) / Decimal::new(9, 0);
                temp_c.to_string()
            }
            Err(_) => {
                app.result = Some("❌ Invalid temperature".to_string());
                return;
            }
        }
    };

    // Convert volume to liters if in Imperial
    let volume_l = if is_metric {
        app.volume.clone()
    } else {
        match Decimal::from_str(&app.volume) {
            Ok(gal) => {
                let liters = gal * Decimal::new(378541, 5); // gallons to liters
                liters.to_string()
            }
            Err(_) => {
                app.result = Some("❌ Invalid volume".to_string());
                return;
            }
        }
    };

    let input = CalcInput::new()
        .add_param("volume", &volume_l) // Always pass in liters
        .add_param("temperature", &temp_c) // Always pass in Celsius
        .add_param("target_co2", &app.target_co2)
        .add_param("method", &app.carb_method)
        .add_param("sugar_type", &app.sugar_type);

    match calc.calculate(input) {
        Ok(res) => {
            if app.carb_method == "keg" {
                app.result = Some(format!("Target PSI: {:.1}", res.output.value));
            } else {
                // Convert priming sugar output based on unit system
                let (amount, weight_unit) = if is_metric {
                    (res.output.value, "g")
                } else {
                    let oz = res.output.value / Decimal::new(2835, 2); // grams to oz
                    (oz, "oz")
                };
                app.result = Some(format!("Priming Sugar: {:.1} {}", amount, weight_unit));
            }
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => {
            app.result = Some(format!("Error: {}", e));
            app.warnings.clear();
            app.metadata.clear();
        }
    }
}
