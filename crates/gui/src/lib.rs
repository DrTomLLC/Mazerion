// Modern, beautiful GUI for Mazerion with tabs and full calculator suite
mod state;

use eframe::egui::{self, Color32, RichText, Vec2};
use mazerion_core::CalcInput;

use state::{AppState, TabView, colors};

pub struct MazerionApp {
    state: AppState,
    result: Option<String>,
    warnings: Vec<String>,
    metadata: Vec<(String, String)>,
}

impl Default for MazerionApp {
    fn default() -> Self {
        Self {
            state: AppState::default(),
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
            self.render_calculator_selector(ui);
            ui.add_space(10.0);
            self.render_calculator_ui(ui);
            ui.add_space(10.0);
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
            .min_size(Vec2::new(120.0, 35.0));

        ui.add(button)
    }

    fn render_calculator_selector(&mut self, ui: &mut egui::Ui) {
        let category = match self.state.current_tab {
            TabView::Basic => "Basic Calculations",
            TabView::Advanced => "Advanced",
            TabView::Brewing => "Brewing",
            TabView::Finishing => "Finishing",
        };

        let calculators = mazerion_core::traits::get_all_calculators();
        let filtered: Vec<_> = calculators.iter()
            .filter(|c| c.category() == category)
            .collect();

        if filtered.is_empty() {
            ui.label(RichText::new("No calculators in this category yet").color(colors::DARK_ORANGE));
            return;
        }

        ui.horizontal(|ui| {
            ui.label(RichText::new("Select Calculator:").strong());
            let display_text = if self.state.selected_calculator.is_empty() {
                "Choose..."
            } else {
                &self.state.selected_calculator
            };

            egui::ComboBox::from_id_salt("calc_selector")
                .selected_text(display_text)
                .show_ui(ui, |ui| {
                    for calc in &filtered {
                        if ui.selectable_value(
                            &mut self.state.selected_calculator,
                            calc.id().to_string(),
                            calc.name()
                        ).clicked() {
                            self.clear_results();
                        }
                    }
                });
        });
    }

    fn render_calculator_ui(&mut self, ui: &mut egui::Ui) {
        if self.state.selected_calculator.is_empty() {
            return;
        }

        let Some(calc) = mazerion_core::traits::get_calculator(&self.state.selected_calculator) else {
            ui.label(RichText::new("Calculator not found").color(colors::CRIMSON));
            return;
        };

        egui::Frame::new()
            .fill(colors::LIGHT_CREAM)
            .stroke(egui::Stroke::new(1.5, colors::HONEY_GOLD))
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.heading(RichText::new(calc.name()).color(colors::SADDLE_BROWN));
                ui.label(calc.description());
                ui.add_space(5.0);

                if ui.button(if self.state.show_help { "Hide Help" } else { "Show Help" }).clicked() {
                    self.state.show_help = !self.state.show_help;
                }

                if self.state.show_help {
                    ui.add_space(5.0);
                    ui.label(RichText::new(calc.help_text()).color(Color32::DARK_GRAY));
                }

                ui.add_space(10.0);

                self.render_inputs_for_calculator(ui, &self.state.selected_calculator.clone());

                ui.add_space(10.0);

                if ui.add(
                    egui::Button::new(RichText::new("Calculate").size(16.0).strong())
                        .fill(colors::FOREST_GREEN)
                        .min_size(Vec2::new(200.0, 40.0))
                ).clicked() {
                    self.perform_calculation(calc.as_ref());
                }
            });
    }

    fn render_inputs_for_calculator(&mut self, ui: &mut egui::Ui, calc_id: &str) {
        match calc_id {
            "abv" => {
                self.input_field(ui, "Original Gravity (OG):", "og", "1.090");
                self.input_field(ui, "Final Gravity (FG):", "fg", "1.010");
            }
            "brix_to_sg" => {
                self.input_field(ui, "Brix (°Bx):", "brix", "15.0");
            }
            "sg_correction" => {
                self.input_field(ui, "Measured SG:", "sg", "1.060");
                self.input_field(ui, "Temperature (°C):", "temperature", "22.0");
            }
            "dilution" => {
                self.input_field(ui, "Current Volume (L):", "current_volume", "19.0");
                self.input_field(ui, "Current ABV (%):", "current_abv", "14.0");
                self.input_field(ui, "Target ABV (%):", "target_abv", "10.0");
            }
            "blending" => {
                self.input_field(ui, "Volume 1 (L):", "volume1", "10.0");
                self.input_field(ui, "ABV 1 (%):", "abv1", "14.0");
                self.input_field(ui, "Volume 2 (L):", "volume2", "5.0");
                self.input_field(ui, "ABV 2 (%):", "abv2", "8.0");
            }
            "refractometer" => {
                self.input_field(ui, "Original Brix (°Bx):", "original_brix", "24.0");
                self.input_field(ui, "Current Brix (°Bx):", "current_brix", "8.0");
            }
            _ => {
                self.input_field(ui, "Value:", "value", "1.0");
            }
        }
    }

    fn input_field(&mut self, ui: &mut egui::Ui, label: &str, key: &str, default: &str) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(label).strong());
            let value = self.state.get_input(key);
            let mut temp = if value.is_empty() { default.to_string() } else { value };
            if ui.text_edit_singleline(&mut temp).changed() {
                self.state.set_input(key, temp);
            }
        });
    }

    fn perform_calculation(&mut self, calc: &dyn mazerion_core::Calculator) {
        let mut input = CalcInput::new();

        for (key, value) in &self.state.inputs {
            input = input.add_param(key, value);
        }

        match calc.calculate(input) {
            Ok(result) => {
                self.result = Some(format!("✓ {} {}", result.output.value, result.output.unit));
                self.warnings = result.warnings;
                self.metadata = result.metadata;
            }
            Err(e) => {
                self.result = Some(format!("Error: {}", e));
                self.warnings.clear();
                self.metadata.clear();
            }
        }
    }

    fn render_results(&self, ui: &mut egui::Ui) {
        if self.result.is_none() && self.warnings.is_empty() {
            return;
        }

        ui.separator();
        ui.add_space(5.0);

        egui::Frame::new()
            .fill(colors::LIGHT_CREAM)
            .stroke(egui::Stroke::new(2.0, colors::HONEY_GOLD))
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.heading(RichText::new("📋 Results").color(colors::SADDLE_BROWN));
                ui.add_space(5.0);

                if let Some(ref result) = self.result {
                    ui.label(RichText::new(result)
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

    fn clear_results(&mut self) {
        self.result = None;
        self.warnings.clear();
        self.metadata.clear();
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