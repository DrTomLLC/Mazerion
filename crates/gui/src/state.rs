//! Application state - COMPLETE with Settings

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    // UI state
    pub current_tab: TabView,

    // Settings
    pub theme: String,
    pub font_size: String,
    pub volume_unit: String,
    pub temp_unit: String,
    pub show_warnings: bool,
    pub show_metadata: bool,
    pub auto_save: bool,
    pub default_batch_size: String,
    pub default_yn_req: String,
    pub default_ferm_temp: String,

    // Basic calculations
    pub og: String,
    pub fg: String,
    pub brix: String,
    pub sg: String,
    pub temp: String,
    pub current_vol: String,
    pub current_abv: String,
    pub target_abv: String,

    // Advanced calculations
    pub vol1: String,
    pub abv1: String,
    pub vol2: String,
    pub abv2: String,
    pub orig_brix: String,
    pub curr_brix: String,

    // Brewing calculations
    pub volume: String,
    pub target_abv_brew: String,
    pub yn_requirement: String,
    pub carb_temp: String,
    pub target_co2: String,
    pub carb_method: String,
    pub sugar_type: String,

    // Finishing calculations
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

    // Results
    pub result: Option<String>,
    pub warnings: Vec<String>,
    pub metadata: Vec<(String, String)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TabView {
    Basic,
    Advanced,
    Brewing,
    Finishing,
    Settings,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_tab: TabView::Basic,

            // Settings defaults
            theme: "soft_blue".to_string(),
            font_size: "medium".to_string(),
            volume_unit: "liters".to_string(),
            temp_unit: "celsius".to_string(),
            show_warnings: true,
            show_metadata: true,
            auto_save: false,
            default_batch_size: "19.0".to_string(),
            default_yn_req: "medium".to_string(),
            default_ferm_temp: "20.0".to_string(),

            // Basic defaults
            og: "1.090".to_string(),
            fg: "1.010".to_string(),
            brix: "15.0".to_string(),
            sg: "1.060".to_string(),
            temp: "20.0".to_string(),
            current_vol: "19.0".to_string(),
            current_abv: "14.0".to_string(),
            target_abv: "10.0".to_string(),

            // Advanced defaults
            vol1: "10.0".to_string(),
            abv1: "14.0".to_string(),
            vol2: "5.0".to_string(),
            abv2: "8.0".to_string(),
            orig_brix: "24.0".to_string(),
            curr_brix: "8.0".to_string(),

            // Brewing defaults
            volume: "19.0".to_string(),
            target_abv_brew: "14.0".to_string(),
            yn_requirement: "medium".to_string(),
            carb_temp: "20.0".to_string(),
            target_co2: "2.5".to_string(),
            carb_method: "priming".to_string(),
            sugar_type: "table_sugar".to_string(),

            // Finishing defaults
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

            // Results
            result: None,
            warnings: Vec::new(),
            metadata: Vec::new(),
        }
    }
}

impl AppState {
    pub fn clear_results(&mut self) {
        self.result = None;
        self.warnings.clear();
        self.metadata.clear();
    }
}