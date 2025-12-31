// Vegetable encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_vegetable_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS vegetables (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Botanical Classification
            scientific_name TEXT,
            vegetable_family TEXT,
            cultivar TEXT,
            origin_region TEXT,
            terroir_characteristics TEXT,

            -- Composition
            sugar_content TEXT,
            starch_content TEXT,
            fiber_content TEXT,
            water_content TEXT,
            protein_content TEXT,
            fat_content TEXT,

            -- Sensory Profile
            aroma_intensity TEXT,
            aroma_primary TEXT,
            aroma_secondary TEXT,
            flavor_intensity TEXT,
            flavor_primary TEXT,
            flavor_secondary TEXT,
            flavor_tertiary TEXT,
            sweetness_level TEXT,
            bitterness_level TEXT,
            umami_level TEXT,
            astringency TEXT,
            mouthfeel TEXT,
            texture_raw TEXT,
            texture_cooked TEXT,

            -- Fermentation Properties
            fermentability TEXT,
            typical_usage_rate TEXT,
            processing_methods TEXT,
            enzymatic_activity TEXT,

            -- Culinary Applications
            recommended_beer_styles TEXT,
            recommended_sake_styles TEXT,
            recommended_fermented_styles TEXT,
            cooking_methods TEXT,
            optimal_cooking_temp TEXT,

            -- Professional Pairing
            protein_pairings TEXT,
            grain_pairings TEXT,
            dairy_pairings TEXT,
            fruit_pairings TEXT,
            spice_pairings TEXT,
            herb_pairings TEXT,
            acid_pairings TEXT,
            fat_pairings TEXT,
            wine_pairings TEXT,
            beer_pairings TEXT,
            spirit_pairings TEXT,

            -- Seasonal Data
            harvest_season TEXT,
            peak_season TEXT,
            storage_requirements TEXT,
            shelf_life TEXT,

            -- Nutritional
            nutritional_highlights TEXT,
            health_benefits TEXT,

            -- Professional Notes
            chef_notes TEXT,
            brewmaster_notes TEXT,
            historical_context TEXT,
            cultural_significance TEXT,
            description TEXT,
            tasting_notes TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create vegetables: {}", e)))?;

    Ok(())
}