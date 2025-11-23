// Brewing tab implementation - Nutrition (TOSNA), Carbonation

use crate::{MazerionApp, state::{BrewingCalculator, colors}};
use eframe::egui::{self, RichText, Rounding};
use mazerion_core::CalcInput;

impl MazerionApp {
    pub fn render_brewing_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Select Calculator:").strong());
            egui::ComboBox::from_id_source("brewing_calc")
                .selected_text(self.get_brewing_calc_name())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.state.brewing_calc, BrewingCalculator::Nutrition, "TOSNA Nutrition Calculator");
                    ui.selectable_value(&mut self.state.brewing_calc, BrewingCalculator::Carbonation, "Carbonation Calculator");
                });
        });

        ui.add_space(10.0);

        egui::Frame::none()
            .fill(colors::LIGHT_CREAM)
            .stroke(egui::Stroke::new(1.5, colors::HONEY_GOLD))
            .rounding(Rounding::same(8.0))
            .inner_margin(15.0)
            .show(ui, |ui| {
                match self.state.brewing_calc {
                    BrewingCalculator::Nutrition => self.render_nutrition_calculator(ui),
                    BrewingCalculator::Carbonation => self.render_carbonation_calculator(ui),
                }
            });
    }

    fn get_brewing_calc_name(&self) -> &str {
        match self.state.brewing_calc {
            BrewingCalculator::Nutrition => "TOSNA Nutrition Calculator",
            BrewingCalculator::Carbonation => "Carbonation Calculator",
        }
    }

    fn render_nutrition_calculator(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("🧪 TOSNA Nutrition Calculator").color(colors::SADDLE_BROWN));
        ui.label("Calculate Fermaid-O schedule using TOSNA 2.0 protocol");
        ui.add_space(10.0);

        self.input_field(ui, "Volume (L):", &mut self.volume, "Total must volume");
        self.input_field(ui, "Target ABV (%):", &mut self.target_abv_brew, "Expected final ABV");

        ui.horizontal(|ui| {
            ui.label(RichText::new("Yeast Nitrogen Needs:").strong());
            egui::ComboBox::from_id_source("yn_req")
                .selected_text(&self.yn_requirement)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.yn_requirement, "low".to_string(), "Low (DV10, QA23)");
                    ui.selectable_value(&mut self.yn_requirement, "medium".to_string(), "Medium (most yeasts)");
                    ui.selectable_value(&mut self.yn_requirement, "high".to_string(), "High (EC-1118, K1-V1116)");
                });
        });

        ui.add_space(10.0);

        if self.calculate_button(ui, "Calculate TOSNA Schedule") {
            self.calc_nutrition();
        }
    }

    fn render_carbonation_calculator(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("🫧 Carbonation Calculator").color(colors::SADDLE_BROWN));
        ui.label("Calculate priming sugar or keg PSI for target carbonation");
        ui.add_space(10.0);

        self.input_field(ui, "Volume (L):", &mut self.volume, "Total volume to carbonate");
        self.input_field(ui, "Temperature (°C):", &mut self.carb_temp, "Current temperature");
        self.input_field(ui, "Target CO₂ (volumes):", &mut self.target_co2, "Desired carbonation level (1.5-4.5)");

        ui.horizontal(|ui| {
            ui.label(RichText::new("Method:").strong());
            egui::ComboBox::from_id_source("carb_method")
                .selected_text(&self.carb_method)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.carb_method, "priming".to_string(), "Bottle Priming");
                    ui.selectable_value(&mut self.carb_method, "keg".to_string(), "Force Carbonation (Keg)");
                });
        });

        if self.carb_method == "priming" {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Sugar Type:").strong());
                egui::ComboBox::from_id_source("sugar_type")
                    .selected_text(&self.sugar_type)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.sugar_type, "table_sugar".to_string(), "Table Sugar (Sucrose)");
                        ui.selectable_value(&mut self.sugar_type, "corn_sugar".to_string(), "Corn Sugar (Dextrose)");
                        ui.selectable_value(&mut self.sugar_type, "honey".to_string(), "Honey");
                        ui.selectable_value(&mut self.sugar_type, "dme".to_string(), "Dry Malt Extract");
                    });
            });
        }

        ui.add_space(10.0);

        if self.calculate_button(ui, "Calculate Carbonation") {
            self.calc_carbonation();
        }
    }

    // Calculation methods
    fn calc_nutrition(&mut self) {
        let calc = match mazerion_core::traits::get_calculator("nutrition") {
            Some(c) => c,
            None => {
                self.result = Some("Error: Nutrition calculator not found".to_string());
                return;
            }
        };

        let input = CalcInput::new()
            .add_param("volume", &self.volume)
            .add_param("target_abv", &self.target_abv_brew)
            .add_param("yn_requirement", &self.yn_requirement);

        match calc.calculate(input) {
            Ok(res) => {
                self.result = Some(format!("Total Fermaid-O: {:.2} g", res.output.value));
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

    fn calc_carbonation(&mut self) {
        let calc = match mazerion_core::traits::get_calculator("carbonation") {
            Some(c) => c,
            None => {
                self.result = Some("Error: Carbonation calculator not found".to_string());
                return;
            }
        };

        let input = CalcInput::new()
            .add_param("volume", &self.volume)
            .add_param("temperature", &self.carb_temp)
            .add_param("target_co2", &self.target_co2)
            .add_param("method", &self.carb_method)
            .add_param("sugar_type", &self.sugar_type);

        match calc.calculate(input) {
            Ok(res) => {
                if self.carb_method == "priming" {
                    self.result = Some(format!("Priming Sugar: {:.1} g", res.output.value));
                } else {
                    self.result = Some(format!("Target PSI: {:.1}", res.output.value));
                }
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