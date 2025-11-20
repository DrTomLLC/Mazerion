//! GUI using egui/eframe.

use eframe::egui;
use mazerion_core::{list_calculators, CalcInput, Measurement, Unit};
use rust_decimal::Decimal;
use std::str::FromStr;

pub struct MazerionApp {
    selected_calc: String,
    og_input: String,
    fg_input: String,
    brix_input: String,
    sg_input: String,
    temp_input: String,
    result: Option<String>,
}

impl Default for MazerionApp {
    fn default() -> Self {
        Self {
            selected_calc: "abv".into(),
            og_input: "1.050".into(),
            fg_input: "1.010".into(),
            brix_input: "12.0".into(),
            sg_input: "1.050".into(),
            temp_input: "25.0".into(),
            result: None,
        }
    }
}

impl eframe::App for MazerionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🍯 Mazerion Calculator");
            ui.separator();
            egui::ComboBox::from_label("Calculator")
                .selected_text(&self.selected_calc)
                .show_ui(ui, |ui| {
                    for calc_id in list_calculators() {
                        ui.selectable_value(&mut self.selected_calc, calc_id.to_string(), calc_id);
                    }
                });
            ui.separator();
            match self.selected_calc.as_str() {
                "abv" => self.ui_abv(ui),
                "brix_to_sg" => self.ui_brix_to_sg(ui),
                "sg_correction" => self.ui_sg_correction(ui),
                _ => {
                    ui.label("Unknown calculator");
                }
            }
            if let Some(ref result) = self.result {
                ui.separator();
                ui.colored_label(egui::Color32::GREEN, result);
            }
        });
    }
}

impl MazerionApp {
    fn ui_abv(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("OG:");
            ui.text_edit_singleline(&mut self.og_input);
        });
        ui.horizontal(|ui| {
            ui.label("FG:");
            ui.text_edit_singleline(&mut self.fg_input);
        });
        if ui.button("Calculate ABV").clicked() {
            self.result = self.calc_abv().ok();
        }
    }

    fn ui_brix_to_sg(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Brix:");
            ui.text_edit_singleline(&mut self.brix_input);
        });
        if ui.button("Convert to SG").clicked() {
            self.result = self.calc_brix_to_sg().ok();
        }
    }

    fn ui_sg_correction(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("SG:");
            ui.text_edit_singleline(&mut self.sg_input);
        });
        ui.horizontal(|ui| {
            ui.label("Temp (°C):");
            ui.text_edit_singleline(&mut self.temp_input);
        });
        if ui.button("Correct SG").clicked() {
            self.result = self.calc_sg_correction().ok();
        }
    }

    fn calc_abv(&self) -> mazerion_core::Result<String> {
        let calc = mazerion_core::get_calculator("abv")
            .ok_or_else(|| mazerion_core::Error::Calculation("Calculator not found".into()))?;
        let input = CalcInput::new()
            .add_param("og", &self.og_input)
            .add_param("fg", &self.fg_input);
        let result = calc.calculate(input)?;
        Ok(format!("ABV: {:.2}%", result.output.value))
    }

    fn calc_brix_to_sg(&self) -> mazerion_core::Result<String> {
        let calc = mazerion_core::get_calculator("brix_to_sg")
            .ok_or_else(|| mazerion_core::Error::Calculation("Calculator not found".into()))?;
        let brix_val = Decimal::from_str(&self.brix_input)
            .map_err(|_| mazerion_core::Error::Parse("Invalid Brix".into()))?;
        let input = CalcInput::new().add_measurement(Measurement::brix(brix_val)?);
        let result = calc.calculate(input)?;
        Ok(format!("SG: {:.4}", result.output.value))
    }

    fn calc_sg_correction(&self) -> mazerion_core::Result<String> {
        let calc = mazerion_core::get_calculator("sg_correction")
            .ok_or_else(|| mazerion_core::Error::Calculation("Calculator not found".into()))?;
        let sg_val = Decimal::from_str(&self.sg_input)
            .map_err(|_| mazerion_core::Error::Parse("Invalid SG".into()))?;
        let temp_val = Decimal::from_str(&self.temp_input)
            .map_err(|_| mazerion_core::Error::Parse("Invalid temp".into()))?;
        let input = CalcInput::new()
            .add_measurement(Measurement::sg(sg_val)?)
            .add_measurement(Measurement::celsius(temp_val)?);
        let result = calc.calculate(input)?;
        Ok(format!("Corrected SG: {:.4}", result.output.value))
    }
}

pub fn run() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mazerion",
        options,
        Box::new(|_cc| Ok(Box::new(MazerionApp::default()))),
    )
}
