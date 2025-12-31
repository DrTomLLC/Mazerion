// Brewing adjunct encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_adjunct_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS adjuncts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Classification
            adjunct_type TEXT,
            source_material TEXT,

            -- Fermentation Properties
            fermentability TEXT,
            potential_gravity TEXT,
            extract_potential TEXT,

            -- Sensory Contribution
            flavor_profile TEXT,
            aroma_contribution TEXT,
            color_contribution TEXT,
            mouthfeel_impact TEXT,

            -- Usage Guidelines
            typical_usage_rate TEXT,
            usage_percentage_max TEXT,
            processing_required TEXT,
            mashing_required TEXT,

            -- Applications
            recommended_beer_styles TEXT,
            recommended_mead_styles TEXT,
            recommended_sake_styles TEXT,

            -- Quality
            quality_considerations TEXT,
            alternatives TEXT,

            -- Technical
            enzymatic_content TEXT,
            protein_content TEXT,
            starch_content TEXT,

            -- Professional Notes
            brewmaster_notes TEXT,
            description TEXT,
            usage_tips TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create adjuncts: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_adjunct_type ON adjuncts(adjunct_type)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}