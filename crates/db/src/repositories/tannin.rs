use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::models::tannin::Tannin;

pub struct TanninRepository<'conn> { conn: &'conn Connection, }

impl<'conn> TanninRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    pub fn create(conn: &Connection, tannin: &Tannin) -> SqliteResult<i64> {
        tannin.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        conn.execute(
            "INSERT INTO tannins (name, tannin_type, source, manufacturer,
                concentration, typical_dosage_grams_per_gallon,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles,
                timing, purpose, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            rusqlite::params![
                tannin.name, tannin.tannin_type, tannin.source, tannin.manufacturer,
                tannin.concentration.map(|d: Decimal| d.to_string()),
                tannin.typical_dosage_grams_per_gallon.map(|d: Decimal| d.to_string()),
                tannin.usage_notes, tannin.flavor_impact, tannin.best_suited_styles,
                tannin.compatible_styles, tannin.timing, tannin.purpose,
                tannin.created_at, tannin.updated_at,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<Tannin> {
        conn.query_row(
            "SELECT id, name, tannin_type, source, manufacturer,
                concentration, typical_dosage_grams_per_gallon,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles,
                timing, purpose, created_at, updated_at
             FROM tannins WHERE id = ?1",
            rusqlite::params![id],
            Self::row_to_tannin
        )
    }

    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<Tannin>> {
        let query = match limit {
            Some(_) => "SELECT id, name, tannin_type, source, manufacturer,
                concentration, typical_dosage_grams_per_gallon,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles,
                timing, purpose, created_at, updated_at FROM tannins ORDER BY name ASC LIMIT ?1",
            None => "SELECT id, name, tannin_type, source, manufacturer,
                concentration, typical_dosage_grams_per_gallon,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles,
                timing, purpose, created_at, updated_at FROM tannins ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_tannin)?
        } else {
            stmt.query_map([], Self::row_to_tannin)?
        };
        rows.collect()
    }

    pub fn search(conn: &Connection, query: &str, limit: Option<i64>) -> SqliteResult<Vec<Tannin>> {
        if query.trim().is_empty() { return Ok(Vec::new()); }

        let pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "SELECT id, name, tannin_type, source, manufacturer,
                concentration, typical_dosage_grams_per_gallon,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles,
                timing, purpose, created_at, updated_at FROM tannins
                WHERE name LIKE ?1 OR source LIKE ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT id, name, tannin_type, source, manufacturer,
                concentration, typical_dosage_grams_per_gallon,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles,
                timing, purpose, created_at, updated_at FROM tannins
                WHERE name LIKE ?1 OR source LIKE ?1 ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![pattern, lim], Self::row_to_tannin)?
        } else {
            stmt.query_map(rusqlite::params![pattern], Self::row_to_tannin)?
        };
        rows.collect()
    }

    pub fn get_by_type(conn: &Connection, tannin_type: &str, limit: Option<i64>) -> SqliteResult<Vec<Tannin>> {
        let sql = match limit {
            Some(_) => "SELECT id, name, tannin_type, source, manufacturer,
                concentration, typical_dosage_grams_per_gallon,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles,
                timing, purpose, created_at, updated_at FROM tannins
                WHERE tannin_type = ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT id, name, tannin_type, source, manufacturer,
                concentration, typical_dosage_grams_per_gallon,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles,
                timing, purpose, created_at, updated_at FROM tannins
                WHERE tannin_type = ?1 ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![tannin_type, lim], Self::row_to_tannin)?
        } else {
            stmt.query_map(rusqlite::params![tannin_type], Self::row_to_tannin)?
        };
        rows.collect()
    }

    pub fn update(conn: &Connection, tannin: &Tannin) -> SqliteResult<()> {
        tannin.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        let rows = conn.execute(
            "UPDATE tannins SET name = ?2, tannin_type = ?3, source = ?4, manufacturer = ?5,
                concentration = ?6, typical_dosage_grams_per_gallon = ?7,
                usage_notes = ?8, flavor_impact = ?9, best_suited_styles = ?10,
                compatible_styles = ?11, timing = ?12, purpose = ?13, updated_at = ?14 WHERE id = ?1",
            rusqlite::params![
                tannin.id, tannin.name, tannin.tannin_type, tannin.source, tannin.manufacturer,
                tannin.concentration.map(|d: Decimal| d.to_string()),
                tannin.typical_dosage_grams_per_gallon.map(|d: Decimal| d.to_string()),
                tannin.usage_notes, tannin.flavor_impact, tannin.best_suited_styles,
                tannin.compatible_styles, tannin.timing, tannin.purpose, tannin.updated_at,
            ],
        )?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let rows = conn.execute("DELETE FROM tannins WHERE id = ?1", rusqlite::params![id])?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM tannins", [], |row| row.get(0))
    }

    pub fn count_by_type(conn: &Connection, tannin_type: &str) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM tannins WHERE tannin_type = ?1",
                       rusqlite::params![tannin_type], |row| row.get(0))
    }

    fn row_to_tannin(row: &Row) -> SqliteResult<Tannin> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }

        Ok(Tannin {
            id: row.get(0)?,
            name: row.get(1)?,
            tannin_type: row.get(2)?,
            source: row.get(3)?,
            manufacturer: row.get(4)?,
            concentration: parse_decimal(row.get(5)?),
            typical_dosage_grams_per_gallon: parse_decimal(row.get(6)?),
            usage_notes: row.get(7)?,
            flavor_impact: row.get(8)?,
            best_suited_styles: row.get(9)?,
            compatible_styles: row.get(10)?,
            timing: row.get(11)?,
            purpose: row.get(12)?,
            created_at: row.get(13)?,
            updated_at: row.get(14)?,
        })
    }
}