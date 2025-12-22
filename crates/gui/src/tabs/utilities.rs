use crate::state::UnitSystem;
use crate::MazerionApp;
use eframe::egui;
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UtilityCalculator {
    BenchTrials,
    RecipeUpscaling,
    BottlesWithLosses,
}

impl Default for UtilityCalculator {
    fn default() -> Self {
        Self::RecipeUpscaling
    }
}

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading("🔧 Utility Calculators");
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        ui.label("Select Calculator:");
        egui::ComboBox::from_id_salt("utility_calc")
            .selected_text(match app.utility_calc {
                UtilityCalculator::BenchTrials => "🧪 Bench Trials",
                UtilityCalculator::RecipeUpscaling => "📈 Recipe Upscaling",
                UtilityCalculator::BottlesWithLosses => "🍾 Bottles (with Losses)",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::BenchTrials, "🧪 Bench Trials");
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::RecipeUpscaling, "📈 Recipe Upscaling");
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::BottlesWithLosses, "🍾 Bottles (with Losses)");
            });
    });

    ui.add_space(15.0);

    match app.utility_calc {
        UtilityCalculator::BenchTrials => render_bench_trials(app, ui),
        UtilityCalculator::RecipeUpscaling => render_recipe_upscaling(app, ui),
        UtilityCalculator::BottlesWithLosses => render_bottles_with_losses(app, ui),
    }
}

fn render_recipe_upscaling(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.label("Scale recipes proportionally");
    ui.add_space(10.0);

    let is_metric = app.state.unit_system == UnitSystem::Metric;
    let vol_unit = if is_metric { "L" } else { "gal" };

    crate::input_field(ui, &format!("Original Size ({}):", vol_unit), &mut app.original_recipe_size, "Original batch size");
    crate::input_field(ui, &format!("Target Size ({}):", vol_unit), &mut app.target_batch_size, "Desired batch size");
    crate::input_field(ui, "Original Amount:", &mut app.original_amount, "Amount of ingredient in original recipe");

    ui.horizontal(|ui| {
        ui.label("Unit:");
        egui::ComboBox::from_id_salt("upscale_unit")
            .selected_text(&app.upscale_unit)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.upscale_unit, "g".to_string(), "g (grams)");
                ui.selectable_value(&mut app.upscale_unit, "kg".to_string(), "kg (kilograms)");
                ui.selectable_value(&mut app.upscale_unit, "lb".to_string(), "lb (pounds)");
                ui.selectable_value(&mut app.upscale_unit, "oz".to_string(), "oz (ounces)");
                ui.selectable_value(&mut app.upscale_unit, "L".to_string(), "L (liters)");
                ui.selectable_value(&mut app.upscale_unit, "mL".to_string(), "mL (milliliters)");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Scaled Amount") {
        app.warnings.clear();
        app.metadata.clear();

        let original_size = match Decimal::from_str(&app.original_recipe_size) {
            Ok(val) => val,
            Err(_) => {
                app.result = Some("Error: Invalid original size".to_string());
                return;
            }
        };

        let target_size = match Decimal::from_str(&app.target_batch_size) {
            Ok(val) => val,
            Err(_) => {
                app.result = Some("Error: Invalid target size".to_string());
                return;
            }
        };

        let original_amount = match Decimal::from_str(&app.original_amount) {
            Ok(val) => val,
            Err(_) => {
                app.result = Some("Error: Invalid original amount".to_string());
                return;
            }
        };

        if original_size <= Decimal::ZERO || target_size <= Decimal::ZERO || original_amount < Decimal::ZERO {
            app.result = Some("Error: Values must be positive".to_string());
            return;
        }

        let scaling_factor = target_size / original_size;
        let scaled_amount = original_amount * scaling_factor;

        app.result = Some(format!("{:.2} {}", scaled_amount, app.upscale_unit));

        app.metadata.push(("Scaling Factor".to_string(), format!("{}x", scaling_factor.round_dp(2))));
        app.metadata.push(("Original".to_string(), format!("{:.2} {} in {} {}", original_amount, app.upscale_unit, original_size, vol_unit)));
        app.metadata.push(("Scaled".to_string(), format!("{:.2} {} in {} {}", scaled_amount, app.upscale_unit, target_size, vol_unit)));
    }
}

fn render_bench_trials(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.label("Scale up bench trial results to full batch");
    ui.add_space(10.0);

    let is_metric = app.state.unit_system == UnitSystem::Metric;
    let vol_unit = if is_metric { "L" } else { "gal" };

    crate::input_field(ui, &format!("Trial Volume ({}):", vol_unit), &mut app.trial_volume, "Small test batch volume");
    crate::input_field(ui, "Addition Amount:", &mut app.trial_addition, "Amount added in trial");
    crate::input_field(ui, &format!("Target Batch ({}):", vol_unit), &mut app.batch_volume_bench, "Full batch size");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Full Batch Amount") {
        app.warnings.clear();
        app.metadata.clear();

        let trial_vol = match Decimal::from_str(&app.trial_volume) {
            Ok(val) => val,
            Err(_) => {
                app.result = Some("Error: Invalid trial volume".to_string());
                return;
            }
        };

        let addition = match Decimal::from_str(&app.trial_addition) {
            Ok(val) => val,
            Err(_) => {
                app.result = Some("Error: Invalid addition amount".to_string());
                return;
            }
        };

        let batch_vol = match Decimal::from_str(&app.batch_volume_bench) {
            Ok(val) => val,
            Err(_) => {
                app.result = Some("Error: Invalid batch volume".to_string());
                return;
            }
        };

        if trial_vol <= Decimal::ZERO || batch_vol <= Decimal::ZERO {
            app.result = Some("Error: Volumes must be positive".to_string());
            return;
        }

        let scaled_addition = addition * (batch_vol / trial_vol);

        app.result = Some(format!("{:.2} g", scaled_addition));
        app.metadata.push(("Trial".to_string(), format!("{:.2} g in {} {}", addition, trial_vol, vol_unit)));
        app.metadata.push(("Full Batch".to_string(), format!("{:.2} g in {} {}", scaled_addition, batch_vol, vol_unit)));
    }
}

