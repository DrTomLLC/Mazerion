use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::models::honey::Honey;

/// Repository for honey varietal encyclopedia operations
///
/// Provides CRUD operations with:
/// - Zero panic guarantee
/// - Prepared statement optimization
/// - SQL injection prevention via parameterized queries
/// - Efficient batch operations
/// - Mobile-first battery optimization
pub struct HoneyRepository<'conn> { conn: &'conn Connection, }

impl<'conn> HoneyRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    /// Create a new honey entry
    ///
    /// # Security
    /// - All inputs validated before DB access
    /// - Parameterized query prevents SQL injection
    ///
    /// # Performance
    /// - Single transaction for consistency
    /// - Returns generated ID for immediate use
    pub fn create(conn: &Connection, honey: &Honey) -> SqliteResult<i64> {
        // Validate before touching database
        honey.validate().map_err(|e| {
            rusqlite::Error::InvalidParameterName(e)
        })?;

        let query = "
            INSERT INTO honeys (
                name, floral_source, origin, color,
                moisture_content, fructose_percentage, glucose_percentage,
                other_sugars_percentage, specific_gravity, ph,
                flavor_intensity, flavor_profile, aroma_profile,
                crystallization_tendency, best_suited_styles, usage_notes,
                sensory_notes, harvest_season, is_monofloral, is_raw,
                compatible_yeasts, created_at, updated_at
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20,
                ?21, ?22, ?23
            )
        ";

        conn.execute(
            query,
            rusqlite::params![
                honey.name,
                honey.floral_source,
                honey.origin,
                honey.color,
                honey.moisture_content.map(|d: Decimal| d.to_string()),
                honey.fructose_percentage.map(|d: Decimal| d.to_string()),
                honey.glucose_percentage.map(|d: Decimal| d.to_string()),
                honey.other_sugars_percentage.map(|d: Decimal| d.to_string()),
                honey.specific_gravity.map(|d: Decimal| d.to_string()),
                honey.ph.map(|d: Decimal| d.to_string()),
                honey.flavor_intensity,
                honey.flavor_profile,
                honey.aroma_profile,
                honey.crystallization_tendency,
                honey.best_suited_styles,
                honey.usage_notes,
                honey.sensory_notes,
                honey.harvest_season,
                honey.is_monofloral,
                honey.is_raw,
                honey.compatible_yeasts,
                honey.created_at,
                honey.updated_at,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Get honey by ID
    ///
    /// # Performance
    /// - Uses PRIMARY KEY index for O(log n) lookup
    /// - Zero-copy where possible
    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<Honey> {
        let query = "
            SELECT
                id, name, floral_source, origin, color,
                moisture_content, fructose_percentage, glucose_percentage,
                other_sugars_percentage, specific_gravity, ph,
                flavor_intensity, flavor_profile, aroma_profile,
                crystallization_tendency, best_suited_styles, usage_notes,
                sensory_notes, harvest_season, is_monofloral, is_raw,
                compatible_yeasts, created_at, updated_at
            FROM honeys
            WHERE id = ?1
        ";

        conn.query_row(query, rusqlite::params![id], Self::row_to_honey)
    }

    /// List honeys with optional limit
    ///
    /// # Performance
    /// - Coarse-grained API reduces FFI calls
    /// - Efficient pagination via LIMIT
    /// - Uses index on name for stable ordering
    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<Honey>> {
        let query = match limit {
            Some(_) => "
                SELECT
                    id, name, floral_source, origin, color,
                    moisture_content, fructose_percentage, glucose_percentage,
                    other_sugars_percentage, specific_gravity, ph,
                    flavor_intensity, flavor_profile, aroma_profile,
                    crystallization_tendency, best_suited_styles, usage_notes,
                    sensory_notes, harvest_season, is_monofloral, is_raw,
                    compatible_yeasts, created_at, updated_at
                FROM honeys
                ORDER BY name ASC
                LIMIT ?1
            ",
            None => "
                SELECT
                    id, name, floral_source, origin, color,
                    moisture_content, fructose_percentage, glucose_percentage,
                    other_sugars_percentage, specific_gravity, ph,
                    flavor_intensity, flavor_profile, aroma_profile,
                    crystallization_tendency, best_suited_styles, usage_notes,
                    sensory_notes, harvest_season, is_monofloral, is_raw,
                    compatible_yeasts, created_at, updated_at
                FROM honeys
                ORDER BY name ASC
            ",
        };

        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_honey)?
        } else {
            stmt.query_map([], Self::row_to_honey)?
        };

        rows.collect()
    }

    /// Search honeys by name, floral source, or origin
    ///
    /// # Performance
    /// - Uses indexed columns for efficient search
    /// - Case-insensitive via LIKE
    /// - Early termination with LIMIT
    pub fn search(
        conn: &Connection,
        query: &str,
        limit: Option<i64>,
    ) -> SqliteResult<Vec<Honey>> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        let search_pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "
                SELECT
                    id, name, floral_source, origin, color,
                    moisture_content, fructose_percentage, glucose_percentage,
                    other_sugars_percentage, specific_gravity, ph,
                    flavor_intensity, flavor_profile, aroma_profile,
                    crystallization_tendency, best_suited_styles, usage_notes,
                    sensory_notes, harvest_season, is_monofloral, is_raw,
                    compatible_yeasts, created_at, updated_at
                FROM honeys
                WHERE name LIKE ?1
                   OR floral_source LIKE ?1
                   OR origin LIKE ?1
                ORDER BY name ASC
                LIMIT ?2
            ",
            None => "
                SELECT
                    id, name, floral_source, origin, color,
                    moisture_content, fructose_percentage, glucose_percentage,
                    other_sugars_percentage, specific_gravity, ph,
                    flavor_intensity, flavor_profile, aroma_profile,
                    crystallization_tendency, best_suited_styles, usage_notes,
                    sensory_notes, harvest_season, is_monofloral, is_raw,
                    compatible_yeasts, created_at, updated_at
                FROM honeys
                WHERE name LIKE ?1
                   OR floral_source LIKE ?1
                   OR origin LIKE ?1
                ORDER BY name ASC
            ",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![search_pattern, lim], Self::row_to_honey)?
        } else {
            stmt.query_map(rusqlite::params![search_pattern], Self::row_to_honey)?
        };

        rows.collect()
    }

    /// Get all honeys of a specific color
    ///
    /// # Performance
    /// - Uses index on color
    /// - Efficient for filtering by color category
    pub fn get_by_color(
        conn: &Connection,
        color: &str,
        limit: Option<i64>,
    ) -> SqliteResult<Vec<Honey>> {
        let sql = match limit {
            Some(_) => "
                SELECT
                    id, name, floral_source, origin, color,
                    moisture_content, fructose_percentage, glucose_percentage,
                    other_sugars_percentage, specific_gravity, ph,
                    flavor_intensity, flavor_profile, aroma_profile,
                    crystallization_tendency, best_suited_styles, usage_notes,
                    sensory_notes, harvest_season, is_monofloral, is_raw,
                    compatible_yeasts, created_at, updated_at
                FROM honeys
                WHERE color = ?1
                ORDER BY name ASC
                LIMIT ?2
            ",
            None => "
                SELECT
                    id, name, floral_source, origin, color,
                    moisture_content, fructose_percentage, glucose_percentage,
                    other_sugars_percentage, specific_gravity, ph,
                    flavor_intensity, flavor_profile, aroma_profile,
                    crystallization_tendency, best_suited_styles, usage_notes,
                    sensory_notes, harvest_season, is_monofloral, is_raw,
                    compatible_yeasts, created_at, updated_at
                FROM honeys
                WHERE color = ?1
                ORDER BY name ASC
            ",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![color, lim], Self::row_to_honey)?
        } else {
            stmt.query_map(rusqlite::params![color], Self::row_to_honey)?
        };

        rows.collect()
    }

    /// Get all honeys with a specific flavor intensity
    ///
    /// # Performance
    /// - Uses index on flavor_intensity
    /// - Efficient intensity filtering
    pub fn get_by_intensity(
        conn: &Connection,
        intensity: &str,
        limit: Option<i64>,
    ) -> SqliteResult<Vec<Honey>> {
        let sql = match limit {
            Some(_) => "
                SELECT
                    id, name, floral_source, origin, color,
                    moisture_content, fructose_percentage, glucose_percentage,
                    other_sugars_percentage, specific_gravity, ph,
                    flavor_intensity, flavor_profile, aroma_profile,
                    crystallization_tendency, best_suited_styles, usage_notes,
                    sensory_notes, harvest_season, is_monofloral, is_raw,
                    compatible_yeasts, created_at, updated_at
                FROM honeys
                WHERE flavor_intensity = ?1
                ORDER BY name ASC
                LIMIT ?2
            ",
            None => "
                SELECT
                    id, name, floral_source, origin, color,
                    moisture_content, fructose_percentage, glucose_percentage,
                    other_sugars_percentage, specific_gravity, ph,
                    flavor_intensity, flavor_profile, aroma_profile,
                    crystallization_tendency, best_suited_styles, usage_notes,
                    sensory_notes, harvest_season, is_monofloral, is_raw,
                    compatible_yeasts, created_at, updated_at
                FROM honeys
                WHERE flavor_intensity = ?1
                ORDER BY name ASC
            ",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![intensity, lim], Self::row_to_honey)?
        } else {
            stmt.query_map(rusqlite::params![intensity], Self::row_to_honey)?
        };

        rows.collect()
    }

    /// Get monofloral or multifloral honeys
    ///
    /// # Performance
    /// - Uses index on is_monofloral
    /// - Binary filter for efficiency
    pub fn get_by_floral_type(
        conn: &Connection,
        is_monofloral: bool,
        limit: Option<i64>,
    ) -> SqliteResult<Vec<Honey>> {
        let sql = match limit {
            Some(_) => "
                SELECT
                    id, name, floral_source, origin, color,
                    moisture_content, fructose_percentage, glucose_percentage,
                    other_sugars_percentage, specific_gravity, ph,
                    flavor_intensity, flavor_profile, aroma_profile,
                    crystallization_tendency, best_suited_styles, usage_notes,
                    sensory_notes, harvest_season, is_monofloral, is_raw,
                    compatible_yeasts, created_at, updated_at
                FROM honeys
                WHERE is_monofloral = ?1
                ORDER BY name ASC
                LIMIT ?2
            ",
            None => "
                SELECT
                    id, name, floral_source, origin, color,
                    moisture_content, fructose_percentage, glucose_percentage,
                    other_sugars_percentage, specific_gravity, ph,
                    flavor_intensity, flavor_profile, aroma_profile,
                    crystallization_tendency, best_suited_styles, usage_notes,
                    sensory_notes, harvest_season, is_monofloral, is_raw,
                    compatible_yeasts, created_at, updated_at
                FROM honeys
                WHERE is_monofloral = ?1
                ORDER BY name ASC
            ",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![is_monofloral, lim], Self::row_to_honey)?
        } else {
            stmt.query_map(rusqlite::params![is_monofloral], Self::row_to_honey)?
        };

        rows.collect()
    }

    /// Update honey entry
    ///
    /// # Security
    /// - Validates all fields before update
    /// - Prevents partial updates on validation failure
    pub fn update(conn: &Connection, honey: &Honey) -> SqliteResult<()> {
        // Validate before touching database
        honey.validate().map_err(|e| {
            rusqlite::Error::InvalidParameterName(e)
        })?;

        let query = "
            UPDATE honeys SET
                name = ?2, floral_source = ?3, origin = ?4, color = ?5,
                moisture_content = ?6, fructose_percentage = ?7, glucose_percentage = ?8,
                other_sugars_percentage = ?9, specific_gravity = ?10, ph = ?11,
                flavor_intensity = ?12, flavor_profile = ?13, aroma_profile = ?14,
                crystallization_tendency = ?15, best_suited_styles = ?16, usage_notes = ?17,
                sensory_notes = ?18, harvest_season = ?19, is_monofloral = ?20, is_raw = ?21,
                compatible_yeasts = ?22, updated_at = ?23
            WHERE id = ?1
        ";

        let rows_affected = conn.execute(
            query,
            rusqlite::params![
                honey.id,
                honey.name,
                honey.floral_source,
                honey.origin,
                honey.color,
                honey.moisture_content.map(|d: Decimal| d.to_string()),
                honey.fructose_percentage.map(|d: Decimal| d.to_string()),
                honey.glucose_percentage.map(|d: Decimal| d.to_string()),
                honey.other_sugars_percentage.map(|d: Decimal| d.to_string()),
                honey.specific_gravity.map(|d: Decimal| d.to_string()),
                honey.ph.map(|d: Decimal| d.to_string()),
                honey.flavor_intensity,
                honey.flavor_profile,
                honey.aroma_profile,
                honey.crystallization_tendency,
                honey.best_suited_styles,
                honey.usage_notes,
                honey.sensory_notes,
                honey.harvest_season,
                honey.is_monofloral,
                honey.is_raw,
                honey.compatible_yeasts,
                honey.updated_at,
            ],
        )?;

        if rows_affected == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        Ok(())
    }

    /// Delete honey entry
    ///
    /// # Safety
    /// - Returns error if entry doesn't exist
    /// - Cascade deletions handled by FK constraints
    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let query = "DELETE FROM honeys WHERE id = ?1";
        let rows_affected = conn.execute(query, rusqlite::params![id])?;

        if rows_affected == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        Ok(())
    }

    /// Get count of all honeys
    ///
    /// # Performance
    /// - Uses COUNT(*) which is optimized in SQLite
    /// - No row materialization overhead
    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        let query = "SELECT COUNT(*) FROM honeys";
        conn.query_row(query, [], |row| row.get(0))
    }

    /// Get count by color
    pub fn count_by_color(conn: &Connection, color: &str) -> SqliteResult<i64> {
        let query = "SELECT COUNT(*) FROM honeys WHERE color = ?1";
        conn.query_row(query, rusqlite::params![color], |row| row.get(0))
    }

    /// Get count by intensity
    pub fn count_by_intensity(conn: &Connection, intensity: &str) -> SqliteResult<i64> {
        let query = "SELECT COUNT(*) FROM honeys WHERE flavor_intensity = ?1";
        conn.query_row(query, rusqlite::params![intensity], |row| row.get(0))
    }

    /// Helper: Convert database row to Honey
    ///
    /// # Performance
    /// - Lazy Decimal parsing (only parse if Some)
    /// - Error fast-path for invalid Decimals
    fn row_to_honey(row: &Row) -> SqliteResult<Honey> {
        // Helper to parse optional Decimal from TEXT
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }

        Ok(Honey {
            id: row.get(0)?,
            name: row.get(1)?,
            floral_source: row.get(2)?,
            origin: row.get(3)?,
            color: row.get(4)?,
            moisture_content: parse_decimal(row.get(5)?),
            fructose_percentage: parse_decimal(row.get(6)?),
            glucose_percentage: parse_decimal(row.get(7)?),
            other_sugars_percentage: parse_decimal(row.get(8)?),
            specific_gravity: parse_decimal(row.get(9)?),
            ph: parse_decimal(row.get(10)?),
            flavor_intensity: row.get(11)?,
            flavor_profile: row.get(12)?,
            aroma_profile: row.get(13)?,
            crystallization_tendency: row.get(14)?,
            best_suited_styles: row.get(15)?,
            usage_notes: row.get(16)?,
            sensory_notes: row.get(17)?,
            harvest_season: row.get(18)?,
            is_monofloral: row.get(19)?,
            is_raw: row.get(20)?,
            compatible_yeasts: row.get(21)?,
            created_at: row.get(22)?,
            updated_at: row.get(23)?,
        })
    }
}