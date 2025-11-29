//! Production GUI with calculator-specific interfaces and automatic linking

mod state;
mod tabs;

use eframe::egui::{self, Color32, RichText, CornerRadius, Stroke, Vec2};
use state::AppState;

pub struct MazerionApp {
    pub state: AppState,

    // Input fields
    pub og: String,
    pub fg: String,
    pub brix: String,
    pub sg: String,
    pub temp: String,
    pub current_vol: String,
    pub current_abv: String,
    pub target_abv: String,
    pub vol1: String,
    pub abv1: String,
    pub vol2: String,
    pub abv2: String,
    pub orig_brix: String,
    pub curr_brix: String,
    pub volume: String,
    pub target_abv_brew: String,
    pub yn_requirement: String,
    pub carb_temp: String,
    pub target_co2: String,
    pub carb_method: String,
    pub sugar_type: String,
    pub sweet_vol: String,
    pub current_sg: String,
    pub target_sg: String,
    pub sweetener: String,
    pub sulfite_vol: String,
    pub ph: String,
    pub target_so2: String,
    pub acid_vol: String,
    pub current_ph: String,
    pub target_ph_acid: String,
    pub acid_type: String,

    pub result: Option<String>,
    pub warnings: Vec<String>,
    pub metadata: Vec<(String, String)>,
}

impl Default for MazerionApp {
    fn default() -> Self {
        mazerion_calculators::init();

        Self {
            state: AppState::default(),
            og: "1.090".to_string(),
            fg: "1.010".to_string(),
            brix: "15.0".to_string(),
            sg: "1.060".to_string(),
            temp: "22.0".to_string(),
            current_vol: "19.0".to_string(),
            current_abv: "14.0".to_string(),
            target_abv: "10.0".to_string(),
            vol1: "10.0".to_string(),
            abv1: "14.0".to_string(),
            vol2: "5.0".to_string(),
            abv2: "8.0".to_string(),
            orig_brix: "24.0".to_string(),
            curr_brix: "8.0".to_string(),
            volume: "19.0".to_string(),
            target_abv_brew: "14.0".to_string(),
            yn_requirement: "medium".to_string(),
            carb_temp: "20.0".to_string(),
            target_co2: "2.5".to_string(),
            carb_method: "priming".to_string(),
            sugar_type: "table_sugar".to_string(),
            sweet_vol: "19.0".to_string(),
            current_sg: "0.995".to_string(),
            target_sg: "1.010".to_string(),
            sweetener: "honey".to_string(),
            sulfite_vol: "19.0".to_string(),
            ph: "3.4".to_string(),
            target_so2: "30.0".to_string(),
            acid_vol: "19.0".to_string(),
            current_ph: "3.8".to_string(),
            target_ph_acid: "3.4".to_string(),
            acid_type: "tartaric".to_string(),
            result: None,
            warnings: Vec::new(),
            metadata: Vec::new(),
        }
    }
}

impl eframe::App for MazerionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_custom_style(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_header(ui);
            ui.add_space(10.0);
            self.render_tabs(ui);
            ui.add_space(10.0);

            match self.state.current_tab {
                state::TabView::Basic => tabs::basic::render(self, ui),
                state::TabView::Advanced => tabs::advanced::render(self, ui),
                state::TabView::Brewing => tabs::brewing::render(self, ui),
                state::TabView::Finishing => tabs::finishing::render(self, ui),
            }

            ui.add_space(10.0);
            self.render_results(ui);
        });
    }
}

impl MazerionApp {
    fn apply_custom_style(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = state::colors::CORNSILK;
        style.visuals.panel_fill = state::colors::LIGHT_CREAM;
        style.visuals.widgets.noninteractive.bg_fill = state::colors::LIGHT_CREAM;
        style.visuals.widgets.inactive.bg_fill = state::colors::LIGHT_CREAM;
        style.visuals.widgets.hovered.bg_fill = state::colors::GOLDENROD;
        style.visuals.widgets.active.bg_fill = state::colors::HONEY_GOLD;
        ctx.set_style(style);
    }

