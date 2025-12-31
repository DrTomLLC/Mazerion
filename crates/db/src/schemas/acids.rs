// Acid encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_acid_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS acids (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Chemistry
            acid_type TEXT,
            chemical_formula TEXT,
            concentration TEXT,
            pka_value TEXT,

            -- pH Impact
            ph_reduction_rate TEXT,
            buffering_capacity TEXT,

            -- Usage Guidelines
            typical_usage_rate_mash TEXT,
            typical_usage_rate_sparge TEXT,
            typical_usage_rate_kettle TEXT,
            typical_usage_rate_fermentation TEXT,
            optimal_usage_timing TEXT,

            -- Sensory Impact
            flavor_contribution TEXT,
            aroma_contribution TEXT,
            perceived_acidity TEXT,
            tartness_character TEXT,

            -- Applications
            mash_acidification TEXT,
            sparge_acidification TEXT,
            kettle_souring TEXT,
            post_fermentation_adjustment TEXT,

            -- Safety
            safety_precautions TEXT,
            handling_requirements TEXT,
            protective_equipment TEXT,
            first_aid TEXT,

            -- Professional Notes
            brewmaster_notes TEXT,
            chemistry_notes TEXT,
            description TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create acids: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_acid_type ON acids(acid_type)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}