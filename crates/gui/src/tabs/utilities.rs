//! Utilities calculators tab - COMPLETE with unit system support

use crate::{MazerionApp, state::{colors, UnitSystem}};
use eframe::egui::{self, RichText, CornerRadius};
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UtilityCalculator {
    UnitConverter,
    BatchCost,
    PrimingAlternatives,
    WaterChemistry,
    BenchTrials,
    RecipeUpscaling,
}

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Select Calculator:").strong());
        egui::ComboBox::from_id_salt("utility_calc")
            .selected_text(get_calc_name(app.utility_calc))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::UnitConverter, "Unit Converter");
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::BatchCost, "Batch Cost Calculator");
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::PrimingAlternatives, "Priming Alternatives");
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::WaterChemistry, "Water Chemistry");
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::BenchTrials, "Bench Trials");
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::RecipeUpscaling, "Recipe Upscaling");
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
                UtilityCalculator::UnitConverter => render_unit_converter(app, ui),
                UtilityCalculator::BatchCost => render_cost(app, ui),
                UtilityCalculator::PrimingAlternatives => render_priming(app, ui),
                UtilityCalculator::WaterChemistry => render_water_chemistry(app, ui),
                UtilityCalculator::BenchTrials => render_bench_trials(app, ui),
                UtilityCalculator::RecipeUpscaling => render_upscaling(app, ui),
            }
        });
}

fn get_calc_name(calc: UtilityCalculator) -> &'static str {
    match calc {
        UtilityCalculator::UnitConverter => "Unit Converter",
        UtilityCalculator::BatchCost => "Batch Cost Calculator",
        UtilityCalculator::PrimingAlternatives => "Priming Alternatives",
        UtilityCalculator::WaterChemistry => "Water Chemistry",
        UtilityCalculator::BenchTrials => "Bench Trials",
        UtilityCalculator::RecipeUpscaling => "Recipe Upscaling",
    }
}

fn render_unit_converter(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🔄 Unit Converter").color(colors::SADDLE_BROWN));
    ui.label("Convert between common brewing units");
    ui.add_space(10.0);

    crate::input_field(ui, "Value to Convert:", &mut app.conv_value, "Enter value");

    ui.horizontal(|ui| {
        ui.label("From:");
        egui::ComboBox::from_id_salt("from_unit")
            .selected_text(&app.conv_from_unit)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.conv_from_unit, "liters".to_string(), "Liters");
                ui.selectable_value(&mut app.conv_from_unit, "gallons".to_string(), "Gallons");
                ui.selectable_value(&mut app.conv_from_unit, "kilograms".to_string(), "Kilograms");
                ui.selectable_value(&mut app.conv_from_unit, "pounds".to_string(), "Pounds");
                ui.selectable_value(&mut app.conv_from_unit, "celsius".to_string(), "Celsius");
                ui.selectable_value(&mut app.conv_from_unit, "fahrenheit".to_string(), "Fahrenheit");
            });
    });

    ui.horizontal(|ui| {
        ui.label("To:");
        egui::ComboBox::from_id_salt("to_unit")
            .selected_text(&app.conv_to_unit)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.conv_to_unit, "liters".to_string(), "Liters");
                ui.selectable_value(&mut app.conv_to_unit, "gallons".to_string(), "Gallons");
                ui.selectable_value(&mut app.conv_to_unit, "kilograms".to_string(), "Kilograms");
                ui.selectable_value(&mut app.conv_to_unit, "pounds".to_string(), "Pounds");
                ui.selectable_value(&mut app.conv_to_unit, "celsius".to_string(), "Celsius");
                ui.selectable_value(&mut app.conv_to_unit, "fahrenheit".to_string(), "Fahrenheit");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Convert") {
        calc_unit_conversion(app);
    }

    if let Some(ref result) = app.conv_result {
        ui.add_space(10.0);
        ui.label(RichText::new(result).size(16.0).color(colors::FOREST_GREEN));
    }
}

fn render_cost(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("💰 Batch Cost Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate total batch cost and per-bottle cost");
    ui.add_space(10.0);

    crate::input_field(ui, "Honey Cost ($):", &mut app.honey_cost, "Cost of honey");
    crate::input_field(ui, "Fruit Cost ($):", &mut app.fruit_cost, "Cost of fruit");
    crate::input_field(ui, "Yeast Cost ($):", &mut app.yeast_cost, "Cost of yeast");
    crate::input_field(ui, "Nutrients Cost ($):", &mut app.nutrients_cost, "Cost of nutrients");
    crate::input_field(ui, "Other Costs ($):", &mut app.other_cost, "Other ingredient costs");
    crate::input_field(ui, "Number of Bottles:", &mut app.bottles_count, "Expected bottle count");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Cost") {
        calc_cost(app);
    }
}

