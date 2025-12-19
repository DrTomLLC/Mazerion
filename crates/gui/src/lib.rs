//! Mazerion GUI using egui
use eframe::egui::{self, Color32, RichText, CornerRadius};

pub mod state;
pub mod tabs;

pub use state::{AppState, TabView, Theme, UnitSystem, BasicCalculator, BeerCalculator};
pub use crate::tabs::MeadStyle;

pub struct MazerionApp {
    pub state: AppState,
    pub og: String,
    pub fg: String,
    pub brix: String,
    pub sg_for_brix: String,
    pub current_vol: String,
    pub current_abv: String,
    pub target_abv: String,
    pub vol1: String,
    pub abv1: String,
    pub vol2: String,
    pub abv2: String,
    pub orig_brix: String,
    pub curr_brix: String,
    pub sg: String,
    pub temp: String,
    pub volume: String,
    pub target_abv_brew: String,
    pub yn_requirement: String,
    pub carb_temp: String,
    pub target_co2: String,
    pub carb_method: String,
    pub sugar_type: String,
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
    pub pasteurization_temp: String,
    pub mead_style: MeadStyle,
    pub mead_volume: String,
    pub mead_target_abv: String,
    pub fruit_weight: String,
    pub fruit_type: String,
    pub juice_percent: String,
    pub maple_percent: String,
    pub bochet_level: String,
    pub honey_percent: String,
    pub malt_percent: String,
    pub malt_weight: String,
    pub spice_type: String,
    pub spice_level: String,
    pub pepper_type: String,
    pub heat_level: String,
    pub utility_calc: crate::tabs::UtilityCalculator,
    pub trial_volume: String,
    pub trial_addition: String,
    pub batch_volume_bench: String,
    pub original_recipe_size: String,
    pub target_batch_size: String,
    pub original_amount: String,
    pub upscale_unit: String,
    pub bottle_volume: String,
    pub waste_initial_volume: String,
    pub waste_num_rackings: String,
    pub waste_vessel_type: String,
    pub waste_process_type: String,
    pub honey_cost: String,
    pub fruit_cost: String,
    pub yeast_cost: String,
    pub nutrients_cost: String,
    pub other_cost: String,
    pub bottles_count: String,
    pub priming_sugar_type: String,
    pub priming_amount: String,
    pub water_calcium: String,
    pub water_magnesium: String,
    pub water_sulfate: String,
    pub water_chloride: String,
    pub conv_value: String,
    pub conv_from_unit: String,
    pub conv_to_unit: String,
    pub conv_result: Option<String>,
    pub result: Option<String>,
    pub warnings: Vec<String>,
    pub metadata: Vec<(String, String)>,
}

impl Default for MazerionApp {
    fn default() -> Self {
        Self {
            state: AppState::default(),
            og: String::new(),
            fg: String::new(),
            brix: String::new(),
            sg_for_brix: String::new(),
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
            yn_requirement: String::new(),
            carb_temp: String::new(),
            target_co2: String::new(),
            carb_method: "priming_sugar".to_string(),
            sugar_type: "corn_sugar".to_string(),
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
            sweetener: "sugar".to_string(),
            sulfite_vol: String::new(),
            ph: String::new(),
            target_so2: String::new(),
            acid_vol: String::new(),
            current_ph: String::new(),
            target_ph_acid: String::new(),
            acid_type: "tartaric".to_string(),
            pasteurization_temp: String::new(),
            mead_style: MeadStyle::Traditional,
            mead_volume: String::new(),
            mead_target_abv: String::new(),
            fruit_weight: String::new(),
            fruit_type: "strawberry".to_string(),
            juice_percent: String::new(),
            maple_percent: String::new(),
            bochet_level: "medium".to_string(),
            honey_percent: String::new(),
            malt_percent: String::new(),
            malt_weight: String::new(),
            spice_type: "cinnamon".to_string(),
            spice_level: "medium".to_string(),
            pepper_type: "jalapeno".to_string(),
            heat_level: "medium".to_string(),
            utility_calc: crate::tabs::UtilityCalculator::BenchTrials,
            trial_volume: String::new(),
            trial_addition: String::new(),
            batch_volume_bench: String::new(),
            original_recipe_size: String::new(),
            target_batch_size: String::new(),
            original_amount: String::new(),
            upscale_unit: "g".to_string(),
            bottle_volume: String::new(),
            waste_initial_volume: String::new(),
            waste_num_rackings: "3".to_string(),
            waste_vessel_type: "carboy".to_string(),
            waste_process_type: "standard".to_string(),
            honey_cost: String::new(),
            fruit_cost: String::new(),
            yeast_cost: String::new(),
            nutrients_cost: String::new(),
            other_cost: String::new(),
            bottles_count: "30".to_string(),
            priming_sugar_type: "corn_sugar".to_string(),
            priming_amount: String::new(),
            water_calcium: String::new(),
            water_magnesium: String::new(),
            water_sulfate: String::new(),
            water_chloride: String::new(),
            conv_value: String::new(),
            conv_from_unit: "liters".to_string(),
            conv_to_unit: "gallons".to_string(),
            conv_result: None,
            result: None,
            warnings: Vec::new(),
            metadata: Vec::new(),
        }
    }
}

