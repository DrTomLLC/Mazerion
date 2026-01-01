/// Honey Varietal Encyclopedia Schema
///
/// Comprehensive honey database with professional beekeeping and brewing standards.
/// Optimized for mobile performance with strategic indexing.

pub const HONEY_SCHEMA: &str = "
-- Honey varieties encyclopedia
-- Professional-level honey varietal database
CREATE TABLE IF NOT EXISTS honeys (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Core identification
    name TEXT NOT NULL,                      -- e.g., 'Orange Blossom', 'Wildflower'
    floral_source TEXT NOT NULL,             -- e.g., 'Citrus sinensis', 'Mixed wildflowers'
    origin TEXT,                             -- Geographic origin/region
    color TEXT NOT NULL,                     -- Honey color classification

    -- Composition (TEXT for Decimal precision)
    moisture_content TEXT,                   -- Percentage, typically 15-20%
    fructose_percentage TEXT,                -- Percentage, typically 35-45%
    glucose_percentage TEXT,                 -- Percentage, typically 25-35%
    other_sugars_percentage TEXT,            -- Sucrose, maltose, etc.
    specific_gravity TEXT,                   -- Typically 1.410-1.450
    ph TEXT,                                 -- Typically 3.5-4.5

    -- Sensory characteristics
    flavor_intensity TEXT NOT NULL,          -- delicate, mild, moderate, strong, robust
    flavor_profile TEXT,                     -- JSON array, professional sommelier vocabulary
    aroma_profile TEXT,                      -- JSON array, professional aroma descriptors

    -- Physical properties
    crystallization_tendency TEXT,           -- rapid, moderate, slow, very slow

    -- Usage information
    best_suited_styles TEXT,                 -- JSON array of beverage styles
    usage_notes TEXT,                        -- Professional brewing notes
    sensory_notes TEXT,                      -- Master-level sensory evaluation
    harvest_season TEXT,                     -- e.g., 'Spring', 'Summer', 'Fall'

    -- Classification
    is_monofloral INTEGER NOT NULL DEFAULT 1, -- Boolean: single floral source
    is_raw INTEGER,                          -- Boolean: unpasteurized/raw

    -- Compatibility
    compatible_yeasts TEXT,                  -- JSON array of yeast types

    -- Metadata
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,

    -- Validation constraints
    CHECK(color IN (
        'extra white', 'white', 'extra light amber', 'light amber',
        'amber', 'dark amber', 'dark'
    )),
    CHECK(flavor_intensity IN ('delicate', 'mild', 'moderate', 'strong', 'robust')),
    CHECK(crystallization_tendency IS NULL OR crystallization_tendency IN (
        'rapid', 'moderate', 'slow', 'very slow'
    )),
    CHECK(is_monofloral IN (0, 1)),
    CHECK(is_raw IS NULL OR is_raw IN (0, 1))
);

-- Performance indexes for mobile-first queries
-- Index on name for search and sorting
CREATE INDEX IF NOT EXISTS idx_honeys_name ON honeys(name);

-- Index on color for filtering by color category
CREATE INDEX IF NOT EXISTS idx_honeys_color ON honeys(color);

-- Index on flavor intensity for filtering by intensity
CREATE INDEX IF NOT EXISTS idx_honeys_intensity ON honeys(flavor_intensity);

-- Index on monofloral classification
CREATE INDEX IF NOT EXISTS idx_honeys_monofloral ON honeys(is_monofloral);

-- Composite index for color + intensity (common query pattern)
CREATE INDEX IF NOT EXISTS idx_honeys_color_intensity ON honeys(color, flavor_intensity);
";

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_schema_creates_successfully() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(HONEY_SCHEMA).unwrap();

        // Verify table exists
        let table_exists: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='honeys'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(table_exists, 1, "Honeys table should exist");
    }

    #[test]
    fn test_schema_has_correct_indexes() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(HONEY_SCHEMA).unwrap();

        // Verify all indexes exist
        let index_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type='index'
                 AND tbl_name='honeys'
                 AND name LIKE 'idx_%'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(index_count, 5, "Should have 5 performance indexes");
    }

    #[test]
    fn test_schema_enforces_color_constraint() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(HONEY_SCHEMA).unwrap();

        // Try to insert invalid color
        let result = conn.execute(
            "INSERT INTO honeys (
                name, floral_source, color, flavor_intensity,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                "Test Honey",
                "Test Flower",
                "purple",  // Invalid color
                "mild",
                "2025-01-01",
                "2025-01-01",
            ],
        );

        assert!(result.is_err(), "Should reject invalid color");
    }

    #[test]
    fn test_schema_enforces_intensity_constraint() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(HONEY_SCHEMA).unwrap();

        // Try to insert invalid intensity
        let result = conn.execute(
            "INSERT INTO honeys (
                name, floral_source, color, flavor_intensity,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                "Test Honey",
                "Test Flower",
                "white",
                "extreme",  // Invalid intensity
                "2025-01-01",
                "2025-01-01",
            ],
        );

        assert!(result.is_err(), "Should reject invalid flavor intensity");
    }

    #[test]
    fn test_schema_enforces_crystallization_constraint() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(HONEY_SCHEMA).unwrap();

        // Try to insert invalid crystallization
        let result = conn.execute(
            "INSERT INTO honeys (
                name, floral_source, color, flavor_intensity,
                crystallization_tendency, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                "Test Honey",
                "Test Flower",
                "white",
                "mild",
                "instant",  // Invalid crystallization
                "2025-01-01",
                "2025-01-01",
            ],
        );

        assert!(result.is_err(), "Should reject invalid crystallization tendency");
    }

    #[test]
    fn test_schema_accepts_valid_data() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(HONEY_SCHEMA).unwrap();

        // Insert valid honey
        let result = conn.execute(
            "INSERT INTO honeys (
                name, floral_source, color, flavor_intensity,
                is_monofloral, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                "Orange Blossom",
                "Citrus sinensis",
                "white",
                "mild",
                1,
                "2025-01-01",
                "2025-01-01",
            ],
        );

        assert!(result.is_ok(), "Should accept valid data");
        assert_eq!(result.unwrap(), 1, "Should insert one row");
    }
}