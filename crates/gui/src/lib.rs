//! Mazerion GUI - Main application

use eframe::egui::{self, Color32, CornerRadius, RichText};

pub mod state;
mod tabs;

use state::{AppState, TabView};

pub struct MazerionApp {
    pub state: AppState,
    pub result: Option<String>,
    pub warnings: Vec<String>,
    pub metadata: Vec<(String, String)>,

    // Basic
    pub og: String,
    pub fg: String,
    pub target_abv: String,
    pub target_og: String,
    pub current_volume: String,
    pub target_volume: String,
    pub sugar_weight: String,
    pub target_co2: String,
    pub temperature: String,
    pub brix_reading: String,
    pub brix: String,
    pub sg_for_brix: String,
    pub current_vol: String,
    pub current_abv: String,

    // Advanced
    pub vol1: String,
    pub sg1: String,
    pub vol2: String,
    pub sg2: String,
    pub refrac_brix: String,
    pub refrac_og: String,
    pub alcohol_sg: String,
    pub measured_abv: String,
    pub temp_sg: String,
    pub temp_reading: String,
    pub abv1: String,
    pub abv2: String,
    pub orig_brix: String,
    pub curr_brix: String,
    pub sg: String,
    pub temp: String,

    // Brewing
    pub volume: String,
    pub abv: String,
    pub schedule: String,
    pub honey_weight: String,
    pub fruit_weight: String,
    pub ingredient_type: String,
    pub calcium: String,
    pub magnesium: String,
    pub sulfate: String,
    pub chloride: String,
    pub bicarbonate: String,
    pub sodium: String,
    pub batch_size: String,
    pub target_abv_brew: String,
    pub yn_requirement: String,
    pub carb_temp: String,
    pub carb_method: String,
    pub sugar_type: String,

    // Beer
    pub hop_weight: String,
    pub hop_aa: String,
    pub boil_time: String,
    pub boil_gravity: String,
    pub grain_weight: String,
    pub grain_color: String,
    pub mash_water: String,
    pub grain_temp: String,
    pub target_temp: String,
    pub total_points: String,
    pub actual_points: String,
    pub alpha_acid: String,
    pub beer_volume: String,
    pub grain_lovibond: String,
    pub mash_target_temp: String,
    pub mash_ratio: String,
    pub grain_ppg: String,
    pub measured_gravity: String,

    // Finishing
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
    pub stabilization_vol: String,

    // Mead Styles
    pub mead_style: String,
    pub mead_volume: String,
    pub mead_target_abv: String,
    pub fruit_type: String,
    pub juice_percent: String,
    pub maple_percent: String,
    pub bochet_level: String,
    pub honey_percent: String,
    pub malt_weight: String,
    pub spice_level: String,

    // Utilities
    pub bench_volume: String,
    pub bench_addition: String,
    pub upscale_original_vol: String,
    pub upscale_target_vol: String,
    pub upscale_ingredient: String,
    pub trial_volume: String,
    pub trial_addition: String,
    pub batch_volume_bench: String,
    pub original_recipe_size: String,
    pub target_batch_size: String,
    pub original_amount: String,
    pub upscale_unit: String,
    pub waste_initial_volume: String,
    pub waste_vessel_type: String,
    pub waste_num_rackings: String,
    pub waste_process_type: String,
    pub utility_calc: tabs::utilities::UtilityCalculator,
}

