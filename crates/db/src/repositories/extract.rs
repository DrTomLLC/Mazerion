use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::models::extract::Extract;

pub struct ExtractRepository<'conn> { conn: &'conn Connection, }

impl<'conn> ExtractRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    pub fn create(conn: &Connection, extract: &Extract) -> SqliteResult<i64> {
        extract.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;
        conn.execute(
            "INSERT INTO extracts (name, extract_type, manufacturer, flavor_profile, aroma_profile,
                best_suited_styles, usage_notes, typical_dosage_oz_per_gallon, alcohol_based,
                compatible_styles, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            rusqlite::params![extract.name, extract.extract_type, extract.manufacturer,
                extract.flavor_profile, extract.aroma_profile, extract.best_suited_styles,
                extract.usage_notes, extract.typical_dosage_oz_per_gallon.map(|d: Decimal| d.to_string()),
                extract.alcohol_based, extract.compatible_styles, extract.created_at, extract.updated_at],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<Extract> {
        conn.query_row("SELECT * FROM extracts WHERE id = ?1", rusqlite::params![id], Self::row_to_extract)
    }

    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<Extract>> {
        let query = match limit {
            Some(_) => "SELECT * FROM extracts ORDER BY name ASC LIMIT ?1",
            None => "SELECT * FROM extracts ORDER BY name ASC",
        };
        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_extract)?
        } else { stmt.query_map([], Self::row_to_extract)? };
        rows.collect()
    }

    pub fn search(conn: &Connection, query: &str, limit: Option<i64>) -> SqliteResult<Vec<Extract>> {
        if query.trim().is_empty() { return Ok(Vec::new()); }
        let pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "SELECT * FROM extracts WHERE name LIKE ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT * FROM extracts WHERE name LIKE ?1 ORDER BY name ASC",
        };
        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![pattern, lim], Self::row_to_extract)?
        } else { stmt.query_map(rusqlite::params![pattern], Self::row_to_extract)? };
        rows.collect()
    }

    pub fn update(conn: &Connection, extract: &Extract) -> SqliteResult<()> {
        extract.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;
        let rows = conn.execute(
            "UPDATE extracts SET name=?2, extract_type=?3, manufacturer=?4, flavor_profile=?5,
                aroma_profile=?6, best_suited_styles=?7, usage_notes=?8,
                typical_dosage_oz_per_gallon=?9, alcohol_based=?10, compatible_styles=?11,
                updated_at=?12 WHERE id=?1",
            rusqlite::params![extract.id, extract.name, extract.extract_type, extract.manufacturer,
                extract.flavor_profile, extract.aroma_profile, extract.best_suited_styles,
                extract.usage_notes, extract.typical_dosage_oz_per_gallon.map(|d: Decimal| d.to_string()),
                extract.alcohol_based, extract.compatible_styles, extract.updated_at],
        )?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let rows = conn.execute("DELETE FROM extracts WHERE id = ?1", rusqlite::params![id])?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM extracts", [], |row| row.get(0))
    }

    fn row_to_extract(row: &Row) -> SqliteResult<Extract> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }
        Ok(Extract {
            id: row.get(0)?, name: row.get(1)?, extract_type: row.get(2)?,
            manufacturer: row.get(3)?, flavor_profile: row.get(4)?,
            aroma_profile: row.get(5)?, best_suited_styles: row.get(6)?,
            usage_notes: row.get(7)?, typical_dosage_oz_per_gallon: parse_decimal(row.get(8)?),
            alcohol_based: row.get(9)?, compatible_styles: row.get(10)?,
            created_at: row.get(11)?, updated_at: row.get(12)?,
        })
    }
}