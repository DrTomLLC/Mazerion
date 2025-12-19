//! Mead style calculators - ALL 10 STYLES WITH DESCRIPTIONS

use crate::{MazerionApp, state::{colors, UnitSystem}};
use eframe::egui::{self, RichText, CornerRadius};
use mazerion_core::CalcInput;
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeadStyle {
    Traditional,
    Hydromel,
    Sack,
    Melomel,
    Cyser,
    Acerglyn,
    Bochet,
    Braggot,
    Capsicumel,
    Metheglin,
}

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Select Mead Style:").strong());
        egui::ComboBox::from_id_salt("mead_style")
            .selected_text(get_style_name(app.mead_style))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.mead_style, MeadStyle::Traditional, "Traditional Mead");
                ui.selectable_value(&mut app.mead_style, MeadStyle::Hydromel, "Hydromel (Session)");
                ui.selectable_value(&mut app.mead_style, MeadStyle::Sack, "Sack Mead (High ABV)");
                ui.selectable_value(&mut app.mead_style, MeadStyle::Melomel, "Melomel (Fruit)");
                ui.selectable_value(&mut app.mead_style, MeadStyle::Cyser, "Cyser (Apple)");
                ui.selectable_value(&mut app.mead_style, MeadStyle::Acerglyn, "Acerglyn (Maple)");
                ui.selectable_value(&mut app.mead_style, MeadStyle::Bochet, "Bochet (Caramelized)");
                ui.selectable_value(&mut app.mead_style, MeadStyle::Braggot, "Braggot (Honey-Malt)");
                ui.selectable_value(&mut app.mead_style, MeadStyle::Capsicumel, "Capsicumel (Pepper)");
                ui.selectable_value(&mut app.mead_style, MeadStyle::Metheglin, "Metheglin (Spiced)");
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
                MeadStyle::Acerglyn => render_acerglyn(app, ui),
                MeadStyle::Bochet => render_bochet(app, ui),
                MeadStyle::Braggot => render_braggot(app, ui),
                MeadStyle::Capsicumel => render_capsicumel(app, ui),
                MeadStyle::Metheglin => render_metheglin(app, ui),
            }
        });
}

fn get_style_name(style: MeadStyle) -> &'static str {
    match style {
        MeadStyle::Traditional => "Traditional Mead",
        MeadStyle::Hydromel => "Hydromel (Session)",
        MeadStyle::Sack => "Sack Mead (High ABV)",
        MeadStyle::Melomel => "Melomel (Fruit)",
        MeadStyle::Cyser => "Cyser (Apple)",
        MeadStyle::Acerglyn => "Acerglyn (Maple)",
        MeadStyle::Bochet => "Bochet (Caramelized)",
        MeadStyle::Braggot => "Braggot (Honey-Malt)",
        MeadStyle::Capsicumel => "Capsicumel (Pepper)",
        MeadStyle::Metheglin => "Metheglin (Spiced)",
    }
}

fn render_traditional(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍯 Traditional Mead").color(colors::SADDLE_BROWN));
    ui.label("Pure honey mead - the timeless classic showcasing honey terroir");
    ui.label(RichText::new("Also known as: Great Mead, Show Mead").weak());
    ui.label("ABV Range: Typically 8-14%, can go higher");
    ui.add_space(10.0);

    let vol = if matches!(app.state.unit_system, UnitSystem::Metric) { "Volume (L):" } else { "Volume (gal):" };
    crate::input_field(ui, vol, &mut app.mead_volume, "Batch volume");
    crate::input_field(ui, "ABV (%):", &mut app.mead_target_abv, "8-14% typical");
    ui.add_space(10.0);
    if crate::calculate_button(ui, "Calculate") { calc_traditional(app); }
}

