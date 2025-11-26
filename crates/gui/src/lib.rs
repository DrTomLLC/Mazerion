// Modern, beautiful GUI for Mazerion with tabs and full calculator suite
mod state;

use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};
use mazerion_core::{CalcInput, Measurement};
use rust_decimal::Decimal;
use std::str::FromStr;
use mazerion_core::traits::get_calculator;

// Import state management
use state::{AppState, TabView, BasicCalculator, AdvancedCalculator, BrewingCalculator, FinishingCalculator, colors};

// ===== STANDALONE HELPER FUNCTIONS (FIXED - moved outside impl block) =====
fn input_field(ui: &mut egui::Ui, label: &str, value: &mut String, hint: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).strong());
        ui.text_edit_singleline(value).on_hover_text(hint);
    });
}

fn calculate_button(ui: &mut egui::Ui, text: &str) -> bool {
    let button = egui::Button::new(RichText::new(text).size(16.0).strong())
        .fill(colors::FOREST_GREEN)
        .rounding(Rounding::same(8.0))
        .min_size(Vec2::new(200.0, 40.0));
    ui.add(button).clicked()
}

pub struct MazerionApp {
    state: AppState,

    // Input fields (organized by calculator type)
    // Basic
    og: String,
    fg: String,
    brix: String,
    sg: String,
    temp: String,
    current_vol: String,
    current_abv: String,
    target_abv: String,

    // Advanced
    vol1: String,
    abv1: String,
    vol2: String,
    abv2: String,
    orig_brix: String,
    curr_brix: String,

    // Brewing
    volume: String,
    target_abv_brew: String,
    yn_requirement: String,
    carb_temp: String,
    target_co2: String,
    carb_method: String,
    sugar_type: String,

    // Finishing
    sweet_vol: String,
    current_sg: String,
    target_sg: String,
    sweetener: String,
    sulfite_vol: String,
    ph: String,
    target_so2: String,
    acid_vol: String,
    current_ph: String,
    target_ph_acid: String,
    acid_type: String,

    // Results
    result: Option<String>,
    warnings: Vec<String>,
    metadata: Vec<(String, String)>,
}

impl Default for MazerionApp {
    fn default() -> Self {
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
        // Custom style
        self.apply_custom_style(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            self.render_header(ui);

            ui.add_space(10.0);

            // Tab selection
            self.render_tabs(ui);

            ui.add_space(10.0);

            // Calculator content based on selected tab
            match self.state.current_tab {
                TabView::Basic => self.render_basic_tab(ui),
                TabView::Advanced => self.render_advanced_tab(ui),
                TabView::Brewing => self.render_brewing_tab(ui),
                TabView::Finishing => self.render_finishing_tab(ui),
            }

            ui.add_space(10.0);

            // Results section
            self.render_results(ui);
        });
    }
}

impl MazerionApp {
    fn apply_custom_style(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = colors::CORNSILK;
        style.visuals.panel_fill = colors::LIGHT_CREAM;
        style.visuals.extreme_bg_color = colors::HONEY_GOLD;
        style.visuals.widgets.noninteractive.bg_fill = colors::LIGHT_CREAM;
        style.visuals.widgets.inactive.bg_fill = colors::LIGHT_CREAM;
        style.visuals.widgets.hovered.bg_fill = colors::GOLDENROD;
        style.visuals.widgets.active.bg_fill = colors::HONEY_GOLD;
        ctx.set_style(style);
    }

