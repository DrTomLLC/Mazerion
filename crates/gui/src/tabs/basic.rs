// Basic tab implementation - ABV, Brix/SG, Dilution

use crate::{MazerionApp, state::{BasicCalculator, colors}};
use eframe::egui::{self, RichText, Rounding};
use mazerion_core::{CalcInput, Measurement};
use std::str::FromStr;
use rust_decimal::Decimal;

impl MazerionApp {
    pub fn render_basic_tab(&mut self, ui: &mut egui::Ui) {
        // Calculator selection
        ui.horizontal(|ui| {
            ui.label(RichText::new("Select Calculator:").strong());
            egui::ComboBox::from_id_source("basic_calc")
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

        self.input_field(ui, "Original Gravity (OG):", &mut self.og, "Starting specific gravity (e.g., 1.090)");
        self.input_field(ui, "Final Gravity (FG):", &mut self.fg, "Ending specific gravity (e.g., 1.010)");

        ui.add_space(10.0);

        if self.calculate_button(ui, "Calculate ABV") {
            self.calc_abv();
        }
    }

    fn render_brix_converter(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("📐 Brix to SG Converter").color(colors::SADDLE_BROWN));
        ui.label("Convert degrees Brix to Specific Gravity");
        ui.add_space(10.0);

        self.input_field(ui, "Brix (°Bx):", &mut self.brix, "Sugar content in degrees Brix (e.g., 15.0)");

        ui.add_space(10.0);

        if self.calculate_button(ui, "Convert to SG") {
            self.calc_brix_to_sg();
        }
    }

    fn render_dilution_calculator(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("💧 Dilution Calculator").color(colors::SADDLE_BROWN));
        ui.label("Calculate water needed to reduce ABV");
        ui.add_space(10.0);

        self.input_field(ui, "Current Volume (L):", &mut self.current_vol, "Current volume in liters");
        self.input_field(ui, "Current ABV (%):", &mut self.current_abv, "Current alcohol percentage");
        self.input_field(ui, "Target ABV (%):", &mut self.target_abv, "Desired alcohol percentage");

        ui.add_space(10.0);

        if self.calculate_button(ui, "Calculate Dilution") {
            self.calc_dilution();
        }
    }

    // Calculation methods
    fn calc_abv(&mut self) {
        let calc = match mazerion_core::traits::get_calculator("abv") {
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
        let calc = match mazerion_core::traits::get_calculator("brix_to_sg") {
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
        let calc = match mazerion_core::traits::get_calculator("dilution") {
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
}