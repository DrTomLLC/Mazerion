//! Utility calculators tab

use crate::{MazerionApp, state::colors};
use eframe::egui::{self, RichText, CornerRadius};
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UtilityCalculator {
    Cost,
    WaterChemistry,
}

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Select Calculator:").strong());
        egui::ComboBox::from_id_salt("utility_calc")
            .selected_text(get_calc_name(app.utility_calc))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::Cost, "Cost Calculator");
                ui.selectable_value(&mut app.utility_calc, UtilityCalculator::WaterChemistry, "Water Chemistry");
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
                UtilityCalculator::WaterChemistry => render_water(app, ui),
            }
        });
}

fn get_calc_name(calc: UtilityCalculator) -> &'static str {
    match calc {
        UtilityCalculator::Cost => "Cost Calculator",
        UtilityCalculator::WaterChemistry => "Water Chemistry",
    }
}

fn render_cost(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("💰 Batch Cost Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate total batch cost and per-bottle pricing");
    ui.add_space(10.0);

    let volume_unit = if app.state.unit_system == crate::state::UnitSystem::Metric { "L" } else { "gal" };

    crate::input_field(ui, "Honey Cost ($):", &mut app.honey_cost, "Cost of honey");
    crate::input_field(ui, "Yeast Cost ($):", &mut app.yeast_cost, "Cost of yeast");
    crate::input_field(ui, "Nutrient Cost ($):", &mut app.nutrient_cost, "Cost of nutrients");
    crate::input_field(ui, "Other Costs ($):", &mut app.other_cost, "Other ingredients/supplies");
    crate::input_field(ui, &format!("Batch Volume ({}):", volume_unit), &mut app.batch_volume, "Final batch size");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Costs") {
        calc_cost(app);
    }
}

fn render_water(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("💧 Water Chemistry").color(colors::SADDLE_BROWN));
    ui.label("Calculate mineral additions for water adjustment");
    ui.add_space(10.0);

    let volume_unit = if app.state.unit_system == crate::state::UnitSystem::Metric { "L" } else { "gal" };

    crate::input_field(ui, &format!("Water Volume ({}):", volume_unit), &mut app.water_volume, "Amount of water to adjust");
    crate::input_field(ui, "Target Calcium (ppm):", &mut app.target_calcium, "Desired calcium level (50-150 typical)");
    crate::input_field(ui, "Current Calcium (ppm):", &mut app.current_calcium, "Current calcium level (0 if unknown)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Additions") {
        calc_water(app);
    }
}

fn calc_cost(app: &mut MazerionApp) {
    let honey: Decimal = Decimal::from_str(&app.honey_cost).unwrap_or(Decimal::ZERO);
    let yeast: Decimal = Decimal::from_str(&app.yeast_cost).unwrap_or(Decimal::ZERO);
    let nutrient: Decimal = Decimal::from_str(&app.nutrient_cost).unwrap_or(Decimal::ZERO);
    let other: Decimal = Decimal::from_str(&app.other_cost).unwrap_or(Decimal::ZERO);
    let volume: Decimal = Decimal::from_str(&app.batch_volume).unwrap_or(Decimal::from(19));

    let total_cost = honey + yeast + nutrient + other;

    let volume_liters = if app.state.unit_system == crate::state::UnitSystem::Metric {
        volume
    } else {
        mazerion_core::gallons_to_liters(volume)
    };

    let bottles = volume_liters / Decimal::new(75, 2);
    let cost_per_bottle = if bottles > Decimal::ZERO {
        total_cost / bottles
    } else {
        Decimal::ZERO
    };

    let cost_per_liter = if volume_liters > Decimal::ZERO {
        total_cost / volume_liters
    } else {
        Decimal::ZERO
    };

    let (display_volume, volume_unit) = if app.state.unit_system == crate::state::UnitSystem::Metric {
        (volume_liters, "L")
    } else {
        (volume, "gal")
    };

    app.result = Some(format!("Total Cost: ${:.2}", total_cost));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("Total Cost".to_string(), format!("${:.2}", total_cost)));
    app.metadata.push(("Cost per Bottle".to_string(), format!("${:.2}", cost_per_bottle)));
    app.metadata.push(("Cost per Liter".to_string(), format!("${:.2}", cost_per_liter)));
    app.metadata.push(("Bottles (750mL)".to_string(), format!("{:.0}", bottles)));
    app.metadata.push(("Batch Volume".to_string(), format!("{:.1} {}", display_volume, volume_unit)));
}

fn calc_water(app: &mut MazerionApp) {
    let volume: Decimal = match Decimal::from_str(&app.water_volume) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid water volume".to_string());
            return;
        }
    };

    let target: Decimal = Decimal::from_str(&app.target_calcium).unwrap_or(Decimal::from(100));
    let current: Decimal = Decimal::from_str(&app.current_calcium).unwrap_or(Decimal::ZERO);

    let calcium_needed = target - current;

    if calcium_needed <= Decimal::ZERO {
        app.result = Some("No calcium addition needed".to_string());
        app.warnings.clear();
        app.metadata.clear();
        return;
    }

    let volume_liters = if app.state.unit_system == crate::state::UnitSystem::Metric {
        volume
    } else {
        mazerion_core::gallons_to_liters(volume)
    };

    let gypsum_per_l = calcium_needed / Decimal::new(1624, 2);
    let total_gypsum = gypsum_per_l * volume_liters;

    let (display_volume, volume_unit) = if app.state.unit_system == crate::state::UnitSystem::Metric {
        (volume_liters, "L")
    } else {
        (volume, "gal")
    };

    app.result = Some(format!("Gypsum: {:.2} g", total_gypsum));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("Gypsum (CaSO4)".to_string(), format!("{:.2} g", total_gypsum)));
    app.metadata.push(("Calcium Increase".to_string(), format!("{:.0} ppm", calcium_needed)));
    app.metadata.push(("Water Volume".to_string(), format!("{:.1} {}", display_volume, volume_unit)));
}