use crate::MazerionApp;
use crate::state::{FinishingCalculator, UnitSystem};
use eframe::egui;
use eframe::epaint::Color32;
use mazerion_calculators::*;
use mazerion_core::{CalcInput, Calculator, Measurement};
use rust_decimal::Decimal;
use std::str::FromStr;

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading("✨ Finishing & Stabilization");
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        ui.selectable_value(
            &mut app.state.finishing_calc,
            FinishingCalculator::Backsweetening,
            "🍯 Backsweetening",
        );
        ui.selectable_value(
            &mut app.state.finishing_calc,
            FinishingCalculator::Sulfite,
            "🧪 Sulfite",
        );
        ui.selectable_value(
            &mut app.state.finishing_calc,
            FinishingCalculator::AcidAddition,
            "🍋 Acid",
        );
        ui.selectable_value(
            &mut app.state.finishing_calc,
            FinishingCalculator::Pasteurization,
            "🔥 Pasteurization",
        );
        ui.selectable_value(
            &mut app.state.finishing_calc,
            FinishingCalculator::Stabilization,
            "🔒 Stabilization",
        );
        ui.selectable_value(
            &mut app.state.finishing_calc,
            FinishingCalculator::SweetnessChart,
            "📊 Sweetness",
        );
    });

    ui.add_space(15.0);

    match app.state.finishing_calc {
        FinishingCalculator::Backsweetening => render_backsweetening(app, ui),
        FinishingCalculator::Sulfite => render_sulfite(app, ui),
        FinishingCalculator::Acid => render_acid(app, ui),
        FinishingCalculator::AcidAddition => render_acid(app, ui),
        FinishingCalculator::Pasteurization => render_pasteurization(app, ui),
        FinishingCalculator::Stabilization => render_stabilization(app, ui),
        FinishingCalculator::SweetnessChart => render_sweetness_chart(app, ui),
    }
}

fn render_backsweetening(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.label("Calculate sweetener needed to reach target gravity");
    ui.add_space(10.0);

    let is_metric = app.state.unit_system == UnitSystem::Metric;
    let vol_unit = if is_metric { "L" } else { "gal" };

    crate::input_field(
        ui,
        &format!("Volume ({}):", vol_unit),
        &mut app.sweet_vol,
        "Batch volume",
    );
    crate::input_field(ui, "Current SG:", &mut app.current_sg, "e.g., 1.000");
    crate::input_field(ui, "Target SG:", &mut app.target_sg, "e.g., 1.015");

    ui.horizontal(|ui| {
        ui.label("Sweetener:");
        egui::ComboBox::from_id_salt("sweetener")
            .selected_text(&app.sweetener)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.sweetener, "honey".to_string(), "Honey");
                ui.selectable_value(&mut app.sweetener, "table_sugar".to_string(), "Table Sugar");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate") {
        app.warnings.clear();
        app.metadata.clear();

        let current_sg_val = match Decimal::from_str(&app.current_sg) {
            Ok(val) => val,
            Err(_) => {
                app.result = Some("Error: Invalid current SG".to_string());
                return;
            }
        };

        let current_sg_meas = match Measurement::sg(current_sg_val) {
            Ok(m) => m,
            Err(e) => {
                app.result = Some(format!("Error: {}", e));
                return;
            }
        };

        let calc = BacksweeteningCalculator;
        let input = CalcInput::new()
            .add_measurement(current_sg_meas)
            .add_param("volume", &app.sweet_vol)
            .add_param("target_sg", &app.target_sg)
            .add_param("sweetener", &app.sweetener);

        match calc.calculate(input) {
            Ok(res) => {
                app.result = Some(format!("{:.1} g {}", res.output.value, app.sweetener));
                app.warnings = res.warnings;
                app.metadata = res.metadata;
            }
            Err(e) => {
                app.result = Some(format!("Error: {}", e));
            }
        }
    }
}

fn render_sulfite(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.label("Calculate K-meta for SO₂ addition");
    ui.add_space(10.0);

    let is_metric = app.state.unit_system == UnitSystem::Metric;
    let vol_unit = if is_metric { "L" } else { "gal" };

    crate::input_field(
        ui,
        &format!("Volume ({}):", vol_unit),
        &mut app.sulfite_vol,
        "Batch volume",
    );
    crate::input_field(ui, "pH:", &mut app.ph, "e.g., 3.5");
    crate::input_field(
        ui,
        "Target Free SO₂ (ppm):",
        &mut app.target_so2,
        "e.g., 50",
    );

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate") {
        app.warnings.clear();
        app.metadata.clear();

        let ph_val = match Decimal::from_str(&app.ph) {
            Ok(val) => val,
            Err(_) => {
                app.result = Some("Error: Invalid pH".to_string());
                return;
            }
        };

        let ph_meas = match Measurement::ph(ph_val) {
            Ok(m) => m,
            Err(e) => {
                app.result = Some(format!("Error: {}", e));
                return;
            }
        };

        let calc = SulfiteCalculator;
        let input = CalcInput::new()
            .add_measurement(ph_meas)
            .add_param("volume", &app.sulfite_vol)
            .add_param("target_free_so2", &app.target_so2);

        match calc.calculate(input) {
            Ok(res) => {
                app.result = Some(format!("{:.1} g K-meta", res.output.value));
                app.warnings = res.warnings;
                app.metadata = res.metadata;
            }
            Err(e) => {
                app.result = Some(format!("Error: {}", e));
            }
        }
    }
}

