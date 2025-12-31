// Yeast nutrient encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_nutrient_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS nutrients (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Source & Type
            manufacturer TEXT,
            nutrient_type TEXT,
            organic_inorganic TEXT,

            -- Composition
            composition TEXT,
            nitrogen_content TEXT,
            nitrogen_type TEXT,
            yan_contribution TEXT,
            vitamin_content TEXT,
            mineral_content TEXT,
            amino_acid_profile TEXT,

            -- Usage Guidelines
            typical_dosage_per_gallon TEXT,
            typical_dosage_per_liter TEXT,
            max_recommended_dose TEXT,

            -- Timing Protocols
            rehydration_usage TEXT,
            pitch_usage TEXT,
            lag_phase_usage TEXT,
            active_fermentation_usage TEXT,
            staggered_addition_schedule TEXT,

            -- Applications
            recommended_beer_styles TEXT,
            recommended_mead_styles TEXT,
            recommended_wine_styles TEXT,
            high_gravity_fermentation TEXT,

            -- Effects
            fermentation_speed_impact TEXT,
            attenuation_impact TEXT,
            flavor_impact TEXT,
            off_flavor_prevention TEXT,

            -- Specific Protocols
            tosna_protocol TEXT,
            tosna_20_schedule TEXT,
            tosna_30_schedule TEXT,

            -- Professional Notes
            brewmaster_notes TEXT,
            mazer_notes TEXT,
            winemaker_notes TEXT,
            description TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create nutrients: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_nutrient_type ON nutrients(nutrient_type)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}