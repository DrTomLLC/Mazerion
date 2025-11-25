//! GUI for Mazerion using egui - Complete implementation with all 11 calculators.

mod state;
mod tabs;

use eframe::egui;
use mazerion_core::{get_all_calculators, Calculator, CalcInput, Measurement};
use rust_decimal::Decimal;
use std::str::FromStr;
use state::{AppState, colors, MeasurementSystem, Theme};

pub struct MazerionApp {
    state: AppState,
    calculators: Vec<Box<dyn Calculator>>,

    // Basic tab inputs
    og: String,
    fg: String,
    brix_input: String,
    sg_input: String,
    temp_input: String,
    dilution_vol: String,
    dilution_current_abv: String,
    dilution_target_abv: String,

    // Advanced tab inputs
    blend_vol1: String,
    blend_abv1: String,
    blend_vol2: String,
    blend_abv2: String,
    refract_og: String,
    refract_fg: String,

    // Brewing tab inputs
    tosna_volume: String,
    tosna_target_abv: String,
    tosna_yn_req: String,
    carb_volume: String,
    carb_temp: String,
    carb_target_co2: String,
    carb_method: String,
    carb_sugar_type: String,

    // Finishing tab inputs
    sweet_volume: String,
    sweet_current_sg: String,
    sweet_target_sg: String,
    sweet_type: String,
    sulfite_volume: String,
    sulfite_ph: String,
    sulfite_target_so2: String,
    acid_volume: String,
    acid_current_ph: String,
    acid_target_ph: String,
    acid_type: String,

    // Results
    result: String,
}

impl Default for MazerionApp {
    fn default() -> Self {
        Self {
            state: AppState::default(),
            calculators: get_all_calculators(),

            og: "1.090".to_string(),
            fg: "1.010".to_string(),
            brix_input: "15.0".to_string(),
            sg_input: "1.050".to_string(),
            temp_input: "25.0".to_string(),
            dilution_vol: "19.0".to_string(),
            dilution_current_abv: "14.0".to_string(),
            dilution_target_abv: "12.0".to_string(),

            blend_vol1: "10.0".to_string(),
            blend_abv1: "14.0".to_string(),
            blend_vol2: "10.0".to_string(),
            blend_abv2: "10.0".to_string(),
            refract_og: "24.0".to_string(),
            refract_fg: "8.0".to_string(),

            tosna_volume: "19.0".to_string(),
            tosna_target_abv: "14.0".to_string(),
            tosna_yn_req: "medium".to_string(),
            carb_volume: "19.0".to_string(),
            carb_temp: "20.0".to_string(),
            carb_target_co2: "2.5".to_string(),
            carb_method: "priming".to_string(),
            carb_sugar_type: "table_sugar".to_string(),

            sweet_volume: "19.0".to_string(),
            sweet_current_sg: "0.995".to_string(),
            sweet_target_sg: "1.010".to_string(),
            sweet_type: "honey".to_string(),
            sulfite_volume: "19.0".to_string(),
            sulfite_ph: "3.4".to_string(),
            sulfite_target_so2: "30.0".to_string(),
            acid_volume: "19.0".to_string(),
            acid_current_ph: "3.8".to_string(),
            acid_target_ph: "3.4".to_string(),
            acid_type: "tartaric".to_string(),

            result: String::new(),
        }
    }
}

impl eframe::App for MazerionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_theme(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_header(ui);
            ui.add_space(10.0);
            self.render_tabs(ui);
            ui.add_space(10.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                match self.state.current_tab {
                    state::TabView::Basic => self.render_basic_tab(ui),
                    state::TabView::Advanced => self.render_advanced_tab(ui),
                    state::TabView::Brewing => self.render_brewing_tab(ui),
                    state::TabView::Finishing => self.render_finishing_tab(ui),
                    state::TabView::Settings => self.render_settings(ui),
                }
            });
        });
    }
}

impl MazerionApp {
    // UNIT CONVERSION HELPERS
    fn format_volume(&self, liters: f64) -> String {
        match self.state.settings.measurement_system {
            MeasurementSystem::Standard => format!("{:.2} gal", liters * 0.264172),
            MeasurementSystem::Metric => format!("{:.2} L", liters),
        }
    }

