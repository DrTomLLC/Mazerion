pub mod yeasts;
pub mod honeys;
pub mod hops;
pub mod malts;
pub mod fruits;
pub mod vegetables;
pub mod spices;
pub mod herbs;
pub mod extracts;
pub mod syrups;
pub mod adjuncts;
pub mod water_profiles;
pub mod water_salts;
pub mod acids;
pub mod nutrients;
pub mod enzymes;
pub mod bacteria;
pub mod tannins;
pub mod prediction;
pub mod profiles;

use rusqlite::Connection;
use mazerion_core::Result;

pub fn init_all_tables(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(yeasts::YEAST_SCHEMA)?;
    conn.execute_batch(honeys::HONEY_SCHEMA)?;
    conn.execute_batch(hops::HOP_SCHEMA)?;
    conn.execute_batch(malts::MALT_SCHEMA)?;
    conn.execute_batch(fruits::FRUIT_SCHEMA)?;
    conn.execute_batch(vegetables::VEGETABLE_SCHEMA)?;
    conn.execute_batch(spices::SPICE_SCHEMA)?;
    conn.execute_batch(herbs::HERB_SCHEMA)?;
    conn.execute_batch(extracts::EXTRACT_SCHEMA)?;
    conn.execute_batch(syrups::SYRUP_SCHEMA)?;
    conn.execute_batch(adjuncts::ADJUNCT_SCHEMA)?;
    conn.execute_batch(water_profiles::WATER_PROFILE_SCHEMA)?;
    conn.execute_batch(water_salts::WATER_SALT_SCHEMA)?;
    conn.execute_batch(acids::ACID_SCHEMA)?;
    conn.execute_batch(nutrients::NUTRIENT_SCHEMA)?;
    conn.execute_batch(enzymes::ENZYME_SCHEMA)?;
    conn.execute_batch(bacteria::BACTERIA_SCHEMA)?;
    conn.execute_batch(tannins::TANNIN_SCHEMA)?;
    Ok(())
}

pub fn create_user_schema(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS batches (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            recipe_id INTEGER,
            category TEXT NOT NULL,
            batch_size_l TEXT NOT NULL,
            brew_date TEXT NOT NULL,
            target_og TEXT,
            target_fg TEXT,
            target_abv TEXT,
            status TEXT NOT NULL DEFAULT 'planning',
            notes TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("Create batches: {}", e)))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS batch_readings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            batch_id INTEGER NOT NULL,
            reading_date TEXT NOT NULL,
            gravity TEXT NOT NULL,
            temperature_c TEXT,
            ph TEXT,
            notes TEXT,
            source TEXT DEFAULT 'manual',
            FOREIGN KEY(batch_id) REFERENCES batches(id) ON DELETE CASCADE
        )",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("Create batch_readings: {}", e)))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS inventory (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            item_type TEXT NOT NULL,
            item_name TEXT NOT NULL,
            quantity TEXT NOT NULL,
            unit TEXT NOT NULL,
            location TEXT,
            purchase_date TEXT,
            expiration_date TEXT,
            cost TEXT,
            notes TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("Create inventory: {}", e)))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_recipes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            subcategory TEXT,
            description TEXT,
            batch_size_l TEXT NOT NULL,
            target_og TEXT,
            target_fg TEXT,
            target_abv TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("Create user_recipes: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_batches_status ON batches(status)",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("{}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_readings_batch ON batch_readings(batch_id)",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("{}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_inventory_type ON inventory(item_type)",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("{}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_user_recipes_category ON user_recipes(category)",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}