// State management for Mazerion GUI

use eframe::egui::Color32;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabView {
    Basic,
    Advanced,
    Brewing,
    Finishing,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_tab: TabView,
    pub selected_calculator: String,
    pub inputs: HashMap<String, String>,
    pub result: Option<String>,
    pub warnings: Vec<String>,
    pub metadata: Vec<(String, String)>,
    pub show_help: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_tab: TabView::Basic,
            selected_calculator: "abv".to_string(),
            inputs: HashMap::new(),
            result: None,
            warnings: Vec::new(),
            metadata: Vec::new(),
            show_help: false,
        }
    }
}

impl AppState {
    pub fn clear_results(&mut self) {
        self.result = None;
        self.warnings.clear();
        self.metadata.clear();
    }

    pub fn set_calculator(&mut self, calc_id: &str) {
        self.selected_calculator = calc_id.to_string();
        self.clear_results();
        self.inputs.clear();
    }

    pub fn get_input(&self, key: &str) -> String {
        self.inputs.get(key).cloned().unwrap_or_default()
    }

    pub fn set_input(&mut self, key: &str, value: String) {
        self.inputs.insert(key.to_string(), value);
    }
}

// Color theme
pub mod colors {
    use eframe::egui::Color32;

    pub const HONEY_GOLD: Color32 = Color32::from_rgb(240, 165, 0);
    pub const SADDLE_BROWN: Color32 = Color32::from_rgb(139, 69, 19);
    pub const CORNSILK: Color32 = Color32::from_rgb(255, 248, 220);
    pub const GOLDENROD: Color32 = Color32::from_rgb(218, 165, 32);
    pub const FOREST_GREEN: Color32 = Color32::from_rgb(34, 139, 34);
    pub const DARK_ORANGE: Color32 = Color32::from_rgb(255, 140, 0);
    pub const CRIMSON: Color32 = Color32::from_rgb(220, 20, 60);
    pub const LIGHT_CREAM: Color32 = Color32::from_rgb(255, 253, 245);
}