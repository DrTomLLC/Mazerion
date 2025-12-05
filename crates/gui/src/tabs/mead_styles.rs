//! Mead style calculators - 10 styles with dynamic labels
//! SAFETY-CRITICAL: All calculations production-ready with unit conversions

use crate::{MazerionApp, state::{colors, UnitSystem}};
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
    crate::input_field(ui, "Apple Juice %:", &mut app.juice_percent, "Percentage of volume as juice (30-50% typical)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Honey + Juice") {
        calc_cyser(app);
    }
}

fn render_acerglyn(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍁 Acerglyn (Maple Mead)").color(colors::SADDLE_BROWN));
    ui.label("Mead with maple syrup");
    ui.add_space(10.0);

    let volume_label = match app.state.unit_system {
        UnitSystem::Metric => "Batch Volume (L):",
        UnitSystem::Imperial => "Batch Volume (gal):",
    };

    crate::input_field(ui, volume_label, &mut app.mead_volume, "Total must volume");
    crate::input_field(ui, "Target ABV (%):", &mut app.mead_target_abv, "Desired alcohol level");
    crate::input_field(ui, "Maple Syrup %:", &mut app.maple_percent, "Percentage of fermentables as maple (20-40% typical)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Honey + Maple") {
        calc_acerglyn(app);
    }
}

fn render_bochet(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🔥 Bochet (Caramelized Honey)").color(colors::SADDLE_BROWN));
    ui.label("Mead with caramelized honey - accounts for sugar loss");
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
                ui.selectable_value(&mut app.bochet_level, "light".to_string(), "Light (golden, 5% loss)");
                ui.selectable_value(&mut app.bochet_level, "medium".to_string(), "Medium (amber, 10% loss)");
                ui.selectable_value(&mut app.bochet_level, "dark".to_string(), "Dark (deep brown, 15% loss)");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Honey (Pre-Caramelization)") {
        calc_bochet(app);
    }
}

fn render_braggot(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍺 Braggot (Honey-Malt Hybrid)").color(colors::SADDLE_BROWN));
    ui.label("Hybrid of mead and beer");
    ui.add_space(10.0);

    let (volume_label, weight_label) = match app.state.unit_system {
        UnitSystem::Metric => ("Batch Volume (L):", "Malt Weight (kg):"),
        UnitSystem::Imperial => ("Batch Volume (gal):", "Malt Weight (lb):"),
    };

    crate::input_field(ui, volume_label, &mut app.mead_volume, "Total must volume");
    crate::input_field(ui, "Target ABV (%):", &mut app.mead_target_abv, "Desired alcohol level");
    crate::input_field(ui, "Honey %:", &mut app.honey_percent, "Percentage of fermentables as honey (30-70% typical)");
    crate::input_field(ui, weight_label, &mut app.malt_weight, "Malt contribution");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Honey + Malt") {
        calc_braggot(app);
    }
}

fn render_capsicumel(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🌶️ Capsicumel (Pepper Mead)").color(colors::SADDLE_BROWN));
    ui.label("Mead with peppers - adds heat");
    ui.add_space(10.0);

    let volume_label = match app.state.unit_system {
        UnitSystem::Metric => "Batch Volume (L):",
        UnitSystem::Imperial => "Batch Volume (gal):",
    };

    crate::input_field(ui, volume_label, &mut app.mead_volume, "Total must volume");
    crate::input_field(ui, "Target ABV (%):", &mut app.mead_target_abv, "Desired alcohol level");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Heat Level:").strong());
        egui::ComboBox::from_id_salt("heat_level")
            .selected_text(&app.heat_level)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.heat_level, "mild".to_string(), "Mild (0.5 g/L pepper)");
                ui.selectable_value(&mut app.heat_level, "medium".to_string(), "Medium (1.0 g/L pepper)");
                ui.selectable_value(&mut app.heat_level, "hot".to_string(), "Hot (1.5 g/L pepper)");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Honey + Pepper") {
        calc_capsicumel(app);
    }
}