fn render_acid(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.label("Calculate acid addition for pH adjustment");
    ui.add_space(10.0);

    let is_metric = app.state.unit_system == UnitSystem::Metric;
    let vol_unit = if is_metric { "L" } else { "gal" };

    crate::input_field(
        ui,
        &format!("Volume ({}):", vol_unit),
        &mut app.acid_vol,
        "Batch volume",
    );
    crate::input_field(ui, "Current pH:", &mut app.current_ph, "e.g., 3.8");
    crate::input_field(ui, "Target pH:", &mut app.target_ph_acid, "e.g., 3.4");

    ui.horizontal(|ui| {
        ui.label("Acid Type:");
        egui::ComboBox::from_id_salt("acid_type")
            .selected_text(&app.acid_type)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.acid_type, "tartaric".to_string(), "Tartaric");
                ui.selectable_value(&mut app.acid_type, "citric".to_string(), "Citric");
                ui.selectable_value(&mut app.acid_type, "malic".to_string(), "Malic");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate") {
        app.warnings.clear();
        app.metadata.clear();

        let current_ph_val = match Decimal::from_str(&app.current_ph) {
            Ok(val) => val,
            Err(_) => {
                app.result = Some("Error: Invalid current pH".to_string());
                return;
            }
        };

        let current_ph_meas = match Measurement::ph(current_ph_val) {
            Ok(m) => m,
            Err(e) => {
                app.result = Some(format!("Error: {}", e));
                return;
            }
        };

        let calc = AcidAdditionCalculator;
        let input = CalcInput::new()
            .add_measurement(current_ph_meas)
            .add_param("volume", &app.acid_vol)
            .add_param("target_ph", &app.target_ph_acid)
            .add_param("acid_type", &app.acid_type);

        match calc.calculate(input) {
            Ok(res) => {
                app.result = Some(format!("{:.1} g {} acid", res.output.value, app.acid_type));
                app.warnings = res.warnings;
                app.metadata = res.metadata;
            }
            Err(e) => {
                app.result = Some(format!("Error: {}", e));
            }
        }
    }
}

fn render_pasteurization(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.label("Calculate pasteurization time for bottled mead");
    ui.add_space(10.0);

    let is_metric = app.state.unit_system == UnitSystem::Metric;
    let temp_unit = if is_metric { "°C" } else { "°F" };

    crate::input_field(
        ui,
        &format!("Temperature ({}):", temp_unit),
        &mut app.pasteurization_temp,
        if is_metric { "e.g., 63" } else { "e.g., 145" },
    );

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate") {
        app.warnings.clear();
        app.metadata.clear();

        // Parse temperature
        let temp_input = match app.pasteurization_temp.parse::<f64>() {
            Ok(val) => val,
            Err(_) => {
                app.result = Some("Error: Invalid temperature".to_string());
                return;
            }
        };

        // Convert F to C if needed
        let temp_celsius = if is_metric {
            temp_input
        } else {
            (temp_input - 32.0) * 5.0 / 9.0
        };

        // Also calculate Fahrenheit for display
        let temp_fahrenheit = if is_metric {
            temp_input * 9.0 / 5.0 + 32.0
        } else {
            temp_input
        };

        let calc = PasteurizationCalculator;
        let input = CalcInput::new().add_param("temperature", temp_celsius.to_string());

        match calc.calculate(input) {
            Ok(res) => {
                app.result = Some(format!("{:.1} minutes", res.output.value));
                app.warnings = res.warnings;

                // Override incorrect metadata with correct temperature values
                app.metadata = res
                    .metadata
                    .into_iter()
                    .map(|(k, v)| {
                        if k == "temperature_c" {
                            (
                                "Temperature (°C)".to_string(),
                                format!("{:.1}°C", temp_celsius),
                            )
                        } else if k == "temperature_f" {
                            (
                                "Temperature (°F)".to_string(),
                                format!("{:.1}°F", temp_fahrenheit),
                            )
                        } else {
                            (k, v)
                        }
                    })
                    .collect();
            }
            Err(e) => {
                app.result = Some(format!("Error: {}", e));
            }
        }
    }

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(5.0);
    ui.label("💡 Tips:");
    ui.label("• LTLT: 63°C (145°F) for 30 minutes - gentlest method");
    ui.label("• HTST: 72°C (161°F) for 15 seconds - quick method");
    ui.label("• Use champagne bottles rated for pressure");
    ui.label("• Monitor temperature constantly - don't exceed target");
    ui.label("• Cool bottles quickly after pasteurization");
}