fn render_hydromel(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🥂 Hydromel").color(colors::SADDLE_BROWN));
    ui.label("Low ABV session mead - highly drinkable, light-bodied");
    ui.label(RichText::new("Perfect for: Summer drinking, outdoor events, day sessions").weak());
    ui.label("ABV Range: 3.5-7.5% - the session mead");
    ui.add_space(10.0);

    let vol = if matches!(app.state.unit_system, UnitSystem::Metric) { "Volume (L):" } else { "Volume (gal):" };
    crate::input_field(ui, vol, &mut app.mead_volume, "Batch volume");
    crate::input_field(ui, "ABV (%):", &mut app.mead_target_abv, "3.5-7.5%");
    ui.add_space(10.0);
    if crate::calculate_button(ui, "Calculate") { calc_hydromel(app); }
}

fn render_sack(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🏆 Sack Mead").color(colors::SADDLE_BROWN));
    ui.label("High ABV dessert mead - rich, sweet, and powerful");
    ui.label(RichText::new("Perfect for: Dessert pairing, after-dinner sipping, special occasions").weak());
    ui.label("ABV Range: 14-18% - the dessert mead");
    ui.add_space(10.0);

    let vol = if matches!(app.state.unit_system, UnitSystem::Metric) { "Volume (L):" } else { "Volume (gal):" };
    crate::input_field(ui, vol, &mut app.mead_volume, "Batch volume");
    crate::input_field(ui, "ABV (%):", &mut app.mead_target_abv, "14-18%");
    ui.add_space(10.0);
    if crate::calculate_button(ui, "Calculate") { calc_sack(app); }
}

fn render_melomel(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍓 Melomel").color(colors::SADDLE_BROWN));
    ui.label("Fruit mead - honey and fruit in perfect harmony");
    ui.label(RichText::new("Popular fruits: Strawberry, blueberry, raspberry, cherry, blackberry").weak());
    ui.label("The fruit provides sugar AND flavor - adjust honey accordingly");
    ui.add_space(10.0);

    let is_metric = matches!(app.state.unit_system, UnitSystem::Metric);
    let vol = if is_metric { "Volume (L):" } else { "Volume (gal):" };
    let wt = if is_metric { "Fruit (kg):" } else { "Fruit (lb):" };

    crate::input_field(ui, vol, &mut app.mead_volume, "Batch volume");
    crate::input_field(ui, "ABV (%):", &mut app.mead_target_abv, "Target ABV");
    crate::input_field(ui, wt, &mut app.fruit_weight, "Fruit weight");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Fruit:").strong());
        egui::ComboBox::from_id_salt("fruit")
            .selected_text(&app.fruit_type)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.fruit_type, "strawberry".to_string(), "Strawberry (6% sugar)");
                ui.selectable_value(&mut app.fruit_type, "blueberry".to_string(), "Blueberry (10% sugar)");
                ui.selectable_value(&mut app.fruit_type, "raspberry".to_string(), "Raspberry (5% sugar)");
                ui.selectable_value(&mut app.fruit_type, "cherry".to_string(), "Cherry (12% sugar)");
                ui.selectable_value(&mut app.fruit_type, "blackberry".to_string(), "Blackberry (9% sugar)");
            });
    });

    ui.add_space(10.0);
    if crate::calculate_button(ui, "Calculate") { calc_melomel(app); }
}

fn render_cyser(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍎 Cyser").color(colors::SADDLE_BROWN));
    ui.label("Apple mead - the best of cider and mead combined");
    ui.label(RichText::new("Traditional pairing: Thanksgiving turkey, fall foods").weak());
    ui.label("Apple juice provides ~10% of fermentables - adjust honey accordingly");
    ui.add_space(10.0);

    let vol = if matches!(app.state.unit_system, UnitSystem::Metric) { "Volume (L):" } else { "Volume (gal):" };
    crate::input_field(ui, vol, &mut app.mead_volume, "Batch volume");
    crate::input_field(ui, "ABV (%):", &mut app.mead_target_abv, "Target ABV");
    crate::input_field(ui, "Juice %:", &mut app.juice_percent, "30-50% typical");
    ui.add_space(10.0);
    if crate::calculate_button(ui, "Calculate") { calc_cyser(app); }
}

