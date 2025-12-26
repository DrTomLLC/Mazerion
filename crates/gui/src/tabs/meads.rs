//! Mead Styles tab - WITH ACTUAL FRUIT DROPDOWN

use crate::MazerionApp;
use crate::state::UnitSystem;
use eframe::egui::{self, RichText, CornerRadius};

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    let c = app.state.custom_colors;
    let is_metric = app.state.unit_system == UnitSystem::Metric;
    let vol_unit = if is_metric { "L" } else { "gal" };
    let weight_unit = if is_metric { "kg" } else { "lbs" };

    ui.heading(RichText::new("🍯 Mead Styles & Calculators")
        .size(26.0)
        .color(c.honey_gold));
    ui.add_space(10.0);

    // Style selector dropdown
    ui.horizontal(|ui| {
        ui.label(RichText::new("Select Style:").strong().size(16.0));

        egui::ComboBox::from_label("")
            .selected_text(get_display_name(&app.mead_style))
            .width(300.0)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.mead_style, "encyclopedia".to_string(), "📚 Encyclopedia (All Styles)");
                ui.separator();
                ui.selectable_value(&mut app.mead_style, "traditional".to_string(), "Traditional Mead");
                ui.selectable_value(&mut app.mead_style, "melomel".to_string(), "Melomel (Fruit)");
                ui.selectable_value(&mut app.mead_style, "cyser".to_string(), "Cyser (Apple)");
                ui.selectable_value(&mut app.mead_style, "pyment".to_string(), "Pyment (Grape)");
                ui.selectable_value(&mut app.mead_style, "metheglin".to_string(), "Metheglin (Spiced)");
                ui.selectable_value(&mut app.mead_style, "braggot".to_string(), "Braggot (Honey-Malt)");
                ui.selectable_value(&mut app.mead_style, "bochet".to_string(), "Bochet (Caramelized)");
                ui.selectable_value(&mut app.mead_style, "acerglyn".to_string(), "Acerglyn (Maple)");
                ui.selectable_value(&mut app.mead_style, "capsicumel".to_string(), "Capsicumel (Pepper)");
                ui.selectable_value(&mut app.mead_style, "sack".to_string(), "Sack Mead (Dessert)");
                ui.selectable_value(&mut app.mead_style, "hydromel".to_string(), "Hydromel (Session)");
            });
    });

    ui.add_space(15.0);

    // Show encyclopedia or calculator
    if app.mead_style == "encyclopedia" {
        super::mead_encyclopedia::render(app, ui);
    } else {
        render_calculator(app, ui, c, vol_unit, weight_unit);
    }
}

fn get_display_name(style: &str) -> &str {
    match style {
        "encyclopedia" => "📚 Encyclopedia (All Styles)",
        "traditional" => "Traditional Mead",
        "melomel" => "Melomel (Fruit)",
        "cyser" => "Cyser (Apple)",
        "pyment" => "Pyment (Grape)",
        "metheglin" => "Metheglin (Spiced)",
        "braggot" => "Braggot (Honey-Malt)",
        "bochet" => "Bochet (Caramelized)",
        "acerglyn" => "Acerglyn (Maple)",
        "capsicumel" => "Capsicumel (Pepper)",
        "sack" => "Sack Mead (Dessert)",
        "hydromel" => "Hydromel (Session)",
        _ => "Select Style",
    }
}

