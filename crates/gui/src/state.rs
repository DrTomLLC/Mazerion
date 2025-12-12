//! Application state

use eframe::egui::Color32;

#[derive(Debug, Clone, Copy)]
pub struct CustomColors {
    pub honey_gold: Color32,
    pub light_cream: Color32,
    pub dark_text: Color32,
    pub saddle_brown: Color32,
    pub dark_orange: Color32,
    pub forest_green: Color32,
    pub dark_red: Color32,
}

impl Default for CustomColors {
    fn default() -> Self {
        Self {
            honey_gold: Color32::from_rgb(218, 165, 32),
            light_cream: Color32::from_rgb(255, 253, 240),
            dark_text: Color32::from_rgb(51, 51, 51),
            saddle_brown: Color32::from_rgb(139, 69, 19),
            dark_orange: Color32::from_rgb(255, 140, 0),
            forest_green: Color32::from_rgb(34, 139, 34),
            dark_red: Color32::from_rgb(139, 0, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabView {
    Basic,
    Advanced,
    Brewing,
    Beer,
    Finishing,
    MeadStyles,
    Utilities,
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
pub enum UnitSystem {
    Metric,
    Imperial,
}

impl UnitSystem {
    pub fn name(&self) -> &'static str {
        match self {
            UnitSystem::Metric => "Metric",
            UnitSystem::Imperial => "Imperial/US",
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
pub enum BeerCalculator {
    Ibu,
    Srm,
    Mash,
    Efficiency,
    StyleGuide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinishingCalculator {
    Backsweetening,
    Sulfite,
    AcidAddition,
    SweetnessChart,
}

impl FinishingCalculator {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Backsweetening,
            Self::Sulfite,
            Self::AcidAddition,
            Self::SweetnessChart,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Backsweetening => "Backsweetening",
            Self::Sulfite => "Sulfite",
            Self::AcidAddition => "Acid Addition",
            Self::SweetnessChart => "Sweetness Guide",
        }
    }
}

pub struct AppState {
    pub current_tab: TabView,
    pub basic_calc: BasicCalculator,
    pub advanced_calc: AdvancedCalculator,
    pub brewing_calc: BrewingCalculator,
    pub beer_calc: BeerCalculator,
    pub finishing_calc: FinishingCalculator,
    pub theme: Theme,
    pub unit_system: UnitSystem,
    pub sg_precision: u32,
    pub ph_precision: u32,
    pub brix_precision: u32,
    pub custom_colors: CustomColors,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_tab: TabView::Basic,
            basic_calc: BasicCalculator::Abv,
            advanced_calc: AdvancedCalculator::Blending,
            brewing_calc: BrewingCalculator::Nutrition,
            beer_calc: BeerCalculator::Ibu,
            finishing_calc: FinishingCalculator::Backsweetening,
            theme: Theme::HoneyGold,
            unit_system: UnitSystem::Metric,
            sg_precision: 4,
            ph_precision: 3,
            brix_precision: 2,
            custom_colors: CustomColors::default(),
        }
    }
}

impl AppState {
    pub fn get_theme_colors(&self) -> (Color32, Color32) {
        match self.theme {
            Theme::HoneyGold => (Color32::from_rgb(255, 250, 240), Color32::from_rgb(255, 253, 240)),
            Theme::ForestGreen => (Color32::from_rgb(240, 255, 240), Color32::from_rgb(245, 255, 245)),
            Theme::OceanBlue => (Color32::from_rgb(240, 248, 255), Color32::from_rgb(245, 250, 255)),
            Theme::SunsetOrange => (Color32::from_rgb(255, 245, 238), Color32::from_rgb(255, 248, 243)),
            Theme::LavenderPurple => (Color32::from_rgb(248, 240, 255), Color32::from_rgb(250, 245, 255)),
        }
    }
}

pub mod colors {
    use eframe::egui::Color32;

    pub const HONEY_GOLD: Color32 = Color32::from_rgb(218, 165, 32);
    pub const GOLDENROD: Color32 = Color32::from_rgb(218, 165, 32);
    pub const LIGHT_CREAM: Color32 = Color32::from_rgb(255, 253, 240);
    pub const DARK_TEXT: Color32 = Color32::from_rgb(51, 51, 51);
    pub const SADDLE_BROWN: Color32 = Color32::from_rgb(139, 69, 19);
    pub const DARK_ORANGE: Color32 = Color32::from_rgb(255, 140, 0);
    pub const FOREST_GREEN: Color32 = Color32::from_rgb(34, 139, 34);
    pub const DARK_RED: Color32 = Color32::from_rgb(139, 0, 0);
}