fn render_bottles_with_losses(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.label("Calculate final bottle count after racking losses");
    ui.add_space(10.0);

    let is_metric = app.state.unit_system == UnitSystem::Metric;
    let vol_unit = if is_metric { "L" } else { "gal" };

    crate::input_field(ui, &format!("Initial Volume ({}):", vol_unit), &mut app.waste_initial_volume, "Starting volume");

    ui.horizontal(|ui| {
        ui.label("Vessel:");
        egui::ComboBox::from_id_salt("vessel_type")
            .selected_text(match app.waste_vessel_type.as_str() {
                "carboy" => "Carboy (standard)",
                "bucket" => "Bucket (wider)",
                "keg" => "Keg (minimal)",
                _ => "Carboy",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.waste_vessel_type, "carboy".to_string(), "Carboy (standard)");
                ui.selectable_value(&mut app.waste_vessel_type, "bucket".to_string(), "Bucket (wider)");
                ui.selectable_value(&mut app.waste_vessel_type, "keg".to_string(), "Keg (minimal)");
            });
    });

    crate::input_field(ui, "Rackings:", &mut app.waste_num_rackings, "Number of times racked");

    ui.horizontal(|ui| {
        ui.label("Process:");
        egui::ComboBox::from_id_salt("process_type")
            .selected_text(match app.waste_process_type.as_str() {
                "standard" => "Standard (normal)",
                "careful" => "Careful (minimal loss)",
                "fruit" => "With Fruit (high loss)",
                _ => "Standard",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.waste_process_type, "standard".to_string(), "Standard (normal)");
                ui.selectable_value(&mut app.waste_process_type, "careful".to_string(), "Careful (minimal loss)");
                ui.selectable_value(&mut app.waste_process_type, "fruit".to_string(), "With Fruit (high loss)");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Final Bottles") {
        app.warnings.clear();
        app.metadata.clear();

        let initial_vol = match Decimal::from_str(&app.waste_initial_volume) {
            Ok(val) => val,
            Err(_) => {
                app.result = Some("Error: Invalid initial volume".to_string());
                return;
            }
        };

        let num_rackings = match app.waste_num_rackings.parse::<u32>() {
            Ok(val) => val,
            Err(_) => {
                app.result = Some("Error: Invalid number of rackings".to_string());
                return;
            }
        };

        if initial_vol <= Decimal::ZERO {
            app.result = Some("Error: Volume must be positive".to_string());
            return;
        }

        // Loss rates based on vessel and process - using safe Decimal::new instead of from_str
        let base_loss = match (app.waste_vessel_type.as_str(), app.waste_process_type.as_str()) {
            ("carboy", "careful") => Decimal::new(3, 2),    // 0.03
            ("carboy", "standard") => Decimal::new(5, 2),   // 0.05
            ("carboy", "fruit") => Decimal::new(10, 2),     // 0.10
            ("bucket", "careful") => Decimal::new(4, 2),    // 0.04
            ("bucket", "standard") => Decimal::new(6, 2),   // 0.06
            ("bucket", "fruit") => Decimal::new(12, 2),     // 0.12
            ("keg", "careful") => Decimal::new(1, 2),       // 0.01
            ("keg", "standard") => Decimal::new(2, 2),      // 0.02
            ("keg", "fruit") => Decimal::new(5, 2),         // 0.05
            _ => Decimal::new(5, 2),                        // 0.05 default
        };

        // Compound losses
        let mut remaining_volume = initial_vol;
        for _ in 0..num_rackings {
            remaining_volume *= (Decimal::ONE - base_loss);
        }

        // Convert to bottles (750ml = 0.75L)
        let liters_per_gallon = Decimal::new(3785411784, 9);  // 3.785411784
        let liters_per_bottle = Decimal::new(75, 2);          // 0.75

        let volume_liters = if is_metric {
            remaining_volume
        } else {
            remaining_volume * liters_per_gallon
        };

        let bottles = volume_liters / liters_per_bottle;
        let total_loss = (initial_vol - remaining_volume) / initial_vol * Decimal::from(100);

        app.result = Some(format!("{:.1} bottles", bottles));

        let vol_label = if is_metric {
            format!("{:.2} L", remaining_volume)
        } else {
            format!("{:.2} gal", remaining_volume)
        };

        app.metadata.push(("Final Volume".to_string(), vol_label));
        app.metadata.push(("Total Loss".to_string(), format!("{:.1}%", total_loss)));
        app.metadata.push(("Loss per Racking".to_string(), format!("{:.1}%", base_loss * Decimal::from(100))));
    }
}