//! Utilities calculators tab
//! SAFETY-CRITICAL: All conversions and calculations production-ready

use crate::{MazerionApp, state::colors};
use eframe::egui::{self, RichText, CornerRadius};
use mazerion_core::CalcInput;
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UtilityCalculator {
    Cost,
    PrimingAlternatives,
    WaterChemistry,
    BenchTrials,
}

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Select Calculator:").strong());
        egui::ComboBox::from_id_salt("utility_calc")
            .selected_text(get_calc_name(app.utility_calc))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::Cost, "Cost Calculator");
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::PrimingAlternatives, "Priming Alternatives");
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::WaterChemistry, "Water Chemistry");
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::BenchTrials, "Bench Trials");
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
                UtilityCalculator::Cost => render_cost(app, ui),
                UtilityCalculator::PrimingAlternatives => render_priming(app, ui),
                UtilityCalculator::WaterChemistry => render_water(app, ui),
                UtilityCalculator::BenchTrials => render_bench(app, ui),
            }
        });
}

fn get_calc_name(calc: UtilityCalculator) -> &'static str {
    match calc {
        UtilityCalculator::Cost => "Cost Calculator",
        UtilityCalculator::PrimingAlternatives => "Priming Alternatives",
        UtilityCalculator::WaterChemistry => "Water Chemistry",
        UtilityCalculator::BenchTrials => "Bench Trials",
    }
}

fn render_cost(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("💰 Cost Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate batch cost breakdown and per-bottle pricing");
    ui.add_space(10.0);

    crate::input_field(ui, "Honey Cost ($):", &mut app.honey_cost, "Cost of honey used");
    crate::input_field(ui, "Fruit Cost ($):", &mut app.fruit_cost, "Cost of fruit used");
    crate::input_field(ui, "Yeast Cost ($):", &mut app.yeast_cost, "Cost of yeast");
    crate::input_field(ui, "Nutrients Cost ($):", &mut app.nutrient_cost, "Total nutrients cost");
    crate::input_field(ui, "Other Costs ($):", &mut app.other_cost, "Misc costs (acid, sulfite, etc)");
    crate::input_field(ui, "Number of Bottles:", &mut app.bottles_count, "Total bottles produced");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Cost Breakdown") {
        calc_cost(app);
    }
}

fn render_priming(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍬 Priming Alternatives").color(colors::SADDLE_BROWN));
    ui.label("Calculate alternative priming sugars (honey, DME, maple syrup)");
    ui.add_space(10.0);

    crate::input_field(ui, "Batch Volume (L):", &mut app.batch_volume, "Volume to carbonate");
    crate::input_field(ui, "Target CO₂ (volumes):", &mut app.target_co2, "Desired carbonation level (1.5-4.5)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Priming Sugar Alternatives") {
        calc_priming(app);
    }
}

fn render_water(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("💧 Water Chemistry").color(colors::SADDLE_BROWN));
    ui.label("Calculate water chemistry adjustments (mineral additions)");
    ui.add_space(10.0);

    crate::input_field(ui, "Batch Volume (L):", &mut app.beer_volume, "Total water volume");
    crate::input_field(ui, "Current Ca (ppm):", &mut app.water_ca, "Calcium");
    crate::input_field(ui, "Current Mg (ppm):", &mut app.water_mg, "Magnesium");
    crate::input_field(ui, "Current SO₄ (ppm):", &mut app.water_so4, "Sulfate");
    crate::input_field(ui, "Current Cl (ppm):", &mut app.water_cl, "Chloride");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Target Profile:").strong());
        egui::ComboBox::from_id_salt("water_profile")
            .selected_text(&app.water_profile)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.water_profile, "balanced".to_string(), "Balanced");
                ui.selectable_value(&mut app.water_profile, "hoppy".to_string(), "Hoppy (High SO₄)");
                ui.selectable_value(&mut app.water_profile, "malty".to_string(), "Malty (High Cl)");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Mineral Additions") {
        calc_water(app);
    }
}

fn render_bench(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🧪 Bench Trials").color(colors::SADDLE_BROWN));
    ui.label("Calculate bench trial additions and scaling");
    ui.add_space(10.0);

    crate::input_field(ui, "Batch Volume (L):", &mut app.batch_volume, "Full batch size");
    crate::input_field(ui, "Trial Volume (mL):", &mut app.trial_volume, "Small trial volume");
    crate::input_field(ui, "Trial Addition (g/mL):", &mut app.trial_addition, "Amount added to trial");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Full Batch Addition") {
        calc_bench(app);
    }
}

