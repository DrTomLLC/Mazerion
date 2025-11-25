//! State management for Mazerion GUI.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabView {
    Basic,
    Advanced,
    Brewing,
    Finishing,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeasurementSystem {
    Standard, // Default: Imperial/US
    Metric,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: Theme,
    pub measurement_system: MeasurementSystem,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            measurement_system: MeasurementSystem::Standard,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_tab: TabView,
    pub settings: Settings,
    pub result_cache: HashMap<String, String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_tab: TabView::Basic,
            settings: Settings::default(),
            result_cache: HashMap::new(),
        }
    }
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