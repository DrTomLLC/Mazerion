// Schema coordinator - manages all database schemas

use rusqlite::Connection;
use mazerion_core::Result;

mod core;
mod yeasts;
mod honeys;
mod hops;
mod malts;
mod fruits;
mod vegetables;
mod spices;
mod herbs;
mod extracts;
mod syrups;
mod adjuncts;
mod water;
mod acids;
mod nutrients;
mod enzymes;
mod bacteria;
mod tannins;
mod equipment;
mod profiles;
mod prediction;

#[cfg(feature = "cannabis")]
mod cannabis;

// Schema version tracking
const SCHEMA_VERSION: i32 = 1;

/// Create user database schema (writable)
pub fn create_user_schema(conn: &Connection) -> Result<()> {
    core::create_user_tables(conn)?;
    create_schema_version(conn)?;
    Ok(())
}

/// Create master encyclopedia schema (read-only from packs)
pub fn create_encyclopedia_master_schema(conn: &Connection) -> Result<()> {
    // Base ingredients
    yeasts::create_yeast_tables(conn)?;
    honeys::create_honey_tables(conn)?;
    hops::create_hop_tables(conn)?;
    malts::create_malt_tables(conn)?;
    fruits::create_fruit_tables(conn)?;
    vegetables::create_vegetable_tables(conn)?;
    spices::create_spice_tables(conn)?;
    herbs::create_herb_tables(conn)?;

    // Additives & modifiers
    extracts::create_extract_tables(conn)?;
    syrups::create_syrup_tables(conn)?;
    adjuncts::create_adjunct_tables(conn)?;
    water::create_water_tables(conn)?;
    acids::create_acid_tables(conn)?;
    nutrients::create_nutrient_tables(conn)?;
    enzymes::create_enzyme_tables(conn)?;
    bacteria::create_bacteria_tables(conn)?;
    tannins::create_tannin_tables(conn)?;
    equipment::create_equipment_tables(conn)?;

    // Prediction & profiling
    profiles::create_profile_tables(conn)?;
    prediction::create_prediction_tables(conn)?;

    // Optional paid feature
    #[cfg(feature = "cannabis")]
    cannabis::create_cannabis_tables(conn)?;

    create_schema_version(conn)?;
    Ok(())
}

/// Create recipes master schema (read-only from packs)
pub fn create_recipes_master_schema(conn: &Connection) -> Result<()> {
    core::create_master_recipe_tables(conn)?;
    create_schema_version(conn)?;
    Ok(())
}

/// Create styles master schema (read-only from packs)
pub fn create_styles_master_schema(conn: &Connection) -> Result<()> {
    core::create_styles_tables(conn)?;
    create_schema_version(conn)?;
    Ok(())
}

/// Track schema version for migrations
fn create_schema_version(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("Schema version: {}", e)))?;

    conn.execute(
        "INSERT OR IGNORE INTO schema_version (version) VALUES (?1)",
        [SCHEMA_VERSION],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("Insert version: {}", e)))?;

    Ok(())
}

/// Verify database integrity
pub fn verify_database_integrity(conn: &Connection) -> Result<()> {
    conn.execute("PRAGMA integrity_check", [])
        .map_err(|e| mazerion_core::Error::DatabaseError(format!("Integrity check: {}", e)))?;
    Ok(())
}

/// Get current schema version
pub fn get_schema_version(conn: &Connection) -> Result<i32> {
    let version = conn
        .query_row("SELECT MAX(version) FROM schema_version", [], |row| {
            row.get(0)
        })
        .map_err(|e| mazerion_core::Error::DatabaseError(format!("Get version: {}", e)))?;
    Ok(version)
}