fn render_stabilization(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.label("Calculate K-meta and sorbate for complete stabilization");
    ui.add_space(10.0);

    let is_metric = app.state.unit_system == UnitSystem::Metric;
    let vol_unit = if is_metric { "L" } else { "gal" };

    crate::input_field(
        ui,
        &format!("Volume ({}):", vol_unit),
        &mut app.stabilization_vol,
        "Batch volume",
    );

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate") {
        app.warnings.clear();
        app.metadata.clear();

        let calc = StabilizationCalculator;
        let input = CalcInput::new().add_param("volume", &app.stabilization_vol);

        match calc.calculate(input) {
            Ok(res) => {
                app.result = Some(format!("{:.1} g K-meta", res.output.value));
                app.warnings = res.warnings;
                app.metadata = res.metadata;
            }
            Err(e) => {
                app.result = Some(format!("Error: {}", e));
            }
        }
    }

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(5.0);
    ui.label("💡 Tips:");
    ui.label("• Standard dosage: K-meta 0.5 g/L + Sorbate 0.75 g/L");
    ui.label("• ALWAYS use both K-meta AND sorbate together");
    ui.label("• Only stabilize fully fermented, clarified mead");
    ui.label("• Wait 24 hours after stabilizing before backsweetening");
    ui.label("• Sorbate prevents reproduction, sulfite kills existing yeast");
}

