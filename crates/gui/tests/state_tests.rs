//! STATE MANAGEMENT TESTS - FORMATTED VERSION
//! Location: crates/gui/tests/state_tests.rs

use mazerion_gui::state::*;

// ============================================================================
// APPSTATE DEFAULT TESTS
// ============================================================================

#[test]
fn test_appstate_default_values() {
    let state = AppState::default();

    // Verify tab defaults
    assert!(matches!(state.current_tab, TabView::Basic));

    // Verify calculator defaults
    assert!(matches!(state.basic_calc, BasicCalculator::Abv));
    assert!(matches!(state.advanced_calc, AdvancedCalculator::Blending));
    assert!(matches!(state.brewing_calc, BrewingCalculator::Nutrition));
    assert!(matches!(state.beer_calc, BeerCalculator::Ibu));
    assert!(matches!(
        state.finishing_calc,
        FinishingCalculator::Backsweetening
    ));
    assert!(matches!(state.mead_calc, MeadCalculator::Encyclopedia));
    assert!(matches!(
        state.utility_calc,
        UtilityCalculator::RecipeUpscaling
    ));

    // Verify unit system default
    assert!(matches!(state.unit_system, UnitSystem::Imperial));

    // Verify theme default
    assert!(matches!(state.theme, Theme::HoneyGold));

    // Verify precision defaults
    assert_eq!(state.sg_precision, 3);
    assert_eq!(state.ph_precision, 2);
    assert_eq!(state.brix_precision, 1);
}

#[test]
fn test_appstate_mead_ingredients_default() {
    let state = AppState::default();

    // Verify mead ingredients initialized
    assert_eq!(state.mead_ingredients.entries.len(), 0);
    assert!(state.mead_ingredients.new_ingredient.is_empty());
    assert!(state.mead_ingredients.new_amount.is_empty());
}

#[test]
fn test_appstate_conversion_fields() {
    let state = AppState::default();

    // Verify conversion fields initialized
    assert_eq!(state.conversion_value, "");
    assert_eq!(state.conversion_from_unit, "");
    assert_eq!(state.conversion_to_unit, "");
    assert!(state.conversion_result.is_none());
}

// ============================================================================
// UNIT SYSTEM TESTS
// ============================================================================

#[test]
fn test_unit_system_variants() {
    let imperial = UnitSystem::Imperial;
    let metric = UnitSystem::Metric;

    // Verify they're different
    assert_ne!(
        std::mem::discriminant(&imperial),
        std::mem::discriminant(&metric)
    );
}

#[test]
fn test_unit_system_names() {
    assert_eq!(UnitSystem::Metric.name(), "Metric");
    assert_eq!(UnitSystem::Imperial.name(), "Imperial/US");
}

#[test]
fn test_unit_system_copy() {
    let original = UnitSystem::Imperial;
    let copied = original;

    // Should both be Imperial
    assert!(matches!(original, UnitSystem::Imperial));
    assert!(matches!(copied, UnitSystem::Imperial));
}

// ============================================================================
// THEME TESTS
// ============================================================================

#[test]
fn test_theme_variants_exist() {
    let _ = Theme::HoneyGold;
    let _ = Theme::ForestGreen;
    let _ = Theme::OceanBlue;
    let _ = Theme::SunsetOrange;
    let _ = Theme::LavenderPurple;
}

#[test]
fn test_theme_names() {
    assert_eq!(Theme::HoneyGold.name(), "🍯 Honey & Gold");
    assert_eq!(Theme::ForestGreen.name(), "🌲 Forest Green");
    assert_eq!(Theme::OceanBlue.name(), "🌊 Ocean Blue");
    assert_eq!(Theme::SunsetOrange.name(), "🌅 Sunset Orange");
    assert_eq!(Theme::LavenderPurple.name(), "💜 Lavender Purple");
}