fn render_metheglin(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🌿 Metheglin (Spiced Mead)").color(colors::SADDLE_BROWN));
    ui.label("Mead with herbs and spices");
    ui.add_space(10.0);

    let volume_label = match app.state.unit_system {
        UnitSystem::Metric => "Batch Volume (L):",
        UnitSystem::Imperial => "Batch Volume (gal):",
    };

    crate::input_field(ui, volume_label, &mut app.mead_volume, "Total must volume");
    crate::input_field(ui, "Target ABV (%):", &mut app.mead_target_abv, "Desired alcohol level");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Spice Intensity:").strong());
        egui::ComboBox::from_id_salt("spice_level")
            .selected_text(&app.spice_level)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.spice_level, "light".to_string(), "Light (0.5 g/L spice)");
                ui.selectable_value(&mut app.spice_level, "medium".to_string(), "Medium (1.0 g/L spice)");
                ui.selectable_value(&mut app.spice_level, "heavy".to_string(), "Heavy (2.0 g/L spice)");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Honey + Spices") {
        calc_metheglin(app);
    }
}

// CALCULATION FUNCTIONS

fn calc_traditional(app: &mut MazerionApp) {
    let volume_str = &app.mead_volume;
    let volume_l = match app.state.unit_system {
        UnitSystem::Metric => match Decimal::from_str(volume_str) {
            Ok(v) => v,
            Err(_) => {
                app.result = Some("Error: Invalid volume".to_string());
                return;
            }
        },
        UnitSystem::Imperial => {
            let gal = match Decimal::from_str(volume_str) {
                Ok(v) => v,
                Err(_) => {
                    app.result = Some("Error: Invalid volume".to_string());
                    return;
                }
            };
            mazerion_core::gallons_to_liters(gal)
        }
    };

    let abv: Decimal = match Decimal::from_str(&app.mead_target_abv) {
        Ok(a) => a,
        Err(_) => {
            app.result = Some("Error: Invalid ABV".to_string());
            return;
        }
    };

    let honey_g = volume_l * abv * Decimal::from(33);  // FIXED: 135 → 33
    let honey_kg = honey_g / Decimal::from(1000);

    let (honey_display, unit) = match app.state.unit_system {
        UnitSystem::Metric => (format!("{:.2}", honey_kg), "kg"),
        UnitSystem::Imperial => {
            let lb = mazerion_core::kilograms_to_pounds(honey_kg);
            (format!("{:.2}", lb), "lb")
        }
    };

    app.result = Some(format!("Honey Needed: {} {}", honey_display, unit));
    app.warnings.clear();
    app.metadata.clear();

    // FIXED: Show correct units based on user selection
    let volume_display = match app.state.unit_system {
        UnitSystem::Metric => format!("{:.2} L", volume_l),
        UnitSystem::Imperial => {
            let gal = mazerion_core::liters_to_gallons(volume_l);
            format!("{:.2} gal", gal)
        }
    };
    app.metadata.push(("Volume".to_string(), volume_display));
    app.metadata.push(("Target ABV".to_string(), format!("{:.1}%", abv)));
}

