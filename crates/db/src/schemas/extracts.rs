// Extract encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_extract_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS extracts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Source & Manufacturing
            manufacturer TEXT,
            extract_type TEXT,
            base_ingredient TEXT,
            extraction_method TEXT,

            -- Composition
            alcohol_content TEXT,
            concentration TEXT,
            specific_gravity TEXT,

            -- Sensory Profile
            aroma_intensity TEXT,
            aroma_fidelity TEXT,
            aroma_character TEXT,
            flavor_intensity TEXT,
            flavor_fidelity TEXT,
            flavor_character TEXT,

            -- Usage Guidelines
            typical_usage_rate_ml_per_liter TEXT,
            typical_usage_rate_tsp_per_gallon TEXT,
            potency_vs_natural TEXT,
            optimal_usage_timing TEXT,
            heat_stability TEXT,

            -- Applications
            recommended_beer_styles TEXT,
            recommended_mead_styles TEXT,
            recommended_spirit_styles TEXT,
            baking_applications TEXT,
            cooking_applications TEXT,
            cocktail_applications TEXT,

            -- Quality & Comparison
            natural_vs_artificial TEXT,
            quality_grade TEXT,
            alternatives_natural TEXT,
            alternatives_extract TEXT,
            cost_comparison TEXT,

            -- Storage
            storage_requirements TEXT,
            shelf_life TEXT,
            degradation_factors TEXT,

            -- Professional Notes
            chef_notes TEXT,
            baker_notes TEXT,
            distiller_notes TEXT,
            description TEXT,
            usage_tips TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create extracts: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_extract_type ON extracts(extract_type)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}