    fn render_header(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading(RichText::new("🍯 Mazerion")
                .size(32.0)
                .color(state::colors::SADDLE_BROWN)
                .strong());
            ui.label(RichText::new("Professional Beverage Calculator Suite")
                .size(16.0)
                .color(state::colors::GOLDENROD));
        });
    }

    fn render_tabs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 5.0;

            if self.tab_button(ui, "📊 Basic", self.state.current_tab == state::TabView::Basic).clicked() {
                self.state.current_tab = state::TabView::Basic;
                self.clear_results();
            }

            if self.tab_button(ui, "🔬 Advanced", self.state.current_tab == state::TabView::Advanced).clicked() {
                self.state.current_tab = state::TabView::Advanced;
                self.clear_results();
            }

            if self.tab_button(ui, "🍺 Brewing", self.state.current_tab == state::TabView::Brewing).clicked() {
                self.state.current_tab = state::TabView::Brewing;
                self.clear_results();
            }

            if self.tab_button(ui, "✨ Finishing", self.state.current_tab == state::TabView::Finishing).clicked() {
                self.state.current_tab = state::TabView::Finishing;
                self.clear_results();
            }
        });
    }

    fn tab_button(&self, ui: &mut egui::Ui, text: &str, active: bool) -> egui::Response {
        let color = if active { state::colors::HONEY_GOLD } else { state::colors::LIGHT_CREAM };
        let text_color = if active { Color32::WHITE } else { state::colors::SADDLE_BROWN };
        let button = egui::Button::new(RichText::new(text).color(text_color).size(14.0))
            .fill(color)
            .corner_radius(CornerRadius::same(5))
            .min_size(Vec2::new(120.0, 35.0));
        ui.add(button)
    }

    fn clear_results(&mut self) {
        self.result = None;
        self.warnings.clear();
        self.metadata.clear();
    }

    fn render_results(&self, ui: &mut egui::Ui) {
        if self.result.is_none() && self.warnings.is_empty() {
            return;
        }

        ui.separator();
        ui.add_space(5.0);

        egui::Frame::new()
            .fill(state::colors::LIGHT_CREAM)
            .stroke(Stroke::new(2.0, state::colors::HONEY_GOLD))
            .corner_radius(CornerRadius::same(10))
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.heading(RichText::new("📋 Results").color(state::colors::SADDLE_BROWN));
                ui.add_space(5.0);

                if let Some(ref result) = self.result {
                    ui.label(RichText::new(format!("✓ {}", result))
                        .size(18.0)
                        .color(state::colors::FOREST_GREEN)
                        .strong());
                }

                if !self.warnings.is_empty() {
                    ui.add_space(8.0);
                    for warning in &self.warnings {
                        ui.label(RichText::new(format!("⚠️ {}", warning))
                            .size(14.0)
                            .color(state::colors::DARK_ORANGE));
                    }
                }

                if !self.metadata.is_empty() {
                    ui.add_space(8.0);
                    ui.collapsing("ℹ️ Additional Information", |ui| {
                        for (key, value) in &self.metadata {
                            ui.label(format!("  • {}: {}", key, value));
                        }
                    });
                }
            });
    }
}

pub fn input_field(ui: &mut egui::Ui, label: &str, value: &mut String, hint: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).strong());
        ui.text_edit_singleline(value).on_hover_text(hint);
    });
}

pub fn calculate_button(ui: &mut egui::Ui, text: &str) -> bool {
    let button = egui::Button::new(RichText::new(text).size(16.0).strong())
        .fill(state::colors::FOREST_GREEN)
        .corner_radius(CornerRadius::same(8))
        .min_size(Vec2::new(200.0, 40.0));
    ui.add(button).clicked()
}

pub fn run() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Mazerion - Professional Beverage Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(MazerionApp::default()))),
    )
}