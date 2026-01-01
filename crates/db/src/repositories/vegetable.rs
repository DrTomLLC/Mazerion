use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::models::vegetable::Vegetable;

pub struct VegetableRepository<'conn> { conn: &'conn Connection, }

impl<'conn> VegetableRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    pub fn create(conn: &Connection, veg: &Vegetable) -> SqliteResult<i64> {
        veg.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;
        conn.execute(
            "INSERT INTO vegetables (name, scientific_name, vegetable_type, origin,
                typical_sugar_content, ph_level, flavor_profile, aroma_profile,
                best_suited_styles, usage_notes, sensory_notes, pounds_per_gallon,
                preparation_method, compatible_styles, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
            rusqlite::params![veg.name, veg.scientific_name, veg.vegetable_type, veg.origin,
                veg.typical_sugar_content.map(|d: Decimal| d.to_string()), veg.ph_level.map(|d: Decimal| d.to_string()),
                veg.flavor_profile, veg.aroma_profile, veg.best_suited_styles, veg.usage_notes,
                veg.sensory_notes, veg.pounds_per_gallon.map(|d: Decimal| d.to_string()),
                veg.preparation_method, veg.compatible_styles, veg.created_at, veg.updated_at],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<Vegetable> {
        conn.query_row("SELECT id, name, scientific_name, vegetable_type, origin,
            typical_sugar_content, ph_level, flavor_profile, aroma_profile, best_suited_styles,
            usage_notes, sensory_notes, pounds_per_gallon, preparation_method, compatible_styles,
            created_at, updated_at FROM vegetables WHERE id = ?1",
                       rusqlite::params![id], Self::row_to_veg)
    }

    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<Vegetable>> {
        let query = match limit {
            Some(_) => "SELECT * FROM vegetables ORDER BY name ASC LIMIT ?1",
            None => "SELECT * FROM vegetables ORDER BY name ASC",
        };
        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_veg)?
        } else { stmt.query_map([], Self::row_to_veg)? };
        rows.collect()
    }

    pub fn search(conn: &Connection, query: &str, limit: Option<i64>) -> SqliteResult<Vec<Vegetable>> {
        if query.trim().is_empty() { return Ok(Vec::new()); }
        let pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "SELECT * FROM vegetables WHERE name LIKE ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT * FROM vegetables WHERE name LIKE ?1 ORDER BY name ASC",
        };
        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![pattern, lim], Self::row_to_veg)?
        } else { stmt.query_map(rusqlite::params![pattern], Self::row_to_veg)? };
        rows.collect()
    }

    pub fn get_by_type(conn: &Connection, veg_type: &str, limit: Option<i64>) -> SqliteResult<Vec<Vegetable>> {
        let sql = match limit {
            Some(_) => "SELECT * FROM vegetables WHERE vegetable_type = ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT * FROM vegetables WHERE vegetable_type = ?1 ORDER BY name ASC",
        };
        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![veg_type, lim], Self::row_to_veg)?
        } else { stmt.query_map(rusqlite::params![veg_type], Self::row_to_veg)? };
        rows.collect()
    }

    pub fn update(conn: &Connection, veg: &Vegetable) -> SqliteResult<()> {
        veg.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;
        let rows = conn.execute(
            "UPDATE vegetables SET name=?2, scientific_name=?3, vegetable_type=?4, origin=?5,
                typical_sugar_content=?6, ph_level=?7, flavor_profile=?8, aroma_profile=?9,
                best_suited_styles=?10, usage_notes=?11, sensory_notes=?12, pounds_per_gallon=?13,
                preparation_method=?14, compatible_styles=?15, updated_at=?16 WHERE id=?1",
            rusqlite::params![veg.id, veg.name, veg.scientific_name, veg.vegetable_type, veg.origin,
                veg.typical_sugar_content.map(|d: Decimal| d.to_string()), veg.ph_level.map(|d: Decimal| d.to_string()),
                veg.flavor_profile, veg.aroma_profile, veg.best_suited_styles, veg.usage_notes,
                veg.sensory_notes, veg.pounds_per_gallon.map(|d: Decimal| d.to_string()),
                veg.preparation_method, veg.compatible_styles, veg.updated_at],
        )?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let rows = conn.execute("DELETE FROM vegetables WHERE id = ?1", rusqlite::params![id])?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM vegetables", [], |row| row.get(0))
    }

    fn row_to_veg(row: &Row) -> SqliteResult<Vegetable> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }
        Ok(Vegetable {
            id: row.get(0)?, name: row.get(1)?, scientific_name: row.get(2)?,
            vegetable_type: row.get(3)?, origin: row.get(4)?,
            typical_sugar_content: parse_decimal(row.get(5)?), ph_level: parse_decimal(row.get(6)?),
            flavor_profile: row.get(7)?, aroma_profile: row.get(8)?,
            best_suited_styles: row.get(9)?, usage_notes: row.get(10)?,
            sensory_notes: row.get(11)?, pounds_per_gallon: parse_decimal(row.get(12)?),
            preparation_method: row.get(13)?, compatible_styles: row.get(14)?,
            created_at: row.get(15)?, updated_at: row.get(16)?,
        })
    }
}