fn calc_hydromel(app: &mut MazerionApp) {
    let volume_str = &app.mead_volume;
    let volume_l = match app.state.unit_system {
        UnitSystem::Metric => match Decimal::from_str(volume_str) {
            Ok(v) => v,
            Err(_) => {
                app.result = Some("Error: Invalid volume".to_string());
                return;
            }
        },
        UnitSystem::Imperial => {
            let gal = match Decimal::from_str(volume_str) {
                Ok(v) => v,
                Err(_) => {
                    app.result = Some("Error: Invalid volume".to_string());
                    return;
                }
            };
            mazerion_core::gallons_to_liters(gal)
        }
    };

    let abv: Decimal = match Decimal::from_str(&app.mead_target_abv) {
        Ok(a) => a,
        Err(_) => {
            app.result = Some("Error: Invalid ABV".to_string());
            return;
        }
    };

    app.warnings.clear();
    if abv > Decimal::from(8) {
        app.warnings.push("⚠️ ABV above typical hydromel range (3.5-7.5%)".to_string());
    }

    let honey_g = volume_l * abv * Decimal::from(33);  // FIXED: 135 → 33
    let honey_kg = honey_g / Decimal::from(1000);

    let (honey_display, unit) = match app.state.unit_system {
        UnitSystem::Metric => (format!("{:.2}", honey_kg), "kg"),
        UnitSystem::Imperial => {
            let lb = mazerion_core::kilograms_to_pounds(honey_kg);
            (format!("{:.2}", lb), "lb")
        }
    };

    app.result = Some(format!("Honey Needed: {} {}", honey_display, unit));
    app.metadata.clear();
    app.metadata.push(("Style".to_string(), "Session/Hydromel".to_string()));
    app.metadata.push(("Target ABV".to_string(), format!("{:.1}%", abv)));
}

fn calc_sack(app: &mut MazerionApp) {
    let volume_str = &app.mead_volume;
    let volume_l = match app.state.unit_system {
        UnitSystem::Metric => match Decimal::from_str(volume_str) {
            Ok(v) => v,
            Err(_) => {
                app.result = Some("Error: Invalid volume".to_string());
                return;
            }
        },
        UnitSystem::Imperial => {
            let gal = match Decimal::from_str(volume_str) {
                Ok(v) => v,
                Err(_) => {
                    app.result = Some("Error: Invalid volume".to_string());
                    return;
                }
            };
            mazerion_core::gallons_to_liters(gal)
        }
    };

    let abv: Decimal = match Decimal::from_str(&app.mead_target_abv) {
        Ok(a) => a,
        Err(_) => {
            app.result = Some("Error: Invalid ABV".to_string());
            return;
        }
    };

    app.warnings.clear();
    if abv < Decimal::from(14) {
        app.warnings.push("⚠️ ABV below typical sack range (14-18%)".to_string());
    }
    if abv > Decimal::from(18) {
        app.warnings.push("⚠️ Very high ABV - consider high-tolerance yeast (EC-1118, K1-V1116)".to_string());
    }

    let honey_g = volume_l * abv * Decimal::from(33);  // FIXED: 135 → 33
    let honey_kg = honey_g / Decimal::from(1000);

    let (honey_display, unit) = match app.state.unit_system {
        UnitSystem::Metric => (format!("{:.2}", honey_kg), "kg"),
        UnitSystem::Imperial => {
            let lb = mazerion_core::kilograms_to_pounds(honey_kg);
            (format!("{:.2}", lb), "lb")
        }
    };

    app.result = Some(format!("Honey Needed: {} {}", honey_display, unit));
    app.metadata.clear();
    app.metadata.push(("Style".to_string(), "Sack/High Gravity".to_string()));
    app.metadata.push(("Target ABV".to_string(), format!("{:.1}%", abv)));
}

