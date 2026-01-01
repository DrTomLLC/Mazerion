pub const MALT_SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS malts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    name TEXT NOT NULL,
    maltster TEXT NOT NULL,
    origin TEXT NOT NULL,
    grain_type TEXT NOT NULL,

    color_lovibond TEXT,
    max_percentage TEXT,
    extract_potential TEXT,
    diastatic_power TEXT,
    moisture_content TEXT,
    protein_content TEXT,

    flavor_profile TEXT,
    aroma_profile TEXT,
    typical_usage TEXT,
    substitutes TEXT,
    best_suited_styles TEXT,
    usage_notes TEXT,
    sensory_notes TEXT,
    requires_mashing INTEGER NOT NULL DEFAULT 1,
    compatible_styles TEXT,

    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,

    CHECK(grain_type IN ('base', 'specialty', 'adjunct')),
    CHECK(requires_mashing IN (0, 1))
);

CREATE INDEX IF NOT EXISTS idx_malts_name ON malts(name);
CREATE INDEX IF NOT EXISTS idx_malts_type ON malts(grain_type);
CREATE INDEX IF NOT EXISTS idx_malts_maltster ON malts(maltster);
CREATE INDEX IF NOT EXISTS idx_malts_type_maltster ON malts(grain_type, maltster);
";

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_schema_creates_successfully() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(MALT_SCHEMA).unwrap();

        let table_exists: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='malts'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(table_exists, 1);
    }

    #[test]
    fn test_schema_has_correct_indexes() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(MALT_SCHEMA).unwrap();

        let index_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type='index'
                 AND tbl_name='malts'
                 AND name LIKE 'idx_%'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(index_count, 4);
    }
}