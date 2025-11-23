// Enhanced state management for Mazerion GUI

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
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
    pub basic_calc: BasicCalculator,
    pub advanced_calc: AdvancedCalculator,
    pub brewing_calc: BrewingCalculator,
    pub finishing_calc: FinishingCalculator,
    pub result_cache: HashMap<String, String>,
    pub show_help: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_tab: TabView::Basic,
            basic_calc: BasicCalculator::Abv,
            advanced_calc: AdvancedCalculator::Blending,
            brewing_calc: BrewingCalculator::Nutrition,
            finishing_calc: FinishingCalculator::Backsweetening,
            result_cache: HashMap::new(),
            show_help: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicCalculator {
    Abv,
    BrixSgConverter,
    Dilution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvancedCalculator {
    Blending,
    Refractometer,
    SgCorrection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrewingCalculator {
    Nutrition,
    Carbonation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinishingCalculator {
    Backsweetening,
    Sulfite,
    AcidAddition,
}

// Color theme constants
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