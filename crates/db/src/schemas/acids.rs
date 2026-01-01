/// Water Salt Encyclopedia Schema
///
/// Comprehensive water salt/mineral database for brewing water chemistry.
/// Optimized for mobile performance with strategic indexing.
/// HARDENED: Multiple CHECK constraints, composite indexes, comprehensive tests.

pub const ACID_SCHEMA: &str = "
-- Water salts encyclopedia
-- Professional brewing water chemistry salts database with security hardening
CREATE TABLE IF NOT EXISTS water_salts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Core identification
    name TEXT NOT NULL,                      -- e.g., 'Gypsum (CaSO₄)', 'Calcium Chloride'
    salt_type TEXT NOT NULL,                 -- gypsum, calcium_chloride, epsom, baking_soda, chalk, table_salt, other
    chemical_formula TEXT,                   -- e.g., 'CaSO₄·2H₂O', NULL if unknown
    manufacturer TEXT,                       -- Brand/producer, NULL if unknown

    -- Ion contributions (TEXT for Decimal precision) - ppm per gram
    calcium_contribution TEXT,               -- Ca²⁺ ppm/g, 0-500, NULL if unknown
    magnesium_contribution TEXT,             -- Mg²⁺ ppm/g, 0-500, NULL if unknown
    sodium_contribution TEXT,                -- Na⁺ ppm/g, 0-500, NULL if unknown
    chloride_contribution TEXT,              -- Cl⁻ ppm/g, 0-500, NULL if unknown
    sulfate_contribution TEXT,               -- SO₄²⁻ ppm/g, 0-500, NULL if unknown
    bicarbonate_contribution TEXT,           -- HCO₃⁻ ppm/g, 0-500, NULL if unknown

    -- Usage characteristics (TEXT for Decimal precision)
    typical_dosage_grams_per_gallon TEXT,    -- Recommended usage 0-10 g/gal
    solubility TEXT,                         -- Solubility description

    -- Professional guidance (JSON arrays for structured data)
    usage_notes TEXT,                        -- Professional brewing notes and warnings
    flavor_impact TEXT,                      -- Description of flavor contributions
    best_suited_styles TEXT,                 -- JSON array of beverage styles
    compatible_styles TEXT,                  -- JSON array of compatible styles

    -- Metadata
    created_at TEXT NOT NULL,                -- ISO 8601 timestamp
    updated_at TEXT NOT NULL,                -- ISO 8601 timestamp

    -- Validation constraints (security hardening)
    CHECK(salt_type IN ('gypsum', 'calcium_chloride', 'epsom', 'baking_soda', 'chalk', 'table_salt', 'other')),
    CHECK(name != ''),                       -- Prevent empty names
    CHECK(length(name) <= 200)               -- Prevent abuse
);

-- Performance indexes for mobile-first queries
-- Index on name for search operations (most common query)
CREATE INDEX IF NOT EXISTS idx_water_salts_name ON water_salts(name);

-- Index on type for filtering by category
CREATE INDEX IF NOT EXISTS idx_water_salts_type ON water_salts(salt_type);

-- Composite index for type + name sorting (common query pattern)
CREATE INDEX IF NOT EXISTS idx_water_salts_type_name ON water_salts(salt_type, name);
";

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_schema_creates_successfully() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(WATER_SALT_SCHEMA).expect("Failed to execute schema");
        let table_exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='water_salts'", [], |row| row.get(0))
            .expect("Failed to query table existence");
        assert_eq!(table_exists, 1, "Water salts table should exist");
    }

    #[test]
    fn test_schema_has_correct_indexes() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(WATER_SALT_SCHEMA).expect("Failed to execute schema");
        let index_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND tbl_name='water_salts' AND name LIKE 'idx_%'", [], |row| row.get(0))
            .expect("Failed to query index count");
        assert_eq!(index_count, 3, "Should have 3 performance indexes including composite");
    }

    #[test]
    fn test_schema_enforces_type_constraint() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(WATER_SALT_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO water_salts (name, salt_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["Test", "invalid", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err(), "Should reject invalid salt type");
    }

    #[test]
    fn test_schema_accepts_valid_types() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(WATER_SALT_SCHEMA).expect("Failed to execute schema");
        for salt_type in vec!["gypsum", "calcium_chloride", "epsom", "baking_soda", "chalk", "table_salt", "other"] {
            let result = conn.execute("INSERT INTO water_salts (name, salt_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                      rusqlite::params![format!("Test {}", salt_type), salt_type, "2025-01-01", "2025-01-01"]);
            assert!(result.is_ok(), "Should accept valid type: {}", salt_type);
        }
    }

    #[test]
    fn test_schema_requires_non_null_fields() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(WATER_SALT_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO water_salts (salt_type, created_at, updated_at) VALUES (?, ?, ?)",
                                  rusqlite::params!["gypsum", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err(), "Should require name field");
    }

    #[test]
    fn test_schema_rejects_empty_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(WATER_SALT_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO water_salts (name, salt_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["", "gypsum", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err(), "Should reject empty name");
    }

    #[test]
    fn test_schema_rejects_oversized_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(WATER_SALT_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO water_salts (name, salt_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["A".repeat(201), "gypsum", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err(), "Should reject names longer than 200 characters");
    }
}