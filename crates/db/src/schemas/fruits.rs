// Fruit encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_fruit_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS fruits (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Botanical Classification
            scientific_name TEXT,
            fruit_family TEXT,
            cultivar TEXT,
            origin_region TEXT,
            terroir_characteristics TEXT,

            -- Appearance
            color_spectrum TEXT,
            size_range TEXT,
            visual_characteristics TEXT,

            -- Chemical Composition
            sugar_content_brix TEXT,
            sugar_types TEXT,
            acid_content TEXT,
            malic_acid TEXT,
            citric_acid TEXT,
            tartaric_acid TEXT,
            ph_range TEXT,
            tannin_level TEXT,
            tannin_type TEXT,
            pectin_content TEXT,

            -- Sensory Profile (Sommelier Level)
            aroma_intensity TEXT,
            aroma_primary TEXT,
            aroma_secondary TEXT,
            aroma_complexity TEXT,
            flavor_intensity TEXT,
            flavor_primary TEXT,
            flavor_secondary TEXT,
            flavor_tertiary TEXT,
            sweetness_level TEXT,
            acidity_level TEXT,
            bitterness_level TEXT,
            astringency_level TEXT,
            mouthfeel TEXT,
            texture TEXT,
            finish_length TEXT,
            finish_character TEXT,

            -- Fermentation Properties
            fermentability TEXT,
            typical_usage_rate_kg_per_l TEXT,
            typical_usage_rate_lbs_per_gal TEXT,
            pressing_yield TEXT,
            juice_extraction TEXT,

            -- Color Contribution
            color_impact TEXT,
            anthocyanin_content TEXT,
            color_stability TEXT,

            -- Processing Methods
            fresh_usage TEXT,
            frozen_usage TEXT,
            puree_usage TEXT,
            juice_usage TEXT,
            concentrate_usage TEXT,
            dried_usage TEXT,
            zest_usage TEXT,

            -- Style Applications
            recommended_beer_styles TEXT,
            recommended_mead_styles TEXT,
            recommended_wine_styles TEXT,
            recommended_cider_styles TEXT,
            recommended_spirit_styles TEXT,

            -- Professional Pairing (Michelin Level)
            cheese_pairings TEXT,
            meat_pairings TEXT,
            seafood_pairings TEXT,
            vegetable_pairings TEXT,
            dessert_pairings TEXT,
            chocolate_pairings TEXT,
            wine_pairings TEXT,
            spirit_pairings TEXT,
            beer_pairings TEXT,
            mead_pairings TEXT,
            sauce_applications TEXT,
            garnish_usage TEXT,

            -- Seasonal & Regional
            harvest_season TEXT,
            peak_season TEXT,
            regional_variations TEXT,
            climate_requirements TEXT,

            -- Quality & Sourcing
            quality_indicators TEXT,
            ripeness_indicators TEXT,
            selection_criteria TEXT,
            storage_requirements TEXT,
            optimal_storage_temp TEXT,
            shelf_life_fresh TEXT,
            shelf_life_frozen TEXT,

            -- Nutritional & Health
            nutritional_highlights TEXT,
            vitamin_content TEXT,
            mineral_content TEXT,
            antioxidant_content TEXT,
            health_benefits TEXT,
            allergen_info TEXT,

            -- Professional Notes
            chef_notes TEXT,
            sommelier_notes TEXT,
            mazer_notes TEXT,
            brewmaster_notes TEXT,
            historical_significance TEXT,
            cultural_context TEXT,
            culinary_traditions TEXT,
            description TEXT,
            tasting_notes TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create fruits: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_fruit_family ON fruits(fruit_family)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}