fn render_calculator(app: &mut MazerionApp, ui: &mut egui::Ui, c: crate::state::CustomColors, vol_unit: &str, weight_unit: &str) {
    let (title, desc) = match app.mead_style.as_str() {
        "traditional" => ("Traditional Mead", "Pure honey, water, and yeast"),
        "melomel" => ("Melomel", "Fruit-infused mead"),
        "cyser" => ("Cyser", "Apple mead"),
        "pyment" => ("Pyment", "Grape mead"),
        "metheglin" => ("Metheglin", "Spiced or herbal mead"),
        "braggot" => ("Braggot", "Honey and malt hybrid"),
        "bochet" => ("Bochet", "Caramelized honey mead"),
        "acerglyn" => ("Acerglyn", "Maple and honey mead"),
        "capsicumel" => ("Capsicumel", "Pepper-infused mead"),
        "sack" => ("Sack Mead", "High-gravity dessert mead"),
        "hydromel" => ("Hydromel", "Session-strength mead"),
        _ => ("Mead Calculator", "Calculate mead recipe"),
    };

    ui.heading(RichText::new(format!("🍯 {}", title)).size(20.0).color(c.honey_gold));
    ui.label(RichText::new(desc).color(c.dark_text));
    ui.add_space(10.0);

    egui::Frame::default()
        .fill(c.light_cream)
        .stroke(egui::Stroke::new(1.5, c.honey_gold))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            // Common inputs
            crate::input_field(ui, &format!("Batch Volume ({}):", vol_unit), &mut app.mead_volume, "Total batch size");
            crate::input_field(ui, "Target ABV (%):", &mut app.mead_target_abv, "Desired alcohol percentage (e.g., 14)");

            // Style-specific inputs
            match app.mead_style.as_str() {
                "melomel" => {
                    // ACTUAL FRUIT DROPDOWN
                    ui.horizontal(|ui| {
                        ui.label("Fruit Type:");
                        egui::ComboBox::from_label("")
                            .selected_text(&app.fruit_type)
                            .width(200.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut app.fruit_type, "Raspberry".to_string(), "Raspberry");
                                ui.selectable_value(&mut app.fruit_type, "Blueberry".to_string(), "Blueberry");
                                ui.selectable_value(&mut app.fruit_type, "Blackberry".to_string(), "Blackberry");
                                ui.selectable_value(&mut app.fruit_type, "Strawberry".to_string(), "Strawberry");
                                ui.selectable_value(&mut app.fruit_type, "Cherry".to_string(), "Cherry");
                                ui.selectable_value(&mut app.fruit_type, "Peach".to_string(), "Peach");
                                ui.selectable_value(&mut app.fruit_type, "Apricot".to_string(), "Apricot");
                                ui.selectable_value(&mut app.fruit_type, "Plum".to_string(), "Plum");
                                ui.separator();
                                ui.selectable_value(&mut app.fruit_type, "Mango".to_string(), "Mango");
                                ui.selectable_value(&mut app.fruit_type, "Pineapple".to_string(), "Pineapple");
                                ui.selectable_value(&mut app.fruit_type, "Passion Fruit".to_string(), "Passion Fruit");
                                ui.selectable_value(&mut app.fruit_type, "Guava".to_string(), "Guava");
                                ui.separator();
                                ui.selectable_value(&mut app.fruit_type, "Elderberry".to_string(), "Elderberry");
                                ui.selectable_value(&mut app.fruit_type, "Mulberry".to_string(), "Mulberry");
                                ui.selectable_value(&mut app.fruit_type, "Other".to_string(), "Other");
                            });
                    });
                    crate::input_field(ui, &format!("Fruit Weight ({}):", weight_unit), &mut app.fruit_weight, "Weight of fruit");
                }
                "cyser" | "pyment" => {
                    crate::input_field(ui, "Juice Percent (%):", &mut app.juice_percent, "Juice as % of volume (e.g., 50)");
                }
                "acerglyn" => {
                    crate::input_field(ui, "Maple Syrup Percent (%):", &mut app.maple_percent, "Maple as % of fermentables (e.g., 25)");
                }
                "bochet" => {
                    ui.horizontal(|ui| {
                        ui.label("Caramelization Level:");
                        egui::ComboBox::from_label("")
                            .selected_text(&app.bochet_level)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut app.bochet_level, "light".to_string(), "Light (5-10 min)");
                                ui.selectable_value(&mut app.bochet_level, "medium".to_string(), "Medium (15-20 min)");
                                ui.selectable_value(&mut app.bochet_level, "dark".to_string(), "Dark (25-30 min)");
                            });
                    });
                }
                "braggot" => {
                    crate::input_field(ui, "Honey Percent (%):", &mut app.honey_percent, "Honey as % of fermentables (e.g., 50)");
                    crate::input_field(ui, &format!("Malt Weight ({}):", weight_unit), &mut app.malt_weight, "Weight of malted grain");
                }
                "metheglin" => {
                    ui.horizontal(|ui| {
                        ui.label("Spice Intensity:");
                        egui::ComboBox::from_label("")
                            .selected_text(&app.spice_level)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut app.spice_level, "subtle".to_string(), "Subtle");
                                ui.selectable_value(&mut app.spice_level, "moderate".to_string(), "Moderate");
                                ui.selectable_value(&mut app.spice_level, "strong".to_string(), "Strong");
                            });
                    });
                }
                _ => {}
            }

            ui.add_space(10.0);

            if crate::calculate_button(ui, "Calculate Recipe") {
                calculate_mead(app);
            }
        });
}

fn calculate_mead(app: &mut MazerionApp) {
    app.result = None;
    app.warnings.clear();
    app.metadata.clear();

    let recipe = match app.mead_style.as_str() {
        "traditional" => format!(
            "Traditional Mead Recipe:\n\
            Volume: {} gal\n\
            Target ABV: {}%\n\
            \n\
            Estimated honey needed: Calculate using great_mead calculator\n\
            Yeast: Wine yeast (71B-1122 or EC-1118)\n\
            Nutrients: TOSNA 2.0 protocol recommended",
            app.mead_volume, app.mead_target_abv
        ),
        "melomel" => format!(
            "Melomel Recipe:\n\
            Volume: {} gal\n\
            Target ABV: {}%\n\
            Fruit: {} ({} lbs)\n\
            \n\
            Add fruit in secondary for best results\n\
            Typical ratio: 3-4 lbs fruit per gallon",
            app.mead_volume, app.mead_target_abv, app.fruit_type, app.fruit_weight
        ),
        "bochet" => format!(
            "Bochet Recipe:\n\
            Volume: {} gal\n\
            Target ABV: {}%\n\
            Caramelization: {}\n\
            \n\
            CAUTION: Watch honey closely - burns quickly!\n\
            Backsweeten to FG 1.020-1.028",
            app.mead_volume, app.mead_target_abv, app.bochet_level
        ),
        "acerglyn" => format!(
            "Acerglyn Recipe:\n\
            Volume: {} gal\n\
            Target ABV: {}%\n\
            Maple: {}% of fermentables\n\
            \n\
            Use real maple syrup only (Grade A or B)",
            app.mead_volume, app.mead_target_abv, app.maple_percent
        ),
        _ => format!(
            "{} Recipe:\n\
            Volume: {} gal\n\
            Target ABV: {}%\n\
            \n\
            Calculator implementation in progress",
            get_display_name(&app.mead_style), app.mead_volume, app.mead_target_abv
        ),
    };

    app.result = Some(recipe);
}