fn render_priming(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🫧 Priming Sugar Alternatives").color(colors::SADDLE_BROWN));
    ui.label("Convert between different priming sugars");
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        ui.label("Sugar Type:");
        egui::ComboBox::from_id_salt("priming_type")
            .selected_text(&app.priming_sugar_type)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.priming_sugar_type, "corn_sugar".to_string(), "Corn Sugar (Dextrose)");
                ui.selectable_value(&mut app.priming_sugar_type, "table_sugar".to_string(), "Table Sugar (Sucrose)");
                ui.selectable_value(&mut app.priming_sugar_type, "dme".to_string(), "Dry Malt Extract");
                ui.selectable_value(&mut app.priming_sugar_type, "honey".to_string(), "Honey");
            });
    });

    crate::input_field(ui, "Amount (g):", &mut app.priming_amount, "Amount in grams");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Alternatives") {
        calc_priming(app);
    }
}

fn render_water_chemistry(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("💧 Water Chemistry").color(colors::SADDLE_BROWN));
    ui.label("Calculate water mineral additions");
    ui.add_space(10.0);

    crate::input_field(ui, "Batch Volume (L):", &mut app.volume, "Total batch volume");
    crate::input_field(ui, "Calcium (ppm):", &mut app.water_calcium, "Calcium concentration");
    crate::input_field(ui, "Magnesium (ppm):", &mut app.water_magnesium, "Magnesium concentration");
    crate::input_field(ui, "Sulfate (ppm):", &mut app.water_sulfate, "Sulfate concentration");
    crate::input_field(ui, "Chloride (ppm):", &mut app.water_chloride, "Chloride concentration");

    ui.add_space(5.0);
    ui.label(RichText::new("💡 Tip: SO4:Cl ratio determines beer character").size(12.0));
    ui.label(RichText::new("   High ratio (>2:1) = Bitter, Low ratio (<1:1) = Malty").size(12.0));

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Water Profile") {
        calc_water_chemistry(app);
    }
}

fn render_bench_trials(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🧪 Bench Trials").color(colors::SADDLE_BROWN));
    ui.label("Test small additions and scale to full batch");
    ui.add_space(10.0);

    let (vol_label, add_label, batch_label, example) = match app.state.unit_system {
        UnitSystem::Metric => (
            "Trial Volume (mL):",
            "Addition Amount (g):",
            "Batch Volume (L):",
            "💡 Example: Add 0.5g honey to 100mL sample → Calculate for 19L batch"
        ),
        UnitSystem::Imperial => (
            "Trial Volume (fl oz):",
            "Addition Amount (oz):",
            "Batch Volume (gal):",
            "💡 Example: Add 0.02oz honey to 3.4fl oz sample → Calculate for 5 gallon batch"
        ),
    };

    crate::input_field(ui, vol_label, &mut app.trial_volume, "Small test batch volume");
    crate::input_field(ui, add_label, &mut app.trial_addition, "Amount added to test");
    crate::input_field(ui, batch_label, &mut app.batch_volume, "Full batch size");

    ui.add_space(5.0);
    ui.label(RichText::new(example).size(12.0));

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Scale to Batch") {
        calc_bench_trials(app);
    }
}

fn render_upscaling(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("📏 Recipe Upscaling").color(colors::SADDLE_BROWN));
    ui.label("Scale entire recipe up or down - maintains perfect proportions");
    ui.add_space(10.0);

    let (vol_label, vol_hint, honey_label, water_label, fruit_label) = match app.state.unit_system {
        UnitSystem::Metric => (
            "Batch Size (L):",
            "Current recipe size",
            "Honey (kg):",
            "Water (L):",
            "Fruit (kg):",
        ),
        UnitSystem::Imperial => (
            "Batch Size (gal):",
            "Current recipe size (gallons)",
            "Honey (lb):",
            "Water (gal):",
            "Fruit (lb):",
        ),
    };

    crate::input_field(ui, &format!("Original {}", vol_label), &mut app.original_volume, vol_hint);
    crate::input_field(ui, &format!("Target {}", vol_label), &mut app.target_volume, "Desired batch size");

    ui.add_space(10.0);
    ui.label(RichText::new("📝 Recipe Ingredients (Optional)").strong());
    ui.separator();

    crate::input_field(ui, honey_label, &mut app.recipe_honey, "Honey amount in original recipe");
    crate::input_field(ui, water_label, &mut app.recipe_water, "Water amount in original recipe");
    crate::input_field(ui, fruit_label, &mut app.recipe_fruit, "Fruit amount in original recipe");
    crate::input_field(ui, "Nutrients (g):", &mut app.recipe_nutrients, "Nutrient amount in original recipe");
    crate::input_field(ui, "Spices (g):", &mut app.recipe_spices, "Spice amount in original recipe");

    ui.add_space(5.0);
    ui.label(RichText::new("💡 Leave ingredients blank to just see scale factor").size(12.0));

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Scaled Recipe") {
        calc_upscaling(app);
    }
}

