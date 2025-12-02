//! Finishing calculators tab

use crate::{MazerionApp, state::{FinishingCalculator, colors}};
use eframe::egui::{self, RichText, CornerRadius};
use mazerion_core::{CalcInput, Measurement};
use std::str::FromStr;
use rust_decimal::Decimal;

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Select Calculator:").strong());
        egui::ComboBox::from_id_salt("finishing_calc")
            .selected_text(get_calc_name(app.state.finishing_calc))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.state.finishing_calc, FinishingCalculator::Backsweetening, "Backsweetening Calculator");
                ui.selectable_value(&mut app.state.finishing_calc, FinishingCalculator::Sulfite, "Sulfite Calculator");
                ui.selectable_value(&mut app.state.finishing_calc, FinishingCalculator::AcidAddition, "Acid Addition Calculator");
            });
    });

    ui.add_space(10.0);

    egui::Frame::new()
        .fill(colors::LIGHT_CREAM)
        .stroke(egui::Stroke::new(1.5, colors::HONEY_GOLD))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            match app.state.finishing_calc {
                FinishingCalculator::Backsweetening => render_backsweetening(app, ui),
                FinishingCalculator::Sulfite => render_sulfite(app, ui),
                FinishingCalculator::AcidAddition => render_acid(app, ui),
            }
        });
}

fn get_calc_name(calc: FinishingCalculator) -> &'static str {
    match calc {
        FinishingCalculator::Backsweetening => "Backsweetening Calculator",
        FinishingCalculator::Sulfite => "Sulfite Calculator",
        FinishingCalculator::AcidAddition => "Acid Addition Calculator",
    }
}

fn render_backsweetening(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍯 Backsweetening Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate sweetener additions to reach target sweetness");
    ui.label(RichText::new("⚠️ MUST stabilize before backsweetening!").color(colors::DARK_ORANGE).strong());
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };

    crate::input_field(ui, &format!("Volume ({}):", vol_unit), &mut app.sweet_vol, "Total volume to sweeten");
    crate::input_field(ui, "Current SG:", &mut app.current_sg, "Current specific gravity");
    crate::input_field(ui, "Target SG:", &mut app.target_sg, "Desired final gravity");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Sweetener:").strong());
        egui::ComboBox::from_id_salt("sweetener")
            .selected_text(&app.sweetener)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.sweetener, "honey".to_string(), "Honey");
                ui.selectable_value(&mut app.sweetener, "table_sugar".to_string(), "Table Sugar");
                ui.selectable_value(&mut app.sweetener, "agave".to_string(), "Agave Nectar");
                ui.selectable_value(&mut app.sweetener, "maple_syrup".to_string(), "Maple Syrup");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Sweetener Amount") {
        calc_backsweetening(app);
    }
}

fn render_sulfite(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🛡️ Sulfite Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate K-meta additions with pH-dependent effectiveness");
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };

    crate::input_field(ui, &format!("Volume ({}):", vol_unit), &mut app.sulfite_vol, "Total volume to treat");
    crate::input_field(ui, "pH:", &mut app.ph, "Current pH (critical for effectiveness!)");
    crate::input_field(ui, "Target Free SO₂ (ppm):", &mut app.target_so2, "Desired free SO₂ level (20-50 ppm typical)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Sulfite Addition") {
        calc_sulfite(app);
    }
}

fn render_acid(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍋 Acid Addition Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate acid additions to adjust pH");
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };

    crate::input_field(ui, &format!("Volume ({}):", vol_unit), &mut app.acid_vol, "Total volume to treat");
    crate::input_field(ui, "Current pH:", &mut app.current_ph, "Current pH measurement");
    crate::input_field(ui, "Target pH:", &mut app.target_ph_acid, "Desired pH (must be lower than current)");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Acid Type:").strong());
        egui::ComboBox::from_id_salt("acid_type")
            .selected_text(&app.acid_type)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.acid_type, "tartaric".to_string(), "Tartaric (strongest, wine)");
                ui.selectable_value(&mut app.acid_type, "citric".to_string(), "Citric (bright, fruity)");
                ui.selectable_value(&mut app.acid_type, "malic".to_string(), "Malic (soft, apple-like)");
                ui.selectable_value(&mut app.acid_type, "lactic".to_string(), "Lactic (smooth, creamy)");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Acid Addition") {
        calc_acid_addition(app);
    }
}

fn calc_backsweetening(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("backsweetening") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Backsweetening calculator not found".to_string());
            return;
        }
    };

    let current_sg_val = match Decimal::from_str(&app.current_sg) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid current SG value".to_string());
            return;
        }
    };

    let sg_meas = match Measurement::sg(current_sg_val) {
        Ok(m) => m,
        Err(e) => {
            app.result = Some(format!("Error: {}", e));
            return;
        }
    };

    let input = CalcInput::new()
        .add_measurement(sg_meas)
        .add_param("volume", &app.sweet_vol)
        .add_param("target_sg", &app.target_sg)
        .add_param("sweetener", &app.sweetener);

    match calc.calculate(input) {
        Ok(res) => {
            let weight_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "g" } else { "oz" };
            app.result = Some(format!("{}: {:.0} {}",
                                      match app.sweetener.as_str() {
                                          "honey" => "Honey",
                                          "table_sugar" => "Table Sugar",
                                          "agave" => "Agave",
                                          "maple_syrup" => "Maple Syrup",
                                          _ => "Sweetener"
                                      },
                                      res.output.value,
                                      weight_unit
            ));
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

fn calc_sulfite(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("sulfite") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Sulfite calculator not found".to_string());
            return;
        }
    };

    let ph_val = match Decimal::from_str(&app.ph) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid pH value".to_string());
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

    let input = CalcInput::new()
        .add_measurement(ph_meas)
        .add_param("volume", &app.sulfite_vol)
        .add_param("target_free_so2", &app.target_so2);

    match calc.calculate(input) {
        Ok(res) => {
            let weight_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "g" } else { "oz" };
            app.result = Some(format!("K-meta: {:.2} {}", res.output.value, weight_unit));
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

fn calc_acid_addition(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("acid_addition") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Acid addition calculator not found".to_string());
            return;
        }
    };

    let current_ph_val = match Decimal::from_str(&app.current_ph) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid current pH value".to_string());
            return;
        }
    };

    let ph_meas = match Measurement::ph(current_ph_val) {
        Ok(m) => m,
        Err(e) => {
            app.result = Some(format!("Error: {}", e));
            return;
        }
    };

    let input = CalcInput::new()
        .add_measurement(ph_meas)
        .add_param("volume", &app.acid_vol)
        .add_param("target_ph", &app.target_ph_acid)
        .add_param("acid_type", &app.acid_type);

    match calc.calculate(input) {
        Ok(res) => {
            let weight_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "g" } else { "oz" };
            app.result = Some(format!("{} Acid: {:.2} {}",
                                      match app.acid_type.as_str() {
                                          "tartaric" => "Tartaric",
                                          "citric" => "Citric",
                                          "malic" => "Malic",
                                          "lactic" => "Lactic",
                                          _ => "Acid"
                                      },
                                      res.output.value,
                                      weight_unit
            ));
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