fn render_sweetness_chart(app: &mut MazerionApp, ui: &mut egui::Ui) {
    let c = &app.state.custom_colors;

    ui.heading(
        egui::RichText::new("🍯 Mead Sweetness Guide")
            .size(24.0)
            .color(c.honey_gold),
    );
    ui.add_space(20.0);

    ui.label(
        egui::RichText::new("Understanding Final Gravity (FG) and Sweetness Perception")
            .size(16.0)
            .color(c.dark_text),
    );
    ui.add_space(5.0);
    ui.label("Final Gravity determines residual sweetness. Use this guide to hit your target sweetness level.");
    ui.add_space(25.0);

    // Sweetness categories - each in its own result-style box
    let categories = [
        (
            "Bone Dry",
            "0.990 - 0.996",
            "Crisp, tart, wine-like finish. Zero residual sugar. Excellent for traditional meads aged on oak. Lets terroir shine through.",
        ),
        (
            "Dry",
            "0.996 - 1.006",
            "Clean finish with very subtle sweetness. Honey character present but not sweet. Ideal for session meads and everyday drinking.",
        ),
        (
            "Semi-Sweet",
            "1.006 - 1.015",
            "Noticeable honey sweetness, well-balanced. Most popular range. Perfect for traditional and fruit meads. Crowd-pleaser.",
        ),
        (
            "Sweet",
            "1.015 - 1.025",
            "Dessert mead territory. Significant sweetness, honey-forward. Pairs excellently with spicy foods and strong cheeses.",
        ),
        (
            "Very Sweet",
            "1.025 - 1.040",
            "Rich dessert mead, heavy sweetness. Best served in small portions. Excellent for sack mead or special occasions.",
        ),
    ];

    for (name, fg_range, desc) in categories {
        egui::Frame::default()
            .fill(Color32::WHITE)
            .stroke(egui::Stroke::new(2.0, c.forest_green))
            .corner_radius(egui::CornerRadius::same(8))
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(name)
                            .size(18.0)
                            .strong()
                            .color(c.honey_gold),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new(format!("FG: {}", fg_range))
                                .size(16.0)
                                .color(c.dark_text),
                        );
                    });
                });
                ui.add_space(8.0);
                ui.label(egui::RichText::new(desc).size(14.0).color(c.dark_text));
            });
        ui.add_space(12.0);
    }

    ui.add_space(25.0);

    // Style recommendations box
    egui::Frame::default()
        .fill(c.light_cream)
        .stroke(egui::Stroke::new(2.0, c.honey_gold))
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new("📋 Recommended FG by Mead Style")
                    .size(18.0)
                    .strong()
                    .color(c.forest_green),
            );
            ui.add_space(12.0);

            let styles = [
                (
                    "Traditional (Show Mead)",
                    "1.000 - 1.010",
                    "Let honey character shine",
                ),
                (
                    "Melomel (Fruit Mead)",
                    "1.008 - 1.015",
                    "Balance fruit acidity",
                ),
                (
                    "Metheglin (Spice Mead)",
                    "1.000 - 1.012",
                    "Prevent overpowering spices",
                ),
                (
                    "Cyser (Apple Mead)",
                    "1.010 - 1.015",
                    "Complement apple tartness",
                ),
                (
                    "Bochet (Caramelized)",
                    "1.015 - 1.025",
                    "Match caramel richness",
                ),
                (
                    "Pyment (Grape Mead)",
                    "0.996 - 1.006",
                    "Wine-like, honey notes",
                ),
                ("Braggot (Malt)", "1.008 - 1.018", "Balance malt sweetness"),
                ("Acerglyn (Maple)", "1.010 - 1.020", "Highlight maple notes"),
            ];

            for (style, target, reason) in styles {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("▸").size(16.0).color(c.honey_gold));
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(style)
                                    .size(15.0)
                                    .strong()
                                    .color(c.dark_text),
                            );
                            ui.label(
                                egui::RichText::new(format!("→ {}", target))
                                    .size(14.0)
                                    .color(c.forest_green),
                            );
                        });
                        ui.label(egui::RichText::new(reason).size(13.0).color(c.dark_text));
                    });
                });
                ui.add_space(8.0);
            }
        });

    ui.add_space(25.0);

    // Professional tips box
    egui::Frame::default()
        .fill(Color32::WHITE)
        .stroke(egui::Stroke::new(2.0, c.sunset_orange))
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.label(egui::RichText::new("💡 Professional Backsweetening Protocol")
                .size(18.0)
                .strong()
                .color(c.sunset_orange));
            ui.add_space(12.0);

            let tips = [
                ("1. Stabilize First", "ALWAYS add K-meta (50 ppm) AND sorbate (0.75 g/L) before any backsweetening. Wait 24 hours minimum."),
                ("2. Calculate Amount", "Use the Backsweetening calculator to determine precise sweetener needed. Account for dilution."),
                ("3. Add Gradually", "Start with 50-60% of calculated amount. Mix thoroughly. Wait 30 minutes for full integration."),
                ("4. Taste Test", "Sample at serving temperature (10-15°C). Sweetness perception changes with temperature."),
                ("5. Adjust Incrementally", "Add remaining sweetener in 10% increments. You can always add more, never remove."),
                ("6. Consider Aging", "Sweetness perception decreases with aging. Slightly over-sweeten if aging >6 months."),
                ("7. Blend Method", "Advanced: Reserve 10% dry mead. Backsweeten remaining 90%. Blend to fine-tune final sweetness."),
                ("8. Document Results", "Record exact amounts and FG achieved. Reference for future batches of same style."),
            ];

            for (title, detail) in tips {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("✓")
                        .size(16.0)
                        .color(c.forest_green));
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(title)
                            .size(15.0)
                            .strong()
                            .color(c.dark_text));
                        ui.label(egui::RichText::new(detail)
                            .size(13.0)
                            .color(c.dark_text));
                    });
                });
                ui.add_space(10.0);
            }
        });

    ui.add_space(25.0);

    // Quick reference calculations
    egui::Frame::default()
        .fill(c.light_cream)
        .stroke(egui::Stroke::new(2.0, c.forest_green))
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.label(egui::RichText::new("📊 Quick Reference: Gravity Points & Honey Weight")
                .size(18.0)
                .strong()
                .color(c.forest_green));
            ui.add_space(12.0);

            ui.label(egui::RichText::new("For a 5 gallon (19L) batch, approximate honey needed:")
                .size(14.0)
                .color(c.dark_text));
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("FG Change").strong().size(14.0).color(c.honey_gold));
                    ui.label("1.000 → 1.005");
                    ui.label("1.000 → 1.010");
                    ui.label("1.000 → 1.015");
                    ui.label("1.000 → 1.020");
                    ui.label("1.000 → 1.025");
                });
                ui.add_space(30.0);
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("Honey (kg)").strong().size(14.0).color(c.honey_gold));
                    ui.label("~0.18 kg");
                    ui.label("~0.36 kg");
                    ui.label("~0.54 kg");
                    ui.label("~0.72 kg");
                    ui.label("~0.90 kg");
                });
                ui.add_space(30.0);
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("Honey (lb)").strong().size(14.0).color(c.honey_gold));
                    ui.label("~0.40 lb");
                    ui.label("~0.79 lb");
                    ui.label("~1.19 lb");
                    ui.label("~1.59 lb");
                    ui.label("~1.98 lb");
                });
            });

            ui.add_space(12.0);
            ui.label(egui::RichText::new("⚠️ These are approximations. Always use the Backsweetening calculator for precise amounts.")
                .size(13.0)
                .color(c.sunset_orange));
        });
}