fn render_acerglyn(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍁 Acerglyn").color(colors::SADDLE_BROWN));
    ui.label("Maple mead - honey meets Canadian gold");
    ui.label(RichText::new("Perfect for: Fall/winter, Canadian pride, unique flavor").weak());
    ui.label("Maple syrup is ~67% sugar - expensive but delicious");
    ui.add_space(10.0);

    let vol = if matches!(app.state.unit_system, UnitSystem::Metric) { "Volume (L):" } else { "Volume (gal):" };
    crate::input_field(ui, vol, &mut app.mead_volume, "Batch volume");
    crate::input_field(ui, "ABV (%):", &mut app.mead_target_abv, "Target ABV");
    crate::input_field(ui, "Maple %:", &mut app.maple_percent, "20-40% typical");
    ui.add_space(10.0);
    if crate::calculate_button(ui, "Calculate") { calc_acerglyn(app); }
}

fn render_bochet(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🔥 Bochet").color(colors::SADDLE_BROWN));
    ui.label("Caramelized honey mead - ancient technique, modern favorite");
    ui.label(RichText::new("Honey is caramelized before fermenting - creates toffee/marshmallow notes").weak());
    ui.label("Sugar loss during caramelization: Light 5%, Medium 10%, Dark 15%");
    ui.add_space(10.0);

    let vol = if matches!(app.state.unit_system, UnitSystem::Metric) { "Volume (L):" } else { "Volume (gal):" };
    crate::input_field(ui, vol, &mut app.mead_volume, "Batch volume");
    crate::input_field(ui, "ABV (%):", &mut app.mead_target_abv, "Target ABV");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Caramel Level:").strong());
        egui::ComboBox::from_id_salt("bochet")
            .selected_text(&app.bochet_level)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.bochet_level, "light".to_string(), "Light (5% loss)");
                ui.selectable_value(&mut app.bochet_level, "medium".to_string(), "Medium (10% loss)");
                ui.selectable_value(&mut app.bochet_level, "dark".to_string(), "Dark (15% loss)");
            });
    });

    ui.add_space(10.0);
    if crate::calculate_button(ui, "Calculate") { calc_bochet(app); }
}

fn render_braggot(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍺 Braggot").color(colors::SADDLE_BROWN));
    ui.label("Honey-beer hybrid - medieval feast beverage");
    ui.label(RichText::new("Combines malt backbone with honey complexity").weak());
    ui.label("Honey %: 30-70% of fermentables - balance is key");
    ui.add_space(10.0);

    let (vol, wt) = if matches!(app.state.unit_system, UnitSystem::Metric) {
        ("Volume (L):", "Malt (kg):")
    } else {
        ("Volume (gal):", "Malt (lb):")
    };

    crate::input_field(ui, vol, &mut app.mead_volume, "Batch volume");
    crate::input_field(ui, "ABV (%):", &mut app.mead_target_abv, "Target ABV");
    crate::input_field(ui, "Honey %:", &mut app.honey_percent, "30-70%");
    crate::input_field(ui, wt, &mut app.malt_weight, "Malt amount");
    ui.add_space(10.0);
    if crate::calculate_button(ui, "Calculate") { calc_braggot(app); }
}

fn render_capsicumel(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🌶️ Capsicumel").color(colors::SADDLE_BROWN));
    ui.label("Pepper mead - adds heat and complexity");
    ui.label(RichText::new("Popular peppers: Jalapeño, habanero, ghost pepper, bell pepper").weak());
    ui.label("Dosage: Mild 0.5 g/L, Medium 1.0 g/L, Hot 1.5 g/L");
    ui.add_space(10.0);

    let vol = if matches!(app.state.unit_system, UnitSystem::Metric) { "Volume (L):" } else { "Volume (gal):" };
    crate::input_field(ui, vol, &mut app.mead_volume, "Batch volume");
    crate::input_field(ui, "ABV (%):", &mut app.mead_target_abv, "Target ABV");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Heat:").strong());
        egui::ComboBox::from_id_salt("heat")
            .selected_text(&app.heat_level)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.heat_level, "mild".to_string(), "Mild");
                ui.selectable_value(&mut app.heat_level, "medium".to_string(), "Medium");
                ui.selectable_value(&mut app.heat_level, "hot".to_string(), "Hot");
            });
    });

    ui.add_space(10.0);
    if crate::calculate_button(ui, "Calculate") { calc_capsicumel(app); }
}

