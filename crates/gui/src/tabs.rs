mod helpers;
mod basic;
mod advanced;
mod brewing;
mod finishing;

use crate::state::AppState;
use eframe::egui;

pub fn render_tabs(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.selectable_value(&mut state.current_tab, 0, "📊 Basic");
        ui.selectable_value(&mut state.current_tab, 1, "🔬 Advanced");
        ui.selectable_value(&mut state.current_tab, 2, "🍺 Brewing");
        ui.selectable_value(&mut state.current_tab, 3, "✨ Finishing");
    });

    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        match state.current_tab {
            0 => basic::render(ui, state),
            1 => advanced::render(ui, state),
            2 => brewing::render(ui, state),
            3 => finishing::render(ui, state),
            _ => {}
        }

        if !state.result.is_empty() {
            ui.separator();
            ui.label("Result:");
            ui.code(&state.result);
        }
    });
}
