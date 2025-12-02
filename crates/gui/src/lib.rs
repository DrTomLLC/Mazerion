//! Production GUI with 39 calculators

use eframe::egui::{self, Color32, RichText, CornerRadius};

pub mod state;
pub mod tabs;

pub use state::{AppState, TabView, Theme, UnitSystem};

pub struct MazerionApp {
    pub state: AppState,
    pub result: Option<String>,
    pub warnings: Vec<String>,
    pub metadata: Vec<(String, String)>,

    // Basic calculator fields
    pub og: String,
    pub fg: String,
    pub brix: String,
    pub current_vol: String,
    pub current_abv: String,
    pub target_abv: String,

    // Advanced calculator fields
    pub vol1: String,
    pub abv1: String,
    pub vol2: String,
    pub abv2: String,
    pub orig_brix: String,
    pub curr_brix: String,
    pub sg: String,
    pub temp: String,

    // Brewing calculator fields
    pub volume: String,
    pub target_abv_brew: String,
    pub yn_requirement: String,
    pub carb_temp: String,
    pub target_co2: String,
    pub carb_method: String,
    pub sugar_type: String,

    // Finishing calculator fields
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

    // Beer calculator fields
    pub beer_calc: tabs::beer::BeerCalculator,
    pub hop_weight: String,
    pub hop_alpha: String,
    pub boil_time: String,
    pub batch_volume: String,
    pub wort_gravity: String,
    pub grain_weight: String,
    pub grain_lovibond: String,

    // Mead style fields
    pub mead_style: tabs::mead_styles::MeadStyle,
    pub target_abv_mead: String,
    pub fruit_ratio: String,
    pub juice_percent: String,

    // Utility fields
    pub utility_calc: tabs::utilities::UtilityCalculator,
    pub honey_cost: String,
    pub yeast_cost: String,
    pub nutrient_cost: String,
    pub other_cost: String,
    pub water_volume: String,
    pub target_calcium: String,
    pub current_calcium: String,
}

impl Default for MazerionApp {
    fn default() -> Self {
        Self {
            state: AppState::default(),
            result: None,
            warnings: Vec::new(),
            metadata: Vec::new(),

            // Basic defaults
            og: "1.090".into(),
            fg: "1.010".into(),
            brix: "15.0".into(),
            current_vol: "20".into(),
            current_abv: "14".into(),
            target_abv: "10".into(),

            // Advanced defaults
            vol1: "10".into(),
            abv1: "14".into(),
            vol2: "10".into(),
            abv2: "10".into(),
            orig_brix: "22.0".into(),
            curr_brix: "8.0".into(),
            sg: "1.050".into(),
            temp: "20".into(),

            // Brewing defaults
            volume: "19".into(),
            target_abv_brew: "14".into(),
            yn_requirement: "medium".into(),
            carb_temp: "20".into(),
            target_co2: "2.5".into(),
            carb_method: "priming".into(),
            sugar_type: "table_sugar".into(),

            // Finishing defaults
            sweet_vol: "19".into(),
            current_sg: "1.000".into(),
            target_sg: "1.010".into(),
            sweetener: "honey".into(),
            sulfite_vol: "19".into(),
            ph: "3.5".into(),
            target_so2: "30".into(),
            acid_vol: "19".into(),
            current_ph: "3.8".into(),
            target_ph_acid: "3.4".into(),
            acid_type: "tartaric".into(),

            // Beer defaults
            beer_calc: tabs::beer::BeerCalculator::Ibu,
            hop_weight: "50".into(),
            hop_alpha: "5.5".into(),
            boil_time: "60".into(),
            batch_volume: "19".into(),
            wort_gravity: "1.050".into(),
            grain_weight: "5".into(),
            grain_lovibond: "5".into(),

            // Mead style defaults
            mead_style: tabs::mead_styles::MeadStyle::Traditional,
            target_abv_mead: "14".into(),
            fruit_ratio: "0.2".into(),
            juice_percent: "50".into(),

            // Utility defaults
            utility_calc: tabs::utilities::UtilityCalculator::Cost,
            honey_cost: "60".into(),
            yeast_cost: "5".into(),
            nutrient_cost: "10".into(),
            other_cost: "15".into(),
            water_volume: "19".into(),
            target_calcium: "100".into(),
            current_calcium: "0".into(),
        }
    }
}

