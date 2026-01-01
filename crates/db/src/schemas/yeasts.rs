/// Yeast Encyclopedia Schema
///
/// Comprehensive yeast strain database with professional brewing standards.
/// Optimized for mobile performance with strategic indexing.

pub const YEAST_SCHEMA: &str = "
-- Yeast strains encyclopedia
-- Master-level professional yeast database
CREATE TABLE IF NOT EXISTS yeasts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Core identification
    name TEXT NOT NULL,                      -- e.g., 'Lalvin 71B-1122'
    laboratory TEXT NOT NULL,                -- e.g., 'Lalvin', 'White Labs', 'Wyeast'
    product_code TEXT NOT NULL,              -- e.g., '71B', 'WLP715'
    yeast_type TEXT NOT NULL,                -- wine, beer, mead, champagne, distillers, wild

    -- Fermentation characteristics (TEXT for Decimal precision)
    alcohol_tolerance TEXT,                  -- Percentage 0.0-25.0
    temperature_range_min TEXT,              -- °F, minimum safe temperature
    temperature_range_max TEXT,              -- °F, maximum safe temperature
    attenuation TEXT,                        -- Percentage 0.0-100.0

    -- Physical & nutritional characteristics
    flocculation TEXT,                       -- low, medium, high, very high
    nutrient_requirements TEXT,              -- low, moderate, high

    -- Professional sensory profiles (JSON arrays)
    flavor_profile TEXT,                     -- Master Sommelier vocabulary
    aroma_profile TEXT,                      -- Professional aroma descriptors

    -- Usage recommendations
    best_suited_styles TEXT,                 -- JSON array of beverage styles
    usage_notes TEXT,                        -- Professional brewing notes

    -- Fermentation timing (TEXT for Decimal precision)
    lag_time_hours TEXT,                     -- Time before fermentation starts
    fermentation_duration_days TEXT,         -- Typical fermentation length

    -- Professional assessment
    sensory_notes TEXT,                      -- Master-level sensory evaluation
    requires_rehydration INTEGER NOT NULL DEFAULT 0,  -- Boolean: needs rehydration

    -- Compatibility
    compatible_ingredients TEXT,             -- JSON array of ingredient types

    -- Metadata
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,

    -- Validation constraints
    CHECK(yeast_type IN ('wine', 'beer', 'mead', 'champagne', 'distillers', 'wild')),
    CHECK(flocculation IS NULL OR flocculation IN ('low', 'medium', 'high', 'very high')),
    CHECK(nutrient_requirements IS NULL OR nutrient_requirements IN ('low', 'moderate', 'high')),
    CHECK(requires_rehydration IN (0, 1))
);

-- Performance indexes for mobile-first queries
-- Index on type for filtering by category (wine/beer/mead)
CREATE INDEX IF NOT EXISTS idx_yeasts_type ON yeasts(yeast_type);

-- Index on laboratory for filtering by manufacturer
CREATE INDEX IF NOT EXISTS idx_yeasts_lab ON yeasts(laboratory);

-- Index on name for search operations
CREATE INDEX IF NOT EXISTS idx_yeasts_name ON yeasts(name);

-- Composite index for type + name sorting (common query pattern)
CREATE INDEX IF NOT EXISTS idx_yeasts_type_name ON yeasts(yeast_type, name);
";

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_schema_creates_successfully() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(YEAST_SCHEMA).expect("Failed to execute schema");

        // Verify table exists
        let table_exists: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='yeasts'",
                [],
                |row| row.get(0),
            )
            .expect("Failed to query table existence");

        assert_eq!(table_exists, 1, "Yeasts table should exist");
    }

    #[test]
    fn test_schema_has_correct_indexes() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(YEAST_SCHEMA).expect("Failed to execute schema");

        // Verify all indexes exist
        let index_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type='index'
                 AND tbl_name='yeasts'
                 AND name LIKE 'idx_%'",
                [],
                |row| row.get(0),
            )
            .expect("Failed to query index count");

        assert_eq!(index_count, 4, "Should have 4 performance indexes");
    }

    #[test]
    fn test_schema_enforces_type_constraint() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(YEAST_SCHEMA).expect("Failed to execute schema");

        // Try to insert invalid type
        let result = conn.execute(
            "INSERT INTO yeasts (
                name, laboratory, product_code, yeast_type,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                "Test Yeast",
                "Test Lab",
                "T1",
                "invalid_type",
                "2025-01-01",
                "2025-01-01",
            ],
        );

        assert!(result.is_err(), "Should reject invalid yeast type");
    }

    #[test]
    fn test_schema_enforces_flocculation_constraint() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(YEAST_SCHEMA).expect("Failed to execute schema");

        // Try to insert invalid flocculation
        let result = conn.execute(
            "INSERT INTO yeasts (
                name, laboratory, product_code, yeast_type,
                flocculation, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                "Test Yeast",
                "Test Lab",
                "T1",
                "wine",
                "super_high",
                "2025-01-01",
                "2025-01-01",
            ],
        );

        assert!(result.is_err(), "Should reject invalid flocculation");
    }

    #[test]
    fn test_schema_enforces_nutrient_constraint() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute_batch(YEAST_SCHEMA).expect("Failed to execute schema");

        // Try to insert invalid nutrient requirement
        let result = conn.execute(
            "INSERT INTO yeasts (
                name, laboratory, product_code, yeast_type,
                nutrient_requirements, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                "Test Yeast",
                "Test Lab",
                "T1",
                "wine",
                "extreme",
                "2025-01-01",
                "2025-01-01",
            ],
        );

        assert!(result.is_err(), "Should reject invalid nutrient requirement");
    }
}