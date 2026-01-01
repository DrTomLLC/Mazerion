use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::models::syrup::Syrup;

pub struct SyrupRepository<'conn> { conn: &'conn Connection, }

impl<'conn> SyrupRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    pub fn create(conn: &Connection, syrup: &Syrup) -> SqliteResult<i64> {
        syrup.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;
        conn.execute(
            "INSERT INTO syrups (name, syrup_type, manufacturer, sugar_content, flavor_profile,
                best_suited_styles, usage_notes, typical_dosage_oz_per_gallon, compatible_styles,
                created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            rusqlite::params![syrup.name, syrup.syrup_type, syrup.manufacturer,
                syrup.sugar_content.map(|d: Decimal| d.to_string()), syrup.flavor_profile,
                syrup.best_suited_styles, syrup.usage_notes,
                syrup.typical_dosage_oz_per_gallon.map(|d: Decimal| d.to_string()),
                syrup.compatible_styles, syrup.created_at, syrup.updated_at],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<Syrup> {
        conn.query_row("SELECT * FROM syrups WHERE id = ?1", rusqlite::params![id], Self::row_to_syrup)
    }

    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<Syrup>> {
        let query = match limit {
            Some(_) => "SELECT * FROM syrups ORDER BY name ASC LIMIT ?1",
            None => "SELECT * FROM syrups ORDER BY name ASC",
        };
        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_syrup)?
        } else { stmt.query_map([], Self::row_to_syrup)? };
        rows.collect()
    }

    pub fn search(conn: &Connection, query: &str, limit: Option<i64>) -> SqliteResult<Vec<Syrup>> {
        if query.trim().is_empty() { return Ok(Vec::new()); }
        let pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "SELECT * FROM syrups WHERE name LIKE ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT * FROM syrups WHERE name LIKE ?1 ORDER BY name ASC",
        };
        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![pattern, lim], Self::row_to_syrup)?
        } else { stmt.query_map(rusqlite::params![pattern], Self::row_to_syrup)? };
        rows.collect()
    }

    pub fn update(conn: &Connection, syrup: &Syrup) -> SqliteResult<()> {
        syrup.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;
        let rows = conn.execute(
            "UPDATE syrups SET name=?2, syrup_type=?3, manufacturer=?4, sugar_content=?5,
                flavor_profile=?6, best_suited_styles=?7, usage_notes=?8,
                typical_dosage_oz_per_gallon=?9, compatible_styles=?10, updated_at=?11 WHERE id=?1",
            rusqlite::params![syrup.id, syrup.name, syrup.syrup_type, syrup.manufacturer,
                syrup.sugar_content.map(|d: Decimal| d.to_string()), syrup.flavor_profile,
                syrup.best_suited_styles, syrup.usage_notes,
                syrup.typical_dosage_oz_per_gallon.map(|d: Decimal| d.to_string()),
                syrup.compatible_styles, syrup.updated_at],
        )?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let rows = conn.execute("DELETE FROM syrups WHERE id = ?1", rusqlite::params![id])?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM syrups", [], |row| row.get(0))
    }

    fn row_to_syrup(row: &Row) -> SqliteResult<Syrup> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }
        Ok(Syrup {
            id: row.get(0)?, name: row.get(1)?, syrup_type: row.get(2)?,
            manufacturer: row.get(3)?, sugar_content: parse_decimal(row.get(4)?),
            flavor_profile: row.get(5)?, best_suited_styles: row.get(6)?,
            usage_notes: row.get(7)?, typical_dosage_oz_per_gallon: parse_decimal(row.get(8)?),
            compatible_styles: row.get(9)?, created_at: row.get(10)?, updated_at: row.get(11)?,
        })
    }
}