/// Nutrient Encyclopedia Schema
///
/// Comprehensive yeast nutrient database for fermentation management.
/// Optimized for mobile performance with strategic indexing.
/// HARDENED: Multiple CHECK constraints, composite indexes, comprehensive tests.

pub const NUTRIENT_SCHEMA: &str = "
-- Nutrients encyclopedia
-- Professional brewing yeast nutrient database with security hardening
CREATE TABLE IF NOT EXISTS nutrients (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Core identification
    name TEXT NOT NULL,                      -- e.g., 'Fermaid K', 'DAP', 'Go-Ferm'
    nutrient_type TEXT NOT NULL,             -- DAP, fermaid, yeast_hulls, urea, complete, other
    manufacturer TEXT,                       -- Brand/producer, NULL if unknown

    -- Nutrient composition (TEXT for Decimal precision) - percentage content
    nitrogen_content TEXT,                   -- N% 0-100, NULL if unknown
    phosphorus_content TEXT,                 -- P% 0-100, NULL if unknown
    potassium_content TEXT,                  -- K% 0-100, NULL if unknown
    typical_dosage_grams_per_gallon TEXT,    -- Recommended usage 0-10 g/gal

    -- Professional guidance (JSON arrays for structured data)
    usage_notes TEXT,                        -- Professional brewing notes and warnings
    best_suited_styles TEXT,                 -- JSON array of beverage styles
    compatible_styles TEXT,                  -- JSON array of compatible styles
    timing TEXT,                             -- When to add (pitch, 24h, 48h, etc)

    -- Metadata
    created_at TEXT NOT NULL,                -- ISO 8601 timestamp
    updated_at TEXT NOT NULL,                -- ISO 8601 timestamp

    -- Validation constraints (security hardening)
    CHECK(nutrient_type IN ('DAP', 'fermaid', 'yeast_hulls', 'urea', 'complete', 'other')),
    CHECK(name != ''),                       -- Prevent empty names
    CHECK(length(name) <= 200)               -- Prevent abuse
);

-- Performance indexes for mobile-first queries
CREATE INDEX IF NOT EXISTS idx_nutrients_name ON nutrients(name);
CREATE INDEX IF NOT EXISTS idx_nutrients_type ON nutrients(nutrient_type);
CREATE INDEX IF NOT EXISTS idx_nutrients_type_name ON nutrients(nutrient_type, name);
";

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_schema_creates_successfully() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(NUTRIENT_SCHEMA).expect("Failed to execute schema");
        let table_exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='nutrients'", [], |row| row.get(0))
            .expect("Failed to query table existence");
        assert_eq!(table_exists, 1);
    }

    #[test]
    fn test_schema_has_correct_indexes() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(NUTRIENT_SCHEMA).expect("Failed to execute schema");
        let index_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND tbl_name='nutrients' AND name LIKE 'idx_%'", [], |row| row.get(0))
            .expect("Failed to query index count");
        assert_eq!(index_count, 3);
    }

    #[test]
    fn test_schema_enforces_type_constraint() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(NUTRIENT_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO nutrients (name, nutrient_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["Test", "invalid", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_schema_accepts_valid_types() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(NUTRIENT_SCHEMA).expect("Failed to execute schema");
        for nutrient_type in vec!["DAP", "fermaid", "yeast_hulls", "urea", "complete", "other"] {
            let result = conn.execute("INSERT INTO nutrients (name, nutrient_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                      rusqlite::params![format!("Test {}", nutrient_type), nutrient_type, "2025-01-01", "2025-01-01"]);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_schema_requires_non_null_fields() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(NUTRIENT_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO nutrients (nutrient_type, created_at, updated_at) VALUES (?, ?, ?)",
                                  rusqlite::params!["DAP", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_schema_rejects_empty_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(NUTRIENT_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO nutrients (name, nutrient_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["", "DAP", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_schema_rejects_oversized_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(NUTRIENT_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO nutrients (name, nutrient_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["A".repeat(201), "DAP", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err());
    }
}