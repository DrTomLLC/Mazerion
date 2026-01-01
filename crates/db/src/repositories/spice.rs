use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::models::spice::Spice;

pub struct SpiceRepository<'conn> { conn: &'conn Connection, }

impl<'conn> SpiceRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    pub fn create(conn: &Connection, spice: &Spice) -> SqliteResult<i64> {
        spice.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;
        conn.execute(
            "INSERT INTO spices (name, scientific_name, spice_type, origin, heat_level,
                flavor_profile, aroma_profile, best_suited_styles, usage_notes, sensory_notes,
                typical_dosage_oz_per_gallon, preparation_method, compatible_styles, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            rusqlite::params![spice.name, spice.scientific_name, spice.spice_type, spice.origin,
                spice.heat_level.map(|d: Decimal| d.to_string()), spice.flavor_profile, spice.aroma_profile,
                spice.best_suited_styles, spice.usage_notes, spice.sensory_notes,
                spice.typical_dosage_oz_per_gallon.map(|d: Decimal| d.to_string()),
                spice.preparation_method, spice.compatible_styles, spice.created_at, spice.updated_at],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<Spice> {
        conn.query_row("SELECT * FROM spices WHERE id = ?1", rusqlite::params![id], Self::row_to_spice)
    }

    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<Spice>> {
        let query = match limit {
            Some(_) => "SELECT * FROM spices ORDER BY name ASC LIMIT ?1",
            None => "SELECT * FROM spices ORDER BY name ASC",
        };
        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_spice)?
        } else { stmt.query_map([], Self::row_to_spice)? };
        rows.collect()
    }

    pub fn search(conn: &Connection, query: &str, limit: Option<i64>) -> SqliteResult<Vec<Spice>> {
        if query.trim().is_empty() { return Ok(Vec::new()); }
        let pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "SELECT * FROM spices WHERE name LIKE ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT * FROM spices WHERE name LIKE ?1 ORDER BY name ASC",
        };
        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![pattern, lim], Self::row_to_spice)?
        } else { stmt.query_map(rusqlite::params![pattern], Self::row_to_spice)? };
        rows.collect()
    }

    pub fn get_by_type(conn: &Connection, spice_type: &str, limit: Option<i64>) -> SqliteResult<Vec<Spice>> {
        let sql = match limit {
            Some(_) => "SELECT * FROM spices WHERE spice_type = ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT * FROM spices WHERE spice_type = ?1 ORDER BY name ASC",
        };
        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![spice_type, lim], Self::row_to_spice)?
        } else { stmt.query_map(rusqlite::params![spice_type], Self::row_to_spice)? };
        rows.collect()
    }

    pub fn update(conn: &Connection, spice: &Spice) -> SqliteResult<()> {
        spice.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;
        let rows = conn.execute(
            "UPDATE spices SET name=?2, scientific_name=?3, spice_type=?4, origin=?5, heat_level=?6,
                flavor_profile=?7, aroma_profile=?8, best_suited_styles=?9, usage_notes=?10,
                sensory_notes=?11, typical_dosage_oz_per_gallon=?12, preparation_method=?13,
                compatible_styles=?14, updated_at=?15 WHERE id=?1",
            rusqlite::params![spice.id, spice.name, spice.scientific_name, spice.spice_type, spice.origin,
                spice.heat_level.map(|d: Decimal| d.to_string()), spice.flavor_profile, spice.aroma_profile,
                spice.best_suited_styles, spice.usage_notes, spice.sensory_notes,
                spice.typical_dosage_oz_per_gallon.map(|d: Decimal| d.to_string()),
                spice.preparation_method, spice.compatible_styles, spice.updated_at],
        )?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let rows = conn.execute("DELETE FROM spices WHERE id = ?1", rusqlite::params![id])?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM spices", [], |row| row.get(0))
    }

    fn row_to_spice(row: &Row) -> SqliteResult<Spice> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }
        Ok(Spice {
            id: row.get(0)?, name: row.get(1)?, scientific_name: row.get(2)?,
            spice_type: row.get(3)?, origin: row.get(4)?, heat_level: parse_decimal(row.get(5)?),
            flavor_profile: row.get(6)?, aroma_profile: row.get(7)?,
            best_suited_styles: row.get(8)?, usage_notes: row.get(9)?,
            sensory_notes: row.get(10)?, typical_dosage_oz_per_gallon: parse_decimal(row.get(11)?),
            preparation_method: row.get(12)?, compatible_styles: row.get(13)?,
            created_at: row.get(14)?, updated_at: row.get(15)?,
        })
    }
}