    fn format_temp(&self, celsius: f64) -> String {
        match self.state.settings.measurement_system {
            MeasurementSystem::Standard => format!("{:.1}°F", (celsius * 9.0 / 5.0) + 32.0),
            MeasurementSystem::Metric => format!("{:.1}°C", celsius),
        }
    }

    fn format_weight(&self, grams: f64) -> String {
        match self.state.settings.measurement_system {
            MeasurementSystem::Standard => {
                if grams >= 453.592 {
                    format!("{:.2} lb", grams / 453.592)
                } else {
                    format!("{:.2} oz", grams * 0.035274)
                }
            }
            MeasurementSystem::Metric => {
                if grams >= 1000.0 {
                    format!("{:.2} kg", grams / 1000.0)
                } else {
                    format!("{:.1} g", grams)
                }
            }
        }
    }

    fn volume_label(&self) -> &str {
        match self.state.settings.measurement_system {
            MeasurementSystem::Standard => "Volume (gal):",
            MeasurementSystem::Metric => "Volume (L):",
        }
    }

    fn temp_label(&self) -> &str {
        match self.state.settings.measurement_system {
            MeasurementSystem::Standard => "Temperature (°F):",
            MeasurementSystem::Metric => "Temperature (°C):",
        }
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        match self.state.settings.theme {
            Theme::Light => ctx.set_visuals(egui::Visuals::light()),
            Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
            Theme::System => {}
        }
    }

