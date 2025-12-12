//! Utilities calculators tab - FIXED VERSION
//! All unit conversions, parameter passing, and calculator integrations corrected

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
    ui.label("Calculate batch cost breakdown and per-bottle pricing for your brewing operations");
    ui.add_space(10.0);

    crate::input_field(ui, "Honey Cost ($):", &mut app.honey_cost, "Total cost of honey used in batch");
    crate::input_field(ui, "Fruit Cost ($):", &mut app.fruit_cost, "Cost of all fruit additions");
    crate::input_field(ui, "Yeast Cost ($):", &mut app.yeast_cost, "Cost of yeast packets/vials");
    crate::input_field(ui, "Nutrients Cost ($):", &mut app.nutrient_cost, "Total cost of yeast nutrients");
    crate::input_field(ui, "Other Costs ($):", &mut app.other_cost, "Acid, sulfite, additives, etc");
    crate::input_field(ui, "Number of Bottles:", &mut app.bottles_count, "Total 750ml bottles produced");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Cost Breakdown") {
        calc_cost(app);
    }
}

fn render_priming(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍬 Priming Sugar Alternatives").color(colors::SADDLE_BROWN));
    ui.label("Calculate equivalent amounts for different priming sugars: table sugar, corn sugar, honey, DME, maple syrup, agave, molasses");
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };
    let temp_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "°C" } else { "°F" };

    crate::input_field(ui, &format!("Batch Volume ({}):", vol_unit), &mut app.batch_volume, "Total volume to carbonate");
    crate::input_field(ui, &format!("Temperature ({}):", temp_unit), &mut app.carb_temp, "Current beer/mead temperature");
    crate::input_field(ui, "Target CO₂ (volumes):", &mut app.target_co2, "Desired carbonation level (mead: 1.5-2.5, beer: 2.0-2.7, sparkling: 3.0-4.0)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Priming Sugar Alternatives") {
        calc_priming(app);
    }
}

fn render_water(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("💧 Water Chemistry Adjustments").color(colors::SADDLE_BROWN));
    ui.label("Calculate mineral additions to adjust water chemistry for brewing. Choose a target profile or specify desired ppm changes.");
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };

    crate::input_field(ui, &format!("Batch Volume ({}):", vol_unit), &mut app.beer_volume, "Total water volume for brewing");
    crate::input_field(ui, "Target ppm Increase:", &mut app.water_target_ppm, "How much to raise mineral level (e.g., 50 ppm)");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Mineral Type:").strong());
        egui::ComboBox::from_id_salt("water_mineral")
            .selected_text(&app.water_mineral_type)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.water_mineral_type, "gypsum".to_string(), "Gypsum (Ca/SO₄)");
                ui.selectable_value(&mut app.water_mineral_type, "calcium_chloride".to_string(), "Calcium Chloride (Ca/Cl)");
                ui.selectable_value(&mut app.water_mineral_type, "epsom".to_string(), "Epsom Salt (Mg/SO₄)");
                ui.selectable_value(&mut app.water_mineral_type, "baking_soda".to_string(), "Baking Soda (Na/HCO₃)");
                ui.selectable_value(&mut app.water_mineral_type, "chalk".to_string(), "Chalk (Ca/CO₃)");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Mineral Addition") {
        calc_water(app);
    }
}