fn render_metheglin(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🌿 Metheglin").color(colors::SADDLE_BROWN));
    ui.label("Spiced mead - herbs and spices meet honey");
    ui.label(RichText::new("Popular spices: Cinnamon, vanilla, ginger, clove, nutmeg").weak());
    ui.label("Dosage: Light 0.5 g/L, Medium 1.0 g/L, Heavy 2.0 g/L");
    ui.add_space(10.0);

    let vol = if matches!(app.state.unit_system, UnitSystem::Metric) { "Volume (L):" } else { "Volume (gal):" };
    crate::input_field(ui, vol, &mut app.mead_volume, "Batch volume");
    crate::input_field(ui, "ABV (%):", &mut app.mead_target_abv, "Target ABV");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Spice Level:").strong());
        egui::ComboBox::from_id_salt("spice")
            .selected_text(&app.spice_level)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.spice_level, "light".to_string(), "Light");
                ui.selectable_value(&mut app.spice_level, "medium".to_string(), "Medium");
                ui.selectable_value(&mut app.spice_level, "heavy".to_string(), "Heavy");
            });
    });

    ui.add_space(10.0);
    if crate::calculate_button(ui, "Calculate") { calc_metheglin(app); }
}

// CALCULATION FUNCTIONS - Call backend calculators

