//! Application state management

use eframe::egui::Color32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabView {
    Basic,
    Advanced,
    Brewing,
    Beer,
    Finishing,
    Meads, // CHANGED FROM MeadStyles
    Utilities,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeadCalculator {
    Encyclopedia,
    Traditional,
    Hydromel,
    SackMead,
    Melomel,
    Cyser,
    Pyment,
    Morat,
    Metheglin,
    Rhodomel,
    Hippocras,
    Bochet,
    Braggot,
    Acerglyn,
    Capsicumel,
}

impl MeadCalculator {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Encyclopedia => "🍯 Mead Styles Encyclopedia",
            Self::Traditional => "Traditional (Show Mead)",
            Self::Hydromel => "Hydromel (Session Mead)",
            Self::SackMead => "Sack Mead (Dessert)",
            Self::Melomel => "Melomel (Fruit Mead)",
            Self::Cyser => "Cyser (Apple Mead)",
            Self::Pyment => "Pyment (Grape)",
            Self::Morat => "Morat (Mulberry)",
            Self::Metheglin => "Metheglin (Spiced)",
            Self::Rhodomel => "Rhodomel (Rose)",
            Self::Hippocras => "Hippocras (Medieval)",
            Self::Bochet => "Bochet (Caramelized)",
            Self::Braggot => "Braggot (Honey-Malt)",
            Self::Acerglyn => "Acerglyn (Maple)",
            Self::Capsicumel => "Capsicumel (Pepper)",
        }
    }
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
    pub goldenrod: Color32,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UtilityCalculator {
    BenchTrials,
    RecipeUpscaling,
    BottlesWithLosses,
}

#[derive(Debug, Clone)]
pub struct IngredientEntry {
    pub name: String,
    pub amount: String,
    pub unit: String,
}

impl Default for IngredientEntry {
    fn default() -> Self {
        Self {
            name: String::new(),
            amount: String::new(),
            unit: "kg".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MeadIngredients {
    pub entries: Vec<IngredientEntry>,
    pub new_ingredient: String,
    pub new_amount: String,
    pub new_unit: String,
}

impl MeadIngredients {
    pub fn add_ingredient(&mut self, name: String, amount: String, unit: String) {
        self.entries.push(IngredientEntry { name, amount, unit });
    }

    pub fn remove_ingredient(&mut self, index: usize) {
        if index < self.entries.len() {
            self.entries.remove(index);
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.new_ingredient.clear();
        self.new_amount.clear();
    }
}

pub struct AppState {
    pub current_tab: TabView,
    pub basic_calc: BasicCalculator,
    pub advanced_calc: AdvancedCalculator,
    pub brewing_calc: BrewingCalculator,
    pub beer_calc: BeerCalculator,
    pub finishing_calc: FinishingCalculator,
    pub mead_calc: MeadCalculator,
    pub mead_ingredients: MeadIngredients,
    pub conversion_value: String,
    pub conversion_from_unit: String,
    pub conversion_to_unit: String,
    pub conversion_result: Option<String>,
    pub utility_calc: UtilityCalculator,
    pub theme: Theme,
    pub custom_colors: CustomColors,
    pub unit_system: UnitSystem,
    pub sg_precision: u32,
    pub ph_precision: u32,
    pub brix_precision: u32,
    pub conv_to_unit: (),
    pub conv_result: (),
    pub conv_value: (),
    pub og: ()
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
            mead_calc: MeadCalculator::Encyclopedia,
            mead_ingredients: MeadIngredients::default(),
            utility_calc: UtilityCalculator::RecipeUpscaling,
            theme,
            custom_colors,
            unit_system: UnitSystem::Imperial,
            sg_precision: 3,
            ph_precision: 2,
            brix_precision: 1,
            conv_to_unit: (),
            conv_result: (),
            conv_value: (),
            conversion_value: "".to_string(),
            conversion_from_unit: "".to_string(),
            conversion_to_unit: "".to_string(),
            conversion_result: None,
            og: (),
        }
    }
}

impl AppState {
    pub fn get_colors_for_theme(theme: Theme) -> CustomColors {
        match theme {
            Theme::HoneyGold => CustomColors {
                background: Color32::from_rgb(18, 18, 22),
                honey_gold: Color32::from_rgb(218, 165, 32),
                forest_green: Color32::from_rgb(34, 139, 34),
                light_cream: Color32::from_rgb(25, 25, 30),
                dark_text: Color32::from_rgb(220, 220, 225),
                sunset_orange: Color32::from_rgb(255, 140, 0),
                saddle_brown: Color32::from_rgb(139, 69, 19),
                goldenrod: Color32::from_rgb(218, 165, 32),
            },
            Theme::ForestGreen => CustomColors {
                background: Color32::from_rgb(15, 20, 15),
                honey_gold: Color32::from_rgb(218, 165, 32),
                forest_green: Color32::from_rgb(50, 205, 50),
                light_cream: Color32::from_rgb(22, 28, 22),
                dark_text: Color32::from_rgb(220, 225, 220),
                sunset_orange: Color32::from_rgb(255, 140, 0),
                saddle_brown: Color32::from_rgb(107, 142, 35),
                goldenrod: Color32::from_rgb(218, 165, 32),
            },
            Theme::OceanBlue => CustomColors {
                background: Color32::from_rgb(12, 15, 25),
                honey_gold: Color32::from_rgb(218, 165, 32),
                forest_green: Color32::from_rgb(70, 130, 180),
                light_cream: Color32::from_rgb(20, 25, 35),
                dark_text: Color32::from_rgb(220, 225, 235),
                sunset_orange: Color32::from_rgb(100, 149, 237),
                saddle_brown: Color32::from_rgb(25, 25, 112),
                goldenrod: Color32::from_rgb(218, 165, 32),
            },
            Theme::SunsetOrange => CustomColors {
                background: Color32::from_rgb(22, 18, 15),
                honey_gold: Color32::from_rgb(255, 215, 0),
                forest_green: Color32::from_rgb(34, 139, 34),
                light_cream: Color32::from_rgb(30, 25, 20),
                dark_text: Color32::from_rgb(255, 245, 235),
                sunset_orange: Color32::from_rgb(255, 99, 71),
                saddle_brown: Color32::from_rgb(205, 92, 92),
                goldenrod: Color32::from_rgb(255, 215, 0),
            },
            Theme::LavenderPurple => CustomColors {
                background: Color32::from_rgb(20, 15, 25),
                honey_gold: Color32::from_rgb(218, 165, 32),
                forest_green: Color32::from_rgb(138, 43, 226),
                light_cream: Color32::from_rgb(28, 22, 35),
                dark_text: Color32::from_rgb(245, 240, 255),
                sunset_orange: Color32::from_rgb(255, 140, 0),
                saddle_brown: Color32::from_rgb(153, 50, 204),
                goldenrod: Color32::from_rgb(218, 165, 32),
            },
        }
    }

    pub fn change_theme(&mut self, new_theme: Theme) {
        self.theme = new_theme;
        self.custom_colors = Self::get_colors_for_theme(new_theme);
    }

    pub fn get_theme_colors(&self) -> CustomColors {
        Self::get_colors_for_theme(self.theme)
    }
}

pub mod colors {
    use eframe::egui::Color32;
    pub const HONEY_GOLD: Color32 = Color32::from_rgb(218, 165, 32);
    pub const LIGHT_CREAM: Color32 = Color32::from_rgb(25, 25, 30);
    pub const SADDLE_BROWN: Color32 = Color32::from_rgb(139, 69, 19);
    pub const GOLDENROD: Color32 = Color32::from_rgb(218, 165, 32);
}
