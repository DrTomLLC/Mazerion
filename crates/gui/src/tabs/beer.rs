//! Beer calculators tab - COMPLETE implementation
//! All 5 calculators fully functional with unit system support

pub(crate) use crate::state::BeerCalculator;
use crate::{state::{colors, UnitSystem}, MazerionApp};
use eframe::egui::{self, CornerRadius, RichText, Color32, ScrollArea};
use rust_decimal::Decimal;
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
                ui.selectable_value(&mut app.state.beer_calc, BeerCalculator::StyleGuide, "Beer Style Guide");
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
                BeerCalculator::StyleGuide => render_style_guide(ui),
            }
        });
}

fn get_calc_name(calc: BeerCalculator) -> &'static str {
    match calc {
        BeerCalculator::Ibu => "IBU Calculator",
        BeerCalculator::Srm => "SRM Color Calculator",
        BeerCalculator::Mash => "Mash Water Calculator",
        BeerCalculator::Efficiency => "Brewhouse Efficiency",
        BeerCalculator::StyleGuide => "Beer Style Guide",
    }
}

fn render_ibu(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍺 IBU Calculator (Tinseth)").color(colors::SADDLE_BROWN));
    ui.label("Calculate International Bitterness Units using Tinseth formula");
    ui.add_space(10.0);

    let (weight_label, volume_label) = match app.state.unit_system {
        UnitSystem::Metric => ("Hop Weight (g):", "Batch Volume (L):"),
        UnitSystem::Imperial => ("Hop Weight (oz):", "Batch Volume (gal):"),
    };

    crate::input_field(ui, weight_label, &mut app.hop_weight, "Weight of hops");
    crate::input_field(ui, "Alpha Acid (%):", &mut app.alpha_acid, "Alpha acid percentage (5-15% typical)");
    crate::input_field(ui, "Boil Time (min):", &mut app.boil_time, "Boil duration in minutes (0-120)");
    crate::input_field(ui, volume_label, &mut app.beer_volume, "Batch volume");
    crate::input_field(ui, "Boil Gravity:", &mut app.boil_gravity, "Specific gravity during boil (e.g., 1.050)");

    ui.add_space(5.0);
    ui.label(RichText::new("💡 Tip: Higher gravity reduces utilization. Longer boils increase bitterness.").size(12.0));

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate IBU") {
        calc_ibu(app);
    }
}

fn render_srm(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🎨 SRM Color Calculator (Morey)").color(colors::SADDLE_BROWN));
    ui.label("Calculate beer color in SRM using Morey equation");
    ui.add_space(10.0);

    let (weight_label, volume_label) = match app.state.unit_system {
        UnitSystem::Metric => ("Grain Weight (kg):", "Batch Volume (L):"),
        UnitSystem::Imperial => ("Grain Weight (lb):", "Batch Volume (gal):"),
    };

    crate::input_field(ui, weight_label, &mut app.grain_weight, "Total grain weight");
    crate::input_field(ui, "Lovibond:", &mut app.grain_lovibond, "Grain color in Lovibond (2-500)");
    crate::input_field(ui, volume_label, &mut app.beer_volume, "Batch volume");

    ui.add_space(5.0);
    ui.label(RichText::new("💡 Common: Pale Malt ~2°L, Crystal ~60°L, Chocolate ~350°L, Black ~500°L").size(12.0));

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate SRM") {
        calc_srm(app);
    }
}

fn render_mash(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🌡️ Mash Water Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate strike water temperature and volume for single-infusion mash");
    ui.add_space(10.0);

    let (weight_label, temp_label, ratio_label, ratio_hint) = match app.state.unit_system {
        UnitSystem::Metric => (
            "Grain Weight (kg):",
            "Target Mash Temp (°C):",
            "Water Ratio (L/kg):",
            "Water-to-grain ratio (1.25-2.0 L/kg typical)"
        ),
        UnitSystem::Imperial => (
            "Grain Weight (lb):",
            "Target Mash Temp (°F):",
            "Water Ratio (qt/lb):",
            "Water-to-grain ratio (1.0-2.0 qt/lb typical)"
        ),
    };

    crate::input_field(ui, weight_label, &mut app.grain_weight, "Total grain weight");
    crate::input_field(ui, temp_label, &mut app.mash_target_temp, "Desired mash temperature (65-68°C / 149-154°F)");
    crate::input_field(ui, "Grain Temp:", &mut app.grain_temp, "Initial grain temperature (usually room temp)");
    crate::input_field(ui, ratio_label, &mut app.mash_ratio, ratio_hint);

    ui.add_space(5.0);
    ui.label(RichText::new("💡 Typical: 66°C (151°F) for balanced, 68°C (154°F) for maltier beer").size(12.0));

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Strike Water") {
        calc_mash(app);
    }
}

