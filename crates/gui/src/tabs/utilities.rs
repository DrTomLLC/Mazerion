//! Utilities calculators - IMPERIAL UNITS FIXED

use crate::{MazerionApp, state::colors};
use eframe::egui::{self, RichText, CornerRadius};
use mazerion_core::CalcInput;
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UtilityCalculator {
    BenchTrials,
    RecipeUpscaling,
    GallonsToBottles,
    Waste,
}

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Select Calculator:").strong());
        egui::ComboBox::from_id_salt("utility_calc")
            .selected_text(get_calc_name(app.utility_calc))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::BenchTrials, "Bench Trials");
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::RecipeUpscaling, "Recipe Upscaling");
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::GallonsToBottles, "Gallons to Bottles (with Losses)");
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::Waste, "Waste/Loss");
            });
    });

    ui.add_space(10.0);

    egui::Frame::new()
        .fill(colors::LIGHT_CREAM)
        .stroke(egui::Stroke::new(1.5, colors::HONEY_GOLD))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            match app.utility_calc {
                UtilityCalculator::BenchTrials => render_bench_trials(app, ui),
                UtilityCalculator::RecipeUpscaling => render_recipe_upscaling(app, ui),
                UtilityCalculator::GallonsToBottles => render_gallons_to_bottles_with_losses(app, ui),
                UtilityCalculator::Waste => render_waste(app, ui),
            }
        });
}

fn get_calc_name(calc: UtilityCalculator) -> &'static str {
    match calc {
        UtilityCalculator::BenchTrials => "Bench Trials",
        UtilityCalculator::RecipeUpscaling => "Recipe Upscaling",
        UtilityCalculator::GallonsToBottles => "Gallons to Bottles (with Losses)",
        UtilityCalculator::Waste => "Waste/Loss",
    }
}

fn render_bench_trials(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🧪 Bench Trials").color(colors::SADDLE_BROWN));
    ui.label("Test small amounts and scale to full batch");
    ui.add_space(10.0);

    crate::input_field(ui, "Trial Volume (mL):", &mut app.trial_volume, "Small test batch");
    crate::input_field(ui, "Trial Addition (g):", &mut app.trial_addition, "Amount added");
    crate::input_field(ui, "Batch Volume (L):", &mut app.batch_volume_bench, "Full batch");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate") {
        calc_bench_trials(app);
    }
}

fn render_recipe_upscaling(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("📏 Recipe Upscaling").color(colors::SADDLE_BROWN));
    ui.label("Scale recipes proportionally");
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };

    crate::input_field(ui, &format!("Original Size ({}):", vol_unit), &mut app.original_recipe_size, "Original batch");
    crate::input_field(ui, &format!("Target Size ({}):", vol_unit), &mut app.target_batch_size, "Desired batch");
    crate::input_field(ui, "Original Amount:", &mut app.original_amount, "Amount in recipe");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Unit:").strong());
        egui::ComboBox::from_id_salt("upscale_unit")
            .selected_text(&app.upscale_unit)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.upscale_unit, "g".to_string(), "g");
                ui.selectable_value(&mut app.upscale_unit, "kg".to_string(), "kg");
                ui.selectable_value(&mut app.upscale_unit, "lb".to_string(), "lb");
                ui.selectable_value(&mut app.upscale_unit, "oz".to_string(), "oz");
                ui.selectable_value(&mut app.upscale_unit, "mL".to_string(), "mL");
                ui.selectable_value(&mut app.upscale_unit, "L".to_string(), "L");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate") {
        calc_recipe_upscaling(app);
    }
}

fn render_gallons_to_bottles_with_losses(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍾 Bottles (with Losses)").color(colors::SADDLE_BROWN));

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);
    let vol_unit = if is_metric { "L" } else { "gal" };

    crate::input_field(ui, &format!("Initial Volume ({}):", vol_unit), &mut app.waste_initial_volume, "Before losses");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Vessel:").strong());
        egui::ComboBox::from_id_salt("bottle_vessel")
            .selected_text(&app.waste_vessel_type)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.waste_vessel_type, "carboy".to_string(), "Carboy (6%)");
                ui.selectable_value(&mut app.waste_vessel_type, "bucket".to_string(), "Bucket (8%)");
                ui.selectable_value(&mut app.waste_vessel_type, "keg".to_string(), "Keg (5%)");
                ui.selectable_value(&mut app.waste_vessel_type, "barrel".to_string(), "Barrel (7%)");
            });
    });

    crate::input_field(ui, "Rackings:", &mut app.waste_num_rackings, "0-10");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Process:").strong());
        egui::ComboBox::from_id_salt("bottle_process")
            .selected_text(&app.waste_process_type)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.waste_process_type, "standard".to_string(), "Standard");
                ui.selectable_value(&mut app.waste_process_type, "fined".to_string(), "Fined");
                ui.selectable_value(&mut app.waste_process_type, "filtered".to_string(), "Filtered");
                ui.selectable_value(&mut app.waste_process_type, "none".to_string(), "None");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate") {
        calc_gallons_to_bottles_with_losses(app);
    }
}