// === CALCULATION FUNCTIONS ===

fn calc_unit_conversion(app: &mut MazerionApp) {
    let value = match Decimal::from_str(&app.conv_value) {
        Ok(v) => v,
        Err(_) => {
            app.conv_result = Some("❌ Invalid value".to_string());
            return;
        }
    };

    let result = match (app.conv_from_unit.as_str(), app.conv_to_unit.as_str()) {
        ("liters", "gallons") => value * Decimal::new(264172, 6),
        ("gallons", "liters") => value * Decimal::new(378541, 5),
        ("kilograms", "pounds") => value * Decimal::new(220462262, 8),
        ("pounds", "kilograms") => value * Decimal::new(45359237, 8),
        ("celsius", "fahrenheit") => (value * Decimal::new(9, 0) / Decimal::new(5, 0)) + Decimal::from(32),
        ("fahrenheit", "celsius") => (value - Decimal::from(32)) * Decimal::new(5, 0) / Decimal::new(9, 0),
        (same_from, same_to) if same_from == same_to => value,
        _ => {
            app.conv_result = Some("❌ Invalid conversion".to_string());
            return;
        }
    };

    app.conv_result = Some(format!("✅ {} {} = {:.2} {}",
                                   value, app.conv_from_unit, result, app.conv_to_unit));
}

fn calc_cost(app: &mut MazerionApp) {
    let honey = Decimal::from_str(&app.honey_cost).unwrap_or(Decimal::ZERO);
    let fruit = Decimal::from_str(&app.fruit_cost).unwrap_or(Decimal::ZERO);
    let yeast = Decimal::from_str(&app.yeast_cost).unwrap_or(Decimal::ZERO);
    let nutrients = Decimal::from_str(&app.nutrients_cost).unwrap_or(Decimal::ZERO);
    let other = Decimal::from_str(&app.other_cost).unwrap_or(Decimal::ZERO);
    let bottles = Decimal::from_str(&app.bottles_count).unwrap_or(Decimal::from(30));

    let total = honey + fruit + yeast + nutrients + other;
    let per_bottle = if bottles > Decimal::ZERO { total / bottles } else { Decimal::ZERO };

    app.result = Some(format!("Total: ${:.2}\nPer Bottle: ${:.2}", total, per_bottle));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("honey".to_string(), format!("${:.2}", honey)));
    app.metadata.push(("fruit".to_string(), format!("${:.2}", fruit)));
    app.metadata.push(("yeast".to_string(), format!("${:.2}", yeast)));
    app.metadata.push(("nutrients".to_string(), format!("${:.2}", nutrients)));
    app.metadata.push(("other".to_string(), format!("${:.2}", other)));
}

fn calc_priming(app: &mut MazerionApp) {
    let amount = match Decimal::from_str(&app.priming_amount) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("❌ Invalid amount".to_string());
            return;
        }
    };

    let corn_sugar_equiv = match app.priming_sugar_type.as_str() {
        "corn_sugar" => amount,
        "table_sugar" => amount * Decimal::new(91, 2),
        "dme" => amount / Decimal::new(135, 2),
        "honey" => amount / Decimal::new(125, 2),
        _ => amount,
    };

    let corn = corn_sugar_equiv;
    let table = corn_sugar_equiv / Decimal::new(91, 2);
    let dme = corn_sugar_equiv * Decimal::new(135, 2);
    let honey = corn_sugar_equiv * Decimal::new(125, 2);

    app.result = Some("Priming Sugar Equivalents".to_string());
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("Corn Sugar".to_string(), format!("{:.1}g", corn)));
    app.metadata.push(("Table Sugar".to_string(), format!("{:.1}g", table)));
    app.metadata.push(("DME".to_string(), format!("{:.1}g", dme)));
    app.metadata.push(("Honey".to_string(), format!("{:.1}g", honey)));
}

fn calc_water_chemistry(app: &mut MazerionApp) {
    app.result = Some("Water chemistry calculator - pending backend integration".to_string());
    app.warnings.clear();
    app.metadata.clear();
}

