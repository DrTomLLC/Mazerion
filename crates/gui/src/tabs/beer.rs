//! Beer calculators tab with IBU, SRM, mash, efficiency

pub(crate) use crate::state::BeerCalculator;
use crate::{state::colors, MazerionApp};
use eframe::egui::{self, CornerRadius, RichText};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use std::str::FromStr;

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Select Calculator:").strong());
        egui::ComboBox::from_id_salt("beer_calc")
            .selected_text(get_calc_name(app.state.beer_calc))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.state.beer_calc, BeerCalculator::Ibu, "IBU Calculator");
                ui.selectable_value(&mut app.state.beer_calc, BeerCalculator::Srm, "SRM Color Calculator");
                ui.selectable_value(&mut app.state.beer_calc, BeerCalculator::Mash, "Mash Water Calculator");
                ui.selectable_value(&mut app.state.beer_calc, BeerCalculator::Efficiency, "Brewhouse Efficiency");
            });
    });

    ui.add_space(10.0);

    egui::Frame::default()
        .fill(colors::LIGHT_CREAM)
        .stroke(egui::Stroke::new(1.5, colors::HONEY_GOLD))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            match app.state.beer_calc {
                BeerCalculator::Ibu => render_ibu(app, ui),
                BeerCalculator::Srm => render_srm(app, ui),
                BeerCalculator::Mash => render_mash(app, ui),
                BeerCalculator::Efficiency => render_efficiency(app, ui),
            }
        });
}

fn get_calc_name(calc: BeerCalculator) -> &'static str {
    match calc {
        BeerCalculator::Ibu => "IBU Calculator",
        BeerCalculator::Srm => "SRM Color Calculator",
        BeerCalculator::Mash => "Mash Water Calculator",
        BeerCalculator::Efficiency => "Brewhouse Efficiency",
    }
}

fn render_ibu(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍺 IBU Calculator (Tinseth)").color(colors::SADDLE_BROWN));
    ui.label("Calculate International Bitterness Units for beer");
    ui.add_space(10.0);

    let (weight_label, volume_label) = if app.state.unit_system == crate::state::UnitSystem::Metric {
        ("Hop Weight (g):", "Volume (L):")
    } else {
        ("Hop Weight (oz):", "Volume (gal):")
    };

    crate::input_field(ui, weight_label, &mut app.hop_weight, "Weight of hops");
    crate::input_field(ui, "Alpha Acid (%):", &mut app.alpha_acid, "Alpha acid percentage (5-15% typical)");
    crate::input_field(ui, "Boil Time (min):", &mut app.boil_time, "Boil duration in minutes (0-120)");
    crate::input_field(ui, volume_label, &mut app.beer_volume, "Batch volume");
    crate::input_field(ui, "Boil Gravity:", &mut app.boil_gravity, "Specific gravity during boil (e.g., 1.050)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate IBU") {
        calc_ibu(app);
    }
}

fn render_srm(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🎨 SRM Color Calculator (Morey)").color(colors::SADDLE_BROWN));
    ui.label("Calculate beer color in SRM using Morey equation");
    ui.add_space(10.0);

    let (weight_label, volume_label) = if app.state.unit_system == crate::state::UnitSystem::Metric {
        ("Grain Weight (kg):", "Volume (L):")
    } else {
        ("Grain Weight (lb):", "Volume (gal):")
    };

    crate::input_field(ui, weight_label, &mut app.grain_weight, "Total grain weight");
    crate::input_field(ui, "Lovibond:", &mut app.grain_lovibond, "Grain color in Lovibond (2-500)");
    crate::input_field(ui, volume_label, &mut app.beer_volume, "Batch volume");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate SRM") {
        calc_srm(app);
    }
}