fn render_efficiency(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("📊 Brewhouse Efficiency").color(colors::SADDLE_BROWN));
    ui.label("Calculate extraction efficiency from grain and measured gravity");
    ui.add_space(10.0);

    let (weight_label, volume_label) = match app.state.unit_system {
        UnitSystem::Metric => ("Grain Weight (kg):", "Final Volume (L):"),
        UnitSystem::Imperial => ("Grain Weight (lb):", "Final Volume (gal):"),
    };

    crate::input_field(ui, weight_label, &mut app.grain_weight, "Total grain weight");
    crate::input_field(ui, "Grain PPG:", &mut app.grain_ppg, "Points per pound per gallon (35-40 typical for base malt)");
    crate::input_field(ui, "Measured OG:", &mut app.measured_gravity, "Actual original gravity measured (e.g., 1.050)");
    crate::input_field(ui, volume_label, &mut app.beer_volume, "Final volume into fermenter");

    ui.add_space(5.0);
    ui.label(RichText::new("💡 Good brewhouse efficiency: 70-80%").size(12.0));

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Efficiency") {
        calc_efficiency(app);
    }
}

fn render_style_guide(ui: &mut egui::Ui) {
    ui.heading(RichText::new("📖 BJCP Beer Style Guide").color(colors::SADDLE_BROWN).size(20.0));
    ui.label("Complete specifications for 29 classic beer styles");
    ui.add_space(15.0);

    ScrollArea::vertical().show(ui, |ui| {
        style_section(ui, "🍺 PALE ALES", &[
            ("American Pale Ale", "1.045-1.060", "1.010-1.015", "30-50", "5-10", "4.5-6.2%", "Moderate hop aroma with citrus, pine, or floral notes. Clean malt backbone. Medium body. Classic American hops shine. Sierra Nevada Pale Ale, Dale's Pale Ale, Stone Pale Ale."),
            ("English Pale Ale", "1.040-1.054", "1.008-1.014", "20-40", "5-14", "3.8-5.4%", "Earthy, herbal hop character. Biscuity malt. Medium-light body. Traditional English hops dominate. Fuller's ESB, Bass Pale Ale, Samuel Smith's."),
            ("IPA (American)", "1.056-1.075", "1.010-1.018", "40-70", "6-14", "5.5-7.5%", "Intense hop aroma: citrus, tropical fruit, pine. Strong bitterness balanced by malt. Medium-full body. The craft beer revolution. Pliny the Elder, Bell's Two Hearted, Dogfish Head 60 Minute."),
            ("English IPA", "1.050-1.070", "1.010-1.015", "40-60", "6-14", "5.0-7.5%", "Earthy, floral hops. More malt-forward than American. Medium body. Traditional British interpretation. Greene King IPA, Goose Island IPA, Samuel Smith's India Ale."),
        ]);

        ui.add_space(10.0);

        style_section(ui, "🥃 AMBER & BROWN ALES", &[
            ("American Amber", "1.045-1.060", "1.010-1.015", "25-40", "10-17", "4.5-6.2%", "Caramel malt sweetness, moderate hops. Balanced, smooth. Medium body. Malt and hops in harmony. Fat Tire, Anderson Valley Boont Amber, Bell's Amber."),
            ("English Brown Ale", "1.040-1.052", "1.008-1.013", "20-30", "12-22", "4.2-5.4%", "Nutty, toffee, caramel notes. Low hops. Medium-light body. Gentle, sessionable. Newcastle Brown, Samuel Smith's Nut Brown, Brooklyn Brown."),
            ("American Brown", "1.045-1.060", "1.010-1.016", "20-40", "18-35", "4.3-6.2%", "Chocolate, caramel malt. More hop presence than English. Medium body. American twist on tradition. Avery Ellie's Brown, Brooklyn Brown, Lost Coast Downtown Brown."),
        ]);

        ui.add_space(10.0);

        style_section(ui, "⚫ STOUTS & PORTERS", &[
            ("Dry Irish Stout", "1.036-1.044", "1.007-1.011", "25-45", "25-40", "4.0-5.0%", "Roasted barley dominates. Dry, coffee-like finish. Medium-light body. Creamy nitro pour. Guinness, Murphy's, Beamish."),
            ("Sweet Stout", "1.044-1.060", "1.012-1.024", "20-40", "30-40", "4.0-6.0%", "Sweet, milk chocolate character. Lactose adds sweetness and body. Full body. Dessert-like. Left Hand Milk Stout, Mackeson's, Samuel Adams Cream Stout."),
            ("Oatmeal Stout", "1.048-1.065", "1.010-1.018", "25-40", "22-40", "4.2-5.9%", "Smooth, silky from oats. Coffee, chocolate notes. Medium-full body. Velvety mouthfeel. Samuel Smith's Oatmeal, Rogue Shakespeare, Anderson Valley Barney Flats."),
            ("American Stout", "1.050-1.075", "1.010-1.022", "35-75", "30-40", "5.0-7.0%", "Bold roast character, strong hop presence. Full body. American hops meet dark malt. Rogue Shakespeare, Sierra Nevada Stout, North Coast Old Rasputin."),
            ("Russian Imperial Stout", "1.075-1.115", "1.018-1.030", "50-90", "30-40", "8.0-12.0%", "Intense roasted malt, dark fruit complexity. Very full body. Rich, warming, complex. Old Rasputin, Founders KBS, Three Floyds Dark Lord."),
            ("Robust Porter", "1.048-1.065", "1.012-1.016", "25-50", "22-35", "4.8-6.5%", "Roasted malt, chocolate, coffee. Balanced bitterness. Medium-full body. Dark but approachable. Anchor Porter, Deschutes Black Butte, Sierra Nevada Porter."),
        ]);

        ui.add_space(10.0);

        style_section(ui, "🍯 WHEAT BEERS", &[
            ("American Wheat", "1.040-1.055", "1.008-1.013", "15-30", "2-6", "4.0-5.5%", "Clean wheat character, citrus hop notes. Light, crisp. Light-medium body. Refreshing summer beer. Bell's Oberon, Boulevard Unfiltered Wheat, Goose Island 312."),
            ("Hefeweizen", "1.044-1.052", "1.010-1.014", "8-15", "2-6", "4.3-5.6%", "Banana and clove from yeast. Cloudy, unfiltered. Medium-light body. Bavarian classic. Weihenstephaner, Paulaner, Franziskaner."),
            ("Witbier", "1.044-1.052", "1.008-1.012", "10-20", "2-4", "4.5-5.5%", "Coriander, orange peel spicing. Cloudy, refreshing. Light-medium body. Belgian summer classic. Hoegaarden, Allagash White, Blue Moon."),
        ]);

        ui.add_space(10.0);

        style_section(ui, "🏆 LAGERS", &[
            ("American Lager", "1.040-1.050", "1.004-1.010", "8-18", "2-4", "4.2-5.3%", "Crisp, clean, neutral. Very light body. Highly carbonated. Mass-market appeal. Budweiser, Miller, Coors."),
            ("Pilsner (German)", "1.044-1.050", "1.008-1.013", "25-45", "2-5", "4.4-5.2%", "Noble hop aroma, crisp bitterness. Light body. Elegant, refined. Original pilsner. Pilsner Urquell, Bitburger, König."),
            ("Pilsner (Czech)", "1.044-1.056", "1.013-1.017", "30-45", "3.5-6", "4.2-5.4%", "Rich malt, spicy Saaz hops. Medium body. More malt-forward than German. Pilsner Urquell, Staropramen, Czechvar."),
            ("Märzen/Oktoberfest", "1.054-1.060", "1.010-1.014", "18-24", "8-17", "5.8-6.3%", "Toasty malt, clean finish. Medium body. Amber lager perfection. Seasonal favorite. Paulaner, Spaten, Sam Adams Octoberfest."),
            ("Bock", "1.064-1.072", "1.013-1.019", "20-27", "14-22", "6.3-7.2%", "Malty, slightly sweet. Full body. Minimal hop presence. Malt showcase. Einbecker, Shiner Bock, Ayinger Celebrator."),
            ("Doppelbock", "1.072-1.112", "1.016-1.024", "16-26", "16-26", "7.0-10.0%", "Rich, intense malt. Very full body. Dark fruit complexity. Strong lager. Paulaner Salvator, Ayinger Celebrator, Spaten Optimator."),
        ]);

        ui.add_space(10.0);

        style_section(ui, "🇧🇪 BELGIAN STYLES", &[
            ("Belgian Blonde", "1.062-1.075", "1.008-1.018", "15-30", "4-7", "6.0-7.5%", "Fruity esters, spicy phenols. Medium body. Complex but approachable. Elegant Belgian ale. Leffe Blonde, Affligem, La Chouffe."),
            ("Belgian Dubbel", "1.062-1.075", "1.008-1.018", "15-25", "10-17", "6.0-7.6%", "Dark fruit, caramel, spice. Medium-full body. Malty complexity. Abbey-style classic. Westmalle Dubbel, Chimay Red, La Trappe."),
            ("Belgian Tripel", "1.075-1.085", "1.008-1.014", "20-40", "4.5-7", "7.5-9.5%", "Fruity, spicy, strong. Medium-light body despite ABV. Deceptively drinkable. Belgian masterpiece. Westmalle Tripel, Chimay White, La Fin du Monde."),
            ("Saison", "1.048-1.065", "1.002-1.012", "20-35", "5-14", "5.0-7.0%", "Fruity, spicy, peppery. Light-medium body. Highly carbonated. Farmhouse refreshment. Saison Dupont, Boulevard Tank 7, Ommegang Hennepin."),
        ]);

        ui.add_space(10.0);

        style_section(ui, "🔥 STRONG ALES", &[
            ("Barleywine (American)", "1.080-1.120", "1.016-1.030", "50-100", "10-19", "8.0-12.0%", "Intense malt, strong hops. Very full body. Rich, complex, age-worthy. Beer for sipping. Sierra Nevada Bigfoot, Anchor Old Foghorn, Stone Old Guardian."),
            ("Barleywine (English)", "1.080-1.120", "1.018-1.030", "35-70", "8-22", "8.0-12.0%", "Fruity, malty, complex. Very full body. Sherry-like with age. British strong ale tradition. J.W. Lees Harvest Ale, Fuller's Golden Pride, Young's Old Nick."),
            ("Scotch Ale (Wee Heavy)", "1.070-1.130", "1.018-1.056", "17-35", "14-25", "6.5-10.0%", "Rich, malty, sweet. Full body. Caramel dominance. Scottish strength. Founders Dirty Bastard, Oskar Blues Old Chub, Traquair House."),
        ]);
    });
}