    fn render_header(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading(RichText::new("🍯 Mazerion")
                .size(32.0)
                .color(colors::SADDLE_BROWN)
                .strong());
            ui.label(RichText::new("Professional Beverage Calculator Suite")
                .size(16.0)
                .color(colors::GOLDENROD));
        });
    }

    fn render_tabs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 5.0;

            if self.tab_button(ui, "📊 Basic", self.state.current_tab == TabView::Basic).clicked() {
                self.state.current_tab = TabView::Basic;
                self.clear_results();
            }

            if self.tab_button(ui, "🔬 Advanced", self.state.current_tab == TabView::Advanced).clicked() {
                self.state.current_tab = TabView::Advanced;
                self.clear_results();
            }

            if self.tab_button(ui, "🍺 Brewing", self.state.current_tab == TabView::Brewing).clicked() {
                self.state.current_tab = TabView::Brewing;
                self.clear_results();
            }

            if self.tab_button(ui, "✨ Finishing", self.state.current_tab == TabView::Finishing).clicked() {
                self.state.current_tab = TabView::Finishing;
                self.clear_results();
            }
        });
    }

    fn tab_button(&self, ui: &mut egui::Ui, text: &str, active: bool) -> egui::Response {
        let color = if active { colors::HONEY_GOLD } else { colors::LIGHT_CREAM };
        let text_color = if active { Color32::WHITE } else { colors::SADDLE_BROWN };

        let button = egui::Button::new(RichText::new(text).color(text_color).size(14.0))
            .fill(color)
            .rounding(Rounding::same(5.0))
            .min_size(Vec2::new(120.0, 35.0));

        ui.add(button)
    }

    fn clear_results(&mut self) {
        self.result = None;
        self.warnings.clear();
        self.metadata.clear();
    }

    // Render results section
    fn render_results(&self, ui: &mut egui::Ui) {
        if self.result.is_none() && self.warnings.is_empty() {
            return;
        }

        ui.separator();
        ui.add_space(5.0);

        egui::Frame::none()
            .fill(colors::LIGHT_CREAM)
            .stroke(Stroke::new(2.0, colors::HONEY_GOLD))
            .rounding(Rounding::same(10.0))
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.heading(RichText::new("📋 Results").color(colors::SADDLE_BROWN));
                ui.add_space(5.0);

                if let Some(ref result) = self.result {
                    ui.label(RichText::new(format!("✓ {}", result))
                        .size(18.0)
                        .color(colors::FOREST_GREEN)
                        .strong());
                }

                if !self.warnings.is_empty() {
                    ui.add_space(8.0);
                    for warning in &self.warnings {
                        ui.label(RichText::new(format!("⚠️ {}", warning))
                            .size(14.0)
                            .color(colors::DARK_ORANGE));
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

    fn render_basic_tab(&mut self, ui: &mut egui::Ui) {
        // Calculator selection
        ui.horizontal(|ui| {
            ui.label(RichText::new("Select Calculator:").strong());
            egui::ComboBox::from_id_salt("basic_calc")
                .selected_text(self.get_basic_calc_name())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.state.basic_calc, BasicCalculator::Abv, "ABV Calculator");
                    ui.selectable_value(&mut self.state.basic_calc, BasicCalculator::BrixSgConverter, "Brix ↔ SG Converter");
                    ui.selectable_value(&mut self.state.basic_calc, BasicCalculator::Dilution, "Dilution Calculator");
                });
        });

        ui.add_space(10.0);

        // Calculator-specific UI
        egui::Frame::none()
            .fill(colors::LIGHT_CREAM)
            .stroke(egui::Stroke::new(1.5, colors::HONEY_GOLD))
            .rounding(Rounding::same(8.0))
            .inner_margin(15.0)
            .show(ui, |ui| {
                match self.state.basic_calc {
                    BasicCalculator::Abv => self.render_abv_calculator(ui),
                    BasicCalculator::BrixSgConverter => self.render_brix_converter(ui),
                    BasicCalculator::Dilution => self.render_dilution_calculator(ui),
                }
            });
    }

    fn get_basic_calc_name(&self) -> &str {
        match self.state.basic_calc {
            BasicCalculator::Abv => "ABV Calculator",
            BasicCalculator::BrixSgConverter => "Brix ↔ SG Converter",
            BasicCalculator::Dilution => "Dilution Calculator",
        }
    }

    fn render_abv_calculator(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("🍺 ABV Calculator").color(colors::SADDLE_BROWN));
        ui.label("Calculate alcohol by volume from gravity readings");
        ui.add_space(10.0);

        input_field(ui, "Original Gravity (OG):", &mut self.og, "Starting specific gravity (e.g., 1.090)");
        input_field(ui, "Final Gravity (FG):", &mut self.fg, "Ending specific gravity (e.g., 1.010)");

        ui.add_space(10.0);

        if calculate_button(ui, "Calculate ABV") {
            self.calc_abv();
        }
    }

    fn render_brix_converter(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("📐 Brix to SG Converter").color(colors::SADDLE_BROWN));
        ui.label("Convert degrees Brix to Specific Gravity");
        ui.add_space(10.0);

        input_field(ui, "Brix (°Bx):", &mut self.brix, "Sugar content in degrees Brix (e.g., 15.0)");

        ui.add_space(10.0);

        if calculate_button(ui, "Convert to SG") {
            self.calc_brix_to_sg();
        }
    }

    fn render_dilution_calculator(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("💧 Dilution Calculator").color(colors::SADDLE_BROWN));
        ui.label("Calculate water needed to reduce ABV");
        ui.add_space(10.0);

        input_field(ui, "Current Volume (L):", &mut self.current_vol, "Current volume in liters");
        input_field(ui, "Current ABV (%):", &mut self.current_abv, "Current alcohol percentage");
        input_field(ui, "Target ABV (%):", &mut self.target_abv, "Desired alcohol percentage");

        ui.add_space(10.0);

        if calculate_button(ui, "Calculate Dilution") {
            self.calc_dilution();
        }
    }

    // Calculation methods
    fn calc_abv(&mut self) {
        let calc = match get_calculator("abv") {
            Some(c) => c,
            None => {
                self.result = Some("Error: ABV calculator not found".to_string());
                return;
            }
        };

        let input = CalcInput::new()
            .add_param("og", &self.og)
            .add_param("fg", &self.fg);

        match calc.calculate(input) {
            Ok(res) => {
                self.result = Some(format!("ABV: {:.2}%", res.output.value));
                self.warnings = res.warnings;
                self.metadata = res.metadata;
            }
            Err(e) => {
                self.result = Some(format!("Error: {}", e));
                self.warnings.clear();
                self.metadata.clear();
            }
        }
    }

    fn calc_brix_to_sg(&mut self) {
        let calc = match get_calculator("brix_to_sg") {
            Some(c) => c,
            None => {
                self.result = Some("Error: Brix converter not found".to_string());
                return;
            }
        };

        let brix_val = match Decimal::from_str(&self.brix) {
            Ok(v) => v,
            Err(_) => {
                self.result = Some("Error: Invalid Brix value".to_string());
                return;
            }
        };

        let measurement = match Measurement::brix(brix_val) {
            Ok(m) => m,
            Err(e) => {
                self.result = Some(format!("Error: {}", e));
                return;
            }
        };

        let input = CalcInput::new().add_measurement(measurement);

        match calc.calculate(input) {
            Ok(res) => {
                self.result = Some(format!("Specific Gravity: {:.4}", res.output.value));
                self.warnings = res.warnings;
                self.metadata = res.metadata;
            }
            Err(e) => {
                self.result = Some(format!("Error: {}", e));
                self.warnings.clear();
                self.metadata.clear();
            }
        }
    }

    fn calc_dilution(&mut self) {
        let calc = match get_calculator("dilution") {
            Some(c) => c,
            None => {
                self.result = Some("Error: Dilution calculator not found".to_string());
                return;
            }
        };

        let input = CalcInput::new()
            .add_param("current_volume", &self.current_vol)
            .add_param("current_abv", &self.current_abv)
            .add_param("target_abv", &self.target_abv);

        match calc.calculate(input) {
            Ok(res) => {
                self.result = Some(format!("Water to Add: {:.2} L", res.output.value));
                self.warnings = res.warnings;
                self.metadata = res.metadata;
            }
            Err(e) => {
                self.result = Some(format!("Error: {}", e));
                self.warnings.clear();
                self.metadata.clear();
            }
        }
    }

    fn render_advanced_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Advanced Calculators - Coming Soon");
    }

    fn render_brewing_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Brewing Calculators - Coming Soon");
    }

    fn render_finishing_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Finishing Calculators - Coming Soon");
    }
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