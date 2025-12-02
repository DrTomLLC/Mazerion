//! Settings tab with working theme selector

use crate::{MazerionApp, Theme, state::UnitSystem};
use eframe::egui::{self, RichText, CornerRadius, Color32, Slider};

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    egui::Frame::new()
        .fill(Color32::WHITE)
        .stroke(egui::Stroke::new(2.0, Color32::from_rgb(240, 165, 0)))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.heading(RichText::new("⚙️ Settings").color(Color32::BLACK).size(24.0));
            ui.add_space(15.0);

            ui.label(RichText::new("Theme Selection").color(Color32::BLACK).size(18.0).strong());
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label(RichText::new("Choose Theme:").strong().color(Color32::BLACK));
                egui::ComboBox::from_id_salt("theme_selector")
                    .selected_text(app.state.theme.name())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.state.theme, Theme::HoneyGold, Theme::HoneyGold.name());
                        ui.selectable_value(&mut app.state.theme, Theme::ForestGreen, Theme::ForestGreen.name());
                        ui.selectable_value(&mut app.state.theme, Theme::OceanBlue, Theme::OceanBlue.name());
                        ui.selectable_value(&mut app.state.theme, Theme::SunsetOrange, Theme::SunsetOrange.name());
                        ui.selectable_value(&mut app.state.theme, Theme::LavenderPurple, Theme::LavenderPurple.name());
                    });
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(15.0);

            ui.label(RichText::new("Unit System").color(Color32::BLACK).size(18.0).strong());
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label(RichText::new("Units:").strong().color(Color32::BLACK));
                egui::ComboBox::from_id_salt("unit_system")
                    .selected_text(app.state.unit_system.name())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.state.unit_system, UnitSystem::Imperial, UnitSystem::Imperial.name());
                        ui.selectable_value(&mut app.state.unit_system, UnitSystem::Metric, UnitSystem::Metric.name());
                    });
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(15.0);

            ui.label(RichText::new("Decimal Accuracy").color(Color32::BLACK).size(18.0).strong());
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label(RichText::new("Specific Gravity:").strong().color(Color32::BLACK));
                ui.add(Slider::new(&mut app.state.sg_decimals, 1..=6).text("decimals"));
            });

            ui.horizontal(|ui| {
                ui.label(RichText::new("pH:").strong().color(Color32::BLACK));
                ui.add(Slider::new(&mut app.state.ph_decimals, 1..=4).text("decimals"));
            });

            ui.horizontal(|ui| {
                ui.label(RichText::new("Brix/Plato:").strong().color(Color32::BLACK));
                ui.add(Slider::new(&mut app.state.brix_decimals, 0..=4).text("decimals"));
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);

            ui.label(RichText::new("About Mazerion").color(Color32::BLACK).size(18.0).strong());
            ui.label(RichText::new("Version: 0.7.0").color(Color32::BLACK).size(14.0));
            ui.label(RichText::new("39 calculators ready").color(Color32::BLACK).size(14.0));
        });
}