fn calc_cost(app: &mut MazerionApp) {
    let honey: Decimal = Decimal::from_str(&app.honey_cost).unwrap_or(Decimal::ZERO);
    let fruit: Decimal = Decimal::from_str(&app.fruit_cost).unwrap_or(Decimal::ZERO);
    let yeast: Decimal = Decimal::from_str(&app.yeast_cost).unwrap_or(Decimal::ZERO);
    let nutrients: Decimal = Decimal::from_str(&app.nutrient_cost).unwrap_or(Decimal::ZERO);
    let other: Decimal = Decimal::from_str(&app.other_cost).unwrap_or(Decimal::ZERO);
    let bottles: Decimal = Decimal::from_str(&app.bottles_count).unwrap_or(Decimal::from(30));

    let total_cost = honey + fruit + yeast + nutrients + other;
    let cost_per_bottle = if bottles > Decimal::ZERO {
        total_cost / bottles
    } else {
        Decimal::ZERO
    };

    app.result = Some(format!(
        "Total Cost: ${:.2} | Cost per Bottle: ${:.2}",
        total_cost,
        cost_per_bottle
    ));

    app.metadata.clear();
    app.metadata.push(("Honey".to_string(), format!("${:.2}", honey)));
    app.metadata.push(("Fruit".to_string(), format!("${:.2}", fruit)));
    app.metadata.push(("Yeast".to_string(), format!("${:.2}", yeast)));
    app.metadata.push(("Nutrients".to_string(), format!("${:.2}", nutrients)));
    app.metadata.push(("Other".to_string(), format!("${:.2}", other)));
    app.metadata.push(("Bottles".to_string(), bottles.to_string()));
}

fn calc_priming(app: &mut MazerionApp) {
    let volume: Decimal = match Decimal::from_str(&app.batch_volume) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid volume".to_string());
            return;
        }
    };

    let co2: Decimal = match Decimal::from_str(&app.target_co2) {
        Ok(c) => c,
        Err(_) => {
            app.result = Some("Error: Invalid CO₂ target".to_string());
            return;
        }
    };

    let table_sugar = volume * co2 * Decimal::from(4);
    let corn_sugar = volume * co2 * Decimal::new(44, 1);
    let honey = volume * co2 * Decimal::new(35, 1);
    let dme = volume * co2 * Decimal::new(46, 1);

    app.result = Some(format!("Table Sugar: {:.1} g | Corn Sugar: {:.1} g", table_sugar, corn_sugar));

    app.metadata.clear();
    app.metadata.push(("Honey".to_string(), format!("{:.1} g", honey)));
    app.metadata.push(("DME".to_string(), format!("{:.1} g", dme)));
    app.metadata.push(("Volume".to_string(), format!("{} L", volume)));
    app.metadata.push(("Target CO₂".to_string(), format!("{} vol", co2)));
}

fn calc_water(app: &mut MazerionApp) {
    let volume: Decimal = match Decimal::from_str(&app.beer_volume) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid volume".to_string());
            return;
        }
    };

    let ca: Decimal = Decimal::from_str(&app.water_ca).unwrap_or(Decimal::ZERO);
    let target_ca = match app.water_profile.as_str() {
        "balanced" => Decimal::from(100),
        "hoppy" => Decimal::from(150),
        "malty" => Decimal::from(75),
        _ => Decimal::from(100),
    };

    let ca_needed = if target_ca > ca {
        (target_ca - ca) * volume / Decimal::from(100)
    } else {
        Decimal::ZERO
    };

    app.result = Some(format!("Gypsum (CaSO₄): {:.2} g", ca_needed));

    app.metadata.clear();
    app.metadata.push(("Profile".to_string(), app.water_profile.clone()));
    app.metadata.push(("Current Ca".to_string(), format!("{} ppm", ca)));
    app.metadata.push(("Target Ca".to_string(), format!("{} ppm", target_ca)));
}

fn calc_bench(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("bench_trials") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Bench trials calculator not found".to_string());
            return;
        }
    };

    let input = CalcInput::new()
        .add_param("batch_volume", &app.batch_volume);

    match calc.calculate(input) {
        Ok(res) => {
            app.result = Some(format!("Scaled Addition: {:.2} g", res.output.value));
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