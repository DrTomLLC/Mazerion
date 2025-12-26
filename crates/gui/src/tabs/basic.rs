//! Basic calculators tab - COMPLETE AND CORRECT
//! ABV, Brix↔SG bidirectional, Dilution with Fl Oz

use crate::{MazerionApp, state::BasicCalculator};
use eframe::egui::{self, CornerRadius, RichText};
use mazerion_core::CalcInput;
use rust_decimal::Decimal;
use std::str::FromStr;

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    let c = app.state.custom_colors;

    ui.horizontal(|ui| {
        ui.label(RichText::new("Select Calculator:").strong());
        egui::ComboBox::from_id_salt("basic_calc")
            .selected_text(get_calc_name(app.state.basic_calc))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut app.state.basic_calc,
                    BasicCalculator::Abv,
                    "ABV Calculator",
                );
                ui.selectable_value(
                    &mut app.state.basic_calc,
                    BasicCalculator::BrixSgConverter,
                    "Brix ↔ SG Converter",
                );
                ui.selectable_value(
                    &mut app.state.basic_calc,
                    BasicCalculator::Dilution,
                    "Dilution Calculator",
                );
            });
    });

    ui.add_space(10.0);

    egui::Frame::new()
        .fill(c.light_cream)
        .stroke(egui::Stroke::new(1.5, c.honey_gold))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| match app.state.basic_calc {
            BasicCalculator::Abv => render_abv(app, ui, c),
            BasicCalculator::BrixSgConverter => render_brix_converter(app, ui, c),
            BasicCalculator::Dilution => render_dilution(app, ui, c),
        });
}

fn get_calc_name(calc: BasicCalculator) -> &'static str {
    match calc {
        BasicCalculator::Abv => "ABV Calculator",
        BasicCalculator::BrixSgConverter => "Brix ↔ SG Converter",
        BasicCalculator::Dilution => "Dilution Calculator",
    }
}

fn render_abv(app: &mut MazerionApp, ui: &mut egui::Ui, c: crate::state::CustomColors) {
    ui.heading(RichText::new("🍺 ABV Calculator").color(c.saddle_brown));
    ui.label("Calculate alcohol by volume from gravity readings");
    ui.add_space(10.0);

    crate::input_field(
        ui,
        "Original Gravity (OG):",
        &mut app.og,
        "Starting specific gravity (e.g., 1.090)",
    );
    crate::input_field(
        ui,
        "Final Gravity (FG):",
        &mut app.fg,
        "Ending specific gravity (e.g., 1.010)",
    );

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate ABV") {
        calc_abv(app);
    }
}

fn render_brix_converter(app: &mut MazerionApp, ui: &mut egui::Ui, c: crate::state::CustomColors) {
    ui.heading(RichText::new("📐 Brix ↔ SG Converter").color(c.saddle_brown));
    ui.label("Convert between degrees Brix and Specific Gravity");
    ui.add_space(10.0);

    // Brix to SG
    ui.label(RichText::new("Brix → SG:").strong());
    crate::input_field(
        ui,
        "Brix (°Bx):",
        &mut app.brix,
        "Degrees Brix to convert to SG",
    );

    ui.add_space(10.0);

    // SG to Brix
    ui.label(RichText::new("SG → Brix:").strong());
    crate::input_field(
        ui,
        "Specific Gravity:",
        &mut app.sg_for_brix,
        "SG to convert to Brix (e.g., 1.083)",
    );

    ui.add_space(5.0);
    ui.label(
        RichText::new("💡 Tip: Fill in either Brix OR SG, leave the other empty")
            .size(12.0)
            .weak(),
    );

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Convert") {
        calc_brix_conversion(app);
    }
}

fn render_dilution(app: &mut MazerionApp, ui: &mut egui::Ui, c: crate::state::CustomColors) {
    ui.heading(RichText::new("💧 Dilution Calculator").color(c.saddle_brown));
    ui.label("Calculate water needed to reduce ABV");
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) {
        "L"
    } else {
        "gal"
    };
    crate::input_field(
        ui,
        &format!("Current Volume ({}):", vol_unit),
        &mut app.current_vol,
        "Current volume",
    );
    crate::input_field(
        ui,
        "Current ABV (%):",
        &mut app.current_abv,
        "Current alcohol percentage",
    );
    crate::input_field(
        ui,
        "Target ABV (%):",
        &mut app.target_abv,
        "Desired alcohol percentage",
    );

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Dilution") {
        calc_dilution(app);
    }
}