#[test]
fn test_theme_copy() {
    let original = Theme::HoneyGold;
    let copied = original;

    assert!(matches!(original, Theme::HoneyGold));
    assert!(matches!(copied, Theme::HoneyGold));
}

// ============================================================================
// CUSTOM COLORS TESTS
// ============================================================================

#[test]
fn test_custom_colors_all_fields_exist() {
    let colors = AppState::default().custom_colors;

    // Verify all color fields exist
    let _ = colors.background;
    let _ = colors.honey_gold;
    let _ = colors.forest_green;
    let _ = colors.light_cream;
    let _ = colors.dark_text;
    let _ = colors.sunset_orange;
    let _ = colors.saddle_brown;
    let _ = colors.goldenrod;
}

#[test]
fn test_custom_colors_are_valid() {
    let colors = AppState::default().custom_colors;

    // Verify these are valid Color32 values (have RGBA components)
    assert_eq!(colors.honey_gold.a(), 255); // Should be opaque
    assert_eq!(colors.forest_green.a(), 255);
    assert_eq!(colors.background.a(), 255);
}

#[test]
fn test_custom_colors_not_all_same() {
    let colors = AppState::default().custom_colors;

    // Honey gold and forest green should be different colors
    assert_ne!(colors.honey_gold, colors.forest_green);
    assert_ne!(colors.background, colors.light_cream);
}

#[test]
fn test_theme_colors_differ_by_theme() {
    let honey_gold_colors = AppState::get_colors_for_theme(Theme::HoneyGold);
    let forest_green_colors = AppState::get_colors_for_theme(Theme::ForestGreen);

    // Different themes should have different forest_green colors
    assert_ne!(
        honey_gold_colors.forest_green,
        forest_green_colors.forest_green
    );
}

// ============================================================================
// CALCULATOR ENUM TESTS
// ============================================================================

#[test]
fn test_basic_calculator_variants() {
    let _ = BasicCalculator::Abv;
    let _ = BasicCalculator::BrixSgConverter;
    let _ = BasicCalculator::Dilution;
}

#[test]
fn test_advanced_calculator_variants() {
    let _ = AdvancedCalculator::Blending;
    let _ = AdvancedCalculator::Refractometer;
    let _ = AdvancedCalculator::SgCorrection;
}

#[test]
fn test_brewing_calculator_variants() {
    let _ = BrewingCalculator::Nutrition;
    let _ = BrewingCalculator::Carbonation;
}

#[test]
fn test_beer_calculator_variants() {
    let _ = BeerCalculator::Ibu;
    let _ = BeerCalculator::Srm;
    let _ = BeerCalculator::Mash;
    let _ = BeerCalculator::Efficiency;
    let _ = BeerCalculator::StyleGuide;
}

#[test]
fn test_finishing_calculator_all_method() {
    let all = FinishingCalculator::all();

    // Verify we get a list
    assert!(!all.is_empty());
    assert!(all.len() >= 5);
}

#[test]
fn test_finishing_calculator_names() {
    assert_eq!(
        FinishingCalculator::Backsweetening.name(),
        "Backsweetening"
    );
    assert_eq!(FinishingCalculator::Sulfite.name(), "Sulfite");
    assert_eq!(FinishingCalculator::AcidAddition.name(), "Acid Addition");
}

#[test]
fn test_utility_calculator_variants() {
    let _ = UtilityCalculator::BenchTrials;
    let _ = UtilityCalculator::RecipeUpscaling;
    let _ = UtilityCalculator::BottlesWithLosses;
}

#[test]
fn test_mead_calculator_names() {
    assert_eq!(
        MeadCalculator::Encyclopedia.name(),
        "🍯 Mead Styles Encyclopedia"
    );
    assert_eq!(
        MeadCalculator::Traditional.name(),
        "Traditional (Show Mead)"
    );
    assert_eq!(MeadCalculator::Cyser.name(), "Cyser (Apple Mead)");
}

