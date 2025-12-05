use eframe::egui::{self, Color32, RichText, CornerRadius, Vec2};

pub mod state;
pub mod tabs;

pub use state::{AppState, TabView, Theme, UnitSystem, BasicCalculator, BeerCalculator};
pub use crate::tabs::MeadStyle;

pub struct MazerionApp {
    pub state: AppState,

    // Basic tab fields
    pub og: String,
    pub fg: String,
    pub brix: String,
    pub current_vol: String,
    pub current_abv: String,
    pub target_abv: String,

    // Advanced tab fields
    pub vol1: String,
    pub abv1: String,
    pub vol2: String,
    pub abv2: String,
    pub orig_brix: String,
    pub curr_brix: String,
    pub sg: String,
    pub temp: String,

    // Brewing tab fields
    pub volume: String,
    pub target_abv_brew: String,
    pub yn_requirement: String,
    pub carb_temp: String,
    pub target_co2: String,
    pub carb_method: String,
    pub sugar_type: String,

    // Beer tab fields
    pub beer_calc: BeerCalculator,
    pub hop_weight: String,
    pub alpha_acid: String,
    pub boil_time: String,
    pub beer_volume: String,
    pub boil_gravity: String,
    pub grain_weight: String,
    pub grain_lovibond: String,
    pub mash_target_temp: String,
    pub grain_temp: String,
    pub mash_ratio: String,
    pub grain_ppg: String,
    pub measured_gravity: String,

    // Finishing tab fields
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

    // Mead Styles tab fields
    pub mead_style: MeadStyle,
    pub mead_volume: String,
    pub mead_target_abv: String,
    pub fruit_weight: String,
    pub fruit_type: String,
    pub juice_percent: String,
    pub maple_percent: String,
    pub bochet_level: String,
    pub honey_percent: String,
    pub malt_weight: String,
    pub heat_level: String,
    pub spice_level: String,

    // Utilities tab fields
    pub utility_calc: tabs::utilities::UtilityCalculator,
    pub honey_cost: String,
    pub fruit_cost: String,
    pub yeast_cost: String,
    pub nutrient_cost: String,
    pub other_cost: String,
    pub bottles_count: String,
    pub water_ca: String,
    pub water_mg: String,
    pub water_so4: String,
    pub water_cl: String,
    pub water_profile: String,
    pub trial_volume: String,
    pub trial_addition: String,
    pub batch_volume: String,

    // Settings/Conversions tab fields
    pub conv_value: String,
    pub conv_from_unit: String,
    pub conv_to_unit: String,
    pub conv_result: Option<String>,

    // Result display
    pub result: Option<String>,
    pub warnings: Vec<String>,
    pub metadata: Vec<(String, String)>,
    nutrients_cost: ()
}

impl Default for MazerionApp {
    fn default() -> Self {
        Self {
            state: AppState::default(),

            og: String::new(),
            fg: String::new(),
            brix: String::new(),
            current_vol: String::new(),
            current_abv: String::new(),
            target_abv: String::new(),

            vol1: String::new(),
            abv1: String::new(),
            vol2: String::new(),
            abv2: String::new(),
            orig_brix: String::new(),
            curr_brix: String::new(),
            sg: String::new(),
            temp: String::new(),

            volume: String::new(),
            target_abv_brew: String::new(),
            yn_requirement: "medium".to_string(),
            carb_temp: String::new(),
            target_co2: String::new(),
            carb_method: "priming".to_string(),
            sugar_type: "table_sugar".to_string(),

            beer_calc: BeerCalculator::Ibu,
            hop_weight: String::new(),
            alpha_acid: String::new(),
            boil_time: String::new(),
            beer_volume: String::new(),
            boil_gravity: String::new(),
            grain_weight: String::new(),
            grain_lovibond: String::new(),
            mash_target_temp: String::new(),
            grain_temp: String::new(),
            mash_ratio: String::new(),
            grain_ppg: String::new(),
            measured_gravity: String::new(),

            sweet_vol: String::new(),
            current_sg: String::new(),
            target_sg: String::new(),
            sweetener: "honey".to_string(),
            sulfite_vol: String::new(),
            ph: String::new(),
            target_so2: String::new(),
            acid_vol: String::new(),
            current_ph: String::new(),
            target_ph_acid: String::new(),
            acid_type: "tartaric".to_string(),

            mead_style: MeadStyle::Traditional,
            mead_volume: String::new(),
            mead_target_abv: String::new(),
            fruit_weight: String::new(),
            fruit_type: "strawberry".to_string(),
            juice_percent: String::new(),
            maple_percent: String::new(),
            bochet_level: "medium".to_string(),
            honey_percent: String::new(),
            malt_weight: String::new(),
            heat_level: "medium".to_string(),
            spice_level: "medium".to_string(),

            utility_calc: tabs::utilities::UtilityCalculator::Cost,
            honey_cost: String::new(),
            fruit_cost: String::new(),
            yeast_cost: String::new(),
            nutrient_cost: String::new(),
            other_cost: String::new(),
            bottles_count: String::new(),
            water_ca: String::new(),
            water_mg: String::new(),
            water_so4: String::new(),
            water_cl: String::new(),
            water_profile: "balanced".to_string(),
            trial_volume: String::new(),
            trial_addition: String::new(),
            batch_volume: String::new(),

            conv_value: String::new(),
            conv_from_unit: "liters".to_string(),
            conv_to_unit: "gallons".to_string(),
            conv_result: None,

            result: None,
            warnings: Vec::new(),
            metadata: Vec::new(),
            nutrients_cost: (),
        }
    }
}

