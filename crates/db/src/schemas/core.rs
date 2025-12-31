// Core user database schema - batches, inventory, recipes, calculations

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_user_tables(conn: &Connection) -> Result<()> {
    // ══════════════════════════════════════════════════════════════════════
    // BATCHES
    // ══════════════════════════════════════════════════════════════════════
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
            status TEXT NOT NULL,
            notes TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP,

            FOREIGN KEY(recipe_id) REFERENCES user_recipes(id) ON DELETE SET NULL
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create batches: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_batches_status ON batches(status)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_batches_category ON batches(category)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    // ══════════════════════════════════════════════════════════════════════
    // BATCH READINGS
    // ══════════════════════════════════════════════════════════════════════
    conn.execute(
        "CREATE TABLE IF NOT EXISTS batch_readings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            batch_id INTEGER NOT NULL,
            reading_date TEXT NOT NULL,
            gravity TEXT NOT NULL,
            temperature_c TEXT,
            ph TEXT,
            notes TEXT,
            source TEXT NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,

            FOREIGN KEY(batch_id) REFERENCES batches(id) ON DELETE CASCADE
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create batch_readings: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_readings_batch ON batch_readings(batch_id)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    // ══════════════════════════════════════════════════════════════════════
    // INVENTORY
    // ══════════════════════════════════════════════════════════════════════
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
    ).map_err(|e| Error::DatabaseError(format!("Create inventory: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_inventory_type ON inventory(item_type)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    // ══════════════════════════════════════════════════════════════════════
    // USER RECIPES
    // ══════════════════════════════════════════════════════════════════════
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
    ).map_err(|e| Error::DatabaseError(format!("Create user_recipes: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_user_recipes_category ON user_recipes(category)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    // ══════════════════════════════════════════════════════════════════════
    // RECIPE INGREDIENTS
    // ══════════════════════════════════════════════════════════════════════
    conn.execute(
        "CREATE TABLE IF NOT EXISTS recipe_ingredients (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            recipe_id INTEGER NOT NULL,
            ingredient_type TEXT NOT NULL,
            ingredient_name TEXT NOT NULL,
            amount TEXT NOT NULL,
            unit TEXT NOT NULL,
            timing TEXT,
            notes TEXT,

            FOREIGN KEY(recipe_id) REFERENCES user_recipes(id) ON DELETE CASCADE
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create recipe_ingredients: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_recipe_ingredients ON recipe_ingredients(recipe_id)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    // ══════════════════════════════════════════════════════════════════════
    // RECIPE INSTRUCTIONS
    // ══════════════════════════════════════════════════════════════════════
    conn.execute(
        "CREATE TABLE IF NOT EXISTS recipe_instructions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            recipe_id INTEGER NOT NULL,
            step_number INTEGER NOT NULL,
            instruction TEXT NOT NULL,
            duration_minutes INTEGER,
            temperature_c TEXT,

            FOREIGN KEY(recipe_id) REFERENCES user_recipes(id) ON DELETE CASCADE
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create recipe_instructions: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_recipe_instructions ON recipe_instructions(recipe_id)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    // ══════════════════════════════════════════════════════════════════════
    // CALCULATION LOG
    // ══════════════════════════════════════════════════════════════════════
    conn.execute(
        "CREATE TABLE IF NOT EXISTS calculation_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            calculator_id TEXT NOT NULL,
            inputs TEXT NOT NULL,
            result TEXT NOT NULL,
            timestamp TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create calculation_log: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_calc_log_calc ON calculation_log(calculator_id)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_calc_log_time ON calculation_log(timestamp)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}

pub fn create_master_recipe_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS recipes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            subcategory TEXT,
            description TEXT,
            author TEXT,
            source TEXT,
            difficulty TEXT,
            batch_size_l TEXT NOT NULL,
            target_og TEXT,
            target_fg TEXT,
            target_abv TEXT
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create recipes: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_recipes_category ON recipes(category)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}

pub fn create_styles_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS bjcp_styles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            category TEXT NOT NULL,
            subcategory TEXT,
            style_name TEXT NOT NULL,
            og_min TEXT,
            og_max TEXT,
            fg_min TEXT,
            fg_max TEXT,
            abv_min TEXT,
            abv_max TEXT,
            ibu_min TEXT,
            ibu_max TEXT,
            srm_min TEXT,
            srm_max TEXT,
            description TEXT
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create bjcp_styles: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_styles_category ON bjcp_styles(category)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}