//! Mead style-specific calculators tab

use crate::{MazerionApp, state::colors};
use eframe::egui::{self, RichText, CornerRadius};
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeadStyle {
    Traditional,
    Hydromel,
    Sack,
    Melomel,
    Cyser,
}

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Select Style:").strong());
        egui::ComboBox::from_id_salt("mead_style")
            .selected_text(get_style_name(app.mead_style))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.mead_style, MeadStyle::Traditional, "Traditional Mead");
                ui.selectable_value(&mut app.mead_style, MeadStyle::Hydromel, "Hydromel (Session)");
                ui.selectable_value(&mut app.mead_style, MeadStyle::Sack, "Sack Mead (High Gravity)");
                ui.selectable_value(&mut app.mead_style, MeadStyle::Melomel, "Melomel (Fruit)");
                ui.selectable_value(&mut app.mead_style, MeadStyle::Cyser, "Cyser (Apple)");
            });
    });

    ui.add_space(10.0);

    egui::Frame::new()
        .fill(colors::LIGHT_CREAM)
        .stroke(egui::Stroke::new(1.5, colors::HONEY_GOLD))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            match app.mead_style {
                MeadStyle::Traditional => render_traditional(app, ui),
                MeadStyle::Hydromel => render_hydromel(app, ui),
                MeadStyle::Sack => render_sack(app, ui),
                MeadStyle::Melomel => render_melomel(app, ui),
                MeadStyle::Cyser => render_cyser(app, ui),
            }
        });
}

fn get_style_name(style: MeadStyle) -> &'static str {
    match style {
        MeadStyle::Traditional => "Traditional Mead",
        MeadStyle::Hydromel => "Hydromel (Session)",
        MeadStyle::Sack => "Sack Mead (High Gravity)",
        MeadStyle::Melomel => "Melomel (Fruit)",
        MeadStyle::Cyser => "Cyser (Apple)",
    }
}

fn render_traditional(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍯 Traditional Mead Recipe").color(colors::SADDLE_BROWN));
    ui.label("Pure honey and water - the foundation of all mead");
    ui.add_space(10.0);

    let volume_unit = if app.state.unit_system == crate::state::UnitSystem::Metric { "L" } else { "gal" };

    crate::input_field(ui, &format!("Batch Volume ({}):", volume_unit), &mut app.batch_volume, "Final batch size");
    crate::input_field(ui, "Target ABV (%):", &mut app.target_abv_mead, "Desired alcohol percentage (10-14% typical)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Recipe") {
        calc_traditional(app);
    }
}

fn render_hydromel(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("💧 Hydromel Recipe").color(colors::SADDLE_BROWN));
    ui.label("Session mead - lower alcohol for easier drinking");
    ui.add_space(10.0);

    let volume_unit = if app.state.unit_system == crate::state::UnitSystem::Metric { "L" } else { "gal" };

    crate::input_field(ui, &format!("Batch Volume ({}):", volume_unit), &mut app.batch_volume, "Final batch size");
    crate::input_field(ui, "Target ABV (%):", &mut app.target_abv_mead, "Desired ABV (5-8% typical for hydromel)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Recipe") {
        calc_hydromel(app);
    }
}

fn render_sack(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🏺 Sack Mead Recipe").color(colors::SADDLE_BROWN));
    ui.label("High gravity mead - rich and full-bodied");
    ui.add_space(10.0);

    let volume_unit = if app.state.unit_system == crate::state::UnitSystem::Metric { "L" } else { "gal" };

    crate::input_field(ui, &format!("Batch Volume ({}):", volume_unit), &mut app.batch_volume, "Final batch size");
    crate::input_field(ui, "Target ABV (%):", &mut app.target_abv_mead, "Desired ABV (14-18% typical for sack)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Recipe") {
        calc_sack(app);
    }
}

fn render_melomel(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍓 Melomel Recipe").color(colors::SADDLE_BROWN));
    ui.label("Fruit mead - honey and fruit combination");
    ui.add_space(10.0);

    let volume_unit = if app.state.unit_system == crate::state::UnitSystem::Metric { "L" } else { "gal" };
    let ratio_unit = if app.state.unit_system == crate::state::UnitSystem::Metric { "kg/L" } else { "lb/gal" };

    crate::input_field(ui, &format!("Batch Volume ({}):", volume_unit), &mut app.batch_volume, "Final batch size");
    crate::input_field(ui, "Target ABV (%):", &mut app.target_abv_mead, "Desired alcohol percentage");
    crate::input_field(ui, &format!("Fruit Ratio ({}):", ratio_unit), &mut app.fruit_ratio, "Fruit amount per unit volume (e.g., 0.2)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Recipe") {
        calc_melomel(app);
    }
}