fn calc_melomel(app: &mut MazerionApp) {
    let volume_str = &app.mead_volume;
    let volume_l = match app.state.unit_system {
        UnitSystem::Metric => match Decimal::from_str(volume_str) {
            Ok(v) => v,
            Err(_) => {
                app.result = Some("Error: Invalid volume".to_string());
                return;
            }
        },
        UnitSystem::Imperial => {
            let gal = match Decimal::from_str(volume_str) {
                Ok(v) => v,
                Err(_) => {
                    app.result = Some("Error: Invalid volume".to_string());
                    return;
                }
            };
            mazerion_core::gallons_to_liters(gal)
        }
    };

    let abv: Decimal = match Decimal::from_str(&app.mead_target_abv) {
        Ok(a) => a,
        Err(_) => {
            app.result = Some("Error: Invalid ABV".to_string());
            return;
        }
    };

    let fruit_weight: Decimal = match Decimal::from_str(&app.fruit_weight) {
        Ok(w) => w,
        Err(_) => {
            app.result = Some("Error: Invalid fruit weight".to_string());
            return;
        }
    };

    let fruit_weight_kg = match app.state.unit_system {
        UnitSystem::Metric => fruit_weight,
        UnitSystem::Imperial => mazerion_core::pounds_to_kilograms(fruit_weight),
    };

    let fruit_sugar_pct = match app.fruit_type.as_str() {
        "strawberry" => Decimal::from(8),
        "blueberry" => Decimal::from(10),
        "raspberry" => Decimal::from(5),
        "cherry" => Decimal::from(12),
        "blackberry" => Decimal::from(9),
        _ => Decimal::from(8),
    };

    // FIXED: Proper unit conversion kg → g before ABV calculation
    let fruit_sugar_kg = fruit_weight_kg * fruit_sugar_pct / Decimal::from(100);
    let fruit_sugar_g = fruit_sugar_kg * Decimal::from(1000);
    let fruit_abv = fruit_sugar_g / (volume_l * Decimal::from(33));  // FIXED: 135 → 33

    let remaining_abv = if abv > fruit_abv { abv - fruit_abv } else { Decimal::ZERO };
    let honey_g = volume_l * remaining_abv * Decimal::from(33);  // Already correct
    let honey_kg = honey_g / Decimal::from(1000);

    let (honey_display, fruit_display, h_unit, f_unit) = match app.state.unit_system {
        UnitSystem::Metric => (
            format!("{:.2}", honey_kg),
            format!("{:.2}", fruit_weight_kg),
            "kg",
            "kg",
        ),
        UnitSystem::Imperial => (
            format!("{:.2}", mazerion_core::kilograms_to_pounds(honey_kg)),
            format!("{:.2}", fruit_weight),
            "lb",
            "lb",
        )
    };

    app.result = Some(format!("Honey: {} {} | Fruit: {} {}", honey_display, h_unit, fruit_display, f_unit));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("Fruit Type".to_string(), app.fruit_type.clone()));
    app.metadata.push(("Fruit ABV".to_string(), format!("{:.1}%", fruit_abv)));
    app.metadata.push(("Honey ABV".to_string(), format!("{:.1}%", remaining_abv)));
}

fn calc_cyser(app: &mut MazerionApp) {
    let volume_str = &app.mead_volume;
    let volume_l = match app.state.unit_system {
        UnitSystem::Metric => match Decimal::from_str(volume_str) {
            Ok(v) => v,
            Err(_) => {
                app.result = Some("Error: Invalid volume".to_string());
                return;
            }
        },
        UnitSystem::Imperial => {
            let gal = match Decimal::from_str(volume_str) {
                Ok(v) => v,
                Err(_) => {
                    app.result = Some("Error: Invalid volume".to_string());
                    return;
                }
            };
            mazerion_core::gallons_to_liters(gal)
        }
    };

    let abv: Decimal = match Decimal::from_str(&app.mead_target_abv) {
        Ok(a) => a,
        Err(_) => {
            app.result = Some("Error: Invalid ABV".to_string());
            return;
        }
    };

    let juice_pct: Decimal = match Decimal::from_str(&app.juice_percent) {
        Ok(p) => p,
        Err(_) => {
            app.result = Some("Error: Invalid juice percentage".to_string());
            return;
        }
    };

    let juice_l = volume_l * juice_pct / Decimal::from(100);
    // FIXED: Proper parentheses for division order
    let juice_sugar_g = juice_l * Decimal::new(104, 0);
    let juice_abv = juice_sugar_g / (volume_l * Decimal::from(33));  // FIXED: 135 → 33, proper parentheses

    let remaining_abv = if abv > juice_abv { abv - juice_abv } else { Decimal::ZERO };
    let honey_g = volume_l * remaining_abv * Decimal::from(33);  // FIXED: 135 → 33
    let honey_kg = honey_g / Decimal::from(1000);

    let (honey_display, juice_display, h_unit, j_unit) = match app.state.unit_system {
        UnitSystem::Metric => (
            format!("{:.2}", honey_kg),
            format!("{:.2}", juice_l),
            "kg",
            "L",
        ),
        UnitSystem::Imperial => {
            let h_lb = mazerion_core::kilograms_to_pounds(honey_kg);
            let j_gal = mazerion_core::liters_to_gallons(juice_l);
            (format!("{:.2}", h_lb), format!("{:.2}", j_gal), "lb", "gal")
        }
    };

    app.result = Some(format!("Honey: {} {} | Apple Juice: {} {}", honey_display, h_unit, juice_display, j_unit));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("Juice Contribution".to_string(), format!("{:.1}% ABV", juice_abv)));
    app.metadata.push(("Honey Contribution".to_string(), format!("{:.1}% ABV", remaining_abv)));
}

