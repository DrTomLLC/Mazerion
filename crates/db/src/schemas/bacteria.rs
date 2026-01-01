/// Bacteria Encyclopedia Schema
///
/// Comprehensive bacteria database for sour fermentation and flavor development.
/// Optimized for mobile performance with strategic indexing.
/// HARDENED: Multiple CHECK constraints, composite indexes, comprehensive tests.

pub const BACTERIA_SCHEMA: &str = "
-- Bacteria encyclopedia
-- Professional brewing bacteria database with security hardening
CREATE TABLE IF NOT EXISTS bacteria (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Core identification
    name TEXT NOT NULL,                      -- e.g., 'Lactobacillus plantarum', 'Pediococcus'
    bacteria_type TEXT NOT NULL,             -- lactobacillus, pediococcus, acetobacter, other
    laboratory TEXT,                         -- e.g., 'White Labs', 'Wyeast', NULL if unknown
    product_code TEXT,                       -- e.g., 'WLP672', NULL if unknown

    -- Fermentation characteristics (TEXT for Decimal precision)
    optimal_temperature_min TEXT,            -- °F 32-212, NULL if unknown
    optimal_temperature_max TEXT,            -- °F 32-212, NULL if unknown
    optimal_ph_min TEXT,                     -- pH 1.0-14.0, NULL if unknown
    optimal_ph_max TEXT,                     -- pH 1.0-14.0, NULL if unknown
    typical_dosage_grams_per_gallon TEXT,    -- Recommended usage 0-10 g/gal

    -- Professional guidance (JSON arrays for structured data)
    usage_notes TEXT,                        -- Professional brewing notes and warnings
    flavor_profile TEXT,                     -- Flavor contributions (sour, funky, etc)
    best_suited_styles TEXT,                 -- JSON array of beverage styles
    compatible_styles TEXT,                  -- JSON array of compatible styles
    timing TEXT,                             -- When to add (pre-ferment, kettle sour, etc)

    -- Metadata
    created_at TEXT NOT NULL,                -- ISO 8601 timestamp
    updated_at TEXT NOT NULL,                -- ISO 8601 timestamp

    -- Validation constraints (security hardening)
    CHECK(bacteria_type IN ('lactobacillus', 'pediococcus', 'acetobacter', 'other')),
    CHECK(name != ''),                       -- Prevent empty names
    CHECK(length(name) <= 200)               -- Prevent abuse
);

-- Performance indexes for mobile-first queries
CREATE INDEX IF NOT EXISTS idx_bacteria_name ON bacteria(name);
CREATE INDEX IF NOT EXISTS idx_bacteria_type ON bacteria(bacteria_type);
CREATE INDEX IF NOT EXISTS idx_bacteria_type_name ON bacteria(bacteria_type, name);
";

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_schema_creates_successfully() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(BACTERIA_SCHEMA).expect("Failed to execute schema");
        let table_exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='bacteria'", [], |row| row.get(0))
            .expect("Failed to query table existence");
        assert_eq!(table_exists, 1);
    }

    #[test]
    fn test_schema_has_correct_indexes() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(BACTERIA_SCHEMA).expect("Failed to execute schema");
        let index_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND tbl_name='bacteria' AND name LIKE 'idx_%'", [], |row| row.get(0))
            .expect("Failed to query index count");
        assert_eq!(index_count, 3);
    }

    #[test]
    fn test_schema_enforces_type_constraint() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(BACTERIA_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO bacteria (name, bacteria_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["Test", "invalid", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_schema_accepts_valid_types() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(BACTERIA_SCHEMA).expect("Failed to execute schema");
        for bacteria_type in vec!["lactobacillus", "pediococcus", "acetobacter", "other"] {
            let result = conn.execute("INSERT INTO bacteria (name, bacteria_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                      rusqlite::params![format!("Test {}", bacteria_type), bacteria_type, "2025-01-01", "2025-01-01"]);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_schema_requires_non_null_fields() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(BACTERIA_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO bacteria (bacteria_type, created_at, updated_at) VALUES (?, ?, ?)",
                                  rusqlite::params!["lactobacillus", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_schema_rejects_empty_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(BACTERIA_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO bacteria (name, bacteria_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["", "lactobacillus", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_schema_rejects_oversized_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(BACTERIA_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO bacteria (name, bacteria_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["A".repeat(201), "lactobacillus", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err());
    }
}