impl Default for MazerionApp {
    fn default() -> Self {
        Self {
            state: AppState::default(),
            result: None,
            warnings: Vec::new(),
            metadata: Vec::new(),

            og: "1.100".to_string(),
            fg: "1.010".to_string(),
            target_abv: "14".to_string(),
            target_og: "1.100".to_string(),
            current_volume: "20".to_string(),
            target_volume: "19".to_string(),
            sugar_weight: "150".to_string(),
            target_co2: "2.5".to_string(),
            temperature: "20".to_string(),
            brix_reading: String::new(),
            brix: String::new(),
            sg_for_brix: String::new(),
            current_vol: String::new(),
            current_abv: String::new(),

            vol1: String::new(),
            sg1: String::new(),
            vol2: String::new(),
            sg2: String::new(),
            refrac_brix: String::new(),
            refrac_og: String::new(),
            alcohol_sg: String::new(),
            measured_abv: String::new(),
            temp_sg: String::new(),
            temp_reading: String::new(),
            abv1: String::new(),
            abv2: String::new(),
            orig_brix: String::new(),
            curr_brix: String::new(),
            sg: String::new(),
            temp: String::new(),

            volume: "19".to_string(),
            abv: String::new(),
            schedule: "tosna_2".to_string(),
            honey_weight: String::new(),
            fruit_weight: String::new(),
            ingredient_type: "honey".to_string(),
            calcium: String::new(),
            magnesium: String::new(),
            sulfate: String::new(),
            chloride: String::new(),
            bicarbonate: String::new(),
            sodium: String::new(),
            batch_size: String::new(),
            target_abv_brew: String::new(),
            yn_requirement: String::new(),
            carb_temp: String::new(),
            carb_method: "bottle".to_string(),
            sugar_type: "dextrose".to_string(),

            hop_weight: String::new(),
            hop_aa: String::new(),
            boil_time: String::new(),
            boil_gravity: String::new(),
            grain_weight: String::new(),
            grain_color: String::new(),
            mash_water: String::new(),
            grain_temp: String::new(),
            target_temp: String::new(),
            total_points: String::new(),
            actual_points: String::new(),
            alpha_acid: String::new(),
            beer_volume: String::new(),
            grain_lovibond: String::new(),
            mash_target_temp: String::new(),
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
            pasteurization_temp: String::new(),
            stabilization_vol: String::new(),

            mead_style: "traditional".to_string(),
            mead_volume: String::new(),
            mead_target_abv: String::new(),
            fruit_type: String::new(),
            juice_percent: String::new(),
            maple_percent: String::new(),
            bochet_level: String::new(),
            honey_percent: String::new(),
            malt_weight: String::new(),
            spice_level: String::new(),

            bench_volume: String::new(),
            bench_addition: String::new(),
            upscale_original_vol: String::new(),
            upscale_target_vol: String::new(),
            upscale_ingredient: String::new(),
            trial_volume: String::new(),
            trial_addition: String::new(),
            batch_volume_bench: String::new(),
            original_recipe_size: String::new(),
            target_batch_size: String::new(),
            original_amount: String::new(),
            upscale_unit: "g".to_string(),
            waste_initial_volume: String::new(),
            waste_vessel_type: "carboy".to_string(),
            waste_num_rackings: String::new(),
            waste_process_type: "standard".to_string(),
            utility_calc: tabs::utilities::UtilityCalculator::RecipeUpscaling,
        }
    }
}

impl eframe::App for MazerionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let c = self.state.custom_colors;
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(c.light_cream))
            .show(ctx, |ui| {
                ui.add_space(10.0);

                ui.heading(
                    RichText::new("🍯 Mazerion - Brewing Calculator Suite")
                        .size(28.0)
                        .color(c.honey_gold),
                );

                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    if tab_button(ui, "📊 Basic", self.state.current_tab == TabView::Basic, &c) {
                        self.state.current_tab = TabView::Basic;
                    }
                    if tab_button(
                        ui,
                        "🔬 Advanced",
                        self.state.current_tab == TabView::Advanced,
                        &c,
                    ) {
                        self.state.current_tab = TabView::Advanced;
                    }
                    if tab_button(
                        ui,
                        "🍺 Brewing",
                        self.state.current_tab == TabView::Brewing,
                        &c,
                    ) {
                        self.state.current_tab = TabView::Brewing;
                    }
                    if tab_button(ui, "🍻 Beer", self.state.current_tab == TabView::Beer, &c) {
                        self.state.current_tab = TabView::Beer;
                    }
                    if tab_button(
                        ui,
                        "✨ Finishing",
                        self.state.current_tab == TabView::Finishing,
                        &c,
                    ) {
                        self.state.current_tab = TabView::Finishing;
                    }
                    if tab_button(ui, "🍯 Meads", self.state.current_tab == TabView::Meads, &c) {
                        self.state.current_tab = TabView::Meads;
                    }
                    if tab_button(
                        ui,
                        "🔧 Utilities",
                        self.state.current_tab == TabView::Utilities,
                        &c,
                    ) {
                        self.state.current_tab = TabView::Utilities;
                    }
                    if tab_button(
                        ui,
                        "⚙️ Settings",
                        self.state.current_tab == TabView::Settings,
                        &c,
                    ) {
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
                        TabView::Meads => tabs::meads::render(self, ui),
                        TabView::Utilities => tabs::utilities::render(self, ui),
                        TabView::Settings => tabs::settings::render(self, ui),
                    }

                    if let Some(ref result_text) = self.result {
                        ui.add_space(20.0);
                        egui::Frame::default()
                            .fill(Color32::WHITE)
                            .stroke(egui::Stroke::new(2.0, c.forest_green))
                            .corner_radius(CornerRadius::same(8))
                            .inner_margin(15.0)
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(result_text).size(28.0).color(Color32::BLACK),
                                );

                                if !self.warnings.is_empty() {
                                    ui.add_space(10.0);
                                    for warning in &self.warnings {
                                        ui.label(
                                            RichText::new(format!("⚠️ {}", warning))
                                                .color(c.sunset_orange),
                                        );
                                    }
                                }

                                if !self.metadata.is_empty() {
                                    ui.add_space(10.0);
                                    for (key, value) in &self.metadata {
                                        ui.label(
                                            RichText::new(format!("{}: {}", key, value))
                                                .size(14.0)
                                                .color(c.dark_text),
                                        );
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
        .fill(if selected {
            c.honey_gold
        } else {
            c.light_cream
        })
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
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Mazerion - Brewing Calculator Suite",
        options,
        Box::new(|_cc| Ok(Box::new(MazerionApp::default()))),
    )
}
