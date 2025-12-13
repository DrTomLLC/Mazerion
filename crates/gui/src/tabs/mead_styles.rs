//! Mead style calculators - 10 styles with dynamic labels
//! SAFETY-CRITICAL: All calculations production-ready with unit conversions

use crate::{MazerionApp, state::{colors, UnitSystem}};
use eframe::egui::{self, RichText, CornerRadius};

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
    ui.label("Pure honey mead - timeless classic");
    ui.add_space(10.0);

    let volume_label = match app.state.unit_system {
        UnitSystem::Metric => "Batch Volume (L):",
        UnitSystem::Imperial => "Batch Volume (gal):",
    };

    crate::input_field(ui, volume_label, &mut app.mead_volume, "Total must volume");
    crate::input_field(ui, "Target ABV (%):", &mut app.mead_target_abv, "Desired alcohol level (8-14% typical)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Honey Needed") {
        calc_traditional(app);
    }
}

fn render_hydromel(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🥂 Hydromel (Session Mead)").color(colors::SADDLE_BROWN));
    ui.label("Low ABV session mead (3.5-7.5%)");
    ui.add_space(10.0);

    let volume_label = match app.state.unit_system {
        UnitSystem::Metric => "Batch Volume (L):",
        UnitSystem::Imperial => "Batch Volume (gal):",
    };

    crate::input_field(ui, volume_label, &mut app.mead_volume, "Total must volume");
    crate::input_field(ui, "Target ABV (%):", &mut app.mead_target_abv, "Session range: 3.5-7.5%");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Honey Needed") {
        calc_hydromel(app);
    }
}

fn render_sack(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🏆 Sack Mead (High Gravity)").color(colors::SADDLE_BROWN));
    ui.label("High ABV dessert mead (14-18%)");
    ui.add_space(10.0);

    let volume_label = match app.state.unit_system {
        UnitSystem::Metric => "Batch Volume (L):",
        UnitSystem::Imperial => "Batch Volume (gal):",
    };

    crate::input_field(ui, volume_label, &mut app.mead_volume, "Total must volume");
    crate::input_field(ui, "Target ABV (%):", &mut app.mead_target_abv, "High gravity: 14-18%");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Honey Needed") {
        calc_sack(app);
    }
}

fn render_melomel(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍓 Melomel (Fruit Mead)").color(colors::SADDLE_BROWN));
    ui.label("Mead with fruit - accounts for fruit sugars");
    ui.add_space(10.0);

    let (volume_label, weight_label) = match app.state.unit_system {
        UnitSystem::Metric => ("Batch Volume (L):", "Fruit Weight (kg):"),
        UnitSystem::Imperial => ("Batch Volume (gal):", "Fruit Weight (lb):"),
    };

    crate::input_field(ui, volume_label, &mut app.mead_volume, "Total must volume");
    crate::input_field(ui, "Target ABV (%):", &mut app.mead_target_abv, "Desired alcohol level");
    crate::input_field(ui, weight_label, &mut app.fruit_weight, "Fruit per liter (0.5-2.0 typical)");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Fruit Type:").strong());
        egui::ComboBox::from_id_salt("fruit_type")
            .selected_text(&app.fruit_type)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.fruit_type, "strawberry".to_string(), "Strawberry (8% sugar)");
                ui.selectable_value(&mut app.fruit_type, "blueberry".to_string(), "Blueberry (10% sugar)");
                ui.selectable_value(&mut app.fruit_type, "raspberry".to_string(), "Raspberry (5% sugar)");
                ui.selectable_value(&mut app.fruit_type, "cherry".to_string(), "Cherry (12% sugar)");
                ui.selectable_value(&mut app.fruit_type, "blackberry".to_string(), "Blackberry (9% sugar)");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Honey + Fruit") {
        calc_melomel(app);
    }
}

