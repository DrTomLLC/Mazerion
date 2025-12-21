//! Application state management

use eframe::egui::Color32;

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

#[derive(Debug, Clone, Copy)]
pub struct CustomColors {
    pub background: Color32,
    pub honey_gold: Color32,
    pub forest_green: Color32,
    pub light_cream: Color32,
    pub dark_text: Color32,
    pub sunset_orange: Color32,
    pub saddle_brown: Color32,
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
    Acid,
    AcidAddition,
    Pasteurization,
    Stabilization,
    SweetnessChart,
}

impl FinishingCalculator {
    pub fn all() -> &'static [Self] {
        &[
            Self::Backsweetening,
            Self::Sulfite,
            Self::AcidAddition,
            Self::Pasteurization,
            Self::Stabilization,
            Self::SweetnessChart,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Backsweetening => "Backsweetening",
            Self::Sulfite => "Sulfite",
            Self::Acid => "Acid",
            Self::AcidAddition => "Acid Addition",
            Self::Pasteurization => "Pasteurization",
            Self::Stabilization => "Stabilization",
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
    pub custom_colors: CustomColors,
    pub unit_system: UnitSystem,
    pub sg_precision: u32,
    pub ph_precision: u32,
    pub brix_precision: u32,
}

impl Default for AppState {
    fn default() -> Self {
        let theme = Theme::HoneyGold;
        let custom_colors = Self::get_colors_for_theme(theme);
        Self {
            current_tab: TabView::Basic,
            basic_calc: BasicCalculator::Abv,
            advanced_calc: AdvancedCalculator::Blending,
            brewing_calc: BrewingCalculator::Nutrition,
            beer_calc: BeerCalculator::Ibu,
            finishing_calc: FinishingCalculator::Backsweetening,
            theme,
            custom_colors,
            unit_system: UnitSystem::Metric,
            sg_precision: 4,
            ph_precision: 3,
            brix_precision: 2,
        }
    }
}

impl AppState {
    pub fn get_theme_colors(&self) -> CustomColors {
        Self::get_colors_for_theme(self.theme)
    }

    fn get_colors_for_theme(theme: Theme) -> CustomColors {
        match theme {
            Theme::HoneyGold => CustomColors {
                background: colors::LIGHT_CREAM,
                honey_gold: colors::HONEY_GOLD,
                forest_green: colors::FOREST_GREEN,
                light_cream: colors::LIGHT_CREAM,
                dark_text: colors::DARK_TEXT,
                sunset_orange: colors::HONEY_GOLD,
                saddle_brown: colors::SADDLE_BROWN,
            },
            Theme::ForestGreen => CustomColors {
                background: Color32::from_rgb(240, 248, 240),
                honey_gold: Color32::from_rgb(34, 139, 34),
                forest_green: Color32::from_rgb(0, 100, 0),
                light_cream: Color32::from_rgb(240, 248, 240),
                dark_text: Color32::from_rgb(25, 50, 25),
                sunset_orange: Color32::from_rgb(34, 139, 34),
                saddle_brown: Color32::from_rgb(0, 100, 0),
            },
            Theme::OceanBlue => CustomColors {
                background: Color32::from_rgb(240, 248, 255),
                honey_gold: Color32::from_rgb(30, 144, 255),
                forest_green: Color32::from_rgb(0, 105, 148),
                light_cream: Color32::from_rgb(240, 248, 255),
                dark_text: Color32::from_rgb(25, 25, 112),
                sunset_orange: Color32::from_rgb(30, 144, 255),
                saddle_brown: Color32::from_rgb(0, 105, 148),
            },
            Theme::SunsetOrange => CustomColors {
                background: Color32::from_rgb(255, 250, 240),
                honey_gold: Color32::from_rgb(255, 140, 0),
                forest_green: Color32::from_rgb(255, 99, 71),
                light_cream: Color32::from_rgb(255, 250, 240),
                dark_text: Color32::from_rgb(139, 69, 19),
                sunset_orange: Color32::from_rgb(255, 69, 0),
                saddle_brown: Color32::from_rgb(139, 69, 19),
            },
            Theme::LavenderPurple => CustomColors {
                background: Color32::from_rgb(248, 240, 255),
                honey_gold: Color32::from_rgb(147, 112, 219),
                forest_green: Color32::from_rgb(138, 43, 226),
                light_cream: Color32::from_rgb(248, 240, 255),
                dark_text: Color32::from_rgb(75, 0, 130),
                sunset_orange: Color32::from_rgb(147, 112, 219),
                saddle_brown: Color32::from_rgb(138, 43, 226),
            },
        }
    }
}

pub mod colors {
    use eframe::egui::Color32;

    pub const HONEY_GOLD: Color32 = Color32::from_rgb(218, 165, 32);
    pub const FOREST_GREEN: Color32 = Color32::from_rgb(34, 139, 34);
    pub const LIGHT_CREAM: Color32 = Color32::from_rgb(255, 253, 240);
    pub const DARK_TEXT: Color32 = Color32::from_rgb(51, 51, 51);
    pub const SADDLE_BROWN: Color32 = Color32::from_rgb(139, 69, 19);
    pub const GOLDENROD: Color32 = Color32::from_rgb(218, 165, 32);
    pub const DARK_RED: Color32 = Color32::from_rgb(139, 0, 0);
    pub const DARK_ORANGE: Color32 = Color32::from_rgb(255, 140, 0);
}