fn calc_traditional(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("great_mead") {
        Some(c) => c,
        None => { app.result = Some("❌ Calculator not found".to_string()); return; }
    };

    let volume_l = to_liters(&app.mead_volume, &app.state.unit_system);
    let input = CalcInput::new()
        .add_param("volume", &volume_l)
        .add_param("target_abv", &app.mead_target_abv);

    match calc.calculate(input) {
        Ok(res) => {
            let honey_g = res.output.value;
            let honey_kg = honey_g / Decimal::from(1000);
            let (display, unit) = fmt_weight(honey_kg, &app.state.unit_system);
            app.result = Some(format!("Honey: {} {}", display, unit));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => { app.result = Some(format!("❌ {}", e)); app.warnings.clear(); app.metadata.clear(); }
    }
}

fn calc_hydromel(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("hydromel") {
        Some(c) => c,
        None => { app.result = Some("❌ Calculator not found".to_string()); return; }
    };

    let volume_l = to_liters(&app.mead_volume, &app.state.unit_system);
    let input = CalcInput::new()
        .add_param("volume", &volume_l)
        .add_param("target_abv", &app.mead_target_abv);

    match calc.calculate(input) {
        Ok(res) => {
            let honey_g = res.output.value;
            let honey_kg = honey_g / Decimal::from(1000);
            let (display, unit) = fmt_weight(honey_kg, &app.state.unit_system);
            app.result = Some(format!("Honey: {} {}", display, unit));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => { app.result = Some(format!("❌ {}", e)); app.warnings.clear(); app.metadata.clear(); }
    }
}

fn calc_sack(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("sack") {
        Some(c) => c,
        None => { app.result = Some("❌ Calculator not found".to_string()); return; }
    };

    let volume_l = to_liters(&app.mead_volume, &app.state.unit_system);
    let input = CalcInput::new()
        .add_param("volume", &volume_l)
        .add_param("target_abv", &app.mead_target_abv);

    match calc.calculate(input) {
        Ok(res) => {
            let honey_g = res.output.value;
            let honey_kg = honey_g / Decimal::from(1000);
            let (display, unit) = fmt_weight(honey_kg, &app.state.unit_system);
            app.result = Some(format!("Honey: {} {}", display, unit));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => { app.result = Some(format!("❌ {}", e)); app.warnings.clear(); app.metadata.clear(); }
    }
}

fn calc_melomel(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("melomel") {
        Some(c) => c,
        None => { app.result = Some("❌ Calculator not found".to_string()); return; }
    };

    let volume_l = to_liters(&app.mead_volume, &app.state.unit_system);
    let fruit_kg = to_kg(&app.fruit_weight, &app.state.unit_system);

    let input = CalcInput::new()
        .add_param("volume", &volume_l)
        .add_param("target_abv", &app.mead_target_abv)
        .add_param("fruit_weight", &fruit_kg)
        .add_param("fruit_type", &app.fruit_type);

    match calc.calculate(input) {
        Ok(res) => {
            let honey_g = res.output.value;
            let honey_kg = honey_g / Decimal::from(1000);
            let fruit_kg_val = Decimal::from_str(&fruit_kg).unwrap_or(Decimal::ZERO);

            let (h_display, h_unit) = fmt_weight(honey_kg, &app.state.unit_system);
            let (f_display, f_unit) = fmt_weight(fruit_kg_val, &app.state.unit_system);

            app.result = Some(format!("Honey: {} {} | Fruit: {} {}", h_display, h_unit, f_display, f_unit));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => { app.result = Some(format!("❌ {}", e)); app.warnings.clear(); app.metadata.clear(); }
    }
}

fn calc_cyser(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("cyser") {
        Some(c) => c,
        None => { app.result = Some("❌ Calculator not found".to_string()); return; }
    };

    let volume_l = to_liters(&app.mead_volume, &app.state.unit_system);
    let input = CalcInput::new()
        .add_param("volume", &volume_l)
        .add_param("target_abv", &app.mead_target_abv)
        .add_param("juice_percent", &app.juice_percent);

    match calc.calculate(input) {
        Ok(res) => {
            let honey_g = res.output.value;
            let honey_kg = honey_g / Decimal::from(1000);
            let (display, unit) = fmt_weight(honey_kg, &app.state.unit_system);
            app.result = Some(format!("Honey: {} {}", display, unit));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => { app.result = Some(format!("❌ {}", e)); app.warnings.clear(); app.metadata.clear(); }
    }
}

fn calc_acerglyn(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("acerglyn") {
        Some(c) => c,
        None => { app.result = Some("❌ Calculator not found".to_string()); return; }
    };

    let volume_l = to_liters(&app.mead_volume, &app.state.unit_system);
    let input = CalcInput::new()
        .add_param("volume", &volume_l)
        .add_param("target_abv", &app.mead_target_abv)
        .add_param("maple_percent", &app.maple_percent);

    match calc.calculate(input) {
        Ok(res) => {
            let honey_g = res.output.value;
            let honey_kg = honey_g / Decimal::from(1000);
            let (display, unit) = fmt_weight(honey_kg, &app.state.unit_system);
            app.result = Some(format!("Honey: {} {}", display, unit));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => { app.result = Some(format!("❌ {}", e)); app.warnings.clear(); app.metadata.clear(); }
    }
}

fn calc_bochet(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("bochet") {
        Some(c) => c,
        None => { app.result = Some("❌ Calculator not found".to_string()); return; }
    };

    let volume_l = to_liters(&app.mead_volume, &app.state.unit_system);
    let input = CalcInput::new()
        .add_param("volume", &volume_l)
        .add_param("target_abv", &app.mead_target_abv)
        .add_param("bochet_level", &app.bochet_level);

    match calc.calculate(input) {
        Ok(res) => {
            let honey_g = res.output.value;
            let honey_kg = honey_g / Decimal::from(1000);
            let (display, unit) = fmt_weight(honey_kg, &app.state.unit_system);
            app.result = Some(format!("Honey (pre-caramel): {} {}", display, unit));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => { app.result = Some(format!("❌ {}", e)); app.warnings.clear(); app.metadata.clear(); }
    }
}

fn calc_braggot(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("braggot") {
        Some(c) => c,
        None => { app.result = Some("❌ Calculator not found".to_string()); return; }
    };

    let volume_l = to_liters(&app.mead_volume, &app.state.unit_system);
    let malt_kg = to_kg(&app.malt_weight, &app.state.unit_system);

    let input = CalcInput::new()
        .add_param("volume", &volume_l)
        .add_param("target_abv", &app.mead_target_abv)
        .add_param("honey_percent", &app.honey_percent)
        .add_param("malt_weight", &malt_kg);

    match calc.calculate(input) {
        Ok(res) => {
            let honey_g = res.output.value;
            let honey_kg = honey_g / Decimal::from(1000);
            let (display, unit) = fmt_weight(honey_kg, &app.state.unit_system);
            app.result = Some(format!("Honey: {} {}", display, unit));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => { app.result = Some(format!("❌ {}", e)); app.warnings.clear(); app.metadata.clear(); }
    }
}

fn calc_capsicumel(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("capsicumel") {
        Some(c) => c,
        None => { app.result = Some("❌ Calculator not found".to_string()); return; }
    };

    let volume_l = to_liters(&app.mead_volume, &app.state.unit_system);
    let input = CalcInput::new()
        .add_param("volume", &volume_l)
        .add_param("target_abv", &app.mead_target_abv);

    match calc.calculate(input) {
        Ok(res) => {
            let honey_g = res.output.value;
            let honey_kg = honey_g / Decimal::from(1000);
            let (display, unit) = fmt_weight(honey_kg, &app.state.unit_system);
            app.result = Some(format!("Honey: {} {}", display, unit));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => { app.result = Some(format!("❌ {}", e)); app.warnings.clear(); app.metadata.clear(); }
    }
}

fn calc_metheglin(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("metheglin") {
        Some(c) => c,
        None => { app.result = Some("❌ Calculator not found".to_string()); return; }
    };

    let volume_l = to_liters(&app.mead_volume, &app.state.unit_system);
    let input = CalcInput::new()
        .add_param("volume", &volume_l)
        .add_param("target_abv", &app.mead_target_abv)
        .add_param("spice_level", &app.spice_level);

    match calc.calculate(input) {
        Ok(res) => {
            let honey_g = res.output.value;
            let honey_kg = honey_g / Decimal::from(1000);
            let (display, unit) = fmt_weight(honey_kg, &app.state.unit_system);
            app.result = Some(format!("Honey: {} {}", display, unit));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => { app.result = Some(format!("❌ {}", e)); app.warnings.clear(); app.metadata.clear(); }
    }
}

// HELPERS

fn to_liters(val: &str, sys: &UnitSystem) -> String {
    match Decimal::from_str(val) {
        Ok(v) => {
            let l = if matches!(sys, UnitSystem::Imperial) {
                v * Decimal::new(378541, 5)
            } else {
                v
            };
            l.to_string()
        }
        Err(_) => "0".to_string(),
    }
}

fn to_kg(val: &str, sys: &UnitSystem) -> String {
    match Decimal::from_str(val) {
        Ok(v) => {
            let kg = if matches!(sys, UnitSystem::Imperial) {
                v * Decimal::new(453592, 6)
            } else {
                v
            };
            kg.to_string()
        }
        Err(_) => "0".to_string(),
    }
}

fn fmt_weight(kg: Decimal, sys: &UnitSystem) -> (String, &'static str) {
    match sys {
        UnitSystem::Metric => (format!("{:.2}", kg), "kg"),
        UnitSystem::Imperial => {
            let lb = kg * Decimal::new(220462, 5);
            (format!("{:.2}", lb), "lb")
        }
    }
}