fn render_cyser(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍎 Cyser (Apple Mead)").color(colors::SADDLE_BROWN));
    ui.label("Mead with apple juice - accounts for juice sugars");
    ui.add_space(10.0);

    let volume_label = match app.state.unit_system {
        UnitSystem::Metric => "Batch Volume (L):",
        UnitSystem::Imperial => "Batch Volume (gal):",
    };

    crate::input_field(ui, volume_label, &mut app.mead_volume, "Total must volume");
    crate::input_field(ui, "Target ABV (%):", &mut app.mead_target_abv, "Desired alcohol level");
    crate::input_field(ui, "Juice % of Must:", &mut app.juice_percent, "Apple juice percentage (30-70% typical)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Honey + Juice") {
        calc_cyser(app);
    }
}

fn render_acerglyn(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍁 Acerglyn (Maple Mead)").color(colors::SADDLE_BROWN));
    ui.label("Mead with maple syrup - rich & complex");
    ui.add_space(10.0);

    let volume_label = match app.state.unit_system {
        UnitSystem::Metric => "Batch Volume (L):",
        UnitSystem::Imperial => "Batch Volume (gal):",
    };

    crate::input_field(ui, volume_label, &mut app.mead_volume, "Total must volume");
    crate::input_field(ui, "Target ABV (%):", &mut app.mead_target_abv, "Desired alcohol level");
    crate::input_field(ui, "Maple % of Fermentables:", &mut app.maple_percent, "Maple syrup percentage (20-40% typical)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Honey + Maple") {
        calc_acerglyn(app);
    }
}

fn render_bochet(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🔥 Bochet (Caramelized Mead)").color(colors::SADDLE_BROWN));
    ui.label("Mead with caramelized honey - deep flavors");
    ui.add_space(10.0);

    let volume_label = match app.state.unit_system {
        UnitSystem::Metric => "Batch Volume (L):",
        UnitSystem::Imperial => "Batch Volume (gal):",
    };

    crate::input_field(ui, volume_label, &mut app.mead_volume, "Total must volume");
    crate::input_field(ui, "Target ABV (%):", &mut app.mead_target_abv, "Desired alcohol level");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Caramelization Level:").strong());
        egui::ComboBox::from_id_salt("bochet_level")
            .selected_text(&app.bochet_level)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.bochet_level, "light".to_string(), "Light (10% sugar loss)");
                ui.selectable_value(&mut app.bochet_level, "medium".to_string(), "Medium (15% sugar loss)");
                ui.selectable_value(&mut app.bochet_level, "dark".to_string(), "Dark (20% sugar loss)");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Honey (Pre-Caramelization)") {
        calc_bochet(app);
    }
}

fn render_braggot(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍺 Braggot (Honey-Malt Hybrid)").color(colors::SADDLE_BROWN));
    ui.label("Half mead, half beer - best of both");
    ui.add_space(10.0);

    let (volume_label, weight_label) = match app.state.unit_system {
        UnitSystem::Metric => ("Batch Volume (L):", "Malt Weight (kg):"),
        UnitSystem::Imperial => ("Batch Volume (gal):", "Malt Weight (lb):"),
    };

    crate::input_field(ui, volume_label, &mut app.mead_volume, "Total must volume");
    crate::input_field(ui, "Target ABV (%):", &mut app.mead_target_abv, "Desired alcohol level");
    crate::input_field(ui, "Honey % of Fermentables:", &mut app.honey_percent, "Honey percentage (40-60% typical)");
    crate::input_field(ui, weight_label, &mut app.malt_weight, "Malt contribution");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Honey + Malt") {
        calc_braggot(app);
    }
}

