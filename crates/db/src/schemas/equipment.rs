// Brewing equipment encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_equipment_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS equipment (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Basic Info
            manufacturer TEXT,
            equipment_type TEXT,
            equipment_category TEXT,

            -- Physical Specs
            material TEXT,
            capacity TEXT,
            dimensions TEXT,
            weight TEXT,

            -- Performance
            efficiency_rating TEXT,
            heat_loss_rate TEXT,
            flow_rate TEXT,
            pressure_rating TEXT,

            -- Usage
            recommended_batch_size TEXT,
            recommended_usage TEXT,
            skill_level_required TEXT,

            -- Maintenance
            cleaning_requirements TEXT,
            maintenance_schedule TEXT,
            replacement_parts TEXT,
            typical_lifespan TEXT,

            -- Economics
            price_range TEXT,
            cost_per_batch TEXT,
            roi_timeline TEXT,

            -- Compatibility
            compatible_equipment TEXT,
            required_accessories TEXT,
            optional_upgrades TEXT,

            -- Alternatives
            alternatives TEXT,
            upgrade_path TEXT,

            -- Professional Notes
            brewmaster_review TEXT,
            pro_tips TEXT,
            common_issues TEXT,
            description TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create equipment: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_equipment_type ON equipment(equipment_type)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_equipment_category ON equipment(equipment_category)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}