fn calc_acerglyn(app: &mut MazerionApp) {
    let volume_str = &app.mead_volume;
    let volume_l = match app.state.unit_system {
        UnitSystem::Metric => match Decimal::from_str(volume_str) {
            Ok(v) => v,
            Err(_) => {
                app.result = Some("Error: Invalid volume".to_string());
                return;
            }
        },
        UnitSystem::Imperial => {
            let gal = match Decimal::from_str(volume_str) {
                Ok(v) => v,
                Err(_) => {
                    app.result = Some("Error: Invalid volume".to_string());
                    return;
                }
            };
            mazerion_core::gallons_to_liters(gal)
        }
    };

    let abv: Decimal = match Decimal::from_str(&app.mead_target_abv) {
        Ok(a) => a,
        Err(_) => {
            app.result = Some("Error: Invalid ABV".to_string());
            return;
        }
    };

    let maple_pct: Decimal = match Decimal::from_str(&app.maple_percent) {
        Ok(p) => p,
        Err(_) => {
            app.result = Some("Error: Invalid maple percentage".to_string());
            return;
        }
    };

    let maple_abv = abv * maple_pct / Decimal::from(100);
    let honey_abv = abv - maple_abv;

    let maple_g = volume_l * maple_abv * Decimal::from(40);   // FIXED: 165 → 40
    let honey_g = volume_l * honey_abv * Decimal::from(33);   // FIXED: 135 → 33

    let maple_kg = maple_g / Decimal::from(1000);
    let honey_kg = honey_g / Decimal::from(1000);

    let (honey_display, maple_display, unit) = match app.state.unit_system {
        UnitSystem::Metric => (
            format!("{:.2}", honey_kg),
            format!("{:.2}", maple_kg),
            "kg",
        ),
        UnitSystem::Imperial => (
            format!("{:.2}", mazerion_core::kilograms_to_pounds(honey_kg)),
            format!("{:.2}", mazerion_core::kilograms_to_pounds(maple_kg)),
            "lb",
        )
    };

    app.result = Some(format!("Honey: {} {} | Maple Syrup: {} {}", honey_display, unit, maple_display, unit));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("Honey ABV".to_string(), format!("{:.1}%", honey_abv)));
    app.metadata.push(("Maple ABV".to_string(), format!("{:.1}%", maple_abv)));
}