fn render_waste(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("📉 Waste/Loss").color(colors::SADDLE_BROWN));

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);
    let vol_unit = if is_metric { "L" } else { "gal" };

    crate::input_field(ui, &format!("Initial Volume ({}):", vol_unit), &mut app.waste_initial_volume, "Start volume");
    crate::input_field(ui, "Rackings:", &mut app.waste_num_rackings, "0-10");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Vessel:").strong());
        egui::ComboBox::from_id_salt("vessel")
            .selected_text(&app.waste_vessel_type)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.waste_vessel_type, "carboy".to_string(), "Carboy");
                ui.selectable_value(&mut app.waste_vessel_type, "bucket".to_string(), "Bucket");
                ui.selectable_value(&mut app.waste_vessel_type, "keg".to_string(), "Keg");
                ui.selectable_value(&mut app.waste_vessel_type, "barrel".to_string(), "Barrel");
            });
    });

    ui.horizontal(|ui| {
        ui.label(RichText::new("Process:").strong());
        egui::ComboBox::from_id_salt("process")
            .selected_text(&app.waste_process_type)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.waste_process_type, "standard".to_string(), "Standard");
                ui.selectable_value(&mut app.waste_process_type, "fined".to_string(), "Fined");
                ui.selectable_value(&mut app.waste_process_type, "filtered".to_string(), "Filtered");
                ui.selectable_value(&mut app.waste_process_type, "none".to_string(), "None");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate") {
        calc_waste(app);
    }
}

fn calc_bench_trials(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("bench_trials") {
        Some(c) => c,
        None => {
            app.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let input = CalcInput::new()
        .add_param("trial_volume", &app.trial_volume)
        .add_param("trial_addition", &app.trial_addition)
        .add_param("batch_volume", &app.batch_volume_bench);

    match calc.calculate(input) {
        Ok(res) => {
            let batch_g = res.output.value;
            app.result = Some(format!("{:.1} g", batch_g));
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

fn calc_recipe_upscaling(app: &mut MazerionApp) {
    let original = match Decimal::from_str(&app.original_recipe_size) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid original size".to_string());
            return;
        }
    };

    let target = match Decimal::from_str(&app.target_batch_size) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid target size".to_string());
            return;
        }
    };

    let amount = match Decimal::from_str(&app.original_amount) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid amount".to_string());
            return;
        }
    };

    let scaled = amount * (target / original);

    app.result = Some(format!("{:.2} {}", scaled, app.upscale_unit));
    app.warnings.clear();
    app.metadata.clear();
}

fn calc_gallons_to_bottles_with_losses(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("gallons_to_bottles_with_losses") {
        Some(c) => c,
        None => {
            app.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);

    let volume_l = if is_metric {
        app.waste_initial_volume.clone()
    } else {
        match Decimal::from_str(&app.waste_initial_volume) {
            Ok(gal) => (gal * Decimal::new(378541, 5)).to_string(),
            Err(_) => {
                app.result = Some("❌ Invalid volume".to_string());
                return;
            }
        }
    };

    let input = CalcInput::new()
        .add_param("initial_volume", &volume_l)
        .add_param("vessel_type", &app.waste_vessel_type)
        .add_param("num_rackings", &app.waste_num_rackings)
        .add_param("process_type", &app.waste_process_type);

    match calc.calculate(input) {
        Ok(res) => {
            let final_l = res.output.value;

            // Get bottles from metadata
            let bottles_750 = res.metadata.iter()
                .find(|(k, _)| k == "bottles_750ml")
                .and_then(|(_, v)| v.split_whitespace().next())
                .unwrap_or("0");

            let bottles_375 = res.metadata.iter()
                .find(|(k, _)| k == "bottles_375ml")
                .and_then(|(_, v)| v.split_whitespace().next())
                .unwrap_or("0");

            let bottles_500 = res.metadata.iter()
                .find(|(k, _)| k == "bottles_500ml")
                .and_then(|(_, v)| v.split_whitespace().next())
                .unwrap_or("0");

            // Display in user's unit system
            let (vol_display, vol_unit) = if is_metric {
                (format!("{:.2}", final_l), "L")
            } else {
                let gal = final_l * Decimal::new(264172, 6);
                (format!("{:.2}", gal), "gal")
            };

            app.result = Some(format!(
                "Final: {} {} | 750mL: {} | 375mL: {} | 500mL: {}",
                vol_display, vol_unit, bottles_750, bottles_375, bottles_500
            ));

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

fn calc_waste(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("waste") {
        Some(c) => c,
        None => {
            app.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);

    let volume_l = if is_metric {
        app.waste_initial_volume.clone()
    } else {
        match Decimal::from_str(&app.waste_initial_volume) {
            Ok(gal) => (gal * Decimal::new(378541, 5)).to_string(),
            Err(_) => {
                app.result = Some("❌ Invalid volume".to_string());
                return;
            }
        }
    };

    let input = CalcInput::new()
        .add_param("initial_volume", &volume_l)
        .add_param("num_rackings", &app.waste_num_rackings)
        .add_param("vessel_type", &app.waste_vessel_type)
        .add_param("process_type", &app.waste_process_type);

    match calc.calculate(input) {
        Ok(res) => {
            let (vol_display, vol_unit) = if is_metric {
                (format!("{:.2}", res.output.value), "L")
            } else {
                let gal = res.output.value * Decimal::new(264172, 6);
                (format!("{:.2}", gal), "gal")
            };

            app.result = Some(format!("Final: {} {}", vol_display, vol_unit));
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