fn render_cyser(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍎 Cyser Recipe").color(colors::SADDLE_BROWN));
    ui.label("Apple mead - honey and apple juice/cider");
    ui.add_space(10.0);

    let volume_unit = if app.state.unit_system == crate::state::UnitSystem::Metric { "L" } else { "gal" };

    crate::input_field(ui, &format!("Batch Volume ({}):", volume_unit), &mut app.batch_volume, "Final batch size");
    crate::input_field(ui, "Target ABV (%):", &mut app.target_abv_mead, "Desired alcohol percentage");
    crate::input_field(ui, "Apple Juice %:", &mut app.juice_percent, "Percentage of total volume (e.g., 50)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Recipe") {
        calc_cyser(app);
    }
}

fn calc_traditional(app: &mut MazerionApp) {
    calc_honey_recipe(app, "Traditional Mead", Decimal::ONE);
}

fn calc_hydromel(app: &mut MazerionApp) {
    calc_honey_recipe(app, "Hydromel", Decimal::ONE);
}

fn calc_sack(app: &mut MazerionApp) {
    calc_honey_recipe(app, "Sack Mead", Decimal::ONE);
}

fn calc_melomel(app: &mut MazerionApp) {
    let fruit_ratio: Decimal = match Decimal::from_str(&app.fruit_ratio) {
        Ok(v) => v,
        Err(_) => Decimal::new(2, 1),
    };

    calc_honey_recipe(app, "Melomel", Decimal::ONE);

    let volume: Decimal = Decimal::from_str(&app.batch_volume).unwrap_or(Decimal::from(19));

    let volume_liters = if app.state.unit_system == crate::state::UnitSystem::Metric {
        volume
    } else {
        mazerion_core::gallons_to_liters(volume)
    };

    let fruit_kg = volume_liters * fruit_ratio;

    let (display_weight, weight_unit) = if app.state.unit_system == crate::state::UnitSystem::Metric {
        (fruit_kg, "kg")
    } else {
        (mazerion_core::kilograms_to_pounds(fruit_kg), "lb")
    };

    app.metadata.push(("Fruit Needed".to_string(), format!("{:.2} {}", display_weight, weight_unit)));
}

fn calc_cyser(app: &mut MazerionApp) {
    let juice_pct: Decimal = match Decimal::from_str(&app.juice_percent) {
        Ok(v) => v / Decimal::from(100),
        Err(_) => Decimal::new(5, 1),
    };

    calc_honey_recipe(app, "Cyser", Decimal::ONE - juice_pct);

    let volume: Decimal = Decimal::from_str(&app.batch_volume).unwrap_or(Decimal::from(19));

    let volume_liters = if app.state.unit_system == crate::state::UnitSystem::Metric {
        volume
    } else {
        mazerion_core::gallons_to_liters(volume)
    };

    let juice_volume = volume_liters * juice_pct;

    let (display_volume, volume_unit) = if app.state.unit_system == crate::state::UnitSystem::Metric {
        (juice_volume, "L")
    } else {
        (mazerion_core::liters_to_gallons(juice_volume), "gal")
    };

    app.metadata.push(("Apple Juice".to_string(), format!("{:.2} {}", display_volume, volume_unit)));
}

fn calc_honey_recipe(app: &mut MazerionApp, style: &str, honey_factor: Decimal) {
    let volume: Decimal = match Decimal::from_str(&app.batch_volume) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid batch volume".to_string());
            return;
        }
    };

    let abv: Decimal = match Decimal::from_str(&app.target_abv_mead) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid target ABV".to_string());
            return;
        }
    };

    let volume_liters = if app.state.unit_system == crate::state::UnitSystem::Metric {
        volume
    } else {
        mazerion_core::gallons_to_liters(volume)
    };

    let honey_per_l_per_abv = Decimal::from(135);
    let total_honey = volume_liters * abv * honey_per_l_per_abv * honey_factor;
    let honey_kg = total_honey / Decimal::from(1000);

    let water_volume = volume_liters - (total_honey / Decimal::from(1420));

    let (display_honey, honey_unit) = if app.state.unit_system == crate::state::UnitSystem::Metric {
        (honey_kg, "kg")
    } else {
        (mazerion_core::kilograms_to_pounds(honey_kg), "lb")
    };

    let (display_water, water_unit) = if app.state.unit_system == crate::state::UnitSystem::Metric {
        (water_volume, "L")
    } else {
        (mazerion_core::liters_to_gallons(water_volume), "gal")
    };

    let (display_final_vol, final_vol_unit) = if app.state.unit_system == crate::state::UnitSystem::Metric {
        (volume_liters, "L")
    } else {
        (volume, "gal")
    };

    app.result = Some(format!("{}: {:.2} {} honey", style, display_honey, honey_unit));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("Honey".to_string(), format!("{:.2} {}", display_honey, honey_unit)));
    app.metadata.push(("Water".to_string(), format!("{:.2} {}", display_water, water_unit)));
    app.metadata.push(("Target ABV".to_string(), format!("{}%", abv)));
    app.metadata.push(("Final Volume".to_string(), format!("{:.1} {}", display_final_vol, final_vol_unit)));
}