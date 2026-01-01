use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::models::herb::Herb;

pub struct HerbRepository<'conn> { conn: &'conn Connection, }

impl<'conn> HerbRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    pub fn create(conn: &Connection, herb: &Herb) -> SqliteResult<i64> {
        herb.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;
        conn.execute(
            "INSERT INTO herbs (name, scientific_name, herb_type, origin, flavor_profile,
                aroma_profile, best_suited_styles, usage_notes, sensory_notes,
                typical_dosage_oz_per_gallon, preparation_method, compatible_styles, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            rusqlite::params![herb.name, herb.scientific_name, herb.herb_type, herb.origin,
                herb.flavor_profile, herb.aroma_profile, herb.best_suited_styles, herb.usage_notes,
                herb.sensory_notes, herb.typical_dosage_oz_per_gallon.map(|d: Decimal| d.to_string()),
                herb.preparation_method, herb.compatible_styles, herb.created_at, herb.updated_at],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<Herb> {
        conn.query_row("SELECT * FROM herbs WHERE id = ?1", rusqlite::params![id], Self::row_to_herb)
    }

    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<Herb>> {
        let query = match limit {
            Some(_) => "SELECT * FROM herbs ORDER BY name ASC LIMIT ?1",
            None => "SELECT * FROM herbs ORDER BY name ASC",
        };
        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_herb)?
        } else { stmt.query_map([], Self::row_to_herb)? };
        rows.collect()
    }

    pub fn search(conn: &Connection, query: &str, limit: Option<i64>) -> SqliteResult<Vec<Herb>> {
        if query.trim().is_empty() { return Ok(Vec::new()); }
        let pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "SELECT * FROM herbs WHERE name LIKE ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT * FROM herbs WHERE name LIKE ?1 ORDER BY name ASC",
        };
        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![pattern, lim], Self::row_to_herb)?
        } else { stmt.query_map(rusqlite::params![pattern], Self::row_to_herb)? };
        rows.collect()
    }

    pub fn get_by_type(conn: &Connection, herb_type: &str, limit: Option<i64>) -> SqliteResult<Vec<Herb>> {
        let sql = match limit {
            Some(_) => "SELECT * FROM herbs WHERE herb_type = ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT * FROM herbs WHERE herb_type = ?1 ORDER BY name ASC",
        };
        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![herb_type, lim], Self::row_to_herb)?
        } else { stmt.query_map(rusqlite::params![herb_type], Self::row_to_herb)? };
        rows.collect()
    }

    pub fn update(conn: &Connection, herb: &Herb) -> SqliteResult<()> {
        herb.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;
        let rows = conn.execute(
            "UPDATE herbs SET name=?2, scientific_name=?3, herb_type=?4, origin=?5,
                flavor_profile=?6, aroma_profile=?7, best_suited_styles=?8, usage_notes=?9,
                sensory_notes=?10, typical_dosage_oz_per_gallon=?11, preparation_method=?12,
                compatible_styles=?13, updated_at=?14 WHERE id=?1",
            rusqlite::params![herb.id, herb.name, herb.scientific_name, herb.herb_type, herb.origin,
                herb.flavor_profile, herb.aroma_profile, herb.best_suited_styles, herb.usage_notes,
                herb.sensory_notes, herb.typical_dosage_oz_per_gallon.map(|d: Decimal| d.to_string()),
                herb.preparation_method, herb.compatible_styles, herb.updated_at],
        )?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let rows = conn.execute("DELETE FROM herbs WHERE id = ?1", rusqlite::params![id])?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM herbs", [], |row| row.get(0))
    }

    fn row_to_herb(row: &Row) -> SqliteResult<Herb> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }
        Ok(Herb {
            id: row.get(0)?, name: row.get(1)?, scientific_name: row.get(2)?,
            herb_type: row.get(3)?, origin: row.get(4)?, flavor_profile: row.get(5)?,
            aroma_profile: row.get(6)?, best_suited_styles: row.get(7)?,
            usage_notes: row.get(8)?, sensory_notes: row.get(9)?,
            typical_dosage_oz_per_gallon: parse_decimal(row.get(10)?),
            preparation_method: row.get(11)?, compatible_styles: row.get(12)?,
            created_at: row.get(13)?, updated_at: row.get(14)?,
        })
    }
}