fn render_mash(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🌡️ Mash Water Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate strike water for single-infusion mash");
    ui.add_space(10.0);

    let (weight_label, temp_label, ratio_label) = if app.state.unit_system == crate::state::UnitSystem::Metric {
        ("Grain Weight (kg):", "Target Mash Temp (°C):", "Water Ratio (L/kg):")
    } else {
        ("Grain Weight (lb):", "Target Mash Temp (°F):", "Water Ratio (qt/lb):")
    };

    crate::input_field(ui, weight_label, &mut app.grain_weight, "Total grain weight");
    crate::input_field(ui, temp_label, &mut app.mash_target_temp, "Desired mash temperature");
    crate::input_field(ui, "Grain Temp (°C/°F):", &mut app.grain_temp, "Initial grain temperature");
    crate::input_field(ui, ratio_label, &mut app.mash_ratio, "Water-to-grain ratio (1.25-2.0 L/kg or 1.5-2.5 qt/lb)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Strike Water") {
        calc_mash(app);
    }
}

fn render_efficiency(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("📊 Brewhouse Efficiency").color(colors::SADDLE_BROWN));
    ui.label("Calculate mash and brewhouse efficiency");
    ui.add_space(10.0);

    let (weight_label, volume_label) = if app.state.unit_system == crate::state::UnitSystem::Metric {
        ("Grain Weight (kg):", "Volume (L):")
    } else {
        ("Grain Weight (lb):", "Volume (gal):")
    };

    crate::input_field(ui, weight_label, &mut app.grain_weight, "Total grain weight");
    crate::input_field(ui, "Grain PPG:", &mut app.grain_ppg, "Points per pound per gallon (35-40 typical)");
    crate::input_field(ui, "Measured OG:", &mut app.measured_gravity, "Actual original gravity measured");
    crate::input_field(ui, volume_label, &mut app.beer_volume, "Final volume into fermenter");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Efficiency") {
        calc_efficiency(app);
    }
}

fn calc_ibu(app: &mut MazerionApp) {
    let hop_weight_val = match Decimal::from_str(&app.hop_weight) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("❌ Invalid hop weight".to_string());
            return;
        }
    };

    let alpha_val = match Decimal::from_str(&app.alpha_acid) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("❌ Invalid alpha acid".to_string());
            return;
        }
    };

    let boil_time_val = match Decimal::from_str(&app.boil_time) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("❌ Invalid boil time".to_string());
            return;
        }
    };

    let volume_val = match Decimal::from_str(&app.beer_volume) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("❌ Invalid volume".to_string());
            return;
        }
    };

    let gravity_val = match Decimal::from_str(&app.boil_gravity) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("❌ Invalid gravity".to_string());
            return;
        }
    };

    let weight_grams = if app.state.unit_system == crate::state::UnitSystem::Imperial {
        mazerion_core::ounces_to_grams(hop_weight_val)
    } else {
        hop_weight_val
    };

    let volume_liters = if app.state.unit_system == crate::state::UnitSystem::Imperial {
        mazerion_core::gallons_to_liters(volume_val)
    } else {
        volume_val
    };

    let boil_time_f64 = boil_time_val.to_string().parse::<f64>().unwrap_or(0.0);
    let gravity_f64 = gravity_val.to_string().parse::<f64>().unwrap_or(1.0);

    let util_f64 = 1.65 * 0.000125_f64.powf(gravity_f64 - 1.0) * (1.0 - (-0.04 * boil_time_f64 / 60.0).exp()) / 4.15;
    let utilization = Decimal::from_f64_retain(util_f64).unwrap_or(Decimal::ZERO);

    let alpha_percent = alpha_val / Decimal::from(100);
    let ibu = (weight_grams * alpha_percent * utilization * Decimal::from(1000)) / volume_liters;
    

    app.result = Some(format!("IBU: {:.1}", ibu));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("formula".to_string(), "Tinseth".to_string()));
    app.metadata.push(("utilization".to_string(), format!("{:.1}%", utilization * Decimal::from(100))));
}

