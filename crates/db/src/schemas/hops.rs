/// Hop Variety Encyclopedia Schema
///
/// Comprehensive hop database with professional brewing standards.
/// Optimized for mobile performance with strategic indexing.

pub const HOP_SCHEMA: &str = "
-- Hop varieties encyclopedia
-- Professional-level hop database
CREATE TABLE IF NOT EXISTS hops (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Core identification
    name TEXT NOT NULL,                      -- e.g., 'Cascade', 'Hallertau'
    origin TEXT NOT NULL,                    -- Country of origin
    hop_type TEXT NOT NULL,                  -- bittering, aroma, dual-purpose

    -- Acid composition (TEXT for Decimal precision)
    alpha_acid TEXT,                         -- Percentage, typically 3-20%
    beta_acid TEXT,                          -- Percentage, typically 2-10%
    cohumulone TEXT,                         -- Percentage of alpha acids

    -- Oil composition (TEXT for Decimal precision)
    total_oil TEXT,                          -- mL/100g, typically 0.5-3.0
    myrcene TEXT,                            -- Percentage of total oils
    humulene TEXT,                           -- Percentage of total oils
    caryophyllene TEXT,                      -- Percentage of total oils
    farnesene TEXT,                          -- Percentage of total oils

    -- Professional sensory profiles (JSON arrays)
    flavor_profile TEXT,                     -- Master Cicerone vocabulary
    aroma_profile TEXT,                      -- Professional aroma descriptors

    -- Usage information
    substitutes TEXT,                        -- JSON array of substitute hops
    best_suited_styles TEXT,                 -- JSON array of beer styles
    usage_notes TEXT,                        -- Professional brewing notes
    sensory_notes TEXT,                      -- Master-level sensory evaluation
    typical_usage TEXT,                      -- Common usage method
    storage_stability TEXT,                  -- Storage characteristics
    compatible_styles TEXT,                  -- JSON array of compatible styles

    -- Metadata
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,

    -- Validation constraints
    CHECK(hop_type IN ('bittering', 'aroma', 'dual-purpose')),
    CHECK(typical_usage IS NULL OR typical_usage IN (
        'bittering', 'aroma', 'dry hop', 'whirlpool', 'first wort'
    )),
    CHECK(storage_stability IS NULL OR storage_stability IN (
        'excellent', 'good', 'fair', 'poor'
    ))
);

-- Performance indexes for mobile-first queries
CREATE INDEX IF NOT EXISTS idx_hops_name ON hops(name);
CREATE INDEX IF NOT EXISTS idx_hops_type ON hops(hop_type);
CREATE INDEX IF NOT EXISTS idx_hops_origin ON hops(origin);
CREATE INDEX IF NOT EXISTS idx_hops_type_origin ON hops(hop_type, origin);
";

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_schema_creates_successfully() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(HOP_SCHEMA).unwrap();

        let table_exists: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='hops'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(table_exists, 1);
    }

    #[test]
    fn test_schema_has_correct_indexes() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(HOP_SCHEMA).unwrap();

        let index_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type='index'
                 AND tbl_name='hops'
                 AND name LIKE 'idx_%'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(index_count, 4);
    }

    #[test]
    fn test_schema_enforces_type_constraint() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(HOP_SCHEMA).unwrap();

        let result = conn.execute(
            "INSERT INTO hops (name, origin, hop_type, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?)",
            rusqlite::params!["Test", "USA", "invalid", "2025-01-01", "2025-01-01"],
        );

        assert!(result.is_err());
    }
}