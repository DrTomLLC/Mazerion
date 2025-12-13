//! Utilities calculators tab

use crate::{MazerionApp, state::colors};
use eframe::egui::{self, RichText, CornerRadius};
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UtilityCalculator {
    BatchCost,
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
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::BatchCost, "Batch Cost Calculator");
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
                UtilityCalculator::BatchCost => render_cost(app, ui),
                UtilityCalculator::PrimingAlternatives => render_priming(app, ui),
                UtilityCalculator::WaterChemistry => render_water_chemistry(app, ui),
                UtilityCalculator::BenchTrials => render_bench_trials(app, ui),
            }
        });
}

fn get_calc_name(calc: UtilityCalculator) -> &'static str {
    match calc {
        UtilityCalculator::BatchCost => "Batch Cost Calculator",
        UtilityCalculator::PrimingAlternatives => "Priming Alternatives",
        UtilityCalculator::WaterChemistry => "Water Chemistry",
        UtilityCalculator::BenchTrials => "Bench Trials",
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
    ui.heading(RichText::new("🍾 Priming Sugar Alternatives").color(colors::SADDLE_BROWN));
    ui.label("Calculate equivalent amounts for different priming sugars");
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        ui.label("Sugar Type:");
        egui::ComboBox::from_id_salt("priming_sugar_type")
            .selected_text(&app.priming_sugar_type)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.priming_sugar_type, "corn_sugar".to_string(), "Corn Sugar (Dextrose)");
                ui.selectable_value(&mut app.priming_sugar_type, "table_sugar".to_string(), "Table Sugar (Sucrose)");
                ui.selectable_value(&mut app.priming_sugar_type, "dme".to_string(), "Dry Malt Extract");
                ui.selectable_value(&mut app.priming_sugar_type, "honey".to_string(), "Honey");
            });
    });

    crate::input_field(ui, "Amount (grams):", &mut app.priming_amount, "Amount of sugar");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Alternatives") {
        calc_priming(app);
    }
}

fn render_water_chemistry(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("💧 Water Chemistry Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate SO4:Cl ratio and water profile");
    ui.add_space(10.0);

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

fn render_bench_trials(_app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🧪 Bench Trials").color(colors::SADDLE_BROWN));
    ui.label("Scale recipes and test additions (Coming Soon)");
}

fn calc_cost(app: &mut MazerionApp) {
    let honey = Decimal::from_str(&app.honey_cost).unwrap_or(Decimal::ZERO);
    let fruit = Decimal::from_str(&app.fruit_cost).unwrap_or(Decimal::ZERO);
    let yeast = Decimal::from_str(&app.yeast_cost).unwrap_or(Decimal::ZERO);
    let nutrients = Decimal::from_str(&app.nutrients_cost).unwrap_or(Decimal::ZERO);
    let other = Decimal::from_str(&app.other_cost).unwrap_or(Decimal::ZERO);
    let bottles = Decimal::from_str(&app.bottles_count).unwrap_or(Decimal::from(30));

    let total = honey + fruit + yeast + nutrients + other;
    let per_bottle = if bottles > Decimal::ZERO {
        total / bottles
    } else {
        Decimal::ZERO
    };

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

    app.result = Some(format!(
        "Corn Sugar: {:.1}g\nTable Sugar: {:.1}g\nDME: {:.1}g\nHoney: {:.1}g",
        corn, table, dme, honey
    ));
    app.warnings.clear();
    app.metadata.clear();
}

fn calc_water_chemistry(app: &mut MazerionApp) {
    let sulfate = Decimal::from_str(&app.water_sulfate).unwrap_or(Decimal::ZERO);
    let chloride = Decimal::from_str(&app.water_chloride).unwrap_or(Decimal::ZERO);

    let ratio = if chloride > Decimal::ZERO {
        sulfate / chloride
    } else {
        Decimal::ZERO
    };

    let profile = if ratio > Decimal::from(2) {
        "Highly Bitter (IPA, Pale Ale)"
    } else if ratio > Decimal::ONE {
        "Moderately Bitter (Amber, ESB)"
    } else if ratio > Decimal::new(5, 1) {
        "Balanced (Lager, Pilsner)"
    } else {
        "Malty/Sweet (Stout, Porter)"
    };

    app.result = Some(format!("SO4:Cl Ratio: {:.2}:1\nProfile: {}", ratio, profile));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("sulfate".to_string(), format!("{}ppm", sulfate)));
    app.metadata.push(("chloride".to_string(), format!("{}ppm", chloride)));
}