fn calc_bochet(app: &mut MazerionApp) {
    let volume_str = &app.mead_volume;
    let volume_l = match app.state.unit_system {
        UnitSystem::Metric => match Decimal::from_str(volume_str) {
            Ok(v) => v,
            Err(_) => {
                app.result = Some("Error: Invalid volume".to_string());
                return;
            }
        },
        UnitSystem::Imperial => {
            let gal = match Decimal::from_str(volume_str) {
                Ok(v) => v,
                Err(_) => {
                    app.result = Some("Error: Invalid volume".to_string());
                    return;
                }
            };
            mazerion_core::gallons_to_liters(gal)
        }
    };

    let abv: Decimal = match Decimal::from_str(&app.mead_target_abv) {
        Ok(a) => a,
        Err(_) => {
            app.result = Some("Error: Invalid ABV".to_string());
            return;
        }
    };

    let loss_factor = match app.bochet_level.as_str() {
        "light" => Decimal::new(95, 2),
        "medium" => Decimal::new(90, 2),
        "dark" => Decimal::new(85, 2),
        _ => Decimal::new(90, 2),
    };

    let honey_g = volume_l * abv * Decimal::from(33) / loss_factor;  // FIXED: 135 → 33
    let honey_kg = honey_g / Decimal::from(1000);

    let (honey_display, unit) = match app.state.unit_system {
        UnitSystem::Metric => (format!("{:.2}", honey_kg), "kg"),
        UnitSystem::Imperial => {
            let lb = mazerion_core::kilograms_to_pounds(honey_kg);
            (format!("{:.2}", lb), "lb")
        }
    };

    app.result = Some(format!("Honey (Pre-Caramelization): {} {}", honey_display, unit));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("Caramelization".to_string(), app.bochet_level.clone()));
    app.metadata.push(("Sugar Loss".to_string(), format!("{:.0}%", (Decimal::ONE - loss_factor) * Decimal::from(100))));
}

fn calc_braggot(app: &mut MazerionApp) {
    let volume_str = &app.mead_volume;
    let volume_l = match app.state.unit_system {
        UnitSystem::Metric => match Decimal::from_str(volume_str) {
            Ok(v) => v,
            Err(_) => {
                app.result = Some("Error: Invalid volume".to_string());
                return;
            }
        },
        UnitSystem::Imperial => {
            let gal = match Decimal::from_str(volume_str) {
                Ok(v) => v,
                Err(_) => {
                    app.result = Some("Error: Invalid volume".to_string());
                    return;
                }
            };
            mazerion_core::gallons_to_liters(gal)
        }
    };

    let abv: Decimal = match Decimal::from_str(&app.mead_target_abv) {
        Ok(a) => a,
        Err(_) => {
            app.result = Some("Error: Invalid ABV".to_string());
            return;
        }
    };

    let honey_pct: Decimal = match Decimal::from_str(&app.honey_percent) {
        Ok(p) => p,
        Err(_) => {
            app.result = Some("Error: Invalid honey percentage".to_string());
            return;
        }
    };

    let honey_abv = abv * honey_pct / Decimal::from(100);
    let malt_abv = abv - honey_abv;

    let honey_g = volume_l * honey_abv * Decimal::from(33);  // FIXED: 135 → 33
    let malt_g = volume_l * malt_abv * Decimal::from(34);    // FIXED: 140 → 34

    let honey_kg = honey_g / Decimal::from(1000);
    let malt_kg = malt_g / Decimal::from(1000);

    let (honey_display, malt_display, unit) = match app.state.unit_system {
        UnitSystem::Metric => (
            format!("{:.2}", honey_kg),
            format!("{:.2}", malt_kg),
            "kg",
        ),
        UnitSystem::Imperial => (
            format!("{:.2}", mazerion_core::kilograms_to_pounds(honey_kg)),
            format!("{:.2}", mazerion_core::kilograms_to_pounds(malt_kg)),
            "lb",
        )
    };

    app.result = Some(format!("Honey: {} {} | Malt: {} {}", honey_display, unit, malt_display, unit));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("Honey ABV".to_string(), format!("{:.1}%", honey_abv)));
    app.metadata.push(("Malt ABV".to_string(), format!("{:.1}%", malt_abv)));
}

