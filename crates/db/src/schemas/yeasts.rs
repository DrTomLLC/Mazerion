// Yeast encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_yeast_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS yeast_strains (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            laboratory TEXT,
            product_code TEXT,

            -- Classification
            yeast_type TEXT,
            species TEXT,
            strain_origin TEXT,
            yeast_format TEXT,

            -- Performance Characteristics
            attenuation_min TEXT,
            attenuation_max TEXT,
            temp_min_c TEXT,
            temp_max_c TEXT,
            temp_optimal_c TEXT,
            alcohol_tolerance TEXT,
            flocculation TEXT,
            sedimentation TEXT,
            pitch_rate_million_cells TEXT,
            viable_cell_count TEXT,

            -- Chemical Production
            ester_production TEXT,
            phenolic_production TEXT,
            diacetyl_production TEXT,
            acetaldehyde_production TEXT,
            sulfur_production TEXT,
            fusel_alcohol_production TEXT,

            -- Sensory Profile (Professional Tasting Notes)
            aroma_primary TEXT,
            aroma_secondary TEXT,
            aroma_tertiary TEXT,
            flavor_primary TEXT,
            flavor_secondary TEXT,
            flavor_tertiary TEXT,
            mouthfeel_contribution TEXT,
            finish_character TEXT,

            -- Professional Usage
            recommended_styles TEXT,
            optimal_og_range TEXT,
            nutrient_requirements TEXT,
            oxygen_requirements TEXT,
            lag_time TEXT,
            fermentation_timeline TEXT,

            -- Pairing & Applications
            food_pairings TEXT,
            spirit_pairings TEXT,
            seasonal_usage TEXT,

            -- Technical Data
            killer_factor TEXT,
            diastatic TEXT,
            pof_positive TEXT,
            respiratory_deficient TEXT,

            -- Quality & Sourcing
            quality_grade TEXT,
            production_method TEXT,
            certification TEXT,
            storage_requirements TEXT,
            shelf_life TEXT,
            alternatives TEXT,

            -- Professional Notes
            brewmaster_notes TEXT,
            historical_context TEXT,
            regional_associations TEXT,
            description TEXT,
            technical_notes TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP,

            UNIQUE(laboratory, product_code)
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create yeast_strains: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_yeast_lab ON yeast_strains(laboratory)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_yeast_type ON yeast_strains(yeast_type)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}