impl eframe::App for MazerionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (bg, panel_bg) = self.state.get_theme_colors();

        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).fill(bg))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(RichText::new("🍯 Mazerion").size(24.0).color(state::colors::HONEY_GOLD));
                    ui.label(RichText::new("v0.8.0").size(14.0).color(state::colors::DARK_TEXT));
                });

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.selectable_label(self.state.current_tab == TabView::Basic, "🧮 Basic").clicked() {
                        self.state.current_tab = TabView::Basic;
                    }
                    if ui.selectable_label(self.state.current_tab == TabView::Advanced, "📊 Advanced").clicked() {
                        self.state.current_tab = TabView::Advanced;
                    }
                    if ui.selectable_label(self.state.current_tab == TabView::Brewing, "🍺 Brewing").clicked() {
                        self.state.current_tab = TabView::Brewing;
                    }
                    if ui.selectable_label(self.state.current_tab == TabView::Finishing, "✨ Finishing").clicked() {
                        self.state.current_tab = TabView::Finishing;
                    }
                    if ui.selectable_label(self.state.current_tab == TabView::Beer, "🍺 Beer").clicked() {
                        self.state.current_tab = TabView::Beer;
                    }
                    if ui.selectable_label(self.state.current_tab == TabView::MeadStyles, "🍯 Styles").clicked() {
                        self.state.current_tab = TabView::MeadStyles;
                    }
                    if ui.selectable_label(self.state.current_tab == TabView::Utilities, "🛠️ Utils").clicked() {
                        self.state.current_tab = TabView::Utilities;
                    }
                    if ui.selectable_label(self.state.current_tab == TabView::Settings, "⚙️ Settings").clicked() {
                        self.state.current_tab = TabView::Settings;
                    }
                });

                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    match self.state.current_tab {
                        TabView::Basic => tabs::basic::render(self, ui),
                        TabView::Advanced => tabs::advanced::render(self, ui),
                        TabView::Brewing => tabs::brewing::render(self, ui),
                        TabView::Finishing => tabs::finishing::render(self, ui),
                        TabView::Beer => tabs::beer::render(self, ui),
                        TabView::MeadStyles => tabs::mead_styles::render(self, ui),
                        TabView::Utilities => tabs::utilities::render(self, ui),
                        TabView::Settings => tabs::settings::render(self, ui),
                    }

                    if let Some(ref result) = self.result {
                        ui.add_space(15.0);
                        egui::Frame::new()
                            .fill(panel_bg)
                            .stroke(egui::Stroke::new(3.0, Color32::from_rgb(0, 150, 0)))
                            .corner_radius(CornerRadius::same(8))
                            .inner_margin(15.0)
                            .show(ui, |ui| {
                                ui.label(RichText::new("✓ RESULT").strong()
                                    .color(Color32::from_rgb(0, 120, 0)).size(18.0));
                                ui.label(RichText::new(result).size(28.0).strong()
                                    .color(Color32::BLACK));
                            });
                    }

                    if !self.warnings.is_empty() {
                        ui.add_space(10.0);
                        egui::Frame::new()
                            .fill(panel_bg)
                            .stroke(egui::Stroke::new(2.0, Color32::from_rgb(255, 140, 0)))
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(12.0)
                            .show(ui, |ui| {
                                ui.label(RichText::new("⚠️ WARNINGS").strong()
                                    .color(Color32::from_rgb(200, 100, 0)).size(16.0));
                                for warning in &self.warnings {
                                    ui.label(RichText::new(format!("• {}", warning))
                                        .size(14.0).color(Color32::BLACK));
                                }
                            });
                    }

                    if !self.metadata.is_empty() {
                        ui.add_space(10.0);
                        egui::Frame::new()
                            .fill(panel_bg)
                            .stroke(egui::Stroke::new(2.0, Color32::from_rgb(70, 130, 180)))
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(12.0)
                            .show(ui, |ui| {
                                ui.label(RichText::new("ℹ️ DETAILS").strong()
                                    .color(Color32::from_rgb(50, 100, 150)).size(16.0));
                                for (key, value) in &self.metadata {
                                    ui.label(RichText::new(format!("{}: {}", key, value))
                                        .size(14.0).color(Color32::BLACK));
                                }
                            });
                    }
                });
            });
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    mazerion_calculators::init()?;
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Mazerion",
        options,
        Box::new(|_| Ok(Box::new(MazerionApp::default()))),
    )?;
    Ok(())
}

pub fn input_field(ui: &mut egui::Ui, label: &str, value: &mut String, hint: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).strong().color(state::colors::DARK_TEXT));
        ui.text_edit_singleline(value).on_hover_text(hint);
    });
}

pub fn calculate_button(ui: &mut egui::Ui, text: &str) -> bool {
    ui.add(
        egui::Button::new(RichText::new(text).color(Color32::WHITE).size(16.0).strong())
            .fill(state::colors::FOREST_GREEN)
            .min_size(egui::Vec2::new(200.0, 40.0))
    )
        .clicked()
}