fn style_section(ui: &mut egui::Ui, title: &str, styles: &[(&str, &str, &str, &str, &str, &str, &str)]) {
    ui.label(RichText::new(title).size(18.0).color(colors::SADDLE_BROWN).strong());
    ui.add_space(5.0);

    for (name, og, fg, ibu, srm, abv, desc) in styles {
        egui::Frame::default()
            .fill(Color32::WHITE)
            .stroke(egui::Stroke::new(1.0, colors::HONEY_GOLD))
            .corner_radius(CornerRadius::same(6))
            .inner_margin(10.0)
            .show(ui, |ui| {
                ui.heading(RichText::new(*name).color(colors::SADDLE_BROWN).size(14.0));
                ui.label(RichText::new(format!("OG: {} | FG: {} | IBU: {} | SRM: {} | ABV: {}", og, fg, ibu, srm, abv))
                    .color(Color32::from_rgb(0, 100, 0)).size(11.0).strong());
                ui.add_space(5.0);
                ui.label(*desc);
            });
        ui.add_space(8.0);
    }
}

// === CALCULATION FUNCTIONS ===

fn calc_ibu(app: &mut MazerionApp) {
    let is_metric = matches!(app.state.unit_system, UnitSystem::Metric);

    let hop_weight_val = match Decimal::from_str(&app.hop_weight) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid hop weight".to_string());
            return;
        }
    };

    let alpha_acid_val = match Decimal::from_str(&app.alpha_acid) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid alpha acid".to_string());
            return;
        }
    };

    let boil_time_val = match Decimal::from_str(&app.boil_time) {
        Ok(v) if v >= Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid boil time".to_string());
            return;
        }
    };

    let volume_val = match Decimal::from_str(&app.beer_volume) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid volume".to_string());
            return;
        }
    };

    let gravity_val = match Decimal::from_str(&app.boil_gravity) {
        Ok(v) if v >= Decimal::ONE => v,
        _ => {
            app.result = Some("❌ Invalid boil gravity".to_string());
            return;
        }
    };

    // Convert to metric
    let weight_g = if is_metric {
        hop_weight_val
    } else {
        hop_weight_val * Decimal::new(2835, 2) // oz to g
    };

    let volume_l = if is_metric {
        volume_val
    } else {
        volume_val * Decimal::new(378541, 5) // gal to L
    };

    // Tinseth formula calculations
    let sg_diff = (gravity_val - Decimal::ONE).to_string().parse::<f64>().unwrap_or(0.05);
    let time_f64 = boil_time_val.to_string().parse::<f64>().unwrap_or(60.0);

    let bigness = 1.65 * 0.000125_f64.powf(sg_diff);
    let boil_factor = (1.0 - (-0.04 * time_f64).exp()) / 4.15;
    let utilization = bigness * boil_factor;

    let util_decimal = Decimal::from_f64_retain(utilization).unwrap_or(Decimal::ZERO);
    let aa_decimal = alpha_acid_val / Decimal::from(100);
    let ibu = (weight_g * aa_decimal * util_decimal * Decimal::from(1000)) / volume_l;

    app.result = Some(format!("IBU: {:.1}", ibu));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("Formula".to_string(), "Tinseth".to_string()));
    app.metadata.push(("Utilization".to_string(), format!("{:.1}%", util_decimal * Decimal::from(100))));
    app.metadata.push(("Bigness Factor".to_string(), format!("{:.4}", bigness)));
    app.metadata.push(("Boil Factor".to_string(), format!("{:.4}", boil_factor)));

    if ibu > Decimal::from(100) {
        app.warnings.push("IBU > 100 is extremely bitter".to_string());
    }
    if alpha_acid_val > Decimal::from(20) {
        app.warnings.push("Alpha acid > 20% is unusually high - verify value".to_string());
    }
}

