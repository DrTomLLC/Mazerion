// Syrup encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_syrup_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS syrups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Source & Manufacturing
            manufacturer TEXT,
            syrup_type TEXT,
            base_ingredient TEXT,
            production_method TEXT,

            -- Composition
            sugar_content TEXT,
            brix_level TEXT,
            specific_gravity TEXT,
            color_contribution TEXT,

            -- Sensory Profile
            flavor_intensity TEXT,
            flavor_profile TEXT,
            sweetness_level TEXT,
            aroma_profile TEXT,
            mouthfeel TEXT,

            -- Fermentation Properties
            fermentability TEXT,
            typical_usage_rate TEXT,
            gravity_contribution TEXT,

            -- Applications
            recommended_beer_styles TEXT,
            recommended_mead_styles TEXT,
            recommended_cider_styles TEXT,
            cocktail_applications TEXT,
            culinary_applications TEXT,

            -- Quality
            quality_grade TEXT,
            alternatives TEXT,
            substitution_ratios TEXT,

            -- Storage
            storage_requirements TEXT,
            optimal_storage_temp TEXT,
            shelf_life TEXT,

            -- Professional Notes
            brewmaster_notes TEXT,
            chef_notes TEXT,
            description TEXT,
            usage_tips TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create syrups: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_syrup_type ON syrups(syrup_type)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}