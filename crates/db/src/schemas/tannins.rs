/// Tannin Encyclopedia Schema
///
/// Comprehensive tannin database for mouthfeel, structure, and preservation.
/// Optimized for mobile performance with strategic indexing.
/// HARDENED: Multiple CHECK constraints, composite indexes, comprehensive tests.

pub const TANNIN_SCHEMA: &str = "
-- Tannins encyclopedia
-- Professional brewing tannin database with security hardening
CREATE TABLE IF NOT EXISTS tannins (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Core identification
    name TEXT NOT NULL,                      -- e.g., 'FT Blanc Soft', 'Oak Tannin'
    tannin_type TEXT NOT NULL,               -- grape, oak, chestnut, quebracho, gallotannin, other
    source TEXT,                             -- Source description, NULL if unknown
    manufacturer TEXT,                       -- Brand/producer, NULL if unknown

    -- Tannin characteristics (TEXT for Decimal precision)
    concentration TEXT,                      -- Percentage 0-100%, NULL if unknown
    typical_dosage_grams_per_gallon TEXT,    -- Recommended usage 0-10 g/gal

    -- Professional guidance (JSON arrays for structured data)
    usage_notes TEXT,                        -- Professional brewing notes and warnings
    flavor_impact TEXT,                      -- Description of flavor/mouthfeel contributions
    best_suited_styles TEXT,                 -- JSON array of beverage styles
    compatible_styles TEXT,                  -- JSON array of compatible styles
    timing TEXT,                             -- When to add (pre-ferment, aging, etc)
    purpose TEXT,                            -- Purpose (structure, antioxidant, clarification)

    -- Metadata
    created_at TEXT NOT NULL,                -- ISO 8601 timestamp
    updated_at TEXT NOT NULL,                -- ISO 8601 timestamp

    -- Validation constraints (security hardening)
    CHECK(tannin_type IN ('grape', 'oak', 'chestnut', 'quebracho', 'gallotannin', 'other')),
    CHECK(name != ''),                       -- Prevent empty names
    CHECK(length(name) <= 200)               -- Prevent abuse
);

-- Performance indexes for mobile-first queries
CREATE INDEX IF NOT EXISTS idx_tannins_name ON tannins(name);
CREATE INDEX IF NOT EXISTS idx_tannins_type ON tannins(tannin_type);
CREATE INDEX IF NOT EXISTS idx_tannins_type_name ON tannins(tannin_type, name);
";

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_schema_creates_successfully() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(TANNIN_SCHEMA).expect("Failed to execute schema");
        let table_exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='tannins'", [], |row| row.get(0))
            .expect("Failed to query table existence");
        assert_eq!(table_exists, 1);
    }

    #[test]
    fn test_schema_has_correct_indexes() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(TANNIN_SCHEMA).expect("Failed to execute schema");
        let index_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND tbl_name='tannins' AND name LIKE 'idx_%'", [], |row| row.get(0))
            .expect("Failed to query index count");
        assert_eq!(index_count, 3);
    }

    #[test]
    fn test_schema_enforces_type_constraint() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(TANNIN_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO tannins (name, tannin_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["Test", "invalid", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_schema_accepts_valid_types() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(TANNIN_SCHEMA).expect("Failed to execute schema");
        for tannin_type in vec!["grape", "oak", "chestnut", "quebracho", "gallotannin", "other"] {
            let result = conn.execute("INSERT INTO tannins (name, tannin_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                      rusqlite::params![format!("Test {}", tannin_type), tannin_type, "2025-01-01", "2025-01-01"]);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_schema_requires_non_null_fields() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(TANNIN_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO tannins (tannin_type, created_at, updated_at) VALUES (?, ?, ?)",
                                  rusqlite::params!["grape", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_schema_rejects_empty_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(TANNIN_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO tannins (name, tannin_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["", "grape", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_schema_rejects_oversized_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(TANNIN_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO tannins (name, tannin_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["A".repeat(201), "grape", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err());
    }
}