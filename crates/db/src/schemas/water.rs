// Water chemistry encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_water_tables(conn: &Connection) -> Result<()> {
    // Water Profiles
    conn.execute(
        "CREATE TABLE IF NOT EXISTS water_profiles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Source
            location TEXT,
            profile_type TEXT,
            source_description TEXT,

            -- Ion Concentrations (ppm)
            calcium TEXT,
            magnesium TEXT,
            sodium TEXT,
            chloride TEXT,
            sulfate TEXT,
            bicarbonate TEXT,

            -- Calculated Values
            ph TEXT,
            total_hardness TEXT,
            temporary_hardness TEXT,
            permanent_hardness TEXT,
            residual_alkalinity TEXT,
            sulfate_chloride_ratio TEXT,

            -- Style Recommendations
            recommended_beer_styles TEXT,
            recommended_mead_styles TEXT,
            beer_style_examples TEXT,

            -- Professional Notes
            brewmaster_notes TEXT,
            water_chemistry_notes TEXT,
            description TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create water_profiles: {}", e)))?;

    // Water Salts & Additions
    conn.execute(
        "CREATE TABLE IF NOT EXISTS water_salts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Chemistry
            chemical_formula TEXT,
            salt_type TEXT,
            molecular_weight TEXT,

            -- Ion Contributions (ppm per gram per gallon)
            calcium_contribution TEXT,
            magnesium_contribution TEXT,
            sodium_contribution TEXT,
            chloride_contribution TEXT,
            sulfate_contribution TEXT,
            bicarbonate_contribution TEXT,

            -- Usage
            typical_usage_rate TEXT,
            max_recommended_dose TEXT,
            solubility TEXT,

            -- Effects
            effect_on_mash_ph TEXT,
            effect_on_flavor TEXT,
            effect_on_mouthfeel TEXT,
            effect_on_yeast TEXT,

            -- Applications
            recommended_usage TEXT,
            style_applications TEXT,

            -- Safety
            safety_notes TEXT,
            food_grade_required TEXT,

            -- Professional Notes
            brewmaster_notes TEXT,
            description TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create water_salts: {}", e)))?;

    // Clarifying/Fining Agents
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clarifying_agents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Classification
            agent_type TEXT,
            mechanism TEXT,
            target_particles TEXT,

            -- Usage
            typical_usage_rate TEXT,
            usage_timing TEXT,
            temperature_requirements TEXT,
            settling_time TEXT,
            contact_time TEXT,

            -- Applications
            recommended_beer_styles TEXT,
            recommended_wine_styles TEXT,
            recommended_mead_styles TEXT,

            -- Quality
            effectiveness_rating TEXT,
            clarity_improvement TEXT,
            flavor_impact TEXT,

            -- Special Considerations
            vegan_friendly TEXT,
            allergen_concerns TEXT,
            shelf_life TEXT,
            storage_requirements TEXT,

            -- Professional Notes
            brewmaster_notes TEXT,
            winemaker_notes TEXT,
            description TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create clarifying_agents: {}", e)))?;

    Ok(())
}