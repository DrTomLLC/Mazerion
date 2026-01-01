/// Water Profile Encyclopedia Schema
///
/// Comprehensive water chemistry database for brewing water management.
/// Optimized for mobile performance with strategic indexing.
/// HARDENED: Multiple CHECK constraints, composite indexes, comprehensive tests.

pub const WATER_PROFILE_SCHEMA: &str = "
-- Water profiles encyclopedia
-- Professional brewing water chemistry database with security hardening
CREATE TABLE IF NOT EXISTS water_profiles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Core identification
    name TEXT NOT NULL,                      -- e.g., 'Burton-on-Trent', 'Pilsen'
    water_type TEXT NOT NULL,                -- tap, distilled, spring, RO, mineral, other
    source TEXT,                             -- Water source description, NULL if unknown
    location TEXT,                           -- Geographic location, NULL if unknown

    -- Mineral content (TEXT for Decimal precision) - parts per million (ppm)
    calcium TEXT,                            -- Ca²⁺ 0-1000 ppm, NULL if unknown
    magnesium TEXT,                          -- Mg²⁺ 0-1000 ppm, NULL if unknown
    sodium TEXT,                             -- Na⁺ 0-1000 ppm, NULL if unknown
    chloride TEXT,                           -- Cl⁻ 0-1000 ppm, NULL if unknown
    sulfate TEXT,                            -- SO₄²⁻ 0-1000 ppm, NULL if unknown
    bicarbonate TEXT,                        -- HCO₃⁻ 0-1000 ppm, NULL if unknown

    -- Water characteristics (TEXT for Decimal precision)
    ph_level TEXT,                           -- pH 5.0-9.0, NULL if unknown
    total_dissolved_solids TEXT,             -- TDS 0-2000 ppm, NULL if unknown
    hardness TEXT,                           -- Total hardness 0-500 ppm as CaCO₃

    -- Professional usage guidance (JSON arrays for structured data)
    best_suited_styles TEXT,                 -- JSON array of beverage styles
    usage_notes TEXT,                        -- Professional brewing notes and warnings
    flavor_impact TEXT,                      -- Description of flavor contributions
    compatible_styles TEXT,                  -- JSON array of compatible styles

    -- Metadata
    created_at TEXT NOT NULL,                -- ISO 8601 timestamp
    updated_at TEXT NOT NULL,                -- ISO 8601 timestamp

    -- Validation constraints (security hardening)
    CHECK(water_type IN ('tap', 'distilled', 'spring', 'RO', 'mineral', 'other')),
    CHECK(name != ''),                       -- Prevent empty names
    CHECK(length(name) <= 200)               -- Prevent abuse
);

-- Performance indexes for mobile-first queries
-- Index on name for search operations (most common query)
CREATE INDEX IF NOT EXISTS idx_water_profiles_name ON water_profiles(name);

-- Index on type for filtering by category
CREATE INDEX IF NOT EXISTS idx_water_profiles_type ON water_profiles(water_type);

-- Composite index for type + name sorting (common query pattern)
CREATE INDEX IF NOT EXISTS idx_water_profiles_type_name ON water_profiles(water_type, name);
";

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_schema_creates_successfully() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(WATER_PROFILE_SCHEMA).expect("Failed to execute schema");
        let table_exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='water_profiles'", [], |row| row.get(0))
            .expect("Failed to query table existence");
        assert_eq!(table_exists, 1, "Water profiles table should exist");
    }

    #[test]
    fn test_schema_has_correct_indexes() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(WATER_PROFILE_SCHEMA).expect("Failed to execute schema");
        let index_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND tbl_name='water_profiles' AND name LIKE 'idx_%'", [], |row| row.get(0))
            .expect("Failed to query index count");
        assert_eq!(index_count, 3, "Should have 3 performance indexes including composite");
    }

    #[test]
    fn test_schema_enforces_type_constraint() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(WATER_PROFILE_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO water_profiles (name, water_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["Test", "invalid", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err(), "Should reject invalid water type");
    }

    #[test]
    fn test_schema_accepts_valid_types() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(WATER_PROFILE_SCHEMA).expect("Failed to execute schema");
        for water_type in vec!["tap", "distilled", "spring", "RO", "mineral", "other"] {
            let result = conn.execute("INSERT INTO water_profiles (name, water_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                      rusqlite::params![format!("Test {}", water_type), water_type, "2025-01-01", "2025-01-01"]);
            assert!(result.is_ok(), "Should accept valid type: {}", water_type);
        }
    }

    #[test]
    fn test_schema_requires_non_null_fields() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(WATER_PROFILE_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO water_profiles (water_type, created_at, updated_at) VALUES (?, ?, ?)",
                                  rusqlite::params!["tap", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err(), "Should require name field");
    }

    #[test]
    fn test_schema_rejects_empty_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(WATER_PROFILE_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO water_profiles (name, water_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["", "tap", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err(), "Should reject empty name");
    }

    #[test]
    fn test_schema_rejects_oversized_name() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(WATER_PROFILE_SCHEMA).expect("Failed to execute schema");
        let result = conn.execute("INSERT INTO water_profiles (name, water_type, created_at, updated_at) VALUES (?, ?, ?, ?)",
                                  rusqlite::params!["A".repeat(201), "tap", "2025-01-01", "2025-01-01"]);
        assert!(result.is_err(), "Should reject names longer than 200 characters");
    }
}