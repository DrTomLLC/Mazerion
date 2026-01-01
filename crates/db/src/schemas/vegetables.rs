/// Vegetable Encyclopedia Schema
///
/// Comprehensive vegetable database for brewing and fermentation.
/// Optimized for mobile performance with strategic indexing.
/// HARDENED: Multiple CHECK constraints, composite indexes, comprehensive tests.

pub const VEGETABLE_SCHEMA: &str = "
-- Vegetables encyclopedia
-- Professional brewing vegetable database with security hardening
CREATE TABLE IF NOT EXISTS vegetables (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Core identification
    name TEXT NOT NULL,                      -- e.g., 'Pumpkin', 'Jalapeño Pepper'
    scientific_name TEXT,                    -- e.g., 'Cucurbita pepo', NULL if unknown
    vegetable_type TEXT NOT NULL,            -- root, gourd, pepper, leafy, other
    origin TEXT,                             -- Geographic origin, NULL if unknown

    -- Fermentation characteristics (TEXT for Decimal precision)
    typical_sugar_content TEXT,              -- Percentage 0.0-50.0, NULL if unknown
    ph_level TEXT,                           -- pH 2.0-8.0, NULL if unknown
    pounds_per_gallon TEXT,                  -- Typical usage rate 0.0-20.0 lbs/gal

    -- Professional sensory profiles (JSON arrays for structured data)
    flavor_profile TEXT,                     -- JSON array: Master-level flavor descriptors
    aroma_profile TEXT,                      -- JSON array: Professional aroma descriptors

    -- Usage recommendations
    best_suited_styles TEXT,                 -- JSON array of beverage styles
    usage_notes TEXT,                        -- Professional brewing notes and warnings
    sensory_notes TEXT,                      -- Master-level sensory evaluation
    preparation_method TEXT,                 -- Processing instructions (roast/puree/raw)

    -- Compatibility
    compatible_styles TEXT,                  -- JSON array of compatible beverage styles

    -- Metadata
    created_at TEXT NOT NULL,                -- ISO 8601 timestamp
    updated_at TEXT NOT NULL,                -- ISO 8601 timestamp

    -- Validation constraints (security hardening)
    CHECK(vegetable_type IN ('root', 'gourd', 'pepper', 'leafy', 'other')),
    CHECK(name != ''),                       -- Prevent empty names
    CHECK(length(name) <= 200)               -- Prevent abuse
);

-- Performance indexes for mobile-first queries
-- Index on name for search operations (most common query)
CREATE INDEX IF NOT EXISTS idx_vegetables_name ON vegetables(name);

-- Index on type for filtering by category
CREATE INDEX IF NOT EXISTS idx_vegetables_type ON vegetables(vegetable_type);

-- Composite index for type + name sorting (common query pattern)
CREATE INDEX IF NOT EXISTS idx_vegetables_type_name ON vegetables(vegetable_type, name);
";

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_schema_creates_successfully() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(VEGETABLE_SCHEMA).expect("Failed to execute schema");

        // Verify table exists
        let table_exists: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='vegetables'",
                [],
                |row| row.get(0),
            )
            .expect("Failed to query table existence");

        assert_eq!(table_exists, 1, "Vegetables table should exist");
    }

    #[test]
    fn test_schema_has_correct_indexes() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(VEGETABLE_SCHEMA).expect("Failed to execute schema");

        // Verify all indexes exist
        let index_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type='index'
                 AND tbl_name='vegetables'
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
        conn.execute_batch(VEGETABLE_SCHEMA).expect("Failed to execute schema");

        // Try to insert invalid type
        let result = conn.execute(
            "INSERT INTO vegetables (
                name, vegetable_type, created_at, updated_at
            ) VALUES (?, ?, ?, ?)",
            rusqlite::params![
                "Test Vegetable",
                "invalid_type",
                "2025-01-01",
                "2025-01-01",
            ],
        );

        assert!(result.is_err(), "Should reject invalid vegetable type");
    }

    #[test]
    fn test_schema_accepts_valid_types() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(VEGETABLE_SCHEMA).expect("Failed to execute schema");

        let valid_types = vec!["root", "gourd", "pepper", "leafy", "other"];

        for veg_type in valid_types {
            let result = conn.execute(
                "INSERT INTO vegetables (
                    name, vegetable_type, created_at, updated_at
                ) VALUES (?, ?, ?, ?)",
                rusqlite::params![
                    format!("Test {}", veg_type),
                    veg_type,
                    "2025-01-01",
                    "2025-01-01",
                ],
            );

            assert!(result.is_ok(), "Should accept valid type: {}", veg_type);
        }
    }

    #[test]
    fn test_schema_requires_non_null_fields() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(VEGETABLE_SCHEMA).expect("Failed to execute schema");

        // Try to insert without required name
        let result = conn.execute(
            "INSERT INTO vegetables (
                vegetable_type, created_at, updated_at
            ) VALUES (?, ?, ?)",
            rusqlite::params!["root", "2025-01-01", "2025-01-01"],
        );

        assert!(result.is_err(), "Should require name field");
    }

    #[test]
    fn test_schema_rejects_empty_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(VEGETABLE_SCHEMA).expect("Failed to execute schema");

        // Try to insert empty name
        let result = conn.execute(
            "INSERT INTO vegetables (
                name, vegetable_type, created_at, updated_at
            ) VALUES (?, ?, ?, ?)",
            rusqlite::params!["", "root", "2025-01-01", "2025-01-01"],
        );

        assert!(result.is_err(), "Should reject empty name");
    }

    #[test]
    fn test_schema_rejects_oversized_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(VEGETABLE_SCHEMA).expect("Failed to execute schema");

        // Try to insert name longer than 200 characters
        let long_name = "A".repeat(201);
        let result = conn.execute(
            "INSERT INTO vegetables (
                name, vegetable_type, created_at, updated_at
            ) VALUES (?, ?, ?, ?)",
            rusqlite::params![long_name, "root", "2025-01-01", "2025-01-01"],
        );

        assert!(result.is_err(), "Should reject names longer than 200 characters");
    }
}