impl eframe::App for MazerionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (bg_main, bg_panel) = self.state.get_theme_colors();

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(bg_main))
            .show(ctx, |ui| {
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.heading(RichText::new("🍯 Mazerion MCL v0.10.0").size(28.0).color(state::colors::SADDLE_BROWN));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(format!("📊 40 Calculators | {} | Precision: SG {:?}, pH {:?}, Brix {:?}",
                                                       self.state.unit_system.name(),
                                                       self.state.sg_precision,
                                                       self.state.ph_precision,
                                                       self.state.brix_precision
                        )).size(12.0).color(state::colors::DARK_TEXT));
                    });
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if tab_button(ui, "📊 Basic", self.state.current_tab == TabView::Basic) {
                        self.state.current_tab = TabView::Basic;
                    }
                    if tab_button(ui, "🔬 Advanced", self.state.current_tab == TabView::Advanced) {
                        self.state.current_tab = TabView::Advanced;
                    }
                    if tab_button(ui, "🧪 Brewing", self.state.current_tab == TabView::Brewing) {
                        self.state.current_tab = TabView::Brewing;
                    }
                    if tab_button(ui, "🍺 Beer", self.state.current_tab == TabView::Beer) {
                        self.state.current_tab = TabView::Beer;
                    }
                    if tab_button(ui, "✨ Finishing", self.state.current_tab == TabView::Finishing) {
                        self.state.current_tab = TabView::Finishing;
                    }
                    if tab_button(ui, "🍯 Mead Styles", self.state.current_tab == TabView::MeadStyles) {
                        self.state.current_tab = TabView::MeadStyles;
                    }
                    if tab_button(ui, "🔧 Utilities", self.state.current_tab == TabView::Utilities) {
                        self.state.current_tab = TabView::Utilities;
                    }
                    if tab_button(ui, "⚙️ Settings", self.state.current_tab == TabView::Settings) {
                        self.state.current_tab = TabView::Settings;
                    }
                });

                ui.add_space(15.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    match self.state.current_tab {
                        TabView::Basic => tabs::basic::render(self, ui),
                        TabView::Advanced => tabs::advanced::render(self, ui),
                        TabView::Brewing => tabs::brewing::render(self, ui),
                        TabView::Beer => tabs::beer::render(self, ui),
                        TabView::Finishing => tabs::finishing::render(self, ui),
                        TabView::MeadStyles => tabs::mead_styles::render(self, ui),
                        TabView::Utilities => tabs::utilities::render(self, ui),
                        TabView::Settings => tabs::settings::render(self, ui),
                    }

                    if let Some(ref result) = self.result {
                        ui.add_space(15.0);
                        egui::Frame::none()
                            .fill(bg_panel)
                            .stroke(egui::Stroke::new(2.0, state::colors::FOREST_GREEN))
                            .corner_radius(CornerRadius::same(8))
                            .inner_margin(15.0)
                            .show(ui, |ui| {
                                ui.label(RichText::new(result).size(16.0).color(state::colors::FOREST_GREEN).strong());

                                if !self.warnings.is_empty() {
                                    ui.add_space(8.0);
                                    ui.label(RichText::new("⚠️ Warnings:").color(state::colors::DARK_ORANGE).strong());
                                    for warning in &self.warnings {
                                        ui.label(RichText::new(format!("  • {}", warning)).color(state::colors::DARK_ORANGE));
                                    }
                                }

                                if !self.metadata.is_empty() {
                                    ui.add_space(8.0);
                                    ui.label(RichText::new("ℹ️ Details:").color(state::colors::DARK_TEXT).strong());
                                    for (key, value) in &self.metadata {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(format!("{}:", key)).color(state::colors::DARK_TEXT));
                                            ui.label(RichText::new(value).color(state::colors::GOLDENROD));
                                        });
                                    }
                                }
                            });
                    }
                });
            });
    }
}

pub fn run() -> eframe::Result {
    if let Err(e) = mazerion_calculators::init() {
        eprintln!("❌ FATAL: Calculator initialization failed: {}", e);
        std::process::exit(1);
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Mazerion MCL v0.9.0 - Precision Beverage Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(MazerionApp::default()))),
    )
}

fn tab_button(ui: &mut egui::Ui, text: &str, selected: bool) -> bool {
    let button_color = if selected {
        state::colors::HONEY_GOLD
    } else {
        Color32::from_rgb(220, 220, 220)
    };

    let text_color = if selected {
        Color32::WHITE
    } else {
        state::colors::DARK_TEXT
    };

    ui.add(
        egui::Button::new(RichText::new(text).color(text_color).size(14.0).strong())
            .fill(button_color)
            .corner_radius(CornerRadius::same(6))
            .min_size(Vec2::new(100.0, 32.0))
    ).clicked()
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
            .corner_radius(CornerRadius::same(6))
            .min_size(Vec2::new(200.0, 40.0))
    ).clicked()
}