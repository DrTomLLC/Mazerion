// AI prediction engine schemas

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_prediction_tables(conn: &Connection) -> Result<()> {
    // ══════════════════════════════════════════════════════════════════════
    // RECIPE PREDICTIONS - AI-generated predictions
    // ══════════════════════════════════════════════════════════════════════
    conn.execute(
        "CREATE TABLE IF NOT EXISTS recipe_predictions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            recipe_id INTEGER,

            -- Input Summary
            ingredient_count INTEGER,
            ingredient_types TEXT,
            primary_ingredients TEXT,
            adjunct_ingredients TEXT,

            -- Predicted Sensory Profile
            predicted_aroma TEXT,
            predicted_flavor TEXT,
            predicted_mouthfeel TEXT,
            predicted_color TEXT,
            predicted_clarity TEXT,
            predicted_carbonation TEXT,

            -- Predicted Metrics
            predicted_og TEXT,
            predicted_fg TEXT,
            predicted_abv TEXT,
            predicted_ibu TEXT,
            predicted_srm TEXT,
            predicted_ph TEXT,

            -- Timeline Predictions
            predicted_fermentation_days TEXT,
            predicted_conditioning_days TEXT,
            predicted_total_days TEXT,
            predicted_optimal_aging_months TEXT,
            predicted_peak_drinking_window TEXT,

            -- Quality Predictions
            balance_score TEXT,
            complexity_score TEXT,
            drinkability_score TEXT,
            unique_character_score TEXT,
            professional_quality_score TEXT,
            overall_rating TEXT,

            -- Pairing Predictions
            predicted_food_pairings TEXT,
            predicted_cheese_pairings TEXT,
            predicted_dessert_pairings TEXT,
            predicted_occasion TEXT,
            predicted_season TEXT,
            predicted_serving_temp TEXT,

            -- Similar Styles & Examples
            similar_commercial_examples TEXT,
            style_category_match TEXT,
            bjcp_style_closest TEXT,
            comparable_award_winners TEXT,

            -- Optimization Suggestions
            suggested_improvements TEXT,
            alternative_ingredients TEXT,
            ratio_adjustments TEXT,
            process_improvements TEXT,

            -- Risk Assessment
            potential_issues TEXT,
            difficulty_rating TEXT,
            fermentation_risks TEXT,
            stability_concerns TEXT,

            -- Confidence Metrics
            prediction_confidence TEXT,
            data_completeness TEXT,
            similar_recipes_count TEXT,

            -- AI Model Info
            model_version TEXT,
            prediction_algorithm TEXT,

            -- Generated timestamp
            predicted_at TEXT DEFAULT CURRENT_TIMESTAMP,

            FOREIGN KEY(recipe_id) REFERENCES user_recipes(id) ON DELETE CASCADE
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create recipe_predictions: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_prediction_recipe ON recipe_predictions(recipe_id)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    // ══════════════════════════════════════════════════════════════════════
    // FLAVOR COMBINATIONS - Recommended ingredient combinations
    // ══════════════════════════════════════════════════════════════════════
    conn.execute(
        "CREATE TABLE IF NOT EXISTS flavor_combinations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,

            -- Base Ingredient
            base_ingredient_type TEXT NOT NULL,
            base_ingredient_id INTEGER NOT NULL,
            base_ingredient_name TEXT NOT NULL,

            -- Combination Category
            combination_type TEXT,
            combination_theme TEXT,

            -- Recommended Pairs (JSON arrays)
            perfect_pairings TEXT,
            good_pairings TEXT,
            interesting_pairings TEXT,
            avoid_pairings TEXT,

            -- Usage Context
            best_for_styles TEXT,
            best_for_occasions TEXT,
            seasonal_timing TEXT,

            -- Professional Curation
            michelin_approved TEXT,
            sommelier_approved TEXT,
            brewmaster_approved TEXT,

            -- Popularity & Success
            popularity_score TEXT,
            success_rate TEXT,
            user_ratings TEXT,

            -- Notes
            pairing_rationale TEXT,
            flavor_theory TEXT,
            cultural_context TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create flavor_combinations: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_combo_base ON flavor_combinations(base_ingredient_type, base_ingredient_id)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    // ══════════════════════════════════════════════════════════════════════
    // RECIPE TEMPLATES - AI-generated recipe suggestions
    // ══════════════════════════════════════════════════════════════════════
    conn.execute(
        "CREATE TABLE IF NOT EXISTS recipe_templates (
            id INTEGER PRIMARY KEY AUTOINCREMENT,

            -- Template Info
            template_name TEXT NOT NULL,
            category TEXT NOT NULL,
            subcategory TEXT,
            difficulty_level TEXT,

            -- Base Recipe (JSON)
            base_ingredients TEXT,
            base_proportions TEXT,
            base_process TEXT,

            -- Variations
            common_variations TEXT,
            seasonal_variations TEXT,
            advanced_variations TEXT,

            -- Predicted Outcomes
            expected_flavor_profile TEXT,
            expected_aroma_profile TEXT,
            expected_abv_range TEXT,
            expected_timeline TEXT,

            -- Success Metrics
            success_rate TEXT,
            beginner_friendly TEXT,
            popularity_score TEXT,

            -- Professional Notes
            expert_tips TEXT,
            common_mistakes TEXT,
            troubleshooting TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create recipe_templates: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_template_category ON recipe_templates(category)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}