//! State management - WITH Conversions tab

// use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabView {
    Basic,
    Advanced,
    Brewing,
    Finishing,
    Conversions,
    Settings,
}

#[derive(Debug, Clone)]
pub struct AppState {
    // Navigation
    pub current_tab: TabView,

    // Input fields - Basic
    pub og: String,
    pub fg: String,
    pub brix: String,
    pub sg: String,
    pub temp: String,
    pub current_vol: String,
    pub current_abv: String,
    pub target_abv: String,

    // Input fields - Advanced
    pub vol1: String,
    pub abv1: String,
    pub vol2: String,
    pub abv2: String,
    pub orig_brix: String,
    pub curr_brix: String,

    // Input fields - Brewing
    pub volume: String,
    pub target_abv_brew: String,
    pub yn_requirement: String,
    pub carb_temp: String,
    pub target_co2: String,
    pub carb_method: String,
    pub sugar_type: String,

    // Input fields - Finishing
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

    // Input fields - Conversions
    pub conv_value: String,
    pub conv_from_unit: String,
    pub conv_to_unit: String,
    pub conv_result: Option<String>,

    // Results
    pub result: Option<String>,
    pub warnings: Vec<String>,
    pub metadata: Vec<(String, String)>,

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
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_tab: TabView::Basic,

            // Basic defaults
            og: "1.090".to_string(),
            fg: "1.010".to_string(),
            brix: "15.0".to_string(),
            sg: "1.060".to_string(),
            temp: "22.0".to_string(),
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

            // Conversions defaults
            conv_value: "1.0".to_string(),
            conv_from_unit: "liters".to_string(),
            conv_to_unit: "gallons".to_string(),
            conv_result: None,

            // Results
            result: None,
            warnings: Vec::new(),
            metadata: Vec::new(),

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
        }
    }
}

// Color theme constants
pub mod colors {
    use eframe::egui::Color32;

    // Soft Blue Theme (default)
    pub const BG_MAIN: Color32 = Color32::from_rgb(225, 235, 245);
    pub const BG_PANEL: Color32 = Color32::from_rgb(245, 250, 255);
    pub const ACCENT: Color32 = Color32::from_rgb(70, 130, 180);
    pub const TAB_ACTIVE: Color32 = Color32::from_rgb(70, 130, 180);
    pub const TAB_INACTIVE: Color32 = Color32::from_rgb(200, 220, 240);
    pub const SCROLLBAR: Color32 = Color32::from_rgb(100, 149, 237);
    pub const TEXT_MAIN: Color32 = Color32::from_rgb(30, 30, 30);
    pub const TEXT_SUCCESS: Color32 = Color32::from_rgb(34, 139, 34);
    pub const TEXT_WARNING: Color32 = Color32::from_rgb(255, 140, 0);

    // Light Gray Theme
    pub const LIGHT_BG_MAIN: Color32 = Color32::from_rgb(240, 240, 240);
    pub const LIGHT_BG_PANEL: Color32 = Color32::from_rgb(250, 250, 250);
    pub const LIGHT_ACCENT: Color32 = Color32::from_rgb(100, 100, 100);

    // Cream Theme
    pub const CREAM_BG_MAIN: Color32 = Color32::from_rgb(255, 248, 220);
    pub const CREAM_BG_PANEL: Color32 = Color32::from_rgb(255, 253, 245);
    pub const CREAM_ACCENT: Color32 = Color32::from_rgb(218, 165, 32);
}