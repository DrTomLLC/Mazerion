use crate::MazerionApp;
use crate::state::colors;
use eframe::egui::{self, RichText, Rounding};

impl MazerionApp {
    pub fn render_settings_tab(&mut self, ui: &mut egui::Ui) {
        egui::Frame::none()
            .fill(colors::LIGHT_CREAM)
            .stroke(egui::Stroke::new(1.5, colors::HONEY_GOLD))
            .rounding(Rounding::same(8.0 as u8))
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.heading(RichText::new("⚙️ Settings").color(colors::DARK_TEXT).size(24.0));
                ui.add_space(15.0);

                ui.label(RichText::new("About Mazerion").color(colors::DARK_TEXT).size(18.0).strong());
                ui.label(RichText::new("Version: 0.7.0").color(colors::DARK_TEXT).size(14.0));
                ui.label(RichText::new("39 calculators ready").color(colors::DARK_TEXT).size(14.0));

                ui.add_space(20.0);
                ui.label(RichText::new("Theme: Honey & Gold").color(colors::DARK_TEXT).size(14.0));
            });
    }
}