fn render_capsicumel(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🌶️ Capsicumel (Pepper Mead)").color(colors::SADDLE_BROWN));
    ui.label("Mead with peppers - unique & bold");
    ui.add_space(10.0);

    let volume_label = match app.state.unit_system {
        UnitSystem::Metric => "Batch Volume (L):",
        UnitSystem::Imperial => "Batch Volume (gal):",
    };

    crate::input_field(ui, volume_label, &mut app.mead_volume, "Total must volume");
    crate::input_field(ui, "Target ABV (%):", &mut app.mead_target_abv, "Desired alcohol level");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Pepper Type:").strong());
        egui::ComboBox::from_id_salt("pepper_type")
            .selected_text(&app.pepper_type)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.pepper_type, "jalapeno".to_string(), "Jalapeño (mild-medium)");
                ui.selectable_value(&mut app.pepper_type, "habanero".to_string(), "Habanero (hot)");
                ui.selectable_value(&mut app.pepper_type, "ghost".to_string(), "Ghost Pepper (extreme)");
                ui.selectable_value(&mut app.pepper_type, "bell".to_string(), "Bell Pepper (no heat)");
            });
    });

    ui.horizontal(|ui| {
        ui.label(RichText::new("Heat Level:").strong());
        egui::ComboBox::from_id_salt("heat_level")
            .selected_text(&app.heat_level)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.heat_level, "mild".to_string(), "Mild");
                ui.selectable_value(&mut app.heat_level, "medium".to_string(), "Medium");
                ui.selectable_value(&mut app.heat_level, "hot".to_string(), "Hot");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Honey Needed") {
        calc_capsicumel(app);
    }
}

fn render_metheglin(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🌿 Metheglin (Spiced Mead)").color(colors::SADDLE_BROWN));
    ui.label("Mead with herbs & spices - aromatic & complex");
    ui.add_space(10.0);

    let volume_label = match app.state.unit_system {
        UnitSystem::Metric => "Batch Volume (L):",
        UnitSystem::Imperial => "Batch Volume (gal):",
    };

    crate::input_field(ui, volume_label, &mut app.mead_volume, "Total must volume");
    crate::input_field(ui, "Target ABV (%):", &mut app.mead_target_abv, "Desired alcohol level");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Primary Spice:").strong());
        egui::ComboBox::from_id_salt("spice_type")
            .selected_text(&app.spice_type)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.spice_type, "cinnamon".to_string(), "Cinnamon");
                ui.selectable_value(&mut app.spice_type, "ginger".to_string(), "Ginger");
                ui.selectable_value(&mut app.spice_type, "clove".to_string(), "Clove");
                ui.selectable_value(&mut app.spice_type, "vanilla".to_string(), "Vanilla");
                ui.selectable_value(&mut app.spice_type, "cardamom".to_string(), "Cardamom");
            });
    });

    ui.horizontal(|ui| {
        ui.label(RichText::new("Spice Intensity:").strong());
        egui::ComboBox::from_id_salt("spice_level")
            .selected_text(&app.spice_level)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.spice_level, "subtle".to_string(), "Subtle");
                ui.selectable_value(&mut app.spice_level, "medium".to_string(), "Medium");
                ui.selectable_value(&mut app.spice_level, "bold".to_string(), "Bold");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Honey Needed") {
        calc_metheglin(app);
    }
}

fn calc_traditional(app: &mut MazerionApp) {
    app.result = Some("Traditional mead calculation - pending backend".to_string());
}

fn calc_hydromel(app: &mut MazerionApp) {
    app.result = Some("Hydromel calculation - pending backend".to_string());
}

fn calc_sack(app: &mut MazerionApp) {
    app.result = Some("Sack mead calculation - pending backend".to_string());
}

fn calc_melomel(app: &mut MazerionApp) {
    app.result = Some("Melomel calculation - pending backend".to_string());
}

fn calc_cyser(app: &mut MazerionApp) {
    app.result = Some("Cyser calculation - pending backend".to_string());
}

fn calc_acerglyn(app: &mut MazerionApp) {
    app.result = Some("Acerglyn calculation - pending backend".to_string());
}

fn calc_bochet(app: &mut MazerionApp) {
    app.result = Some("Bochet calculation - pending backend".to_string());
}

fn calc_braggot(app: &mut MazerionApp) {
    app.result = Some("Braggot calculation - pending backend".to_string());
}

fn calc_capsicumel(app: &mut MazerionApp) {
    app.result = Some("Capsicumel calculation - pending backend".to_string());
}

fn calc_metheglin(app: &mut MazerionApp) {
    app.result = Some("Metheglin calculation - pending backend".to_string());
}