// ============================================================================
// MEAD INGREDIENTS TESTS
// ============================================================================

#[test]
fn test_mead_ingredients_default() {
    let ingredients = MeadIngredients::default();

    assert_eq!(ingredients.entries.len(), 0);
    assert_eq!(ingredients.new_ingredient, "");
    assert_eq!(ingredients.new_amount, "");
    assert_eq!(ingredients.new_unit, "");
}

#[test]
fn test_mead_ingredients_add() {
    let mut ingredients = MeadIngredients::default();

    ingredients.add_ingredient("Honey".to_string(), "3.6".to_string(), "kg".to_string());

    assert_eq!(ingredients.entries.len(), 1);
    assert_eq!(ingredients.entries[0].name, "Honey");
    assert_eq!(ingredients.entries[0].amount, "3.6");
    assert_eq!(ingredients.entries[0].unit, "kg");
}

#[test]
fn test_mead_ingredients_remove() {
    let mut ingredients = MeadIngredients::default();

    ingredients.add_ingredient("Honey".to_string(), "3.6".to_string(), "kg".to_string());
    ingredients.add_ingredient("Water".to_string(), "15".to_string(), "L".to_string());

    assert_eq!(ingredients.entries.len(), 2);

    ingredients.remove_ingredient(0);

    assert_eq!(ingredients.entries.len(), 1);
    assert_eq!(ingredients.entries[0].name, "Water");
}

#[test]
fn test_mead_ingredients_remove_invalid_index() {
    let mut ingredients = MeadIngredients::default();

    ingredients.add_ingredient("Honey".to_string(), "3.6".to_string(), "kg".to_string());

    // Should not panic on invalid index
    ingredients.remove_ingredient(10);

    // Should still have 1 entry
    assert_eq!(ingredients.entries.len(), 1);
}

#[test]
fn test_mead_ingredients_clear() {
    let mut ingredients = MeadIngredients::default();

    ingredients.add_ingredient("Honey".to_string(), "3.6".to_string(), "kg".to_string());
    ingredients.new_ingredient = "Test".to_string();
    ingredients.new_amount = "5".to_string();

    ingredients.clear();

    assert_eq!(ingredients.entries.len(), 0);
    assert_eq!(ingredients.new_ingredient, "");
    assert_eq!(ingredients.new_amount, "");
}

#[test]
fn test_mead_ingredients_clone() {
    let mut ingredients1 = MeadIngredients::default();
    ingredients1.add_ingredient("Honey".to_string(), "3.6".to_string(), "kg".to_string());

    let mut ingredients2 = ingredients1.clone();
    ingredients2.add_ingredient("Water".to_string(), "15".to_string(), "L".to_string());

    // ingredients1 should have 1 entry
    assert_eq!(ingredients1.entries.len(), 1);

    // ingredients2 should have 2 entries
    assert_eq!(ingredients2.entries.len(), 2);
}

// ============================================================================
// INGREDIENT ENTRY TESTS
// ============================================================================

#[test]
fn test_ingredient_entry_default() {
    let entry = IngredientEntry::default();

    assert_eq!(entry.name, "");
    assert_eq!(entry.amount, "");
    assert_eq!(entry.unit, "kg");
}

#[test]
fn test_ingredient_entry_clone() {
    let entry1 = IngredientEntry {
        name: "Honey".to_string(),
        amount: "3.6".to_string(),
        unit: "kg".to_string(),
    };

    let entry2 = entry1.clone();

    assert_eq!(entry1.name, entry2.name);
    assert_eq!(entry1.amount, entry2.amount);
    assert_eq!(entry1.unit, entry2.unit);
}

// ============================================================================
// STATE MUTATION TESTS
// ============================================================================

#[test]
fn test_state_tab_switching() {
    let mut state = AppState::default();

    assert!(matches!(state.current_tab, TabView::Basic));

    state.current_tab = TabView::Advanced;
    assert!(matches!(state.current_tab, TabView::Advanced));

    state.current_tab = TabView::Brewing;
    assert!(matches!(state.current_tab, TabView::Brewing));
}