fn calc_srm(app: &mut MazerionApp) {
    let is_metric = matches!(app.state.unit_system, UnitSystem::Metric);

    let grain_weight_val = match Decimal::from_str(&app.grain_weight) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid grain weight".to_string());
            return;
        }
    };

    let lovibond_val = match Decimal::from_str(&app.grain_lovibond) {
        Ok(v) if v >= Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid Lovibond".to_string());
            return;
        }
    };

    let volume_val = match Decimal::from_str(&app.beer_volume) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid volume".to_string());
            return;
        }
    };

    // Convert to Imperial (lbs and gallons) for MCU calculation
    let weight_lbs = if is_metric {
        grain_weight_val * Decimal::new(220462262, 8) // kg to lb
    } else {
        grain_weight_val
    };

    let volume_gal = if is_metric {
        volume_val * Decimal::new(264172, 6) // L to gal
    } else {
        volume_val
    };

    // MCU = (grain_lbs × lovibond) / volume_gal
    let mcu = (weight_lbs * lovibond_val) / volume_gal;
    let mcu_f64 = mcu.to_string().parse::<f64>().unwrap_or(0.0);

    // Morey: SRM = 1.4922 × (MCU^0.6859)
    let srm_f64 = 1.4922 * mcu_f64.powf(0.6859);
    let srm = Decimal::from_f64_retain(srm_f64).unwrap_or(mcu);

    let color_desc = match srm_f64 as i32 {
        0..=3 => "Pale Straw",
        4..=6 => "Straw to Pale Gold",
        7..=9 => "Deep Gold to Pale Amber",
        10..=13 => "Amber",
        14..=17 => "Deep Amber to Copper",
        18..=20 => "Copper to Light Brown",
        21..=24 => "Brown",
        25..=30 => "Dark Brown",
        31..=40 => "Very Dark Brown",
        _ => "Black",
    };

    app.result = Some(format!("SRM: {:.1}", srm));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("Formula".to_string(), "Morey".to_string()));
    app.metadata.push(("MCU".to_string(), format!("{:.1}", mcu)));
    app.metadata.push(("Color Description".to_string(), color_desc.to_string()));
}

