// Brewing enzyme encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_enzyme_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS enzymes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Classification
            enzyme_type TEXT,
            enzyme_class TEXT,
            source_organism TEXT,

            -- Operating Parameters
            optimal_temp_c TEXT,
            temp_range_c TEXT,
            optimal_ph TEXT,
            ph_range TEXT,
            optimal_contact_time TEXT,

            -- Activity
            activity_units TEXT,
            substrate_specificity TEXT,
            reaction_products TEXT,

            -- Usage
            typical_usage_rate TEXT,
            dosage_units TEXT,
            usage_timing TEXT,

            -- Effects
            effect_on_fermentability TEXT,
            effect_on_body TEXT,
            effect_on_clarity TEXT,
            effect_on_flavor TEXT,
            effect_on_mouthfeel TEXT,

            -- Applications
            mash_applications TEXT,
            fermentation_applications TEXT,
            post_fermentation_applications TEXT,
            gluten_reduction TEXT,
            haze_prevention TEXT,

            -- Specific Uses
            recommended_beer_styles TEXT,
            recommended_cider_styles TEXT,
            gluten_free_brewing TEXT,

            -- Storage & Handling
            storage_requirements TEXT,
            shelf_life TEXT,
            stability TEXT,

            -- Professional Notes
            brewmaster_notes TEXT,
            technical_notes TEXT,
            description TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create enzymes: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_enzyme_type ON enzymes(enzyme_type)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}