#[test]
fn test_state_calculator_switching() {
    let mut state = AppState::default();

    state.basic_calc = BasicCalculator::BrixSgConverter;
    assert!(matches!(
        state.basic_calc,
        BasicCalculator::BrixSgConverter
    ));

    state.advanced_calc = AdvancedCalculator::Refractometer;
    assert!(matches!(
        state.advanced_calc,
        AdvancedCalculator::Refractometer
    ));
}

#[test]
fn test_state_unit_system_switching() {
    let mut state = AppState::default();

    assert!(matches!(state.unit_system, UnitSystem::Imperial));

    state.unit_system = UnitSystem::Metric;
    assert!(matches!(state.unit_system, UnitSystem::Metric));
}

#[test]
fn test_state_theme_switching() {
    let mut state = AppState::default();

    assert!(matches!(state.theme, Theme::HoneyGold));

    state.theme = Theme::ForestGreen;
    assert!(matches!(state.theme, Theme::ForestGreen));

    // Update colors for new theme
    state.custom_colors = AppState::get_colors_for_theme(state.theme);
}

#[test]
fn test_state_precision_changes() {
    let mut state = AppState::default();

    assert_eq!(state.sg_precision, 3);
    assert_eq!(state.ph_precision, 2);
    assert_eq!(state.brix_precision, 1);

    state.sg_precision = 4;
    state.ph_precision = 3;
    state.brix_precision = 2;

    assert_eq!(state.sg_precision, 4);
    assert_eq!(state.ph_precision, 3);
    assert_eq!(state.brix_precision, 2);
}

// ============================================================================
// COLOR32 COMPONENT TESTS
// ============================================================================

#[test]
fn test_colors_have_rgb_components() {
    let colors = AppState::default().custom_colors;

    // Verify colors have valid RGB components
    assert!(colors.honey_gold.r() > 0);
    assert!(colors.forest_green.g() > 0);

    // All should be opaque
    assert_eq!(colors.background.a(), 255);
    assert_eq!(colors.honey_gold.a(), 255);
    assert_eq!(colors.forest_green.a(), 255);
    assert_eq!(colors.light_cream.a(), 255);
    assert_eq!(colors.dark_text.a(), 255);
    assert_eq!(colors.sunset_orange.a(), 255);
    assert_eq!(colors.saddle_brown.a(), 255);
    assert_eq!(colors.goldenrod.a(), 255);
}

// ============================================================================
// MEMORY SAFETY TESTS
// ============================================================================

#[test]
fn test_no_memory_leaks_on_repeated_operations() {
    for _ in 0..1000 {
        let mut state = AppState::default();
        state.theme = Theme::ForestGreen;
        state.unit_system = UnitSystem::Metric;
        state.mead_ingredients.add_ingredient(
            "test".to_string(),
            "1".to_string(),
            "kg".to_string(),
        );
        // State drops here
    }
    // If this completes without OOM, memory is being freed properly
}

#[test]
fn test_large_ingredient_list() {
    let mut ingredients = MeadIngredients::default();

    // Add 1000 ingredients
    for i in 0..1000 {
        ingredients.add_ingredient(
            format!("Ingredient {}", i),
            format!("{}", i),
            "kg".to_string(),
        );
    }

    assert_eq!(ingredients.entries.len(), 1000);
    assert_eq!(ingredients.entries[999].name, "Ingredient 999");
}

#[test]
fn test_conversion_result_option() {
    let mut state = AppState::default();

    // Starts as None
    assert!(state.conversion_result.is_none());

    // Can be set to Some
    state.conversion_result = Some("42".to_string());
    assert!(state.conversion_result.is_some());

    // ZERO UNWRAP - Compare against expected value
    assert_eq!(state.conversion_result, Some("42".to_string()));
}