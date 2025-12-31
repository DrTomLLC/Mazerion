// Bacteria culture encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_bacteria_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS bacteria_cultures (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Source
            laboratory TEXT,
            product_code TEXT,
            bacteria_type TEXT,
            species TEXT,
            strain TEXT,

            -- Growth Parameters
            temp_range_c TEXT,
            optimal_temp_c TEXT,
            ph_tolerance TEXT,
            optimal_ph TEXT,
            oxygen_tolerance TEXT,

            -- Acid Production
            lactic_acid_production TEXT,
            acetic_acid_production TEXT,
            other_acid_production TEXT,
            acid_production_rate TEXT,
            final_ph_range TEXT,

            -- Sensory Profile
            flavor_profile TEXT,
            aroma_profile TEXT,
            tartness_character TEXT,
            funkiness_level TEXT,

            -- Co-fermentation
            yeast_compatibility TEXT,
            other_bacteria_compatibility TEXT,
            sequential_inoculation TEXT,

            -- Applications
            recommended_beer_styles TEXT,
            recommended_kombucha TEXT,
            recommended_other_ferments TEXT,

            -- Timing & Usage
            pitch_rate TEXT,
            typical_fermentation_time TEXT,
            usage_notes TEXT,

            -- Professional Notes
            brewmaster_notes TEXT,
            microbiologist_notes TEXT,
            description TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP,

            UNIQUE(laboratory, product_code)
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create bacteria_cultures: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_bacteria_type ON bacteria_cultures(bacteria_type)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}