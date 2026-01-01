use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::models::adjunct::Adjunct;

pub struct AdjunctRepository<'conn> { conn: &'conn Connection, }

impl<'conn> AdjunctRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    pub fn create(conn: &Connection, adjunct: &Adjunct) -> SqliteResult<i64> {
        adjunct.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;
        conn.execute(
            "INSERT INTO adjuncts (name, adjunct_type, manufacturer, fermentability, flavor_profile,
                best_suited_styles, usage_notes, typical_percentage, compatible_styles, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            rusqlite::params![adjunct.name, adjunct.adjunct_type, adjunct.manufacturer,
                adjunct.fermentability.map(|d: Decimal| d.to_string()), adjunct.flavor_profile,
                adjunct.best_suited_styles, adjunct.usage_notes,
                adjunct.typical_percentage.map(|d: Decimal| d.to_string()),
                adjunct.compatible_styles, adjunct.created_at, adjunct.updated_at],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<Adjunct> {
        conn.query_row("SELECT * FROM adjuncts WHERE id = ?1", rusqlite::params![id], Self::row_to_adjunct)
    }

    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<Adjunct>> {
        let query = match limit {
            Some(_) => "SELECT * FROM adjuncts ORDER BY name ASC LIMIT ?1",
            None => "SELECT * FROM adjuncts ORDER BY name ASC",
        };
        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_adjunct)?
        } else { stmt.query_map([], Self::row_to_adjunct)? };
        rows.collect()
    }

    pub fn search(conn: &Connection, query: &str, limit: Option<i64>) -> SqliteResult<Vec<Adjunct>> {
        if query.trim().is_empty() { return Ok(Vec::new()); }
        let pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "SELECT * FROM adjuncts WHERE name LIKE ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT * FROM adjuncts WHERE name LIKE ?1 ORDER BY name ASC",
        };
        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![pattern, lim], Self::row_to_adjunct)?
        } else { stmt.query_map(rusqlite::params![pattern], Self::row_to_adjunct)? };
        rows.collect()
    }

    pub fn update(conn: &Connection, adjunct: &Adjunct) -> SqliteResult<()> {
        adjunct.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;
        let rows = conn.execute(
            "UPDATE adjuncts SET name=?2, adjunct_type=?3, manufacturer=?4, fermentability=?5,
                flavor_profile=?6, best_suited_styles=?7, usage_notes=?8, typical_percentage=?9,
                compatible_styles=?10, updated_at=?11 WHERE id=?1",
            rusqlite::params![adjunct.id, adjunct.name, adjunct.adjunct_type, adjunct.manufacturer,
                adjunct.fermentability.map(|d: Decimal| d.to_string()), adjunct.flavor_profile,
                adjunct.best_suited_styles, adjunct.usage_notes,
                adjunct.typical_percentage.map(|d: Decimal| d.to_string()),
                adjunct.compatible_styles, adjunct.updated_at],
        )?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let rows = conn.execute("DELETE FROM adjuncts WHERE id = ?1", rusqlite::params![id])?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM adjuncts", [], |row| row.get(0))
    }

    fn row_to_adjunct(row: &Row) -> SqliteResult<Adjunct> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }
        Ok(Adjunct {
            id: row.get(0)?, name: row.get(1)?, adjunct_type: row.get(2)?,
            manufacturer: row.get(3)?, fermentability: parse_decimal(row.get(4)?),
            flavor_profile: row.get(5)?, best_suited_styles: row.get(6)?,
            usage_notes: row.get(7)?, typical_percentage: parse_decimal(row.get(8)?),
            compatible_styles: row.get(9)?, created_at: row.get(10)?, updated_at: row.get(11)?,
        })
    }
}