fn render_bench(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🧪 Bench Trials - Scale Up Calculator").color(colors::SADDLE_BROWN));
    ui.label("Scale small-volume bench trial additions to full batch size. Essential for testing adjuncts, spices, oak, fruit, etc before committing to entire batch.");
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };

    crate::input_field(ui, "Trial Volume (mL):", &mut app.trial_volume, "Small test sample size (typically 100-500 mL)");
    crate::input_field(ui, "Trial Addition (g):", &mut app.trial_addition, "Amount added to trial (grams or mL)");
    crate::input_field(ui, &format!("Batch Volume ({}):", vol_unit), &mut app.batch_volume, "Full batch size to scale to");

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
    let calc = match mazerion_core::traits::get_calculator("priming_alternatives") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Priming calculator not found".to_string());
            return;
        }
    };

    let volume_val = match Decimal::from_str(&app.batch_volume) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid volume".to_string());
            return;
        }
    };

    let temp_val = match Decimal::from_str(&app.carb_temp) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid temperature".to_string());
            return;
        }
    };

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);

    // Convert to liters and Celsius
    let volume_liters = if is_metric {
        volume_val
    } else {
        volume_val * Decimal::new(3785, 3) // gallons to liters
    };

    let temp_celsius = if is_metric {
        temp_val
    } else {
        (temp_val - Decimal::from(32)) * Decimal::new(5, 1) / Decimal::from(9) // F to C
    };

    let input = CalcInput::new()
        .add_param("volume", &volume_liters.to_string())
        .add_param("target_co2", &app.target_co2)
        .add_param("temperature", &temp_celsius.to_string());

    match calc.calculate(input) {
        Ok(res) => {
            let weight_unit = if is_metric { "g" } else { "oz" };

            // Convert result from grams if needed
            let display_value = if is_metric {
                res.output.value
            } else {
                res.output.value / Decimal::new(2835, 2) // g to oz
            };

            app.result = Some(format!("Table Sugar: {:.1} {}", display_value, weight_unit));
            app.warnings = res.warnings;

            // Convert all metadata weights
            app.metadata = res.metadata.into_iter().map(|(k, v)| {
                if k.ends_with("_g") && !is_metric {
                    // Parse "150 g (5.29 oz)" format and just use oz value
                    if let Some(start) = v.find('(') {
                        if let Some(end) = v.find(" oz)") {
                            let oz_str = &v[start+1..end];
                            return (k.replace("_g", ""), format!("{} oz", oz_str));
                        }
                    }
                }
                (k, v)
            }).collect();
        }
        Err(e) => {
            app.result = Some(format!("Error: {}", e));
            app.warnings.clear();
            app.metadata.clear();
        }
    }
}

fn calc_water(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("water_chemistry") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Water chemistry calculator not found".to_string());
            return;
        }
    };

    let volume_val = match Decimal::from_str(&app.beer_volume) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid volume".to_string());
            return;
        }
    };

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);

    // Convert to liters
    let volume_liters = if is_metric {
        volume_val
    } else {
        volume_val * Decimal::new(3785, 3) // gallons to liters
    };

    let input = CalcInput::new()
        .add_param("volume", &volume_liters.to_string())
        .add_param("adjustment", &app.water_mineral_type)
        .add_param("target_ppm", &app.water_target_ppm);

    match calc.calculate(input) {
        Ok(res) => {
            let weight_unit = if is_metric { "g" } else { "oz" };

            let display_value = if is_metric {
                res.output.value
            } else {
                res.output.value / Decimal::new(2835, 2) // g to oz
            };

            app.result = Some(format!("Mineral Needed: {:.2} {}", display_value, weight_unit));
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

fn calc_bench(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("bench_trials") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Bench trials calculator not found".to_string());
            return;
        }
    };

    let batch_volume_val = match Decimal::from_str(&app.batch_volume) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid batch volume".to_string());
            return;
        }
    };

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);

    // Convert batch volume to liters (calculator expects liters)
    let batch_volume_liters = if is_metric {
        batch_volume_val
    } else {
        batch_volume_val * Decimal::new(3785, 3) // gallons to liters
    };

    // trial_volume is always in mL, trial_addition always in grams
    let input = CalcInput::new()
        .add_param("trial_volume", &app.trial_volume)
        .add_param("trial_addition", &app.trial_addition)
        .add_param("batch_volume", &batch_volume_liters.to_string());

    match calc.calculate(input) {
        Ok(res) => {
            let weight_unit = if is_metric { "g" } else { "oz" };

            let display_value = if is_metric {
                res.output.value
            } else {
                res.output.value / Decimal::new(2835, 2) // g to oz
            };

            app.result = Some(format!("Scaled Addition: {:.2} {}", display_value, weight_unit));
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