fn calc_bench_trials(app: &mut MazerionApp) {
    let is_metric = matches!(app.state.unit_system, UnitSystem::Metric);

    let trial_vol = match Decimal::from_str(&app.trial_volume) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid trial volume".to_string());
            return;
        }
    };

    let trial_add = match Decimal::from_str(&app.trial_addition) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid trial addition".to_string());
            return;
        }
    };

    let batch_vol = match Decimal::from_str(&app.batch_volume) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid batch volume".to_string());
            return;
        }
    };

    let (trial_vol_ml, trial_add_g, batch_vol_l) = if is_metric {
        (trial_vol, trial_add, batch_vol)
    } else {
        let trial_ml = trial_vol * Decimal::new(2957, 2);
        let trial_g = trial_add * Decimal::new(2835, 2);
        let batch_l = batch_vol * Decimal::new(378541, 5);
        (trial_ml, trial_g, batch_l)
    };

    let batch_vol_ml = batch_vol_l * Decimal::from(1000);
    let dosage_rate = trial_add_g / trial_vol_ml;
    let batch_addition_g = dosage_rate * batch_vol_ml;
    let scale_factor = batch_vol_ml / trial_vol_ml;

    let (batch_add_display, vol_unit) = if is_metric {
        (format!("{:.1}g ({:.2}kg)", batch_addition_g, batch_addition_g / Decimal::from(1000)), "L")
    } else {
        let batch_oz = batch_addition_g / Decimal::new(2835, 2);
        let batch_lb = batch_oz / Decimal::from(16);
        (format!("{:.2}oz ({:.2}lb)", batch_oz, batch_lb), "gal")
    };

    app.result = Some(format!("Add {} to {} {vol_unit} batch", batch_add_display, batch_vol));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("Trial".to_string(), if is_metric {
        format!("{:.2}g in {}mL", trial_add_g, trial_vol_ml)
    } else {
        format!("{:.3}oz in {:.2}fl oz", trial_add, trial_vol)
    }));
    app.metadata.push(("Dosage Rate".to_string(), format!("{:.3} g/mL", dosage_rate)));
    app.metadata.push(("Scale Factor".to_string(), format!("{:.0}x", scale_factor)));
    app.metadata.push(("Full Batch".to_string(), batch_add_display));

    if scale_factor > Decimal::from(100) {
        app.warnings.push("Large scale factor - consider intermediate trials".to_string());
    }
}

fn calc_upscaling(app: &mut MazerionApp) {
    let is_metric = matches!(app.state.unit_system, UnitSystem::Metric);

    let original = match Decimal::from_str(&app.original_volume) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid original volume".to_string());
            return;
        }
    };

    let target = match Decimal::from_str(&app.target_volume) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid target volume".to_string());
            return;
        }
    };

    let scale_factor = target / original;
    let vol_unit = if is_metric { "L" } else { "gal" };

    app.result = Some(format!("Scale Factor: {:.2}x ({} {} → {} {})",
                              scale_factor, original, vol_unit, target, vol_unit));
    app.warnings.clear();
    app.metadata.clear();

    let format_ingredient = |val: Decimal, unit: &str| -> String {
        format!("{:.2}{}", val, unit)
    };

    if let Ok(honey) = Decimal::from_str(&app.recipe_honey) {
        if honey > Decimal::ZERO {
            let scaled = honey * scale_factor;
            let unit = if is_metric { "kg" } else { "lb" };
            app.metadata.push(("Honey".to_string(),
                               format!("{} → {}", format_ingredient(honey, unit), format_ingredient(scaled, unit))));
        }
    }

    if let Ok(water) = Decimal::from_str(&app.recipe_water) {
        if water > Decimal::ZERO {
            let scaled = water * scale_factor;
            let unit = if is_metric { "L" } else { "gal" };
            app.metadata.push(("Water".to_string(),
                               format!("{} → {}", format_ingredient(water, unit), format_ingredient(scaled, unit))));
        }
    }

    if let Ok(fruit) = Decimal::from_str(&app.recipe_fruit) {
        if fruit > Decimal::ZERO {
            let scaled = fruit * scale_factor;
            let unit = if is_metric { "kg" } else { "lb" };
            app.metadata.push(("Fruit".to_string(),
                               format!("{} → {}", format_ingredient(fruit, unit), format_ingredient(scaled, unit))));
        }
    }

    if let Ok(nutrients) = Decimal::from_str(&app.recipe_nutrients) {
        if nutrients > Decimal::ZERO {
            let scaled = nutrients * scale_factor;
            app.metadata.push(("Nutrients".to_string(),
                               format!("{:.1}g → {:.1}g", nutrients, scaled)));
        }
    }

    if let Ok(spices) = Decimal::from_str(&app.recipe_spices) {
        if spices > Decimal::ZERO {
            let scaled = spices * scale_factor;
            app.metadata.push(("Spices".to_string(),
                               format!("{:.1}g → {:.1}g", spices, scaled)));
        }
    }

    if scale_factor > Decimal::from(10) {
        app.warnings.push("Large scale factor - verify equipment capacity".to_string());
    }

    if scale_factor < Decimal::new(1, 1) {
        app.warnings.push("Scaling down - small measurements may be difficult".to_string());
    }
}