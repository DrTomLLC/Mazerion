// Advanced tab implementation - Blending, Refractometer, SG Correction

use crate::{MazerionApp, state::{AdvancedCalculator, colors}};
use eframe::egui::{self, RichText, Rounding};
use mazerion_core::{CalcInput, Measurement};
use std::str::FromStr;
use rust_decimal::Decimal;

impl MazerionApp {
    pub fn render_advanced_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Select Calculator:").strong());
            egui::ComboBox::from_id_source("advanced_calc")
                .selected_text(self.get_advanced_calc_name())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.state.advanced_calc, AdvancedCalculator::Blending, "Blending Calculator");
                    ui.selectable_value(&mut self.state.advanced_calc, AdvancedCalculator::Refractometer, "Refractometer Correction");
                    ui.selectable_value(&mut self.state.advanced_calc, AdvancedCalculator::SgCorrection, "SG Temperature Correction");
                });
        });

        ui.add_space(10.0);

        egui::Frame::none()
            .fill(colors::LIGHT_CREAM)
            .stroke(egui::Stroke::new(1.5, colors::HONEY_GOLD))
            .rounding(Rounding::same(8.0))
            .inner_margin(15.0)
            .show(ui, |ui| {
                match self.state.advanced_calc {
                    AdvancedCalculator::Blending => self.render_blending_calculator(ui),
                    AdvancedCalculator::Refractometer => self.render_refractometer_calculator(ui),
                    AdvancedCalculator::SgCorrection => self.render_sg_correction_calculator(ui),
                }
            });
    }

    fn get_advanced_calc_name(&self) -> &str {
        match self.state.advanced_calc {
            AdvancedCalculator::Blending => "Blending Calculator",
            AdvancedCalculator::Refractometer => "Refractometer Correction",
            AdvancedCalculator::SgCorrection => "SG Temperature Correction",
        }
    }

    fn render_blending_calculator(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("🔀 Blending Calculator").color(colors::SADDLE_BROWN));
        ui.label("Calculate final properties when mixing two batches");
        ui.add_space(10.0);

        ui.label(RichText::new("Batch 1:").strong().color(colors::GOLDENROD));
        self.input_field(ui, "Volume (L):", &mut self.vol1, "Volume of first batch");
        self.input_field(ui, "ABV (%):", &mut self.abv1, "ABV of first batch");

        ui.add_space(8.0);
        ui.label(RichText::new("Batch 2:").strong().color(colors::GOLDENROD));
        self.input_field(ui, "Volume (L):", &mut self.vol2, "Volume of second batch");
        self.input_field(ui, "ABV (%):", &mut self.abv2, "ABV of second batch");

        ui.add_space(10.0);

        if self.calculate_button(ui, "Calculate Blend") {
            self.calc_blending();
        }
    }

    fn render_refractometer_calculator(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("🔍 Refractometer Correction").color(colors::SADDLE_BROWN));
        ui.label("Correct refractometer readings for alcohol presence (Terrill cubic)");
        ui.add_space(10.0);

        self.input_field(ui, "Original Brix (°Bx):", &mut self.orig_brix, "Original reading before fermentation");
        self.input_field(ui, "Current Brix (°Bx):", &mut self.curr_brix, "Current reading during/after fermentation");

        ui.add_space(10.0);

        if self.calculate_button(ui, "Calculate True SG") {
            self.calc_refractometer();
        }
    }

    fn render_sg_correction_calculator(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("🌡️ SG Temperature Correction").color(colors::SADDLE_BROWN));
        ui.label("Correct gravity readings for temperature (calibrated at 20°C)");
        ui.add_space(10.0);

        self.input_field(ui, "Measured SG:", &mut self.sg, "Specific gravity reading");
        self.input_field(ui, "Temperature (°C):", &mut self.temp, "Temperature at measurement");

        ui.add_space(10.0);

        if self.calculate_button(ui, "Correct for Temperature") {
            self.calc_sg_correction();
        }
    }

    // Calculation methods
    fn calc_blending(&mut self) {
        let calc = match mazerion_core::traits::get_calculator("blending") {
            Some(c) => c,
            None => {
                self.result = Some("Error: Blending calculator not found".to_string());
                return;
            }
        };

        let input = CalcInput::new()
            .add_param("volume1", &self.vol1)
            .add_param("abv1", &self.abv1)
            .add_param("volume2", &self.vol2)
            .add_param("abv2", &self.abv2);

        match calc.calculate(input) {
            Ok(res) => {
                self.result = Some(format!("Blended ABV: {:.2}%", res.output.value));
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

    fn calc_refractometer(&mut self) {
        let calc = match mazerion_core::traits::get_calculator("refractometer") {
            Some(c) => c,
            None => {
                self.result = Some("Error: Refractometer calculator not found".to_string());
                return;
            }
        };

        let orig_brix_val = match Decimal::from_str(&self.orig_brix) {
            Ok(v) => v,
            Err(_) => {
                self.result = Some("Error: Invalid original Brix value".to_string());
                return;
            }
        };

        let measurement = match Measurement::brix(orig_brix_val) {
            Ok(m) => m,
            Err(e) => {
                self.result = Some(format!("Error: {}", e));
                return;
            }
        };

        let input = CalcInput::new()
            .add_measurement(measurement)
            .add_param("current_brix", &self.curr_brix);

        match calc.calculate(input) {
            Ok(res) => {
                self.result = Some(format!("Corrected FG: {:.4}", res.output.value));
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

    fn calc_sg_correction(&mut self) {
        let calc = match mazerion_core::traits::get_calculator("sg_correction") {
            Some(c) => c,
            None => {
                self.result = Some("Error: SG correction calculator not found".to_string());
                return;
            }
        };

        let sg_val = match Decimal::from_str(&self.sg) {
            Ok(v) => v,
            Err(_) => {
                self.result = Some("Error: Invalid SG value".to_string());
                return;
            }
        };

        let temp_val = match Decimal::from_str(&self.temp) {
            Ok(v) => v,
            Err(_) => {
                self.result = Some("Error: Invalid temperature value".to_string());
                return;
            }
        };

        let sg_meas = match Measurement::sg(sg_val) {
            Ok(m) => m,
            Err(e) => {
                self.result = Some(format!("Error: {}", e));
                return;
            }
        };

        let temp_meas = match Measurement::celsius(temp_val) {
            Ok(m) => m,
            Err(e) => {
                self.result = Some(format!("Error: {}", e));
                return;
            }
        };

        let input = CalcInput::new()
            .add_measurement(sg_meas)
            .add_measurement(temp_meas);

        match calc.calculate(input) {
            Ok(res) => {
                self.result = Some(format!("Corrected SG: {:.4}", res.output.value));
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