impl eframe::App for MazerionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let c = self.state.custom_colors;

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(c.background))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(RichText::new("🍯 Mazerion 🍺")
                        .size(32.0)
                        .color(c.honey_gold));
                    ui.label(RichText::new(
                        format!("📊 48 Calculators | {} | Precision: SG {:?}, pH {:?}, Brix {:?}",
                                self.state.unit_system.name(),
                                self.state.sg_precision,
                                self.state.ph_precision,
                                self.state.brix_precision
                        )).size(12.0).color(c.dark_text));
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if tab_button(ui, "📊 Basic", self.state.current_tab == TabView::Basic, &c) {
                        self.state.current_tab = TabView::Basic;
                    }
                    if tab_button(ui, "🔬 Advanced", self.state.current_tab == TabView::Advanced, &c) {
                        self.state.current_tab = TabView::Advanced;
                    }
                    if tab_button(ui, "🧪 Brewing", self.state.current_tab == TabView::Brewing, &c) {
                        self.state.current_tab = TabView::Brewing;
                    }
                    if tab_button(ui, "🍺 Beer", self.state.current_tab == TabView::Beer, &c) {
                        self.state.current_tab = TabView::Beer;
                    }
                    if tab_button(ui, "🎨 Finishing", self.state.current_tab == TabView::Finishing, &c) {
                        self.state.current_tab = TabView::Finishing;
                    }
                    if tab_button(ui, "🍯 Mead Styles", self.state.current_tab == TabView::MeadStyles, &c) {
                        self.state.current_tab = TabView::MeadStyles;
                    }
                    if tab_button(ui, "🔧 Utilities", self.state.current_tab == TabView::Utilities, &c) {
                        self.state.current_tab = TabView::Utilities;
                    }
                    if tab_button(ui, "⚙️ Settings", self.state.current_tab == TabView::Settings, &c) {
                        self.state.current_tab = TabView::Settings;
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    match self.state.current_tab {
                        TabView::Basic => tabs::render_basic(self, ui),
                        TabView::Advanced => tabs::render_advanced(self, ui),
                        TabView::Brewing => tabs::render_brewing(self, ui),
                        TabView::Beer => tabs::render_beer(self, ui),
                        TabView::Finishing => tabs::render_finishing(self, ui),
                        TabView::MeadStyles => tabs::render_mead_styles(self, ui),
                        TabView::Utilities => tabs::render_utilities(self, ui),
                        TabView::Settings => tabs::render_settings(self, ui),
                    }

                    if let Some(ref result_text) = self.result {
                        ui.add_space(15.0);
                        egui::Frame::default()
                            .fill(Color32::WHITE)
                            .stroke(egui::Stroke::new(2.0, c.forest_green))
                            .corner_radius(CornerRadius::same(8))
                            .inner_margin(12.0)
                            .show(ui, |ui| {
                                ui.label(RichText::new(result_text)
                                    .size(28.0)
                                    .color(Color32::BLACK));

                                if !self.warnings.is_empty() {
                                    ui.add_space(8.0);
                                    for warning in &self.warnings {
                                        ui.label(RichText::new(format!("⚠️ {}", warning))
                                            .color(c.sunset_orange));
                                    }
                                }

                                if !self.metadata.is_empty() {
                                    ui.add_space(10.0);
                                    for (key, value) in &self.metadata {
                                        ui.label(RichText::new(format!("{}: {}", key, value))
                                            .size(14.0)
                                            .color(c.dark_text));
                                    }
                                }
                            });
                    }
                });
            });
    }
}

fn tab_button(ui: &mut egui::Ui, text: &str, selected: bool, c: &state::CustomColors) -> bool {
    let button = egui::Button::new(RichText::new(text).size(14.0))
        .fill(if selected { c.honey_gold } else { c.light_cream })
        .stroke(egui::Stroke::new(1.0, c.dark_text));
    ui.add(button).clicked()
}

pub fn input_field(ui: &mut egui::Ui, label: &str, value: &mut String, hint: &str) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.text_edit_singleline(value).on_hover_text(hint);
    });
}

pub fn calculate_button(ui: &mut egui::Ui, text: &str) -> bool {
    ui.button(RichText::new(text).size(16.0)).clicked()
}

pub fn run() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Mazerion - Brewing Calculator Suite",
        options,
        Box::new(|_cc| Ok(Box::new(MazerionApp::default()))),
    )
}