    fn render_header(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new("🍯 Mazerion").size(32.0).strong());
            ui.label(egui::RichText::new(format!("Professional Beverage Calculator Suite - {} calculators loaded", self.calculators.len())).size(14.0));
        });
    }

    fn render_tabs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 5.0;

            if self.tab_button(ui, "📊 Basic", self.state.current_tab == state::TabView::Basic).clicked() {
                self.state.current_tab = state::TabView::Basic;
                self.result.clear();
            }
            if self.tab_button(ui, "🔬 Advanced", self.state.current_tab == state::TabView::Advanced).clicked() {
                self.state.current_tab = state::TabView::Advanced;
                self.result.clear();
            }
            if self.tab_button(ui, "🍺 Brewing", self.state.current_tab == state::TabView::Brewing).clicked() {
                self.state.current_tab = state::TabView::Brewing;
                self.result.clear();
            }
            if self.tab_button(ui, "✨ Finishing", self.state.current_tab == state::TabView::Finishing).clicked() {
                self.state.current_tab = state::TabView::Finishing;
                self.result.clear();
            }
            if self.tab_button(ui, "⚙️ Settings", self.state.current_tab == state::TabView::Settings).clicked() {
                self.state.current_tab = state::TabView::Settings;
            }
        });
    }

    fn tab_button(&self, ui: &mut egui::Ui, text: &str, active: bool) -> egui::Response {
        let color = if active { colors::HONEY_GOLD } else { colors::LIGHT_CREAM };
        let text_color = if active { egui::Color32::WHITE } else { colors::SADDLE_BROWN };
        ui.add(egui::Button::new(egui::RichText::new(text).color(text_color).size(14.0))
            .fill(color)
            .corner_radius(5.0)
            .min_size(egui::Vec2::new(120.0, 35.0)))
    }

    // BASIC TAB
    fn render_basic_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("📊 Basic Calculators");
        ui.add_space(10.0);

        // ABV Calculator
        ui.group(|ui| {
            ui.heading("🍺 ABV Calculator");
            ui.label("Calculate alcohol by volume from gravity readings");
            egui::Grid::new("abv_grid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                ui.label("Original Gravity (OG):");
                ui.text_edit_singleline(&mut self.og);
                ui.end_row();
                ui.label("Final Gravity (FG):");
                ui.text_edit_singleline(&mut self.fg);
                ui.end_row();
            });
            if ui.button("📊 Calculate ABV").clicked() {
                self.calc_abv();
            }
        });

        ui.add_space(10.0);

        // Brix to SG Converter
        ui.group(|ui| {
            ui.heading("📐 Brix to SG Converter");
            ui.label("Convert degrees Brix to Specific Gravity");
            egui::Grid::new("brix_grid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                ui.label("Brix (°Bx):");
                ui.text_edit_singleline(&mut self.brix_input);
                ui.end_row();
            });
            if ui.button("🔄 Convert to SG").clicked() {
                self.calc_brix_to_sg();
            }
        });

        ui.add_space(10.0);

        // SG Temperature Correction
        ui.group(|ui| {
            ui.heading("🌡️ SG Temperature Correction");
            ui.label("Adjust gravity readings for temperature (calibrated at 20°C/68°F)");
            egui::Grid::new("sg_corr_grid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                ui.label("Measured SG:");
                ui.text_edit_singleline(&mut self.sg_input);
                ui.end_row();
                ui.label(self.temp_label());
                ui.text_edit_singleline(&mut self.temp_input);
                ui.end_row();
            });
            if ui.button("✅ Correct for Temperature").clicked() {
                self.calc_sg_correction();
            }
        });

        ui.add_space(10.0);

        // Dilution Calculator
        ui.group(|ui| {
            ui.heading("💧 Dilution Calculator");
            ui.label("Calculate water needed to reduce ABV");
            egui::Grid::new("dilution_grid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                ui.label(self.volume_label());
                ui.text_edit_singleline(&mut self.dilution_vol);
                ui.end_row();
                ui.label("Current ABV (%):");
                ui.text_edit_singleline(&mut self.dilution_current_abv);
                ui.end_row();
                ui.label("Target ABV (%):");
                ui.text_edit_singleline(&mut self.dilution_target_abv);
                ui.end_row();
            });
            if ui.button("💧 Calculate Water Needed").clicked() {
                self.calc_dilution();
            }
        });

        self.render_result(ui);
    }

    // ADVANCED TAB
    fn render_advanced_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔬 Advanced Calculators");
        ui.add_space(10.0);

        // Blending Calculator
        ui.group(|ui| {
            ui.heading("🔀 Blending Calculator");
            ui.label("Calculate final ABV when mixing two batches");
            egui::Grid::new("blend_grid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                ui.label(format!("Batch 1 {}", self.volume_label()));
                ui.text_edit_singleline(&mut self.blend_vol1);
                ui.end_row();
                ui.label("Batch 1 ABV (%):");
                ui.text_edit_singleline(&mut self.blend_abv1);
                ui.end_row();
                ui.label(format!("Batch 2 {}", self.volume_label()));
                ui.text_edit_singleline(&mut self.blend_vol2);
                ui.end_row();
                ui.label("Batch 2 ABV (%):");
                ui.text_edit_singleline(&mut self.blend_abv2);
                ui.end_row();
            });
            if ui.button("🔀 Calculate Blend").clicked() {
                self.calc_blending();
            }
        });

        ui.add_space(10.0);

        // Refractometer Correction
        ui.group(|ui| {
            ui.heading("🔍 Refractometer Correction");
            ui.label("Correct refractometer readings for alcohol (Terrill cubic equation)");
            egui::Grid::new("refract_grid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                ui.label("Original Brix (°Bx):");
                ui.text_edit_singleline(&mut self.refract_og);
                ui.end_row();
                ui.label("Current Brix (°Bx):");
                ui.text_edit_singleline(&mut self.refract_fg);
                ui.end_row();
            });
            if ui.button("🔍 Calculate True SG").clicked() {
                self.calc_refractometer();
            }
        });

        self.render_result(ui);
    }

    // BREWING TAB
    fn render_brewing_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("🍺 Brewing Calculators");
        ui.add_space(10.0);

        // TOSNA Nutrition Calculator
        ui.group(|ui| {
            ui.heading("🧪 TOSNA Nutrition Calculator");
            ui.label("Calculate Fermaid-O schedule using TOSNA 2.0 protocol");
            egui::Grid::new("tosna_grid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                ui.label(self.volume_label());
                ui.text_edit_singleline(&mut self.tosna_volume);
                ui.end_row();
                ui.label("Target ABV (%):");
                ui.text_edit_singleline(&mut self.tosna_target_abv);
                ui.end_row();
                ui.label("Yeast Nitrogen Need:");
                egui::ComboBox::from_id_source("yn_req")
                    .selected_text(&self.tosna_yn_req)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.tosna_yn_req, "low".to_string(), "Low (DV10, QA23)");
                        ui.selectable_value(&mut self.tosna_yn_req, "medium".to_string(), "Medium (most yeasts)");
                        ui.selectable_value(&mut self.tosna_yn_req, "high".to_string(), "High (EC-1118, K1-V1116)");
                    });
                ui.end_row();
            });
            if ui.button("🧪 Calculate TOSNA Schedule").clicked() {
                self.calc_nutrition();
            }
        });

        ui.add_space(10.0);

        // Carbonation Calculator
        ui.group(|ui| {
            ui.heading("🫧 Carbonation Calculator");
            ui.label("Calculate priming sugar or keg PSI for carbonation");
            egui::Grid::new("carb_grid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                ui.label(self.volume_label());
                ui.text_edit_singleline(&mut self.carb_volume);
                ui.end_row();
                ui.label(self.temp_label());
                ui.text_edit_singleline(&mut self.carb_temp);
                ui.end_row();
                ui.label("Target CO₂ (volumes):");
                ui.text_edit_singleline(&mut self.carb_target_co2);
                ui.end_row();
                ui.label("Method:");
                egui::ComboBox::from_id_source("carb_method")
                    .selected_text(&self.carb_method)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.carb_method, "priming".to_string(), "Bottle Priming");
                        ui.selectable_value(&mut self.carb_method, "keg".to_string(), "Force Carbonation (Keg)");
                    });
                ui.end_row();
                if self.carb_method == "priming" {
                    ui.label("Sugar Type:");
                    egui::ComboBox::from_id_source("sugar_type")
                        .selected_text(&self.carb_sugar_type)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.carb_sugar_type, "table_sugar".to_string(), "Table Sugar (Sucrose)");
                            ui.selectable_value(&mut self.carb_sugar_type, "corn_sugar".to_string(), "Corn Sugar (Dextrose)");
                            ui.selectable_value(&mut self.carb_sugar_type, "honey".to_string(), "Honey");
                            ui.selectable_value(&mut self.carb_sugar_type, "dme".to_string(), "Dry Malt Extract");
                        });
                    ui.end_row();
                }
            });
            if ui.button("🫧 Calculate Carbonation").clicked() {
                self.calc_carbonation();
            }
        });

        self.render_result(ui);
    }

    // FINISHING TAB
    fn render_finishing_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("✨ Finishing Calculators");
        ui.add_space(10.0);

        // Backsweetening Calculator
        ui.group(|ui| {
            ui.heading("🍯 Backsweetening Calculator");
            ui.colored_label(egui::Color32::from_rgb(255, 100, 0), "⚠️ CRITICAL: Stabilize with K-meta and K-sorbate FIRST!");
            ui.label("Calculate sweetener additions to reach target gravity");
            egui::Grid::new("sweet_grid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                ui.label(self.volume_label());
                ui.text_edit_singleline(&mut self.sweet_volume);
                ui.end_row();
                ui.label("Current SG:");
                ui.text_edit_singleline(&mut self.sweet_current_sg);
                ui.end_row();
                ui.label("Target SG:");
                ui.text_edit_singleline(&mut self.sweet_target_sg);
                ui.end_row();
                ui.label("Sweetener Type:");
                egui::ComboBox::from_id_source("sweetener")
                    .selected_text(&self.sweet_type)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.sweet_type, "honey".to_string(), "Honey");
                        ui.selectable_value(&mut self.sweet_type, "table_sugar".to_string(), "Table Sugar");
                        ui.selectable_value(&mut self.sweet_type, "agave".to_string(), "Agave Nectar");
                        ui.selectable_value(&mut self.sweet_type, "maple_syrup".to_string(), "Maple Syrup");
                    });
                ui.end_row();
            });
            if ui.button("🍯 Calculate Sweetener Amount").clicked() {
                self.calc_backsweetening();
            }
        });

        ui.add_space(10.0);

        // Sulfite Calculator
        ui.group(|ui| {
            ui.heading("🛡️ Sulfite Calculator");
            ui.label("Calculate K-meta additions with pH-dependent effectiveness");
            egui::Grid::new("sulfite_grid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                ui.label(self.volume_label());
                ui.text_edit_singleline(&mut self.sulfite_volume);
                ui.end_row();
                ui.label("pH:");
                ui.text_edit_singleline(&mut self.sulfite_ph);
                ui.end_row();
                ui.label("Target Free SO₂ (ppm):");
                ui.text_edit_singleline(&mut self.sulfite_target_so2);
                ui.end_row();
            });
            if ui.button("🛡️ Calculate Sulfite Addition").clicked() {
                self.calc_sulfite();
            }
        });

        ui.add_space(10.0);

        // Acid Addition Calculator
        ui.group(|ui| {
            ui.heading("🍋 Acid Addition Calculator");
            ui.label("Calculate acid additions to adjust pH");
            egui::Grid::new("acid_grid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                ui.label(self.volume_label());
                ui.text_edit_singleline(&mut self.acid_volume);
                ui.end_row();
                ui.label("Current pH:");
                ui.text_edit_singleline(&mut self.acid_current_ph);
                ui.end_row();
                ui.label("Target pH:");
                ui.text_edit_singleline(&mut self.acid_target_ph);
                ui.end_row();
                ui.label("Acid Type:");
                egui::ComboBox::from_id_source("acid_type")
                    .selected_text(&self.acid_type)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.acid_type, "tartaric".to_string(), "Tartaric (strongest, wine)");
                        ui.selectable_value(&mut self.acid_type, "citric".to_string(), "Citric (bright, fruity)");
                        ui.selectable_value(&mut self.acid_type, "malic".to_string(), "Malic (soft, apple-like)");
                        ui.selectable_value(&mut self.acid_type, "lactic".to_string(), "Lactic (smooth, creamy)");
                    });
                ui.end_row();
            });
            if ui.button("🍋 Calculate Acid Addition").clicked() {
                self.calc_acid_addition();
            }
        });

        self.render_result(ui);
    }

    // SETTINGS TAB
    fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("⚙️ Settings");
        ui.add_space(15.0);

        egui::Grid::new("settings_grid")
            .num_columns(2)
            .spacing([40.0, 15.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Theme:").size(16.0).strong());
                egui::ComboBox::from_id_source("theme_selector")
                    .selected_text(format!("{:?}", self.state.settings.theme))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.state.settings.theme, Theme::Light, "☀️ Light");
                        ui.selectable_value(&mut self.state.settings.theme, Theme::Dark, "🌙 Dark");
                        ui.selectable_value(&mut self.state.settings.theme, Theme::System, "💻 System");
                    });
                ui.end_row();

                ui.label(egui::RichText::new("Measurement System:").size(16.0).strong());
                egui::ComboBox::from_id_source("measurement_selector")
                    .selected_text(match self.state.settings.measurement_system {
                        MeasurementSystem::Standard => "🇺🇸 Standard (Imperial/US)",
                        MeasurementSystem::Metric => "🌍 Metric",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.state.settings.measurement_system, MeasurementSystem::Standard, "🇺🇸 Standard (Imperial/US) - Default");
                        ui.selectable_value(&mut self.state.settings.measurement_system, MeasurementSystem::Metric, "🌍 Metric");
                    });
                ui.end_row();
            });

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        ui.label(egui::RichText::new("💡 Theme changes apply immediately!").size(13.0).color(egui::Color32::DARK_GRAY));
        ui.label(egui::RichText::new("📏 Measurement system changes labels and results.").size(13.0).color(egui::Color32::DARK_GRAY));
    }

    fn render_result(&self, ui: &mut egui::Ui) {
        if !self.result.is_empty() {
            ui.add_space(15.0);
            ui.separator();
            ui.add_space(10.0);

            egui::Frame::none()
                .fill(egui::Color32::from_rgba_premultiplied(200, 255, 200, 30))
                .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(50, 150, 50)))
                .corner_radius(8.0)
                .inner_margin(15.0)
                .show(ui, |ui| {
                    ui.label(egui::RichText::new(&self.result).size(14.0));
                });
        }
    }

    // CALCULATION METHODS
    fn calc_abv(&mut self) {
        match self.calculators.iter().find(|c| c.id() == "abv") {
            Some(calc) => {
                let input = CalcInput::new()
                    .add_param("og", &self.og)
                    .add_param("fg", &self.fg);
                match calc.calculate(input) {
                    Ok(res) => {
                        self.result = format!("✅ ABV: {:.2}%", res.output.value);
                        for w in &res.warnings { self.result.push_str(&format!("\n⚠️ {}", w)); }
                        for (k, v) in &res.metadata { self.result.push_str(&format!("\nℹ️ {}: {}", k, v)); }
                    }
                    Err(e) => self.result = format!("❌ Error: {}", e),
                }
            }
            None => self.result = "❌ ABV calculator not found".to_string(),
        }
    }

    fn calc_brix_to_sg(&mut self) {
        match self.calculators.iter().find(|c| c.id() == "brix_to_sg") {
            Some(calc) => {
                match Decimal::from_str(&self.brix_input) {
                    Ok(brix_val) => {
                        match Measurement::brix(brix_val) {
                            Ok(meas) => {
                                let input = CalcInput::new().add_measurement(meas);
                                match calc.calculate(input) {
                                    Ok(res) => {
                                        self.result = format!("✅ Specific Gravity: {:.4}", res.output.value);
                                        for w in &res.warnings { self.result.push_str(&format!("\n⚠️ {}", w)); }
                                    }
                                    Err(e) => self.result = format!("❌ Error: {}", e),
                                }
                            }
                            Err(e) => self.result = format!("❌ Invalid Brix: {}", e),
                        }
                    }
                    Err(_) => self.result = "❌ Invalid Brix value".to_string(),
                }
            }
            None => self.result = "❌ Brix converter not found".to_string(),
        }
    }

    fn calc_sg_correction(&mut self) {
        match self.calculators.iter().find(|c| c.id() == "sg_correction") {
            Some(calc) => {
                match (Decimal::from_str(&self.sg_input), Decimal::from_str(&self.temp_input)) {
                    (Ok(sg_val), Ok(temp_val)) => {
                        match (Measurement::sg(sg_val), Measurement::celsius(temp_val)) {
                            (Ok(sg_meas), Ok(temp_meas)) => {
                                let input = CalcInput::new()
                                    .add_measurement(sg_meas)
                                    .add_measurement(temp_meas);
                                match calc.calculate(input) {
                                    Ok(res) => {
                                        self.result = format!("✅ Corrected SG: {:.4}", res.output.value);
                                        for w in &res.warnings { self.result.push_str(&format!("\n⚠️ {}", w)); }
                                        for (k, v) in &res.metadata { self.result.push_str(&format!("\nℹ️ {}: {}", k, v)); }
                                    }
                                    Err(e) => self.result = format!("❌ Error: {}", e),
                                }
                            }
                            _ => self.result = "❌ Invalid SG or temperature".to_string(),
                        }
                    }
                    _ => self.result = "❌ Invalid input values".to_string(),
                }
            }
            None => self.result = "❌ SG correction calculator not found".to_string(),
        }
    }

    fn calc_dilution(&mut self) {
        match self.calculators.iter().find(|c| c.id() == "dilution") {
            Some(calc) => {
                let input = CalcInput::new()
                    .add_param("current_volume", &self.dilution_vol)
                    .add_param("current_abv", &self.dilution_current_abv)
                    .add_param("target_abv", &self.dilution_target_abv);
                match calc.calculate(input) {
                    Ok(res) => {
                        let liters = res.output.value.to_string().parse::<f64>().unwrap_or(0.0);
                        self.result = format!("✅ Water to Add: {}", self.format_volume(liters));
                        for w in &res.warnings { self.result.push_str(&format!("\n⚠️ {}", w)); }
                        for (k, v) in &res.metadata { self.result.push_str(&format!("\nℹ️ {}: {}", k, v)); }
                    }
                    Err(e) => self.result = format!("❌ Error: {}", e),
                }
            }
            None => self.result = "❌ Dilution calculator not found".to_string(),
        }
    }

    fn calc_blending(&mut self) {
        match self.calculators.iter().find(|c| c.id() == "blending") {
            Some(calc) => {
                let input = CalcInput::new()
                    .add_param("volume1", &self.blend_vol1)
                    .add_param("abv1", &self.blend_abv1)
                    .add_param("volume2", &self.blend_vol2)
                    .add_param("abv2", &self.blend_abv2);
                match calc.calculate(input) {
                    Ok(res) => {
                        self.result = format!("✅ Blended ABV: {:.2}%", res.output.value);
                        for w in &res.warnings { self.result.push_str(&format!("\n⚠️ {}", w)); }
                        for (k, v) in &res.metadata { self.result.push_str(&format!("\nℹ️ {}: {}", k, v)); }
                    }
                    Err(e) => self.result = format!("❌ Error: {}", e),
                }
            }
            None => self.result = "❌ Blending calculator not found".to_string(),
        }
    }

    fn calc_refractometer(&mut self) {
        match self.calculators.iter().find(|c| c.id() == "refractometer") {
            Some(calc) => {
                match Decimal::from_str(&self.refract_og) {
                    Ok(og_brix) => {
                        match Measurement::brix(og_brix) {
                            Ok(meas) => {
                                let input = CalcInput::new()
                                    .add_measurement(meas)
                                    .add_param("current_brix", &self.refract_fg);
                                match calc.calculate(input) {
                                    Ok(res) => {
                                        self.result = format!("✅ Corrected FG: {:.4}", res.output.value);
                                        for w in &res.warnings { self.result.push_str(&format!("\n⚠️ {}", w)); }
                                        for (k, v) in &res.metadata { self.result.push_str(&format!("\nℹ️ {}: {}", k, v)); }
                                    }
                                    Err(e) => self.result = format!("❌ Error: {}", e),
                                }
                            }
                            Err(e) => self.result = format!("❌ Invalid Brix: {}", e),
                        }
                    }
                    Err(_) => self.result = "❌ Invalid Brix value".to_string(),
                }
            }
            None => self.result = "❌ Refractometer calculator not found".to_string(),
        }
    }

    fn calc_nutrition(&mut self) {
        match self.calculators.iter().find(|c| c.id() == "nutrition") {
            Some(calc) => {
                let input = CalcInput::new()
                    .add_param("volume", &self.tosna_volume)
                    .add_param("target_abv", &self.tosna_target_abv)
                    .add_param("yn_requirement", &self.tosna_yn_req);
                match calc.calculate(input) {
                    Ok(res) => {
                        let grams = res.output.value.to_string().parse::<f64>().unwrap_or(0.0);
                        self.result = format!("✅ Total Fermaid-O: {}", self.format_weight(grams));
                        for w in &res.warnings { self.result.push_str(&format!("\n⚠️ {}", w)); }
                        for (k, v) in &res.metadata { self.result.push_str(&format!("\nℹ️ {}: {}", k, v)); }
                    }
                    Err(e) => self.result = format!("❌ Error: {}", e),
                }
            }
            None => self.result = "❌ Nutrition calculator not found".to_string(),
        }
    }

    fn calc_carbonation(&mut self) {
        match self.calculators.iter().find(|c| c.id() == "carbonation") {
            Some(calc) => {
                let input = CalcInput::new()
                    .add_param("volume", &self.carb_volume)
                    .add_param("temperature", &self.carb_temp)
                    .add_param("target_co2", &self.carb_target_co2)
                    .add_param("method", &self.carb_method)
                    .add_param("sugar_type", &self.carb_sugar_type);
                match calc.calculate(input) {
                    Ok(res) => {
                        if self.carb_method == "priming" {
                            let grams = res.output.value.to_string().parse::<f64>().unwrap_or(0.0);
                            self.result = format!("✅ Priming Sugar: {}", self.format_weight(grams));
                        } else {
                            self.result = format!("✅ Target PSI: {:.1}", res.output.value);
                        }
                        for w in &res.warnings { self.result.push_str(&format!("\n⚠️ {}", w)); }
                        for (k, v) in &res.metadata { self.result.push_str(&format!("\nℹ️ {}: {}", k, v)); }
                    }
                    Err(e) => self.result = format!("❌ Error: {}", e),
                }
            }
            None => self.result = "❌ Carbonation calculator not found".to_string(),
        }
    }

    fn calc_backsweetening(&mut self) {
        match self.calculators.iter().find(|c| c.id() == "backsweetening") {
            Some(calc) => {
                match Decimal::from_str(&self.sweet_current_sg) {
                    Ok(sg_val) => {
                        match Measurement::sg(sg_val) {
                            Ok(meas) => {
                                let input = CalcInput::new()
                                    .add_measurement(meas)
                                    .add_param("volume", &self.sweet_volume)
                                    .add_param("target_sg", &self.sweet_target_sg)
                                    .add_param("sweetener", &self.sweet_type);
                                match calc.calculate(input) {
                                    Ok(res) => {
                                        let grams = res.output.value.to_string().parse::<f64>().unwrap_or(0.0);
                                        let sweetener_name = match self.sweet_type.as_str() {
                                            "honey" => "Honey",
                                            "table_sugar" => "Table Sugar",
                                            "agave" => "Agave",
                                            "maple_syrup" => "Maple Syrup",
                                            _ => "Sweetener"
                                        };
                                        self.result = format!("✅ {} Needed: {}", sweetener_name, self.format_weight(grams));
                                        for w in &res.warnings { self.result.push_str(&format!("\n⚠️ {}", w)); }
                                        for (k, v) in &res.metadata { self.result.push_str(&format!("\nℹ️ {}: {}", k, v)); }
                                    }
                                    Err(e) => self.result = format!("❌ Error: {}", e),
                                }
                            }
                            Err(e) => self.result = format!("❌ Invalid SG: {}", e),
                        }
                    }
                    Err(_) => self.result = "❌ Invalid SG value".to_string(),
                }
            }
            None => self.result = "❌ Backsweetening calculator not found".to_string(),
        }
    }

    fn calc_sulfite(&mut self) {
        match self.calculators.iter().find(|c| c.id() == "sulfite") {
            Some(calc) => {
                match Decimal::from_str(&self.sulfite_ph) {
                    Ok(ph_val) => {
                        match Measurement::ph(ph_val) {
                            Ok(meas) => {
                                let input = CalcInput::new()
                                    .add_measurement(meas)
                                    .add_param("volume", &self.sulfite_volume)
                                    .add_param("target_free_so2", &self.sulfite_target_so2);
                                match calc.calculate(input) {
                                    Ok(res) => {
                                        let grams = res.output.value.to_string().parse::<f64>().unwrap_or(0.0);
                                        self.result = format!("✅ K-meta Needed: {}", self.format_weight(grams));
                                        for w in &res.warnings { self.result.push_str(&format!("\n⚠️ {}", w)); }
                                        for (k, v) in &res.metadata { self.result.push_str(&format!("\nℹ️ {}: {}", k, v)); }
                                    }
                                    Err(e) => self.result = format!("❌ Error: {}", e),
                                }
                            }
                            Err(e) => self.result = format!("❌ Invalid pH: {}", e),
                        }
                    }
                    Err(_) => self.result = "❌ Invalid pH value".to_string(),
                }
            }
            None => self.result = "❌ Sulfite calculator not found".to_string(),
        }
    }

    fn calc_acid_addition(&mut self) {
        match self.calculators.iter().find(|c| c.id() == "acid_addition") {
            Some(calc) => {
                match Decimal::from_str(&self.acid_current_ph) {
                    Ok(ph_val) => {
                        match Measurement::ph(ph_val) {
                            Ok(meas) => {
                                let input = CalcInput::new()
                                    .add_measurement(meas)
                                    .add_param("volume", &self.acid_volume)
                                    .add_param("target_ph", &self.acid_target_ph)
                                    .add_param("acid_type", &self.acid_type);
                                match calc.calculate(input) {
                                    Ok(res) => {
                                        let grams = res.output.value.to_string().parse::<f64>().unwrap_or(0.0);
                                        let acid_name = match self.acid_type.as_str() {
                                            "tartaric" => "Tartaric",
                                            "citric" => "Citric",
                                            "malic" => "Malic",
                                            "lactic" => "Lactic",
                                            _ => "Acid"
                                        };
                                        self.result = format!("✅ {} Acid Needed: {}", acid_name, self.format_weight(grams));
                                        for w in &res.warnings { self.result.push_str(&format!("\n⚠️ {}", w)); }
                                        for (k, v) in &res.metadata { self.result.push_str(&format!("\nℹ️ {}: {}", k, v)); }
                                    }
                                    Err(e) => self.result = format!("❌ Error: {}", e),
                                }
                            }
                            Err(e) => self.result = format!("❌ Invalid pH: {}", e),
                        }
                    }
                    Err(_) => self.result = "❌ Invalid pH value".to_string(),
                }
            }
            None => self.result = "❌ Acid addition calculator not found".to_string(),
        }
    }
}

pub fn run() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0])
            .with_min_inner_size([900.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Mazerion - Professional Beverage Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(MazerionApp::default()))),
    )
}