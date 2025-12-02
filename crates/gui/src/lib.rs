use eframe::egui::Color32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabView {
    Basic,
    Advanced,
    Brewing,
    Finishing,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    HoneyGold,
    ForestGreen,
    OceanBlue,
    SunsetOrange,
    LavenderPurple,
}

impl Theme {
    pub fn name(&self) -> &'static str {
        match self {
            Theme::HoneyGold => "🍯 Honey & Gold",
            Theme::ForestGreen => "🌲 Forest Green",
            Theme::OceanBlue => "🌊 Ocean Blue",
            Theme::SunsetOrange => "🌅 Sunset Orange",
            Theme::LavenderPurple => "💜 Lavender Purple",
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

pub struct AppState {
    pub current_tab: TabView,
    pub basic_calc: BasicCalculator,
    pub advanced_calc: AdvancedCalculator,
    pub brewing_calc: BrewingCalculator,
    pub finishing_calc: FinishingCalculator,
    pub theme: Theme,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_tab: TabView::Basic,
            basic_calc: BasicCalculator::Abv,
            advanced_calc: AdvancedCalculator::Blending,
            brewing_calc: BrewingCalculator::Nutrition,
            finishing_calc: FinishingCalculator::Backsweetening,
            theme: Theme::HoneyGold,
        }
    }
}

impl AppState {
    pub fn get_theme_colors(&self) -> (Color32, Color32) {
        match self.theme {
            Theme::HoneyGold => (colors::CORNSILK, colors::LIGHT_CREAM),
            Theme::ForestGreen => (Color32::from_rgb(240, 255, 240), Color32::from_rgb(245, 255, 245)),
            Theme::OceanBlue => (Color32::from_rgb(240, 248, 255), Color32::from_rgb(245, 250, 255)),
            Theme::SunsetOrange => (Color32::from_rgb(255, 245, 235), Color32::from_rgb(255, 250, 240)),
            Theme::LavenderPurple => (Color32::from_rgb(245, 240, 255), Color32::from_rgb(250, 245, 255)),
        }
    }
}

pub mod colors {
    use eframe::egui::Color32;
    pub const HONEY_GOLD: Color32 = Color32::from_rgb(240, 165, 0);
    pub const CORNSILK: Color32 = Color32::from_rgb(255, 248, 220);
    pub const FOREST_GREEN: Color32 = Color32::from_rgb(34, 139, 34);
    pub const LIGHT_CREAM: Color32 = Color32::from_rgb(255, 253, 245);
    pub const DARK_TEXT: Color32 = Color32::from_rgb(40, 40, 40);
}