fn calc_mash(app: &mut MazerionApp) {
    let is_metric = matches!(app.state.unit_system, UnitSystem::Metric);

    let target_temp_val = match Decimal::from_str(&app.mash_target_temp) {
        Ok(v) => v,
        _ => {
            app.result = Some("❌ Invalid target temperature".to_string());
            return;
        }
    };

    let grain_temp_val = match Decimal::from_str(&app.grain_temp) {
        Ok(v) => v,
        _ => {
            app.result = Some("❌ Invalid grain temperature".to_string());
            return;
        }
    };

    let grain_weight_val = match Decimal::from_str(&app.grain_weight) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid grain weight".to_string());
            return;
        }
    };

    let ratio_val = match Decimal::from_str(&app.mash_ratio) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid water ratio".to_string());
            return;
        }
    };

    // Convert imperial temps to celsius for calculation
    let (target_c, grain_c) = if is_metric {
        (target_temp_val, grain_temp_val)
    } else {
        let target_c = (target_temp_val - Decimal::from(32)) * Decimal::new(5, 0) / Decimal::new(9, 0);
        let grain_c = (grain_temp_val - Decimal::from(32)) * Decimal::new(5, 0) / Decimal::new(9, 0);
        (target_c, grain_c)
    };

    // Convert ratio to L/kg if needed
    let ratio_l_kg = if is_metric {
        ratio_val
    } else {
        // qt/lb to L/kg: 1 qt/lb = 2.0864 L/kg
        ratio_val * Decimal::new(20864, 4)
    };

    // Strike temp = target + (0.2/ratio) × (target - grain)
    let temp_diff = target_c - grain_c;
    let thermal_constant = Decimal::new(2, 1) / ratio_l_kg;
    let strike_temp_c = target_c + (thermal_constant * temp_diff);

    // Water volume
    let water_volume_l = grain_weight_val * ratio_l_kg;

    // Convert back to user units for display
    let (strike_display, water_display) = if is_metric {
        (
            format!("{:.1}°C", strike_temp_c),
            format!("{:.2} L", water_volume_l)
        )
    } else {
        let strike_f = (strike_temp_c * Decimal::new(9, 0) / Decimal::new(5, 0)) + Decimal::from(32);
        let water_gal = water_volume_l * Decimal::new(264172, 6);
        (
            format!("{:.1}°F", strike_f),
            format!("{:.2} gal", water_gal)
        )
    };

    app.result = Some(format!("Strike Water: {}", strike_display));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("Strike Temperature".to_string(), strike_display));
    app.metadata.push(("Water Volume".to_string(), water_display));
    app.metadata.push(("Mash Ratio".to_string(), if is_metric {
        format!("{:.2} L/kg", ratio_l_kg)
    } else {
        format!("{:.2} qt/lb", ratio_val)
    }));

    if strike_temp_c > Decimal::from(80) {
        app.warnings.push("Strike temp >80°C (176°F) may extract tannins".to_string());
    }
    if strike_temp_c < Decimal::from(50) {
        app.warnings.push("Strike temp <50°C (122°F) may be too cold".to_string());
    }
}

