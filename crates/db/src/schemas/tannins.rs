// Tannin encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_tannin_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tannins (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Source
            tannin_source TEXT,
            tannin_type TEXT,
            botanical_origin TEXT,

            -- Processing
            processing_form TEXT,
            extraction_method TEXT,
            purity_level TEXT,

            -- Chemical Profile
            tannin_concentration TEXT,
            polyphenol_content TEXT,
            gallotannin_percentage TEXT,
            ellagitannin_percentage TEXT,

            -- Sensory Contribution
            astringency_level TEXT,
            bitterness_contribution TEXT,
            flavor_contribution TEXT,
            mouthfeel_impact TEXT,
            color_contribution TEXT,

            -- Usage Guidelines
            typical_usage_rate TEXT,
            max_recommended_dose TEXT,
            usage_timing TEXT,
            contact_time TEXT,

            -- Applications
            recommended_beer_styles TEXT,
            recommended_wine_styles TEXT,
            recommended_mead_styles TEXT,
            oak_aging_simulation TEXT,

            -- Effects
            aging_enhancement TEXT,
            oxidation_resistance TEXT,
            clarity_improvement TEXT,
            shelf_life_extension TEXT,

            -- Pairing
            hop_pairing_notes TEXT,
            fruit_pairing_notes TEXT,

            -- Professional Notes
            winemaker_notes TEXT,
            brewmaster_notes TEXT,
            description TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create tannins: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tannin_source ON tannins(tannin_source)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}