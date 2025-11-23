// Finishing tab implementation - Backsweetening, Sulfite, Acid Addition

use crate::{MazerionApp, state::{FinishingCalculator, colors}};
use eframe::egui::{self, RichText, Rounding};
use mazerion_core::{CalcInput, Measurement};
use std::str::FromStr;
use rust_decimal::Decimal;

impl MazerionApp {
    pub fn render_finishing_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Select Calculator:").strong());
            egui::ComboBox::from_id_source("finishing_calc")
                .selected_text(self.get_finishing_calc_name())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.state.finishing_calc, FinishingCalculator::Backsweetening, "Backsweetening Calculator");
                    ui.selectable_value(&mut self.state.finishing_calc, FinishingCalculator::Sulfite, "Sulfite Calculator");
                    ui.selectable_value(&mut self.state.finishing_calc, FinishingCalculator::AcidAddition, "Acid Addition Calculator");
                });
        });

        ui.add_space(10.0);

        egui::Frame::none()
            .fill(colors::LIGHT_CREAM)
            .stroke(egui::Stroke::new(1.5, colors::HONEY_GOLD))
            .rounding(Rounding::same(8.0))
            .inner_margin(15.0)
            .show(ui, |ui| {
                match self.state.finishing_calc {
                    FinishingCalculator::Backsweetening => self.render_backsweetening_calculator(ui),
                    FinishingCalculator::Sulfite => self.render_sulfite_calculator(ui),
                    FinishingCalculator::AcidAddition => self.render_acid_calculator(ui),
                }
            });
    }

    fn get_finishing_calc_name(&self) -> &str {
        match self.state.finishing_calc {
            FinishingCalculator::Backsweetening => "Backsweetening Calculator",
            FinishingCalculator::Sulfite => "Sulfite Calculator",
            FinishingCalculator::AcidAddition => "Acid Addition Calculator",
        }
    }

    fn render_backsweetening_calculator(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("🍯 Backsweetening Calculator").color(colors::SADDLE_BROWN));
        ui.label("Calculate sweetener additions to reach target sweetness");

        ui.label(RichText::new("⚠️ MUST stabilize before backsweetening!").color(colors::DARK_ORANGE).strong());
        ui.add_space(10.0);

        self.input_field(ui, "Volume (L):", &mut self.sweet_vol, "Total volume to sweeten");
        self.input_field(ui, "Current SG:", &mut self.current_sg, "Current specific gravity");
        self.input_field(ui, "Target SG:", &mut self.target_sg, "Desired final gravity");

        ui.horizontal(|ui| {
            ui.label(RichText::new("Sweetener:").strong());
            egui::ComboBox::from_id_source("sweetener")
                .selected_text(&self.sweetener)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.sweetener, "honey".to_string(), "Honey");
                    ui.selectable_value(&mut self.sweetener, "table_sugar".to_string(), "Table Sugar");
                    ui.selectable_value(&mut self.sweetener, "agave".to_string(), "Agave Nectar");
                    ui.selectable_value(&mut self.sweetener, "maple_syrup".to_string(), "Maple Syrup");
                });
        });

        ui.add_space(10.0);

        if self.calculate_button(ui, "Calculate Sweetener Amount") {
            self.calc_backsweetening();
        }
    }

    fn render_sulfite_calculator(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("🛡️ Sulfite Calculator").color(colors::SADDLE_BROWN));
        ui.label("Calculate K-meta additions with pH-dependent effectiveness");
        ui.add_space(10.0);

        self.input_field(ui, "Volume (L):", &mut self.sulfite_vol, "Total volume to treat");
        self.input_field(ui, "pH:", &mut self.ph, "Current pH (critical for effectiveness!)");
        self.input_field(ui, "Target Free SO₂ (ppm):", &mut self.target_so2, "Desired free SO₂ level (20-50 ppm typical)");

        ui.add_space(10.0);

        if self.calculate_button(ui, "Calculate Sulfite Addition") {
            self.calc_sulfite();
        }
    }

    fn render_acid_calculator(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("🍋 Acid Addition Calculator").color(colors::SADDLE_BROWN));
        ui.label("Calculate acid additions to adjust pH");
        ui.add_space(10.0);

        self.input_field(ui, "Volume (L):", &mut self.acid_vol, "Total volume to treat");
        self.input_field(ui, "Current pH:", &mut self.current_ph, "Current pH measurement");
        self.input_field(ui, "Target pH:", &mut self.target_ph_acid, "Desired pH (must be lower than current)");

        ui.horizontal(|ui| {
            ui.label(RichText::new("Acid Type:").strong());
            egui::ComboBox::from_id_source("acid_type")
                .selected_text(&self.acid_type)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.acid_type, "tartaric".to_string(), "Tartaric (strongest, wine)");
                    ui.selectable_value(&mut self.acid_type, "citric".to_string(), "Citric (bright, fruity)");
                    ui.selectable_value(&mut self.acid_type, "malic".to_string(), "Malic (soft, apple-like)");
                    ui.selectable_value(&mut self.acid_type, "lactic".to_string(), "Lactic (smooth, creamy)");
                });
        });

        ui.add_space(10.0);

        if self.calculate_button(ui, "Calculate Acid Addition") {
            self.calc_acid_addition();
        }
    }

    // Calculation methods
    fn calc_backsweetening(&mut self) {
        let calc = match mazerion_core::traits::get_calculator("backsweetening") {
            Some(c) => c,
            None => {
                self.result = Some("Error: Backsweetening calculator not found".to_string());
                return;
            }
        };

        let current_sg_val = match Decimal::from_str(&self.current_sg) {
            Ok(v) => v,
            Err(_) => {
                self.result = Some("Error: Invalid current SG value".to_string());
                return;
            }
        };

        let sg_meas = match Measurement::sg(current_sg_val) {
            Ok(m) => m,
            Err(e) => {
                self.result = Some(format!("Error: {}", e));
                return;
            }
        };

        let input = CalcInput::new()
            .add_measurement(sg_meas)
            .add_param("volume", &self.sweet_vol)
            .add_param("target_sg", &self.target_sg)
            .add_param("sweetener", &self.sweetener);

        match calc.calculate(input) {
            Ok(res) => {
                self.result = Some(format!("{}: {:.0} g ({:.2} kg)",
                                           match self.sweetener.as_str() {
                                               "honey" => "Honey",
                                               "table_sugar" => "Table Sugar",
                                               "agave" => "Agave",
                                               "maple_syrup" => "Maple Syrup",
                                               _ => "Sweetener"
                                           },
                                           res.output.value,
                                           res.output.value / Decimal::from(1000)
                ));
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

    fn calc_sulfite(&mut self) {
        let calc = match mazerion_core::traits::get_calculator("sulfite") {
            Some(c) => c,
            None => {
                self.result = Some("Error: Sulfite calculator not found".to_string());
                return;
            }
        };

        let ph_val = match Decimal::from_str(&self.ph) {
            Ok(v) => v,
            Err(_) => {
                self.result = Some("Error: Invalid pH value".to_string());
                return;
            }
        };

        let ph_meas = match Measurement::ph(ph_val) {
            Ok(m) => m,
            Err(e) => {
                self.result = Some(format!("Error: {}", e));
                return;
            }
        };

        let input = CalcInput::new()
            .add_measurement(ph_meas)
            .add_param("volume", &self.sulfite_vol)
            .add_param("target_free_so2", &self.target_so2);

        match calc.calculate(input) {
            Ok(res) => {
                self.result = Some(format!("K-meta: {:.2} g", res.output.value));
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

    fn calc_acid_addition(&mut self) {
        let calc = match mazerion_core::traits::get_calculator("acid_addition") {
            Some(c) => c,
            None => {
                self.result = Some("Error: Acid addition calculator not found".to_string());
                return;
            }
        };

        let current_ph_val = match Decimal::from_str(&self.current_ph) {
            Ok(v) => v,
            Err(_) => {
                self.result = Some("Error: Invalid current pH value".to_string());
                return;
            }
        };

        let ph_meas = match Measurement::ph(current_ph_val) {
            Ok(m) => m,
            Err(e) => {
                self.result = Some(format!("Error: {}", e));
                return;
            }
        };

        let input = CalcInput::new()
            .add_measurement(ph_meas)
            .add_param("volume", &self.acid_vol)
            .add_param("target_ph", &self.target_ph_acid)
            .add_param("acid_type", &self.acid_type);

        match calc.calculate(input) {
            Ok(res) => {
                self.result = Some(format!("{} Acid: {:.2} g",
                                           match self.acid_type.as_str() {
                                               "tartaric" => "Tartaric",
                                               "citric" => "Citric",
                                               "malic" => "Malic",
                                               "lactic" => "Lactic",
                                               _ => "Acid"
                                           },
                                           res.output.value
                ));
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
}