// === CALCULATION FUNCTIONS ===

fn calc_abv(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("abv") {
        Some(c) => c,
        None => {
            app.result = Some("❌ ABV calculator not found".to_string());
            return;
        }
    };

    let input = CalcInput::new()
        .add_param("og", &app.og)
        .add_param("fg", &app.fg);

    match calc.calculate(input) {
        Ok(res) => {
            app.result = Some(format!("ABV: {:.2}%", res.output.value));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => {
            app.result = Some(format!("❌ {}", e));
            app.warnings.clear();
            app.metadata.clear();
        }
    }
}

fn calc_brix_conversion(app: &mut MazerionApp) {
    // Try Brix to SG first
    if !app.brix.is_empty() {
        let calc = match mazerion_core::traits::get_calculator("brix_to_sg") {
            Some(c) => c,
            None => {
                app.result = Some("❌ Brix to SG calculator not found".to_string());
                return;
            }
        };

        let brix_val = match Decimal::from_str(&app.brix) {
            Ok(v) => v,
            Err(_) => {
                app.result = Some("❌ Invalid Brix value".to_string());
                return;
            }
        };

        let measurement = match mazerion_core::Measurement::brix(brix_val) {
            Ok(m) => m,
            Err(e) => {
                app.result = Some(format!("❌ {}", e));
                return;
            }
        };

        let input = CalcInput::new().add_measurement(measurement);

        match calc.calculate(input) {
            Ok(res) => {
                app.result = Some(format!("SG: {:.4}", res.output.value));
                app.warnings = res.warnings;
                app.metadata = res.metadata;
            }
            Err(e) => {
                app.result = Some(format!("❌ {}", e));
                app.warnings.clear();
                app.metadata.clear();
            }
        }
    }
    // Try SG to Brix if Brix field is empty
    else if !app.sg_for_brix.is_empty() {
        let calc = match mazerion_core::traits::get_calculator("sg_to_brix") {
            Some(c) => c,
            None => {
                app.result = Some("❌ SG to Brix calculator not found".to_string());
                return;
            }
        };

        let sg_val = match Decimal::from_str(&app.sg_for_brix) {
            Ok(v) => v,
            Err(_) => {
                app.result = Some("❌ Invalid SG value".to_string());
                return;
            }
        };

        let measurement = match mazerion_core::Measurement::sg(sg_val) {
            Ok(m) => m,
            Err(e) => {
                app.result = Some(format!("❌ {}", e));
                return;
            }
        };

        let input = CalcInput::new().add_measurement(measurement);

        match calc.calculate(input) {
            Ok(res) => {
                app.result = Some(format!("Brix: {:.2}°Bx", res.output.value));
                app.warnings = res.warnings;
                app.metadata = res.metadata;
            }
            Err(e) => {
                app.result = Some(format!("❌ {}", e));
                app.warnings.clear();
                app.metadata.clear();
            }
        }
    } else {
        app.result = Some("❌ Enter either Brix OR SG".to_string());
        app.warnings.clear();
        app.metadata.clear();
    }
}

fn calc_dilution(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("dilution") {
        Some(c) => c,
        None => {
            app.result = Some("❌ Dilution calculator not found".to_string());
            return;
        }
    };

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);

    // Convert input to liters if in Imperial
    let volume_l = if is_metric {
        app.current_vol.clone()
    } else {
        match Decimal::from_str(&app.current_vol) {
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
        .add_param("current_volume", &volume_l)
        .add_param("current_abv", &app.current_abv)
        .add_param("target_abv", &app.target_abv);

    match calc.calculate(input) {
        Ok(res) => {
            let water_l = res.output.value;

            if is_metric {
                let ml = water_l * Decimal::from(1000);
                app.result = Some(format!("Water to add: {:.2} L / {:.0} mL", water_l, ml));
            } else {
                let gal = water_l * Decimal::new(264172, 6); // liters to gallons
                let fl_oz = water_l * Decimal::new(33814, 3); // liters to fluid ounces
                app.result = Some(format!("Water to add: {:.2} gal / {:.1} fl oz", gal, fl_oz));
            }

            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => {
            app.result = Some(format!("❌ {}", e));
            app.warnings.clear();
            app.metadata.clear();
        }
    }
}
