/// Extract Encyclopedia Schema
///
/// Comprehensive extract database for brewing and fermentation.
/// Optimized for mobile performance with strategic indexing.
/// HARDENED: Multiple CHECK constraints, composite indexes, comprehensive tests.

pub const EXTRACT_SCHEMA: &str = "
-- Extracts encyclopedia
-- Professional brewing extract database with security hardening
CREATE TABLE IF NOT EXISTS extracts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Core identification
    name TEXT NOT NULL,                      -- e.g., 'Madagascar Vanilla Extract'
    extract_type TEXT NOT NULL,              -- vanilla, almond, fruit, coffee, chocolate, other
    manufacturer TEXT,                       -- Brand/producer, NULL if unknown

    -- Dosage characteristics (TEXT for Decimal precision)
    typical_dosage_oz_per_gallon TEXT,       -- Recommended usage 0.0-10.0 oz/gal

    -- Extract properties
    alcohol_based INTEGER NOT NULL DEFAULT 1, -- Boolean: alcohol (1) or glycerin based (0)

    -- Professional sensory profiles (JSON arrays for structured data)
    flavor_profile TEXT,                     -- JSON array: Master-level flavor descriptors
    aroma_profile TEXT,                      -- JSON array: Professional aroma descriptors

    -- Usage recommendations
    best_suited_styles TEXT,                 -- JSON array of beverage styles
    usage_notes TEXT,                        -- Professional brewing notes and warnings

    -- Compatibility
    compatible_styles TEXT,                  -- JSON array of compatible beverage styles

    -- Metadata
    created_at TEXT NOT NULL,                -- ISO 8601 timestamp
    updated_at TEXT NOT NULL,                -- ISO 8601 timestamp

    -- Validation constraints (security hardening)
    CHECK(extract_type IN ('vanilla', 'almond', 'fruit', 'coffee', 'chocolate', 'other')),
    CHECK(alcohol_based IN (0, 1)),          -- Boolean validation
    CHECK(name != ''),                       -- Prevent empty names
    CHECK(length(name) <= 200)               -- Prevent abuse
);

-- Performance indexes for mobile-first queries
-- Index on name for search operations (most common query)
CREATE INDEX IF NOT EXISTS idx_extracts_name ON extracts(name);

-- Index on type for filtering by category
CREATE INDEX IF NOT EXISTS idx_extracts_type ON extracts(extract_type);

-- Composite index for type + name sorting (common query pattern)
CREATE INDEX IF NOT EXISTS idx_extracts_type_name ON extracts(extract_type, name);
";

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_schema_creates_successfully() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(EXTRACT_SCHEMA).expect("Failed to execute schema");
        let table_exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='extracts'", [], |row| row.get(0))
            .expect("Failed to query table existence");
        assert_eq!(table_exists, 1, "Extracts table should exist");
    }

    #[test]
    fn test_schema_has_correct_indexes() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(EXTRACT_SCHEMA).expect("Failed to execute schema");
        let index_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND tbl_name='extracts' AND name LIKE 'idx_%'", [], |row| row.get(0))
            .expect("Failed to query index count");
        assert_eq!(index_count, 3, "Should have 3 performance indexes including composite");
    }

    #[test]
    fn test_schema_enforces_type_constraint() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(EXTRACT_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO extracts (name, extract_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["Test", "invalid", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err(), "Should reject invalid extract type");
    }

    #[test]
    fn test_schema_accepts_valid_types() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(EXTRACT_SCHEMA).expect("Failed to execute schema");
        for extract_type in vec!["vanilla", "almond", "fruit", "coffee", "chocolate", "other"] {
            let result = conn.execute("INSERT INTO extracts (name, extract_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                      rusqlite::params![format!("Test {}", extract_type), extract_type, "2025-01-01", "2025-01-01"]);
            assert!(result.is_ok(), "Should accept valid type: {}", extract_type);
        }
    }

    #[test]
    fn test_schema_enforces_alcohol_based_constraint() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(EXTRACT_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO extracts (name, extract_type, alcohol_based, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
                                  rusqlite::params!["Test", "vanilla", 2, "2025-01-01", "2025-01-01"]);
        assert!(result.is_err(), "Should reject invalid alcohol_based value");
    }

    #[test]
    fn test_schema_default_alcohol_based() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(EXTRACT_SCHEMA).expect("Failed to execute schema");
        conn.execute("INSERT INTO extracts (name, extract_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                     rusqlite::params!["Test", "vanilla", "2025-01-01", "2025-01-01"])
            .expect("Failed to insert test data");
        let alcohol_based: i32 = conn.query_row(
            "SELECT alcohol_based FROM extracts WHERE name = 'Test'", [], |row| row.get(0))
            .expect("Failed to query alcohol_based");
        assert_eq!(alcohol_based, 1, "Should default alcohol_based to 1");
    }

    #[test]
    fn test_schema_rejects_empty_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(EXTRACT_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO extracts (name, extract_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["", "vanilla", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err(), "Should reject empty name");
    }

    #[test]
    fn test_schema_rejects_oversized_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(EXTRACT_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO extracts (name, extract_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["A".repeat(201), "vanilla", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err(), "Should reject names longer than 200 characters");
    }
}