use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::models::nutrient::Nutrient;

pub struct NutrientRepository<'conn> { conn: &'conn Connection, }

impl<'conn> NutrientRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    pub fn create(conn: &Connection, nutrient: &Nutrient) -> SqliteResult<i64> {
        nutrient.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        conn.execute(
            "INSERT INTO nutrients (name, nutrient_type, manufacturer,
                nitrogen_content, phosphorus_content, potassium_content,
                typical_dosage_grams_per_gallon, usage_notes,
                best_suited_styles, compatible_styles, timing,
                created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            rusqlite::params![
                nutrient.name, nutrient.nutrient_type, nutrient.manufacturer,
                nutrient.nitrogen_content.map(|d: Decimal| d.to_string()),
                nutrient.phosphorus_content.map(|d: Decimal| d.to_string()),
                nutrient.potassium_content.map(|d: Decimal| d.to_string()),
                nutrient.typical_dosage_grams_per_gallon.map(|d: Decimal| d.to_string()),
                nutrient.usage_notes, nutrient.best_suited_styles,
                nutrient.compatible_styles, nutrient.timing,
                nutrient.created_at, nutrient.updated_at,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<Nutrient> {
        conn.query_row(
            "SELECT id, name, nutrient_type, manufacturer,
                nitrogen_content, phosphorus_content, potassium_content,
                typical_dosage_grams_per_gallon, usage_notes,
                best_suited_styles, compatible_styles, timing,
                created_at, updated_at
             FROM nutrients WHERE id = ?1",
            rusqlite::params![id],
            Self::row_to_nutrient
        )
    }

    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<Nutrient>> {
        let query = match limit {
            Some(_) => "SELECT id, name, nutrient_type, manufacturer,
                nitrogen_content, phosphorus_content, potassium_content,
                typical_dosage_grams_per_gallon, usage_notes,
                best_suited_styles, compatible_styles, timing,
                created_at, updated_at FROM nutrients ORDER BY name ASC LIMIT ?1",
            None => "SELECT id, name, nutrient_type, manufacturer,
                nitrogen_content, phosphorus_content, potassium_content,
                typical_dosage_grams_per_gallon, usage_notes,
                best_suited_styles, compatible_styles, timing,
                created_at, updated_at FROM nutrients ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_nutrient)?
        } else {
            stmt.query_map([], Self::row_to_nutrient)?
        };
        rows.collect()
    }

    pub fn search(conn: &Connection, query: &str, limit: Option<i64>) -> SqliteResult<Vec<Nutrient>> {
        if query.trim().is_empty() { return Ok(Vec::new()); }

        let pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "SELECT id, name, nutrient_type, manufacturer,
                nitrogen_content, phosphorus_content, potassium_content,
                typical_dosage_grams_per_gallon, usage_notes,
                best_suited_styles, compatible_styles, timing,
                created_at, updated_at FROM nutrients
                WHERE name LIKE ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT id, name, nutrient_type, manufacturer,
                nitrogen_content, phosphorus_content, potassium_content,
                typical_dosage_grams_per_gallon, usage_notes,
                best_suited_styles, compatible_styles, timing,
                created_at, updated_at FROM nutrients
                WHERE name LIKE ?1 ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![pattern, lim], Self::row_to_nutrient)?
        } else {
            stmt.query_map(rusqlite::params![pattern], Self::row_to_nutrient)?
        };
        rows.collect()
    }

    pub fn get_by_type(conn: &Connection, nutrient_type: &str, limit: Option<i64>) -> SqliteResult<Vec<Nutrient>> {
        let sql = match limit {
            Some(_) => "SELECT id, name, nutrient_type, manufacturer,
                nitrogen_content, phosphorus_content, potassium_content,
                typical_dosage_grams_per_gallon, usage_notes,
                best_suited_styles, compatible_styles, timing,
                created_at, updated_at FROM nutrients
                WHERE nutrient_type = ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT id, name, nutrient_type, manufacturer,
                nitrogen_content, phosphorus_content, potassium_content,
                typical_dosage_grams_per_gallon, usage_notes,
                best_suited_styles, compatible_styles, timing,
                created_at, updated_at FROM nutrients
                WHERE nutrient_type = ?1 ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![nutrient_type, lim], Self::row_to_nutrient)?
        } else {
            stmt.query_map(rusqlite::params![nutrient_type], Self::row_to_nutrient)?
        };
        rows.collect()
    }

    pub fn update(conn: &Connection, nutrient: &Nutrient) -> SqliteResult<()> {
        nutrient.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        let rows = conn.execute(
            "UPDATE nutrients SET name = ?2, nutrient_type = ?3, manufacturer = ?4,
                nitrogen_content = ?5, phosphorus_content = ?6, potassium_content = ?7,
                typical_dosage_grams_per_gallon = ?8, usage_notes = ?9,
                best_suited_styles = ?10, compatible_styles = ?11, timing = ?12,
                updated_at = ?13 WHERE id = ?1",
            rusqlite::params![
                nutrient.id, nutrient.name, nutrient.nutrient_type, nutrient.manufacturer,
                nutrient.nitrogen_content.map(|d: Decimal| d.to_string()),
                nutrient.phosphorus_content.map(|d: Decimal| d.to_string()),
                nutrient.potassium_content.map(|d: Decimal| d.to_string()),
                nutrient.typical_dosage_grams_per_gallon.map(|d: Decimal| d.to_string()),
                nutrient.usage_notes, nutrient.best_suited_styles,
                nutrient.compatible_styles, nutrient.timing, nutrient.updated_at,
            ],
        )?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let rows = conn.execute("DELETE FROM nutrients WHERE id = ?1", rusqlite::params![id])?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM nutrients", [], |row| row.get(0))
    }

    pub fn count_by_type(conn: &Connection, nutrient_type: &str) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM nutrients WHERE nutrient_type = ?1",
                       rusqlite::params![nutrient_type], |row| row.get(0))
    }

    fn row_to_nutrient(row: &Row) -> SqliteResult<Nutrient> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }

        Ok(Nutrient {
            id: row.get(0)?,
            name: row.get(1)?,
            nutrient_type: row.get(2)?,
            manufacturer: row.get(3)?,
            nitrogen_content: parse_decimal(row.get(4)?),
            phosphorus_content: parse_decimal(row.get(5)?),
            potassium_content: parse_decimal(row.get(6)?),
            typical_dosage_grams_per_gallon: parse_decimal(row.get(7)?),
            usage_notes: row.get(8)?,
            best_suited_styles: row.get(9)?,
            compatible_styles: row.get(10)?,
            timing: row.get(11)?,
            created_at: row.get(12)?,
            updated_at: row.get(13)?,
        })
    }
}