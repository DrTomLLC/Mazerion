/// Spice Encyclopedia Schema
///
/// Comprehensive spice database for brewing and fermentation.
/// Optimized for mobile performance with strategic indexing.
/// HARDENED: Multiple CHECK constraints, composite indexes, comprehensive tests.

pub const SPICE_SCHEMA: &str = "
-- Spices encyclopedia
-- Professional brewing spice database with security hardening
CREATE TABLE IF NOT EXISTS spices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Core identification
    name TEXT NOT NULL,                      -- e.g., 'Cinnamon', 'Coriander'
    scientific_name TEXT,                    -- e.g., 'Cinnamomum verum', NULL if unknown
    spice_type TEXT NOT NULL,                -- warming, savory, sweet, hot, aromatic, other
    origin TEXT,                             -- Geographic origin, NULL if unknown

    -- Heat characteristics (TEXT for Decimal precision)
    heat_level TEXT,                         -- 0.0-10.0 Scoville-inspired scale, NULL if unknown
    typical_dosage_oz_per_gallon TEXT,       -- Recommended usage 0.0-10.0 oz/gal

    -- Professional sensory profiles (JSON arrays for structured data)
    flavor_profile TEXT,                     -- JSON array: Master-level flavor descriptors
    aroma_profile TEXT,                      -- JSON array: Professional aroma descriptors

    -- Usage recommendations
    best_suited_styles TEXT,                 -- JSON array of beverage styles
    usage_notes TEXT,                        -- Professional brewing notes and warnings
    sensory_notes TEXT,                      -- Master-level sensory evaluation
    preparation_method TEXT,                 -- Processing instructions (whole/ground/toasted)

    -- Compatibility
    compatible_styles TEXT,                  -- JSON array of compatible beverage styles

    -- Metadata
    created_at TEXT NOT NULL,                -- ISO 8601 timestamp
    updated_at TEXT NOT NULL,                -- ISO 8601 timestamp

    -- Validation constraints (security hardening)
    CHECK(spice_type IN ('warming', 'savory', 'sweet', 'hot', 'aromatic', 'other')),
    CHECK(name != ''),                       -- Prevent empty names
    CHECK(length(name) <= 200)               -- Prevent abuse
);

-- Performance indexes for mobile-first queries
-- Index on name for search operations (most common query)
CREATE INDEX IF NOT EXISTS idx_spices_name ON spices(name);

-- Index on type for filtering by category
CREATE INDEX IF NOT EXISTS idx_spices_type ON spices(spice_type);

-- Composite index for type + name sorting (common query pattern)
CREATE INDEX IF NOT EXISTS idx_spices_type_name ON spices(spice_type, name);
";

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_schema_creates_successfully() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(SPICE_SCHEMA).expect("Failed to execute schema");

        let table_exists: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='spices'",
                [],
                |row| row.get(0),
            )
            .expect("Failed to query table existence");

        assert_eq!(table_exists, 1, "Spices table should exist");
    }

    #[test]
    fn test_schema_has_correct_indexes() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(SPICE_SCHEMA).expect("Failed to execute schema");

        let index_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type='index'
                 AND tbl_name='spices'
                 AND name LIKE 'idx_%'",
                [],
                |row| row.get(0),
            )
            .expect("Failed to query index count");

        assert_eq!(index_count, 3, "Should have 3 performance indexes including composite");
    }

    #[test]
    fn test_schema_enforces_type_constraint() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(SPICE_SCHEMA).expect("Failed to execute schema");

        let result = conn.execute(
            "INSERT INTO spices (name, spice_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
            rusqlite::params!["Test", "invalid", "2025-01-01", "2025-01-01"],
        );

        assert!(result.is_err(), "Should reject invalid spice type");
    }

    #[test]
    fn test_schema_accepts_valid_types() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(SPICE_SCHEMA).expect("Failed to execute schema");

        let valid_types = vec!["warming", "savory", "sweet", "hot", "aromatic", "other"];

        for spice_type in valid_types {
            let result = conn.execute(
                "INSERT INTO spices (name, spice_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                rusqlite::params![format!("Test {}", spice_type), spice_type, "2025-01-01", "2025-01-01"],
            );
            assert!(result.is_ok(), "Should accept valid type: {}", spice_type);
        }
    }

    #[test]
    fn test_schema_requires_non_null_fields() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(SPICE_SCHEMA).expect("Failed to execute schema");

        let result = conn.execute(
            "INSERT INTO spices (spice_type, created_at, updated_at) VALUES (?, ?, ?)",
            rusqlite::params!["warming", "2025-01-01", "2025-01-01"],
        );

        assert!(result.is_err(), "Should require name field");
    }

    #[test]
    fn test_schema_rejects_empty_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(SPICE_SCHEMA).expect("Failed to execute schema");

        let result = conn.execute(
            "INSERT INTO spices (name, spice_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
            rusqlite::params!["", "warming", "2025-01-01", "2025-01-01"],
        );

        assert!(result.is_err(), "Should reject empty name");
    }

    #[test]
    fn test_schema_rejects_oversized_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(SPICE_SCHEMA).expect("Failed to execute schema");

        let long_name = "A".repeat(201);
        let result = conn.execute(
            "INSERT INTO spices (name, spice_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
            rusqlite::params![long_name, "warming", "2025-01-01", "2025-01-01"],
        );

        assert!(result.is_err(), "Should reject names longer than 200 characters");
    }
}