fn calc_efficiency(app: &mut MazerionApp) {
    let is_metric = matches!(app.state.unit_system, UnitSystem::Metric);

    let grain_weight_val = match Decimal::from_str(&app.grain_weight) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid grain weight".to_string());
            return;
        }
    };

    let ppg_val = match Decimal::from_str(&app.grain_ppg) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid PPG".to_string());
            return;
        }
    };

    let gravity_val = match Decimal::from_str(&app.measured_gravity) {
        Ok(v) if v >= Decimal::ONE => v,
        _ => {
            app.result = Some("❌ Invalid measured gravity".to_string());
            return;
        }
    };

    let volume_val = match Decimal::from_str(&app.beer_volume) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid volume".to_string());
            return;
        }
    };

    // Convert to Imperial for calculation
    let weight_lbs = if is_metric {
        grain_weight_val * Decimal::new(220462262, 8)
    } else {
        grain_weight_val
    };

    let volume_gal = if is_metric {
        volume_val * Decimal::new(264172, 6)
    } else {
        volume_val
    };

    // Gravity points
    let measured_points = (gravity_val - Decimal::ONE) * Decimal::from(1000);
    let total_measured_points = measured_points * volume_gal;

    // Potential points
    let potential_points = weight_lbs * ppg_val;

    // Efficiency
    let efficiency = (total_measured_points / potential_points) * Decimal::from(100);

    app.result = Some(format!("Efficiency: {:.1}%", efficiency));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("Brewhouse Efficiency".to_string(), format!("{:.1}%", efficiency)));
    app.metadata.push(("Measured Points".to_string(), format!("{:.1}", measured_points)));
    app.metadata.push(("Total Points".to_string(), format!("{:.1}", total_measured_points)));
    app.metadata.push(("Potential Points".to_string(), format!("{:.1}", potential_points)));

    if efficiency < Decimal::from(60) {
        app.warnings.push("Low efficiency (<60%) - check mash process".to_string());
    }
    if efficiency > Decimal::from(90) {
        app.warnings.push("Very high efficiency (>90%) - verify measurements".to_string());
    }
}