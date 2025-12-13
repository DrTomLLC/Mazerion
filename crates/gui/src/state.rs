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
    AcidAddition,
    SweetnessChart,
}

impl FinishingCalculator {
    pub fn all() -> &'static [Self] {
        &[
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
    pub custom_colors: CustomColors,
    pub unit_system: UnitSystem,
    pub sg_precision: u32,
    pub ph_precision: u32,
    pub brix_precision: u32,
}

impl Default for AppState {
    fn default() -> Self {
        let theme = Theme::HoneyGold;
        Self {
            current_tab: TabView::Basic,
            basic_calc: BasicCalculator::Abv,
            advanced_calc: AdvancedCalculator::Blending,
            brewing_calc: BrewingCalculator::Nutrition,
            beer_calc: BeerCalculator::Ibu,
            finishing_calc: FinishingCalculator::Backsweetening,
            theme,
            custom_colors: Self::get_colors_for_theme(theme),
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
                background: Color32::from_rgb(245, 240, 230),  // Darker cream
                honey_gold: Color32::from_rgb(184, 134, 11),   // Darker gold
                forest_green: Color32::from_rgb(34, 139, 34),
                light_cream: Color32::from_rgb(250, 245, 235), // Slightly darker cream
                dark_text: Color32::from_rgb(40, 40, 40),      // Darker text
                sunset_orange: Color32::from_rgb(184, 134, 11),
                saddle_brown: Color32::from_rgb(115, 58, 15),  // Darker brown
            },
            Theme::ForestGreen => CustomColors {
                background: Color32::from_rgb(230, 240, 230),  // Darker green bg
                honey_gold: Color32::from_rgb(25, 111, 25),    // Darker green
                forest_green: Color32::from_rgb(0, 80, 0),     // Much darker green
                light_cream: Color32::from_rgb(235, 245, 235),
                dark_text: Color32::from_rgb(20, 40, 20),      // Darker text
                sunset_orange: Color32::from_rgb(25, 111, 25),
                saddle_brown: Color32::from_rgb(0, 80, 0),
            },
            Theme::OceanBlue => CustomColors {
                background: Color32::from_rgb(230, 240, 245),  // Darker blue bg
                honey_gold: Color32::from_rgb(20, 105, 180),   // Darker blue
                forest_green: Color32::from_rgb(0, 85, 120),   // Darker teal
                light_cream: Color32::from_rgb(235, 245, 250),
                dark_text: Color32::from_rgb(20, 20, 90),      // Darker navy
                sunset_orange: Color32::from_rgb(20, 105, 180),
                saddle_brown: Color32::from_rgb(0, 85, 120),
            },
            Theme::SunsetOrange => CustomColors {
                background: Color32::from_rgb(245, 240, 230),  // Darker warm bg
                honey_gold: Color32::from_rgb(205, 92, 0),     // Darker orange
                forest_green: Color32::from_rgb(205, 50, 30),  // Darker red-orange
                light_cream: Color32::from_rgb(250, 245, 235),
                dark_text: Color32::from_rgb(110, 55, 15),     // Darker brown
                sunset_orange: Color32::from_rgb(205, 35, 0),  // Darker red-orange
                saddle_brown: Color32::from_rgb(110, 55, 15),
            },
            Theme::LavenderPurple => CustomColors {
                background: Color32::from_rgb(240, 230, 245),  // Darker purple bg
                honey_gold: Color32::from_rgb(115, 75, 175),   // Darker purple
                forest_green: Color32::from_rgb(110, 35, 180), // Darker violet
                light_cream: Color32::from_rgb(245, 235, 250),
                dark_text: Color32::from_rgb(60, 0, 100),      // Darker indigo
                sunset_orange: Color32::from_rgb(115, 75, 175),
                saddle_brown: Color32::from_rgb(110, 35, 180),
            },
        }
    }
}

pub mod colors {
    use eframe::egui::Color32;

    pub const HONEY_GOLD: Color32 = Color32::from_rgb(184, 134, 11);      // Darker gold
    pub const FOREST_GREEN: Color32 = Color32::from_rgb(34, 139, 34);
    pub const LIGHT_CREAM: Color32 = Color32::from_rgb(250, 245, 235);    // Darker cream
    pub const DARK_TEXT: Color32 = Color32::from_rgb(40, 40, 40);         // Darker
    pub const SADDLE_BROWN: Color32 = Color32::from_rgb(115, 58, 15);     // Darker brown
    pub const GOLDENROD: Color32 = Color32::from_rgb(184, 134, 11);
    pub const DARK_RED: Color32 = Color32::from_rgb(139, 0, 0);
    pub const DARK_ORANGE: Color32 = Color32::from_rgb(205, 92, 0);       // Darker orange
}