fn calc_capsicumel(app: &mut MazerionApp) {
    let volume_str = &app.mead_volume;
    let volume_l = match app.state.unit_system {
        UnitSystem::Metric => match Decimal::from_str(volume_str) {
            Ok(v) => v,
            Err(_) => {
                app.result = Some("Error: Invalid volume".to_string());
                return;
            }
        },
        UnitSystem::Imperial => {
            let gal = match Decimal::from_str(volume_str) {
                Ok(v) => v,
                Err(_) => {
                    app.result = Some("Error: Invalid volume".to_string());
                    return;
                }
            };
            mazerion_core::gallons_to_liters(gal)
        }
    };

    let abv: Decimal = match Decimal::from_str(&app.mead_target_abv) {
        Ok(a) => a,
        Err(_) => {
            app.result = Some("Error: Invalid ABV".to_string());
            return;
        }
    };

    let pepper_rate = match app.heat_level.as_str() {
        "mild" => Decimal::new(5, 1),
        "medium" => Decimal::ONE,
        "hot" => Decimal::new(15, 1),
        _ => Decimal::ONE,
    };

    let honey_g = volume_l * abv * Decimal::from(33);  // FIXED: 135 → 33
    let honey_kg = honey_g / Decimal::from(1000);
    let pepper_g = volume_l * pepper_rate;

    let (honey_display, pepper_display, h_unit, p_unit) = match app.state.unit_system {
        UnitSystem::Metric => (
            format!("{:.2}", honey_kg),
            format!("{:.1}", pepper_g),
            "kg",
            "g",
        ),
        UnitSystem::Imperial => {
            let h_lb = mazerion_core::kilograms_to_pounds(honey_kg);
            let p_oz = mazerion_core::grams_to_ounces(pepper_g);
            (format!("{:.2}", h_lb), format!("{:.2}", p_oz), "lb", "oz")
        }
    };

    app.result = Some(format!("Honey: {} {} | Peppers: {} {}", honey_display, h_unit, pepper_display, p_unit));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("Heat Level".to_string(), app.heat_level.clone()));
}

fn calc_metheglin(app: &mut MazerionApp) {
    let volume_str = &app.mead_volume;
    let volume_l = match app.state.unit_system {
        UnitSystem::Metric => match Decimal::from_str(volume_str) {
            Ok(v) => v,
            Err(_) => {
                app.result = Some("Error: Invalid volume".to_string());
                return;
            }
        },
        UnitSystem::Imperial => {
            let gal = match Decimal::from_str(volume_str) {
                Ok(v) => v,
                Err(_) => {
                    app.result = Some("Error: Invalid volume".to_string());
                    return;
                }
            };
            mazerion_core::gallons_to_liters(gal)
        }
    };

    let abv: Decimal = match Decimal::from_str(&app.mead_target_abv) {
        Ok(a) => a,
        Err(_) => {
            app.result = Some("Error: Invalid ABV".to_string());
            return;
        }
    };

    let spice_rate = match app.spice_level.as_str() {
        "light" => Decimal::new(5, 1),
        "medium" => Decimal::ONE,
        "heavy" => Decimal::from(2),
        _ => Decimal::ONE,
    };

    let honey_g = volume_l * abv * Decimal::from(33);  // FIXED: 135 → 33
    let honey_kg = honey_g / Decimal::from(1000);
    let spice_g = volume_l * spice_rate;

    let (honey_display, spice_display, h_unit, s_unit) = match app.state.unit_system {
        UnitSystem::Metric => (
            format!("{:.2}", honey_kg),
            format!("{:.1}", spice_g),
            "kg",
            "g",
        ),
        UnitSystem::Imperial => {
            let h_lb = mazerion_core::kilograms_to_pounds(honey_kg);
            let s_oz = mazerion_core::grams_to_ounces(spice_g);
            (format!("{:.2}", h_lb), format!("{:.2}", s_oz), "lb", "oz")
        }
    };

    app.result = Some(format!("Honey: {} {} | Spices: {} {}", honey_display, h_unit, spice_display, s_unit));
    app.warnings.clear();
    app.metadata.clear();
    app.metadata.push(("Intensity".to_string(), app.spice_level.clone()));
}