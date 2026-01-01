/// Adjunct Encyclopedia Schema
///
/// Comprehensive adjunct database for brewing and fermentation.
/// Optimized for mobile performance with strategic indexing.
/// HARDENED: Multiple CHECK constraints, composite indexes, comprehensive tests.

pub const ADJUNCT_SCHEMA: &str = "
-- Adjuncts encyclopedia
-- Professional brewing adjunct database with security hardening
CREATE TABLE IF NOT EXISTS adjuncts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Core identification
    name TEXT NOT NULL,                      -- e.g., 'Corn Sugar (Dextrose)'
    adjunct_type TEXT NOT NULL,              -- sugar, honey, grain, other
    manufacturer TEXT,                       -- Brand/producer, NULL if unknown

    -- Fermentation characteristics (TEXT for Decimal precision)
    fermentability TEXT,                     -- Percentage 0.0-100.0, NULL if unknown
    typical_percentage TEXT,                 -- Typical usage 0.0-100.0% of fermentables

    -- Professional sensory profiles (JSON arrays for structured data)
    flavor_profile TEXT,                     -- JSON array: Master-level flavor descriptors

    -- Usage recommendations
    best_suited_styles TEXT,                 -- JSON array of beverage styles
    usage_notes TEXT,                        -- Professional brewing notes and warnings

    -- Compatibility
    compatible_styles TEXT,                  -- JSON array of compatible beverage styles

    -- Metadata
    created_at TEXT NOT NULL,                -- ISO 8601 timestamp
    updated_at TEXT NOT NULL,                -- ISO 8601 timestamp

    -- Validation constraints (security hardening)
    CHECK(adjunct_type IN ('sugar', 'honey', 'grain', 'other')),
    CHECK(name != ''),                       -- Prevent empty names
    CHECK(length(name) <= 200)               -- Prevent abuse
);

-- Performance indexes for mobile-first queries
-- Index on name for search operations (most common query)
CREATE INDEX IF NOT EXISTS idx_adjuncts_name ON adjuncts(name);

-- Index on type for filtering by category
CREATE INDEX IF NOT EXISTS idx_adjuncts_type ON adjuncts(adjunct_type);

-- Composite index for type + name sorting (common query pattern)
CREATE INDEX IF NOT EXISTS idx_adjuncts_type_name ON adjuncts(adjunct_type, name);
";

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_schema_creates_successfully() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(ADJUNCT_SCHEMA).expect("Failed to execute schema");
        let table_exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='adjuncts'", [], |row| row.get(0))
            .expect("Failed to query table existence");
        assert_eq!(table_exists, 1, "Adjuncts table should exist");
    }

    #[test]
    fn test_schema_has_correct_indexes() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(ADJUNCT_SCHEMA).expect("Failed to execute schema");
        let index_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND tbl_name='adjuncts' AND name LIKE 'idx_%'", [], |row| row.get(0))
            .expect("Failed to query index count");
        assert_eq!(index_count, 3, "Should have 3 performance indexes including composite");
    }

    #[test]
    fn test_schema_enforces_type_constraint() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(ADJUNCT_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO adjuncts (name, adjunct_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["Test", "invalid", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err(), "Should reject invalid adjunct type");
    }

    #[test]
    fn test_schema_accepts_valid_types() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(ADJUNCT_SCHEMA).expect("Failed to execute schema");
        for adjunct_type in vec!["sugar", "honey", "grain", "other"] {
            let result = conn.execute("INSERT INTO adjuncts (name, adjunct_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                      rusqlite::params![format!("Test {}", adjunct_type), adjunct_type, "2025-01-01", "2025-01-01"]);
            assert!(result.is_ok(), "Should accept valid type: {}", adjunct_type);
        }
    }

    #[test]
    fn test_schema_requires_non_null_fields() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(ADJUNCT_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO adjuncts (adjunct_type, created_at, updated_at) VALUES (?, ?, ?)",
                                  rusqlite::params!["sugar", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err(), "Should require name field");
    }

    #[test]
    fn test_schema_rejects_empty_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(ADJUNCT_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO adjuncts (name, adjunct_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["", "sugar", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err(), "Should reject empty name");
    }

    #[test]
    fn test_schema_rejects_oversized_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(ADJUNCT_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO adjuncts (name, adjunct_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["A".repeat(201), "sugar", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err(), "Should reject names longer than 200 characters");
    }
}