// Malt & grain encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_malt_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS malts_grains (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Source & Processing
            manufacturer TEXT,
            maltster_location TEXT,
            grain_type TEXT,
            grain_variety TEXT,
            malt_type TEXT,
            processing_method TEXT,
            roasting_profile TEXT,
            origin_country TEXT,
            terroir_notes TEXT,

            -- Technical Specifications
            color_lovibond TEXT,
            color_srm TEXT,
            color_ebc TEXT,
            potential_gravity TEXT,
            extract_dry_basis TEXT,
            extract_as_is TEXT,
            extract_coarse_fine_diff TEXT,

            -- Malt Analysis
            moisture_content TEXT,
            protein_total TEXT,
            protein_soluble TEXT,
            kolbach_index TEXT,
            diastatic_power TEXT,
            alpha_amylase TEXT,
            beta_glucan TEXT,
            viscosity TEXT,
            friability TEXT,

            -- Sensory Profile
            aroma_profile TEXT,
            flavor_primary TEXT,
            flavor_secondary TEXT,
            flavor_intensity TEXT,
            sweetness_level TEXT,
            bitterness_contribution TEXT,
            astringency TEXT,
            mouthfeel_contribution TEXT,

            -- Usage Guidelines
            usage_percentage_min TEXT,
            usage_percentage_max TEXT,
            usage_percentage_typical TEXT,
            requires_mashing TEXT,
            recommended_mash_temp TEXT,
            recommended_mash_ph TEXT,

            -- Style Recommendations
            recommended_styles TEXT,
            classic_beer_examples TEXT,

            -- Pairing
            flavor_pairings TEXT,
            hop_pairings TEXT,
            yeast_pairings TEXT,

            -- Quality & Sourcing
            harvest_year TEXT,
            quality_grade TEXT,
            certification TEXT,
            sustainability_notes TEXT,

            -- Professional Notes
            brewmaster_notes TEXT,
            historical_context TEXT,
            regional_traditions TEXT,
            description TEXT,
            technical_notes TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create malts_grains: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_malt_type ON malts_grains(malt_type)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}