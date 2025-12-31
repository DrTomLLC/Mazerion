// Herb encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_herb_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS herbs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Classification
            scientific_name TEXT,
            herb_family TEXT,
            part_used TEXT,
            origin_region TEXT,
            growing_conditions TEXT,

            -- Sensory Profile
            aroma_intensity TEXT,
            aroma_primary TEXT,
            aroma_secondary TEXT,
            aroma_tertiary TEXT,
            aroma_fresh TEXT,
            aroma_dried TEXT,
            flavor_intensity TEXT,
            flavor_primary TEXT,
            flavor_secondary TEXT,
            flavor_fresh TEXT,
            flavor_dried TEXT,
            bitterness_level TEXT,
            astringency TEXT,
            menthol_cooling TEXT,
            resinous_character TEXT,

            -- Chemical Profile
            essential_oil_content TEXT,
            volatile_compounds TEXT,
            chlorophyll_content TEXT,

            -- Usage Guidelines
            potency_fresh TEXT,
            potency_dried TEXT,
            fresh_to_dried_ratio TEXT,
            typical_usage_rate TEXT,
            optimal_usage_timing TEXT,
            heat_sensitivity TEXT,

            -- Processing Forms
            fresh_usage TEXT,
            dried_usage TEXT,
            frozen_usage TEXT,
            extract_usage TEXT,
            tea_infusion TEXT,

            -- Fermentation Applications
            recommended_beer_styles TEXT,
            recommended_mead_styles TEXT,
            recommended_wine_styles TEXT,
            recommended_spirit_styles TEXT,
            botanical_gin_usage TEXT,
            vermouth_usage TEXT,

            -- Professional Pairing
            complementary_herbs TEXT,
            contrasting_herbs TEXT,
            spice_pairings TEXT,
            protein_pairings TEXT,
            vegetable_pairings TEXT,
            fruit_pairings TEXT,
            dairy_pairings TEXT,
            grain_pairings TEXT,
            fat_pairings TEXT,
            acid_pairings TEXT,
            wine_pairings TEXT,
            spirit_pairings TEXT,
            cheese_pairings TEXT,

            -- Culinary Traditions
            cuisine_associations TEXT,
            traditional_combinations TEXT,
            classic_dishes TEXT,

            -- Seasonal & Quality
            harvest_season TEXT,
            peak_season TEXT,
            quality_indicators TEXT,
            storage_requirements TEXT,
            shelf_life_fresh TEXT,
            shelf_life_dried TEXT,

            -- Health & Medicine
            medicinal_properties TEXT,
            therapeutic_uses TEXT,
            bioactive_compounds TEXT,
            health_benefits TEXT,
            contraindications TEXT,

            -- Professional Notes
            chef_notes TEXT,
            herbalist_notes TEXT,
            distiller_notes TEXT,
            historical_uses TEXT,
            cultural_significance TEXT,
            description TEXT,
            tasting_notes TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create herbs: {}", e)))?;

    Ok(())
}