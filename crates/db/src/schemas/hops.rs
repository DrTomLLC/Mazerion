// Hop encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_hop_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS hop_varieties (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Classification & Origin
            hop_type TEXT,
            lineage TEXT,
            origin_country TEXT,
            growing_region TEXT,
            terroir_characteristics TEXT,

            -- Chemical Analysis (Professional Grade)
            alpha_acid_min TEXT,
            alpha_acid_max TEXT,
            alpha_acid_typical TEXT,
            beta_acid_min TEXT,
            beta_acid_max TEXT,
            beta_acid_typical TEXT,
            cohumulone_percentage TEXT,
            alpha_beta_ratio TEXT,

            -- Essential Oils (Complete Profile)
            total_oils_ml_per_100g TEXT,
            myrcene_percentage TEXT,
            humulene_percentage TEXT,
            caryophyllene_percentage TEXT,
            farnesene_percentage TEXT,
            linalool_percentage TEXT,
            geraniol_percentage TEXT,
            other_oils TEXT,
            oil_composition_notes TEXT,

            -- Sensory Profile (Cicerone Level)
            aroma_intensity TEXT,
            aroma_primary TEXT,
            aroma_secondary TEXT,
            aroma_tertiary TEXT,
            aroma_descriptors TEXT,
            flavor_intensity TEXT,
            flavor_primary TEXT,
            flavor_secondary TEXT,
            flavor_tertiary TEXT,
            flavor_descriptors TEXT,
            bitterness_quality TEXT,

            -- Professional Usage
            typical_usage TEXT,
            bittering_potential TEXT,
            aroma_potential TEXT,
            dual_purpose_rating TEXT,
            optimal_addition_timing TEXT,
            whirlpool_characteristics TEXT,
            dry_hop_characteristics TEXT,
            biotransformation_potential TEXT,

            -- Beer Style Pairing
            recommended_styles TEXT,
            classic_beer_examples TEXT,
            emerging_applications TEXT,

            -- Food & Beverage Pairing
            food_pairings TEXT,
            cheese_pairings TEXT,
            spirit_pairings TEXT,
            cuisine_pairings TEXT,

            -- Agricultural Data
            harvest_season TEXT,
            typical_yield TEXT,
            growing_difficulty TEXT,
            disease_resistance TEXT,

            -- Quality & Storage
            storage_stability_index TEXT,
            optimal_storage_temp TEXT,
            optimal_storage_conditions TEXT,
            degradation_rate TEXT,
            shelf_life TEXT,

            -- Substitution & Blending
            substitutes TEXT,
            complementary_hops TEXT,
            blending_suggestions TEXT,

            -- Professional Notes
            brewmaster_notes TEXT,
            historical_significance TEXT,
            regional_traditions TEXT,
            description TEXT,
            tasting_notes TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create hop_varieties: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_hop_type ON hop_varieties(hop_type)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_hop_origin ON hop_varieties(origin_country)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}