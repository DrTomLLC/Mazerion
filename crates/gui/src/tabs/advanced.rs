//! Advanced calculators tab

use crate::{MazerionApp, state::{AdvancedCalculator, colors}};
use eframe::egui::{self, RichText, CornerRadius};
use mazerion_core::{CalcInput, Measurement};
use std::str::FromStr;
use rust_decimal::Decimal;

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Select Calculator:").strong());
        egui::ComboBox::from_id_salt("advanced_calc")
            .selected_text(get_calc_name(app.state.advanced_calc))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.state.advanced_calc, AdvancedCalculator::Blending, "Blending Calculator");
                ui.selectable_value(&mut app.state.advanced_calc, AdvancedCalculator::Refractometer, "Refractometer Correction");
                ui.selectable_value(&mut app.state.advanced_calc, AdvancedCalculator::SgCorrection, "SG Temperature Correction");
            });
    });

    ui.add_space(10.0);

    egui::Frame::new()
        .fill(colors::LIGHT_CREAM)
        .stroke(egui::Stroke::new(1.5, colors::HONEY_GOLD))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            match app.state.advanced_calc {
                AdvancedCalculator::Blending => render_blending(app, ui),
                AdvancedCalculator::Refractometer => render_refractometer(app, ui),
                AdvancedCalculator::SgCorrection => render_sg_correction(app, ui),
            }
        });
}

fn get_calc_name(calc: AdvancedCalculator) -> &'static str {
    match calc {
        AdvancedCalculator::Blending => "Blending Calculator",
        AdvancedCalculator::Refractometer => "Refractometer Correction",
        AdvancedCalculator::SgCorrection => "SG Temperature Correction",
    }
}

fn render_blending(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🔀 Blending Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate final properties when mixing two batches");
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };

    ui.label(RichText::new("Batch 1:").strong().color(colors::GOLDENROD));
    crate::input_field(ui, &format!("Volume ({}):", vol_unit), &mut app.vol1, "Volume of first batch");
    crate::input_field(ui, "ABV (%):", &mut app.abv1, "ABV of first batch");

    ui.add_space(8.0);
    ui.label(RichText::new("Batch 2:").strong().color(colors::GOLDENROD));
    crate::input_field(ui, &format!("Volume ({}):", vol_unit), &mut app.vol2, "Volume of second batch");
    crate::input_field(ui, "ABV (%):", &mut app.abv2, "ABV of second batch");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Blend") {
        calc_blending(app);
    }
}

fn render_refractometer(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🔍 Refractometer Correction").color(colors::SADDLE_BROWN));
    ui.label("Correct refractometer readings for alcohol presence (Terrill cubic)");
    ui.add_space(10.0);

    crate::input_field(ui, "Original Brix (°Bx):", &mut app.orig_brix, "Original reading before fermentation");
    crate::input_field(ui, "Current Brix (°Bx):", &mut app.curr_brix, "Current reading during/after fermentation");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate True SG") {
        calc_refractometer(app);
    }
}

fn render_sg_correction(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🌡️ SG Temperature Correction").color(colors::SADDLE_BROWN));
    ui.label("Correct gravity readings for temperature (calibrated at 20°C)");
    ui.add_space(10.0);

    let temp_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "°C" } else { "°F" };

    crate::input_field(ui, "Measured SG:", &mut app.sg, "Specific gravity reading");
    crate::input_field(ui, &format!("Temperature ({}):", temp_unit), &mut app.temp, "Temperature at measurement");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Correct for Temperature") {
        calc_sg_correction(app);
    }
}

fn calc_blending(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("blending") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Blending calculator not found".to_string());
            return;
        }
    };

    let input = CalcInput::new()
        .add_param("volume1", &app.vol1)
        .add_param("abv1", &app.abv1)
        .add_param("volume2", &app.vol2)
        .add_param("abv2", &app.abv2);

    match calc.calculate(input) {
        Ok(res) => {
            app.result = Some(format!("Blended ABV: {:.2}%", res.output.value));
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

fn calc_refractometer(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("refractometer") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Refractometer calculator not found".to_string());
            return;
        }
    };

    let orig_brix_val = match Decimal::from_str(&app.orig_brix) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid original Brix value".to_string());
            return;
        }
    };

    let measurement = match Measurement::brix(orig_brix_val) {
        Ok(m) => m,
        Err(e) => {
            app.result = Some(format!("Error: {}", e));
            return;
        }
    };

    let input = CalcInput::new()
        .add_measurement(measurement)
        .add_param("current_brix", &app.curr_brix);

    match calc.calculate(input) {
        Ok(res) => {
            app.result = Some(format!("Corrected FG: {:.4}", res.output.value));
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

fn calc_sg_correction(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("sg_correction") {
        Some(c) => c,
        None => {
            app.result = Some("Error: SG correction calculator not found".to_string());
            return;
        }
    };

    let sg_val = match Decimal::from_str(&app.sg) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid SG value".to_string());
            return;
        }
    };

    let temp_val = match Decimal::from_str(&app.temp) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid temperature value".to_string());
            return;
        }
    };

    let sg_meas = match Measurement::sg(sg_val) {
        Ok(m) => m,
        Err(e) => {
            app.result = Some(format!("Error: {}", e));
            return;
        }
    };

    let temp_meas = match Measurement::celsius(temp_val) {
        Ok(m) => m,
        Err(e) => {
            app.result = Some(format!("Error: {}", e));
            return;
        }
    };

    let input = CalcInput::new()
        .add_measurement(sg_meas)
        .add_measurement(temp_meas);

    match calc.calculate(input) {
        Ok(res) => {
            app.result = Some(format!("Corrected SG: {:.4}", res.output.value));
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