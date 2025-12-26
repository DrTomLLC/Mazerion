//! Settings tab with theme and unit selection
//! ONLY settings - NO unit converter or other functionality

use crate::MazerionApp;
use crate::state::{Theme, UnitSystem};
use eframe::egui::{self, CornerRadius, RichText};

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    egui::Frame::default()
        .fill(crate::state::colors::LIGHT_CREAM)
        .stroke(egui::Stroke::new(1.5, crate::state::colors::HONEY_GOLD))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.heading(RichText::new("⚙️ Settings").color(crate::state::colors::SADDLE_BROWN));
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Color Theme:");
                egui::ComboBox::from_id_salt("theme")
                    .selected_text(app.state.theme.name())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut app.state.theme,
                            Theme::HoneyGold,
                            "🍯 Honey & Gold",
                        );
                        ui.selectable_value(
                            &mut app.state.theme,
                            Theme::ForestGreen,
                            "🌲 Forest Green",
                        );
                        ui.selectable_value(
                            &mut app.state.theme,
                            Theme::OceanBlue,
                            "🌊 Ocean Blue",
                        );
                        ui.selectable_value(
                            &mut app.state.theme,
                            Theme::SunsetOrange,
                            "🌅 Sunset Orange",
                        );
                        ui.selectable_value(
                            &mut app.state.theme,
                            Theme::LavenderPurple,
                            "💜 Lavender Purple",
                        );
                    });
            });
            app.state.custom_colors = app.state.get_theme_colors();

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Unit System:");
                egui::ComboBox::from_id_salt("units")
                    .selected_text(app.state.unit_system.name())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut app.state.unit_system,
                            UnitSystem::Metric,
                            "Metric",
                        );
                        ui.selectable_value(
                            &mut app.state.unit_system,
                            UnitSystem::Imperial,
                            "Imperial/US",
                        );
                    });
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("SG Precision:");
                egui::ComboBox::from_id_salt("sg_precision")
                    .selected_text(format!("{} decimals", app.state.sg_precision))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.state.sg_precision, 3, "3 decimals");
                        ui.selectable_value(&mut app.state.sg_precision, 4, "4 decimals");
                        ui.selectable_value(&mut app.state.sg_precision, 5, "5 decimals");
                    });
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("pH Precision:");
                egui::ComboBox::from_id_salt("ph_precision")
                    .selected_text(format!("{} decimals", app.state.ph_precision))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.state.ph_precision, 2, "2 decimals");
                        ui.selectable_value(&mut app.state.ph_precision, 3, "3 decimals");
                        ui.selectable_value(&mut app.state.ph_precision, 4, "4 decimals");
                    });
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Brix Precision:");
                egui::ComboBox::from_id_salt("brix_precision")
                    .selected_text(format!("{} decimals", app.state.brix_precision))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.state.brix_precision, 1, "1 decimal");
                        ui.selectable_value(&mut app.state.brix_precision, 2, "2 decimals");
                        ui.selectable_value(&mut app.state.brix_precision, 3, "3 decimals");
                    });
            });
        });
}
