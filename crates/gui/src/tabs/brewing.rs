//! Brewing calculators tab

use crate::{MazerionApp, state::{BrewingCalculator, colors}};
use eframe::egui::{self, RichText, CornerRadius};
use mazerion_core::CalcInput;

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Select Calculator:").strong());
        egui::ComboBox::from_id_salt("brewing_calc")
            .selected_text(get_calc_name(app.state.brewing_calc))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.state.brewing_calc, BrewingCalculator::Nutrition, "TOSNA Nutrition Calculator");
                ui.selectable_value(&mut app.state.brewing_calc, BrewingCalculator::Carbonation, "Carbonation Calculator");
            });
    });

    ui.add_space(10.0);

    egui::Frame::new()
        .fill(colors::LIGHT_CREAM)
        .stroke(egui::Stroke::new(1.5, colors::HONEY_GOLD))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            match app.state.brewing_calc {
                BrewingCalculator::Nutrition => render_nutrition(app, ui),
                BrewingCalculator::Carbonation => render_carbonation(app, ui),
            }
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
    ui.label("Calculate Fermaid-O schedule using TOSNA 2.0 protocol");
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };

    crate::input_field(ui, &format!("Volume ({}):", vol_unit), &mut app.volume, "Total must volume");
    crate::input_field(ui, "Target ABV (%):", &mut app.target_abv_brew, "Expected final ABV");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Yeast Nitrogen Needs:").strong());
        egui::ComboBox::from_id_salt("yn_req")
            .selected_text(&app.yn_requirement)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.yn_requirement, "low".to_string(), "Low (DV10, QA23)");
                ui.selectable_value(&mut app.yn_requirement, "medium".to_string(), "Medium (most yeasts)");
                ui.selectable_value(&mut app.yn_requirement, "high".to_string(), "High (EC-1118, K1-V1116)");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate TOSNA Schedule") {
        calc_nutrition(app);
    }
}

fn render_carbonation(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🫧 Carbonation Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate priming sugar or keg PSI for target carbonation");
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };
    let temp_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "°C" } else { "°F" };

    crate::input_field(ui, &format!("Volume ({}):", vol_unit), &mut app.volume, "Total volume to carbonate");
    crate::input_field(ui, &format!("Temperature ({}):", temp_unit), &mut app.carb_temp, "Current temperature");
    crate::input_field(ui, "Target CO₂ (volumes):", &mut app.target_co2, "Desired carbonation level (1.5-4.5)");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Method:").strong());
        egui::ComboBox::from_id_salt("carb_method")
            .selected_text(&app.carb_method)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.carb_method, "priming".to_string(), "Bottle Priming");
                ui.selectable_value(&mut app.carb_method, "keg".to_string(), "Force Carbonation (Keg)");
            });
    });

    if app.carb_method == "priming" {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Sugar Type:").strong());
            egui::ComboBox::from_id_salt("sugar_type")
                .selected_text(&app.sugar_type)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.sugar_type, "table_sugar".to_string(), "Table Sugar (Sucrose)");
                    ui.selectable_value(&mut app.sugar_type, "corn_sugar".to_string(), "Corn Sugar (Dextrose)");
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

fn calc_nutrition(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("nutrition") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Nutrition calculator not found".to_string());
            return;
        }
    };

    let input = CalcInput::new()
        .add_param("volume", &app.volume)
        .add_param("target_abv", &app.target_abv_brew)
        .add_param("yn_requirement", &app.yn_requirement);

    match calc.calculate(input) {
        Ok(res) => {
            app.result = Some(format!("Total Fermaid-O: {:.2} g", res.output.value));
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

fn calc_carbonation(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("carbonation") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Carbonation calculator not found".to_string());
            return;
        }
    };

    let input = CalcInput::new()
        .add_param("volume", &app.volume)
        .add_param("temperature", &app.carb_temp)
        .add_param("target_co2", &app.target_co2)
        .add_param("method", &app.carb_method)
        .add_param("sugar_type", &app.sugar_type);

    match calc.calculate(input) {
        Ok(res) => {
            if app.carb_method == "priming" {
                app.result = Some(format!("Priming Sugar: {:.1} g", res.output.value));
            } else {
                app.result = Some(format!("Target PSI: {:.1}", res.output.value));
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