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
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_tab: TabView::Basic,
            basic_calc: BasicCalculator::Abv,
            advanced_calc: AdvancedCalculator::Blending,
            brewing_calc: BrewingCalculator::Nutrition,
            finishing_calc: FinishingCalculator::Backsweetening,
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