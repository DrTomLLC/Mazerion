use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::models::hop::Hop;

/// Repository for hop variety encyclopedia operations
///
/// Provides CRUD operations with zero panic guarantee and mobile-first optimization.
pub struct HopRepository<'conn> { conn: &'conn Connection, }

impl<'conn> HopRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    /// Create a new hop entry
    pub fn create(conn: &Connection, hop: &Hop) -> SqliteResult<i64> {
        hop.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        let query = "
            INSERT INTO hops (
                name, origin, hop_type, alpha_acid, beta_acid,
                cohumulone, total_oil, myrcene, humulene,
                caryophyllene, farnesene, flavor_profile, aroma_profile,
                substitutes, best_suited_styles, usage_notes,
                sensory_notes, typical_usage, storage_stability,
                compatible_styles, created_at, updated_at
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22
            )
        ";

        conn.execute(
            query,
            rusqlite::params![
                hop.name, hop.origin, hop.hop_type,
                hop.alpha_acid.map(|d: Decimal| d.to_string()),
                hop.beta_acid.map(|d: Decimal| d.to_string()),
                hop.cohumulone.map(|d: Decimal| d.to_string()),
                hop.total_oil.map(|d: Decimal| d.to_string()),
                hop.myrcene.map(|d: Decimal| d.to_string()),
                hop.humulene.map(|d: Decimal| d.to_string()),
                hop.caryophyllene.map(|d: Decimal| d.to_string()),
                hop.farnesene.map(|d: Decimal| d.to_string()),
                hop.flavor_profile, hop.aroma_profile,
                hop.substitutes, hop.best_suited_styles,
                hop.usage_notes, hop.sensory_notes,
                hop.typical_usage, hop.storage_stability,
                hop.compatible_styles, hop.created_at, hop.updated_at,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Get hop by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<Hop> {
        let query = "
            SELECT 
                id, name, origin, hop_type, alpha_acid, beta_acid,
                cohumulone, total_oil, myrcene, humulene,
                caryophyllene, farnesene, flavor_profile, aroma_profile,
                substitutes, best_suited_styles, usage_notes,
                sensory_notes, typical_usage, storage_stability,
                compatible_styles, created_at, updated_at
            FROM hops
            WHERE id = ?1
        ";

        conn.query_row(query, rusqlite::params![id], Self::row_to_hop)
    }

    /// List hops with optional limit
    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<Hop>> {
        let query = match limit {
            Some(_) => "
                SELECT 
                    id, name, origin, hop_type, alpha_acid, beta_acid,
                    cohumulone, total_oil, myrcene, humulene,
                    caryophyllene, farnesene, flavor_profile, aroma_profile,
                    substitutes, best_suited_styles, usage_notes,
                    sensory_notes, typical_usage, storage_stability,
                    compatible_styles, created_at, updated_at
                FROM hops
                ORDER BY name ASC
                LIMIT ?1
            ",
            None => "
                SELECT 
                    id, name, origin, hop_type, alpha_acid, beta_acid,
                    cohumulone, total_oil, myrcene, humulene,
                    caryophyllene, farnesene, flavor_profile, aroma_profile,
                    substitutes, best_suited_styles, usage_notes,
                    sensory_notes, typical_usage, storage_stability,
                    compatible_styles, created_at, updated_at
                FROM hops
                ORDER BY name ASC
            ",
        };

        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_hop)?
        } else {
            stmt.query_map([], Self::row_to_hop)?
        };

        rows.collect()
    }

    /// Search hops by name or origin
    pub fn search(conn: &Connection, query: &str, limit: Option<i64>) -> SqliteResult<Vec<Hop>> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        let search_pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "
                SELECT 
                    id, name, origin, hop_type, alpha_acid, beta_acid,
                    cohumulone, total_oil, myrcene, humulene,
                    caryophyllene, farnesene, flavor_profile, aroma_profile,
                    substitutes, best_suited_styles, usage_notes,
                    sensory_notes, typical_usage, storage_stability,
                    compatible_styles, created_at, updated_at
                FROM hops
                WHERE name LIKE ?1 OR origin LIKE ?1
                ORDER BY name ASC
                LIMIT ?2
            ",
            None => "
                SELECT 
                    id, name, origin, hop_type, alpha_acid, beta_acid,
                    cohumulone, total_oil, myrcene, humulene,
                    caryophyllene, farnesene, flavor_profile, aroma_profile,
                    substitutes, best_suited_styles, usage_notes,
                    sensory_notes, typical_usage, storage_stability,
                    compatible_styles, created_at, updated_at
                FROM hops
                WHERE name LIKE ?1 OR origin LIKE ?1
                ORDER BY name ASC
            ",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![search_pattern, lim], Self::row_to_hop)?
        } else {
            stmt.query_map(rusqlite::params![search_pattern], Self::row_to_hop)?
        };

        rows.collect()
    }

    /// Get hops by type
    pub fn get_by_type(conn: &Connection, hop_type: &str, limit: Option<i64>) -> SqliteResult<Vec<Hop>> {
        let sql = match limit {
            Some(_) => "
                SELECT 
                    id, name, origin, hop_type, alpha_acid, beta_acid,
                    cohumulone, total_oil, myrcene, humulene,
                    caryophyllene, farnesene, flavor_profile, aroma_profile,
                    substitutes, best_suited_styles, usage_notes,
                    sensory_notes, typical_usage, storage_stability,
                    compatible_styles, created_at, updated_at
                FROM hops
                WHERE hop_type = ?1
                ORDER BY name ASC
                LIMIT ?2
            ",
            None => "
                SELECT 
                    id, name, origin, hop_type, alpha_acid, beta_acid,
                    cohumulone, total_oil, myrcene, humulene,
                    caryophyllene, farnesene, flavor_profile, aroma_profile,
                    substitutes, best_suited_styles, usage_notes,
                    sensory_notes, typical_usage, storage_stability,
                    compatible_styles, created_at, updated_at
                FROM hops
                WHERE hop_type = ?1
                ORDER BY name ASC
            ",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![hop_type, lim], Self::row_to_hop)?
        } else {
            stmt.query_map(rusqlite::params![hop_type], Self::row_to_hop)?
        };

        rows.collect()
    }

    /// Get hops by origin
    pub fn get_by_origin(conn: &Connection, origin: &str, limit: Option<i64>) -> SqliteResult<Vec<Hop>> {
        let sql = match limit {
            Some(_) => "
                SELECT 
                    id, name, origin, hop_type, alpha_acid, beta_acid,
                    cohumulone, total_oil, myrcene, humulene,
                    caryophyllene, farnesene, flavor_profile, aroma_profile,
                    substitutes, best_suited_styles, usage_notes,
                    sensory_notes, typical_usage, storage_stability,
                    compatible_styles, created_at, updated_at
                FROM hops
                WHERE origin = ?1
                ORDER BY name ASC
                LIMIT ?2
            ",
            None => "
                SELECT 
                    id, name, origin, hop_type, alpha_acid, beta_acid,
                    cohumulone, total_oil, myrcene, humulene,
                    caryophyllene, farnesene, flavor_profile, aroma_profile,
                    substitutes, best_suited_styles, usage_notes,
                    sensory_notes, typical_usage, storage_stability,
                    compatible_styles, created_at, updated_at
                FROM hops
                WHERE origin = ?1
                ORDER BY name ASC
            ",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![origin, lim], Self::row_to_hop)?
        } else {
            stmt.query_map(rusqlite::params![origin], Self::row_to_hop)?
        };

        rows.collect()
    }

    /// Update hop entry
    pub fn update(conn: &Connection, hop: &Hop) -> SqliteResult<()> {
        hop.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        let query = "
            UPDATE hops SET
                name = ?2, origin = ?3, hop_type = ?4, alpha_acid = ?5, beta_acid = ?6,
                cohumulone = ?7, total_oil = ?8, myrcene = ?9, humulene = ?10,
                caryophyllene = ?11, farnesene = ?12, flavor_profile = ?13, aroma_profile = ?14,
                substitutes = ?15, best_suited_styles = ?16, usage_notes = ?17,
                sensory_notes = ?18, typical_usage = ?19, storage_stability = ?20,
                compatible_styles = ?21, updated_at = ?22
            WHERE id = ?1
        ";

        let rows_affected = conn.execute(
            query,
            rusqlite::params![
                hop.id, hop.name, hop.origin, hop.hop_type,
                hop.alpha_acid.map(|d: Decimal| d.to_string()),
                hop.beta_acid.map(|d: Decimal| d.to_string()),
                hop.cohumulone.map(|d: Decimal| d.to_string()),
                hop.total_oil.map(|d: Decimal| d.to_string()),
                hop.myrcene.map(|d: Decimal| d.to_string()),
                hop.humulene.map(|d: Decimal| d.to_string()),
                hop.caryophyllene.map(|d: Decimal| d.to_string()),
                hop.farnesene.map(|d: Decimal| d.to_string()),
                hop.flavor_profile, hop.aroma_profile,
                hop.substitutes, hop.best_suited_styles,
                hop.usage_notes, hop.sensory_notes,
                hop.typical_usage, hop.storage_stability,
                hop.compatible_styles, hop.updated_at,
            ],
        )?;

        if rows_affected == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        Ok(())
    }

    /// Delete hop entry
    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let query = "DELETE FROM hops WHERE id = ?1";
        let rows_affected = conn.execute(query, rusqlite::params![id])?;

        if rows_affected == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        Ok(())
    }

    /// Get count of all hops
    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        let query = "SELECT COUNT(*) FROM hops";
        conn.query_row(query, [], |row| row.get(0))
    }

    /// Get count by type
    pub fn count_by_type(conn: &Connection, hop_type: &str) -> SqliteResult<i64> {
        let query = "SELECT COUNT(*) FROM hops WHERE hop_type = ?1";
        conn.query_row(query, rusqlite::params![hop_type], |row| row.get(0))
    }

    /// Get count by origin
    pub fn count_by_origin(conn: &Connection, origin: &str) -> SqliteResult<i64> {
        let query = "SELECT COUNT(*) FROM hops WHERE origin = ?1";
        conn.query_row(query, rusqlite::params![origin], |row| row.get(0))
    }

    /// Helper: Convert database row to Hop
    fn row_to_hop(row: &Row) -> SqliteResult<Hop> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }

        Ok(Hop {
            id: row.get(0)?,
            name: row.get(1)?,
            origin: row.get(2)?,
            hop_type: row.get(3)?,
            alpha_acid: parse_decimal(row.get(4)?),
            beta_acid: parse_decimal(row.get(5)?),
            cohumulone: parse_decimal(row.get(6)?),
            total_oil: parse_decimal(row.get(7)?),
            myrcene: parse_decimal(row.get(8)?),
            humulene: parse_decimal(row.get(9)?),
            caryophyllene: parse_decimal(row.get(10)?),
            farnesene: parse_decimal(row.get(11)?),
            flavor_profile: row.get(12)?,
            aroma_profile: row.get(13)?,
            substitutes: row.get(14)?,
            best_suited_styles: row.get(15)?,
            usage_notes: row.get(16)?,
            sensory_notes: row.get(17)?,
            typical_usage: row.get(18)?,
            storage_stability: row.get(19)?,
            compatible_styles: row.get(20)?,
            created_at: row.get(21)?,
            updated_at: row.get(22)?,
            })
    }
}