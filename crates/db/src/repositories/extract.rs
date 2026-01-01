use mazerion_core::{Error, Result};
use crate::models::Extract;
use rusqlite::{params, Connection, OptionalExtension, Row};
use rust_decimal::Decimal;
use std::str::FromStr;

const MAX_RESULTS: usize = 1000;

pub struct ExtractRepository<'conn> {
    conn: &'conn Connection,
}

impl<'conn> ExtractRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self {
        Self { conn }
    }

    pub fn create(conn: &Connection, extract: &Extract) -> Result<i64> {
        extract.validate()
            .map_err(|e| Error::Validation(e))?;

        let alcohol_based_int = if extract.alcohol_based { 1 } else { 0 };

        conn.execute(
            "INSERT INTO extracts (name, extract_type, manufacturer, alcohol_based, flavor_profile, aroma_profile, best_suited_styles, usage_notes, typical_dosage_oz_per_gallon, compatible_styles, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
            params![
            &extract.name,
            &extract.extract_type,
            &extract.manufacturer,
            alcohol_based_int,
            &extract.flavor_profile,
            &extract.aroma_profile,
            &extract.best_suited_styles,
            &extract.usage_notes,
            extract.typical_dosage_oz_per_gallon.as_ref().map(|d| d.to_string()),
            &extract.compatible_styles,
        ],
        )
            .map_err(|e| Error::DatabaseError(format!("Insert extract: {}", e)))?;

        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Extract> {
        let mut stmt = conn
            .prepare("SELECT id, name, extract_type, manufacturer, alcohol_based, flavor_profile, aroma_profile, best_suited_styles, usage_notes, typical_dosage_oz_per_gallon, compatible_styles, created_at, updated_at FROM extracts WHERE id = ?1")
            .map_err(|e| Error::DatabaseError(format!("Prepare query: {}", e)))?;

        stmt.query_row([id], |row| Self::row_to_extract(row))
            .optional()
            .map_err(|e| Error::DatabaseError(format!("Query extract: {}", e)))?
            .ok_or_else(|| Error::DatabaseError(format!("Extract {} not found", id)))
    }

    pub fn list(conn: &Connection, extract_type: Option<&str>) -> Result<Vec<Extract>> {
        let (query, params): (String, Vec<Box<dyn rusqlite::ToSql>>) = if let Some(et) = extract_type {
            (
                "SELECT id, name, extract_type, manufacturer, alcohol_based, flavor_profile, aroma_profile, best_suited_styles, usage_notes, typical_dosage_oz_per_gallon, compatible_styles, created_at, updated_at FROM extracts WHERE extract_type = ?1 ORDER BY name".to_string(),
                vec![Box::new(et.to_string())],
            )
        } else {
            (
                "SELECT id, name, extract_type, manufacturer, alcohol_based, flavor_profile, aroma_profile, best_suited_styles, usage_notes, typical_dosage_oz_per_gallon, compatible_styles, created_at, updated_at FROM extracts ORDER BY name".to_string(),
                vec![],
            )
        };

        let mut stmt = conn
            .prepare(&query)
            .map_err(|e| Error::DatabaseError(format!("Prepare list: {}", e)))?;

        let params_ref: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        let rows = stmt
            .query_map(&params_ref[..], |row| Self::row_to_extract(row))
            .map_err(|e| Error::DatabaseError(format!("Query list: {}", e)))?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row.map_err(|e| Error::DatabaseError(format!("Row error: {}", e)))?);
        }
        Ok(results)
    }

    pub fn search(conn: &Connection, query: &str, extract_type: Option<&str>) -> Result<Vec<Extract>> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        let search_pattern = format!("%{}%", query.trim());
        let (sql, params): (String, Vec<Box<dyn rusqlite::ToSql>>) = if let Some(et) = extract_type {
            (
                "SELECT id, name, extract_type, manufacturer, alcohol_based, flavor_profile, aroma_profile, best_suited_styles, usage_notes, typical_dosage_oz_per_gallon, compatible_styles, created_at, updated_at
                 FROM extracts
                 WHERE name LIKE ?1 AND extract_type = ?2
                 ORDER BY name
                 LIMIT ?3".to_string(),
                vec![
                    Box::new(search_pattern),
                    Box::new(et.to_string()),
                    Box::new(MAX_RESULTS as i64),
                ],
            )
        } else {
            (
                "SELECT id, name, extract_type, manufacturer, alcohol_based, flavor_profile, aroma_profile, best_suited_styles, usage_notes, typical_dosage_oz_per_gallon, compatible_styles, created_at, updated_at
                 FROM extracts
                 WHERE name LIKE ?1
                 ORDER BY name
                 LIMIT ?2".to_string(),
                vec![Box::new(search_pattern), Box::new(MAX_RESULTS as i64)],
            )
        };

        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| Error::DatabaseError(format!("Prepare search: {}", e)))?;

        let params_ref: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        let rows = stmt
            .query_map(&params_ref[..], |row| Self::row_to_extract(row))
            .map_err(|e| Error::DatabaseError(format!("Query search: {}", e)))?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row.map_err(|e| Error::DatabaseError(format!("Row error: {}", e)))?);
        }
        Ok(results)
    }

    pub fn update(conn: &Connection, extract: &Extract) -> Result<()> {
        extract.validate()
            .map_err(|e| Error::Validation(e))?;

        let alcohol_based_int = if extract.alcohol_based { 1 } else { 0 };

        conn.execute(
            "UPDATE extracts
             SET name = ?1, extract_type = ?2, manufacturer = ?3, alcohol_based = ?4,
                 flavor_profile = ?5, aroma_profile = ?6, best_suited_styles = ?7,
                 usage_notes = ?8, typical_dosage_oz_per_gallon = ?9, compatible_styles = ?10,
                 updated_at = CURRENT_TIMESTAMP
             WHERE id = ?11",
            params![
                &extract.name,
                &extract.extract_type,
                &extract.manufacturer,
                alcohol_based_int,
                &extract.flavor_profile,
                &extract.aroma_profile,
                &extract.best_suited_styles,
                &extract.usage_notes,
                extract.typical_dosage_oz_per_gallon.as_ref().map(|d| d.to_string()),
                &extract.compatible_styles,
                extract.id,
            ],
        )
            .map_err(|e| Error::DatabaseError(format!("Update extract: {}", e)))?;

        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM extracts WHERE id = ?1", params![id])
            .map_err(|e| Error::DatabaseError(format!("Delete extract: {}", e)))?;
        Ok(())
    }

    pub fn count(conn: &Connection) -> Result<u32> {
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM extracts", [], |row| row.get(0))
            .map_err(|e| Error::DatabaseError(format!("Count extracts: {}", e)))?;
        Ok(count as u32)
    }

    fn row_to_extract(row: &Row) -> rusqlite::Result<Extract> {
        let alcohol_based_int: i32 = row.get(4)?;
        let alcohol_based = alcohol_based_int != 0;

        Ok(Extract {
            id: row.get(0)?,
            name: row.get(1)?,
            extract_type: row.get(2)?,
            manufacturer: row.get(3)?,
            flavor_profile: row.get(5)?,
            aroma_profile: row.get(6)?,
            best_suited_styles: row.get(7)?,
            usage_notes: row.get(8)?,
            typical_dosage_oz_per_gallon: row.get::<_,
                Option<String>>(9)?
                .and_then(|s| Decimal::from_str(&s).ok()),
            alcohol_based,
            compatible_styles: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
            concentration: (),
            notes: ()
        })
    }
}