fn calc_srm(app: &mut MazerionApp) {
    let grain_weight_val = match Decimal::from_str(&app.grain_weight) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("❌ Invalid grain weight".to_string());
            return;
        }
    };

    let lovibond_val = match Decimal::from_str(&app.grain_lovibond) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("❌ Invalid Lovibond".to_string());
            return;
        }
    };

    let volume_val = match Decimal::from_str(&app.beer_volume) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("❌ Invalid volume".to_string());
            return;
        }
    };

    let weight_lbs = if app.state.unit_system == crate::state::UnitSystem::Metric {
        mazerion_core::kilograms_to_pounds(grain_weight_val)
    } else {
        grain_weight_val
    };

    let volume_gal = if app.state.unit_system == crate::state::UnitSystem::Metric {
        mazerion_core::liters_to_gallons(volume_val)
    } else {
        volume_val
    };

    let mcu = volume_gal / (weight_lbs * lovibond_val);
    // Convert to f64, calculate power, and convert back to Decimal
    let mcu_f64 = mcu.to_f64().unwrap_or(0.0);
    let power_f64 = mcu_f64.powf(0.6859);
    let srm = Decimal::new(14922, 4) * Decimal::from_f64_retain(power_f64).unwrap_or(mcu);

    app.result = Some(format!("SRM: {:.1}", srm));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("formula".to_string(), "Morey".to_string()));
    app.metadata.push(("mcu".to_string(), format!("{:.1}", mcu)));
}

fn calc_mash(app: &mut MazerionApp) {
    let grain_weight_val = match Decimal::from_str(&app.grain_weight) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("❌ Invalid grain weight".to_string());
            return;
        }
    };

    let target_temp_val = match Decimal::from_str(&app.mash_target_temp) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("❌ Invalid target temperature".to_string());
            return;
        }
    };

    let grain_temp_val = match Decimal::from_str(&app.grain_temp) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("❌ Invalid grain temperature".to_string());
            return;
        }
    };

    let ratio_val = match Decimal::from_str(&app.mash_ratio) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("❌ Invalid water ratio".to_string());
            return;
        }
    };

    let water_volume = grain_weight_val * ratio_val;
    let strike_temp = (Decimal::new(4, 1) / ratio_val) * (target_temp_val - grain_temp_val) + target_temp_val;

    let (vol_unit, temp_unit) = if app.state.unit_system == crate::state::UnitSystem::Metric {
        ("L", "°C")
    } else {
        ("qt", "°F")
    };

    app.result = Some(format!("Strike Water: {:.2} {} at {:.1} {}", water_volume, vol_unit, strike_temp, temp_unit));
    app.warnings.clear();
    app.metadata.clear();
}

fn calc_efficiency(app: &mut MazerionApp) {
    let grain_weight_val = match Decimal::from_str(&app.grain_weight) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("❌ Invalid grain weight".to_string());
            return;
        }
    };

    let ppg_val = match Decimal::from_str(&app.grain_ppg) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("❌ Invalid PPG".to_string());
            return;
        }
    };

    let og_val = match Decimal::from_str(&app.measured_gravity) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("❌ Invalid OG".to_string());
            return;
        }
    };

    let volume_val = match Decimal::from_str(&app.beer_volume) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("❌ Invalid volume".to_string());
            return;
        }
    };

    let weight_lbs = if app.state.unit_system == crate::state::UnitSystem::Metric {
        mazerion_core::kilograms_to_pounds(grain_weight_val)
    } else {
        grain_weight_val
    };

    let volume_gal = if app.state.unit_system == crate::state::UnitSystem::Metric {
        mazerion_core::liters_to_gallons(volume_val)
    } else {
        volume_val
    };

    let potential_points = weight_lbs * ppg_val;
    let actual_points = (og_val - Decimal::ONE) * Decimal::from(1000) * volume_gal;
    let efficiency = (actual_points / potential_points) * Decimal::from(100);

    app.result = Some(format!("Efficiency: {:.1}%", efficiency));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("potential_points".to_string(), format!("{:.0}", potential_points)));
    app.metadata.push(("actual_points".to_string(), format!("{:.0}", actual_points)));
}