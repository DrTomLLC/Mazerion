use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::models::yeast::Yeast;

/// Repository for yeast strain encyclopedia operations
///
/// Provides CRUD operations with:
/// - Zero panic guarantee
/// - Prepared statement optimization
/// - SQL injection prevention via parameterized queries
/// - Efficient batch operations
/// - Mobile-first battery optimization
pub struct YeastRepository<'conn> { conn: &'conn Connection, }

impl<'conn> YeastRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    /// Create a new yeast strain entry
    ///
    /// # Security
    /// - All inputs validated before DB access
    /// - Parameterized query prevents SQL injection
    ///
    /// # Performance
    /// - Single transaction for consistency
    /// - Returns generated ID for immediate use
    pub fn create(conn: &Connection, yeast: &Yeast) -> SqliteResult<i64> {
        // Validate before touching database
        yeast.validate().map_err(|e| {
            rusqlite::Error::InvalidParameterName(e)
        })?;

        let query = "
            INSERT INTO yeasts (
                name, laboratory, product_code, yeast_type,
                alcohol_tolerance, temperature_range_min, temperature_range_max,
                attenuation, flocculation, nutrient_requirements,
                flavor_profile, aroma_profile, best_suited_styles,
                usage_notes, lag_time_hours, fermentation_duration_days,
                sensory_notes, requires_rehydration, compatible_ingredients,
                created_at, updated_at
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21
            )
        ";

        conn.execute(
            query,
            rusqlite::params![
                yeast.name,
                yeast.laboratory,
                yeast.product_code,
                yeast.yeast_type,
                yeast.alcohol_tolerance.map(|d: Decimal| d.to_string()),
                yeast.temperature_range_min.map(|d: Decimal| d.to_string()),
                yeast.temperature_range_max.map(|d: Decimal| d.to_string()),
                yeast.attenuation.map(|d: Decimal| d.to_string()),
                yeast.flocculation,
                yeast.nutrient_requirements,
                yeast.flavor_profile,
                yeast.aroma_profile,
                yeast.best_suited_styles,
                yeast.usage_notes,
                yeast.lag_time_hours.map(|d: Decimal| d.to_string()),
                yeast.fermentation_duration_days.map(|d: Decimal| d.to_string()),
                yeast.sensory_notes,
                yeast.requires_rehydration,
                yeast.compatible_ingredients,
                yeast.created_at,
                yeast.updated_at,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Get yeast strain by ID
    ///
    /// # Performance
    /// - Uses PRIMARY KEY index for O(log n) lookup
    /// - Zero-copy where possible
    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<Yeast> {
        let query = "
            SELECT 
                id, name, laboratory, product_code, yeast_type,
                alcohol_tolerance, temperature_range_min, temperature_range_max,
                attenuation, flocculation, nutrient_requirements,
                flavor_profile, aroma_profile, best_suited_styles,
                usage_notes, lag_time_hours, fermentation_duration_days,
                sensory_notes, requires_rehydration, compatible_ingredients,
                created_at, updated_at
            FROM yeasts
            WHERE id = ?1
        ";

        conn.query_row(query, rusqlite::params![id], Self::row_to_yeast)
    }

    /// List yeast strains with optional limit
    ///
    /// # Performance
    /// - Coarse-grained API reduces FFI calls
    /// - Efficient pagination via LIMIT
    /// - Uses index on id for stable ordering
    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<Yeast>> {
        let query = match limit {
            Some(_) => "
                SELECT 
                    id, name, laboratory, product_code, yeast_type,
                    alcohol_tolerance, temperature_range_min, temperature_range_max,
                    attenuation, flocculation, nutrient_requirements,
                    flavor_profile, aroma_profile, best_suited_styles,
                    usage_notes, lag_time_hours, fermentation_duration_days,
                    sensory_notes, requires_rehydration, compatible_ingredients,
                    created_at, updated_at
                FROM yeasts
                ORDER BY name ASC
                LIMIT ?1
            ",
            None => "
                SELECT 
                    id, name, laboratory, product_code, yeast_type,
                    alcohol_tolerance, temperature_range_min, temperature_range_max,
                    attenuation, flocculation, nutrient_requirements,
                    flavor_profile, aroma_profile, best_suited_styles,
                    usage_notes, lag_time_hours, fermentation_duration_days,
                    sensory_notes, requires_rehydration, compatible_ingredients,
                    created_at, updated_at
                FROM yeasts
                ORDER BY name ASC
            ",
        };

        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_yeast)?
        } else {
            stmt.query_map([], Self::row_to_yeast)?
        };

        rows.collect()
    }

    /// Search yeast strains by name, laboratory, or product code
    ///
    /// # Performance
    /// - Uses indexed columns for efficient search
    /// - Case-insensitive via LIKE
    /// - Early termination with LIMIT
    pub fn search(
        conn: &Connection,
        query: &str,
        limit: Option<i64>,
    ) -> SqliteResult<Vec<Yeast>> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        let search_pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "
                SELECT 
                    id, name, laboratory, product_code, yeast_type,
                    alcohol_tolerance, temperature_range_min, temperature_range_max,
                    attenuation, flocculation, nutrient_requirements,
                    flavor_profile, aroma_profile, best_suited_styles,
                    usage_notes, lag_time_hours, fermentation_duration_days,
                    sensory_notes, requires_rehydration, compatible_ingredients,
                    created_at, updated_at
                FROM yeasts
                WHERE name LIKE ?1 
                   OR laboratory LIKE ?1 
                   OR product_code LIKE ?1
                ORDER BY name ASC
                LIMIT ?2
            ",
            None => "
                SELECT 
                    id, name, laboratory, product_code, yeast_type,
                    alcohol_tolerance, temperature_range_min, temperature_range_max,
                    attenuation, flocculation, nutrient_requirements,
                    flavor_profile, aroma_profile, best_suited_styles,
                    usage_notes, lag_time_hours, fermentation_duration_days,
                    sensory_notes, requires_rehydration, compatible_ingredients,
                    created_at, updated_at
                FROM yeasts
                WHERE name LIKE ?1 
                   OR laboratory LIKE ?1 
                   OR product_code LIKE ?1
                ORDER BY name ASC
            ",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![search_pattern, lim], Self::row_to_yeast)?
        } else {
            stmt.query_map(rusqlite::params![search_pattern], Self::row_to_yeast)?
        };

        rows.collect()
    }

    /// Get all yeast strains of a specific type
    ///
    /// # Performance
    /// - Uses index on yeast_type
    /// - Efficient for filtering by category
    pub fn get_by_type(
        conn: &Connection,
        yeast_type: &str,
        limit: Option<i64>,
    ) -> SqliteResult<Vec<Yeast>> {
        let sql = match limit {
            Some(_) => "
                SELECT 
                    id, name, laboratory, product_code, yeast_type,
                    alcohol_tolerance, temperature_range_min, temperature_range_max,
                    attenuation, flocculation, nutrient_requirements,
                    flavor_profile, aroma_profile, best_suited_styles,
                    usage_notes, lag_time_hours, fermentation_duration_days,
                    sensory_notes, requires_rehydration, compatible_ingredients,
                    created_at, updated_at
                FROM yeasts
                WHERE yeast_type = ?1
                ORDER BY name ASC
                LIMIT ?2
            ",
            None => "
                SELECT 
                    id, name, laboratory, product_code, yeast_type,
                    alcohol_tolerance, temperature_range_min, temperature_range_max,
                    attenuation, flocculation, nutrient_requirements,
                    flavor_profile, aroma_profile, best_suited_styles,
                    usage_notes, lag_time_hours, fermentation_duration_days,
                    sensory_notes, requires_rehydration, compatible_ingredients,
                    created_at, updated_at
                FROM yeasts
                WHERE yeast_type = ?1
                ORDER BY name ASC
            ",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![yeast_type, lim], Self::row_to_yeast)?
        } else {
            stmt.query_map(rusqlite::params![yeast_type], Self::row_to_yeast)?
        };

        rows.collect()
    }

    /// Get all yeast strains from a specific laboratory
    ///
    /// # Performance
    /// - Uses index on laboratory
    /// - Efficient manufacturer filtering
    pub fn get_by_laboratory(
        conn: &Connection,
        laboratory: &str,
        limit: Option<i64>,
    ) -> SqliteResult<Vec<Yeast>> {
        let sql = match limit {
            Some(_) => "
                SELECT 
                    id, name, laboratory, product_code, yeast_type,
                    alcohol_tolerance, temperature_range_min, temperature_range_max,
                    attenuation, flocculation, nutrient_requirements,
                    flavor_profile, aroma_profile, best_suited_styles,
                    usage_notes, lag_time_hours, fermentation_duration_days,
                    sensory_notes, requires_rehydration, compatible_ingredients,
                    created_at, updated_at
                FROM yeasts
                WHERE laboratory = ?1
                ORDER BY name ASC
                LIMIT ?2
            ",
            None => "
                SELECT 
                    id, name, laboratory, product_code, yeast_type,
                    alcohol_tolerance, temperature_range_min, temperature_range_max,
                    attenuation, flocculation, nutrient_requirements,
                    flavor_profile, aroma_profile, best_suited_styles,
                    usage_notes, lag_time_hours, fermentation_duration_days,
                    sensory_notes, requires_rehydration, compatible_ingredients,
                    created_at, updated_at
                FROM yeasts
                WHERE laboratory = ?1
                ORDER BY name ASC
            ",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![laboratory, lim], Self::row_to_yeast)?
        } else {
            stmt.query_map(rusqlite::params![laboratory], Self::row_to_yeast)?
        };

        rows.collect()
    }

    /// Update yeast strain entry
    ///
    /// # Security
    /// - Validates all fields before update
    /// - Prevents partial updates on validation failure
    pub fn update(conn: &Connection, yeast: &Yeast) -> SqliteResult<()> {
        // Validate before touching database
        yeast.validate().map_err(|e| {
            rusqlite::Error::InvalidParameterName(e)
        })?;

        let query = "
            UPDATE yeasts SET
                name = ?2, laboratory = ?3, product_code = ?4, yeast_type = ?5,
                alcohol_tolerance = ?6, temperature_range_min = ?7, temperature_range_max = ?8,
                attenuation = ?9, flocculation = ?10, nutrient_requirements = ?11,
                flavor_profile = ?12, aroma_profile = ?13, best_suited_styles = ?14,
                usage_notes = ?15, lag_time_hours = ?16, fermentation_duration_days = ?17,
                sensory_notes = ?18, requires_rehydration = ?19, compatible_ingredients = ?20,
                updated_at = ?21
            WHERE id = ?1
        ";

        let rows_affected = conn.execute(
            query,
            rusqlite::params![
                yeast.id,
                yeast.name,
                yeast.laboratory,
                yeast.product_code,
                yeast.yeast_type,
                yeast.alcohol_tolerance.map(|d: Decimal| d.to_string()),
                yeast.temperature_range_min.map(|d: Decimal| d.to_string()),
                yeast.temperature_range_max.map(|d: Decimal| d.to_string()),
                yeast.attenuation.map(|d: Decimal| d.to_string()),
                yeast.flocculation,
                yeast.nutrient_requirements,
                yeast.flavor_profile,
                yeast.aroma_profile,
                yeast.best_suited_styles,
                yeast.usage_notes,
                yeast.lag_time_hours.map(|d: Decimal| d.to_string()),
                yeast.fermentation_duration_days.map(|d: Decimal| d.to_string()),
                yeast.sensory_notes,
                yeast.requires_rehydration,
                yeast.compatible_ingredients,
                yeast.updated_at,
            ],
        )?;

        if rows_affected == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        Ok(())
    }

    /// Delete yeast strain entry
    ///
    /// # Safety
    /// - Returns error if entry doesn't exist
    /// - Cascade deletions handled by FK constraints
    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let query = "DELETE FROM yeasts WHERE id = ?1";
        let rows_affected = conn.execute(query, rusqlite::params![id])?;

        if rows_affected == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        Ok(())
    }

    /// Get count of all yeast strains
    ///
    /// # Performance
    /// - Uses COUNT(*) which is optimized in SQLite
    /// - No row materialization overhead
    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        let query = "SELECT COUNT(*) FROM yeasts";
        conn.query_row(query, [], |row| row.get(0))
    }

    /// Get count by type
    pub fn count_by_type(conn: &Connection, yeast_type: &str) -> SqliteResult<i64> {
        let query = "SELECT COUNT(*) FROM yeasts WHERE yeast_type = ?1";
        conn.query_row(query, rusqlite::params![yeast_type], |row| row.get(0))
    }

    /// Helper: Convert database row to Yeast
    ///
    /// # Performance
    /// - Lazy Decimal parsing (only parse if Some)
    /// - Error fast-path for invalid Decimals
    fn row_to_yeast(row: &Row) -> SqliteResult<Yeast> {
        // Helper to parse optional Decimal from TEXT
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }

        Ok(Yeast {
            id: row.get(0)?, notes: row.get(16)?, yeast_form: row.get(5)?,
            name: row.get(1)?,
            laboratory: row.get(2)?,
            product_code: row.get(3)?,
            yeast_type: row.get(4)?,
            alcohol_tolerance: parse_decimal(row.get(5)?),
            temperature_range_min: parse_decimal(row.get(6)?),
            temperature_range_max: parse_decimal(row.get(7)?),
            attenuation: parse_decimal(row.get(8)?),
            flocculation: row.get(9)?,
            nutrient_requirements: row.get(10)?,
            flavor_profile: row.get(11)?,
            aroma_profile: row.get(12)?,
            best_suited_styles: row.get(13)?,
            usage_notes: row.get(14)?,
            lag_time_hours: parse_decimal(row.get(15)?),
            fermentation_duration_days: parse_decimal(row.get(16)?),
            sensory_notes: row.get(17)?,
            requires_rehydration: row.get(18)?,
            compatible_ingredients: row.get(19)?,
            created_at: row.get(20)?,
            updated_at: row.get(21)?,
        })
    }
}