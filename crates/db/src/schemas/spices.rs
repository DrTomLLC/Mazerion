// Spice encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_spice_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS spices (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Classification
            scientific_name TEXT,
            spice_family TEXT,
            part_used TEXT,
            origin_region TEXT,
            terroir_notes TEXT,

            -- Sensory Profile (Master Level)
            aroma_intensity TEXT,
            aroma_primary TEXT,
            aroma_secondary TEXT,
            aroma_tertiary TEXT,
            aroma_volatility TEXT,
            flavor_intensity TEXT,
            flavor_primary TEXT,
            flavor_secondary TEXT,
            flavor_tertiary TEXT,
            heat_level_scoville TEXT,
            heat_character TEXT,
            sweetness_level TEXT,
            bitterness_level TEXT,
            astringency TEXT,
            pungency TEXT,
            mouthfeel TEXT,
            numbing_effect TEXT,
            finish_length TEXT,
            finish_character TEXT,

            -- Chemical Components
            essential_oil_content TEXT,
            oleoresin_content TEXT,
            capsaicin_content TEXT,
            piperine_content TEXT,
            volatile_compounds TEXT,

            -- Usage Guidelines
            potency_rating TEXT,
            typical_usage_rate_per_liter TEXT,
            typical_usage_rate_per_gallon TEXT,
            optimal_usage_timing TEXT,
            heat_stability TEXT,
            alcohol_solubility TEXT,
            water_solubility TEXT,

            -- Processing Forms
            whole_usage TEXT,
            ground_usage TEXT,
            toasted_usage TEXT,
            fresh_usage TEXT,
            dried_usage TEXT,
            extract_usage TEXT,

            -- Fermentation Applications
            recommended_beer_styles TEXT,
            recommended_mead_styles TEXT,
            recommended_wine_styles TEXT,
            recommended_spirit_styles TEXT,

            -- Professional Pairing (Michelin Level)
            complementary_spices TEXT,
            contrasting_spices TEXT,
            protein_pairings TEXT,
            vegetable_pairings TEXT,
            fruit_pairings TEXT,
            grain_pairings TEXT,
            dairy_pairings TEXT,
            fat_pairings TEXT,
            acid_pairings TEXT,
            sweet_pairings TEXT,
            wine_pairings TEXT,
            beer_pairings TEXT,
            spirit_pairings TEXT,
            cheese_pairings TEXT,
            chocolate_pairings TEXT,

            -- Regional Cuisine
            cuisine_associations TEXT,
            regional_blends TEXT,
            traditional_applications TEXT,

            -- Quality & Sourcing
            quality_grades TEXT,
            harvest_timing TEXT,
            processing_quality_factors TEXT,
            storage_requirements TEXT,
            optimal_storage_temp TEXT,
            shelf_life_whole TEXT,
            shelf_life_ground TEXT,
            degradation_factors TEXT,

            -- Health & Nutrition
            health_benefits TEXT,
            medicinal_properties TEXT,
            bioactive_compounds TEXT,
            contraindications TEXT,

            -- Professional Notes
            chef_notes TEXT,
            sommelier_notes TEXT,
            distiller_notes TEXT,
            historical_significance TEXT,
            cultural_context TEXT,
            description TEXT,
            tasting_notes TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create spices: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_spice_family ON spices(spice_family)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}