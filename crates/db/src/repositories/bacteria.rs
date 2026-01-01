use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::models::bacteria::Bacteria;

pub struct BacteriaRepository<'conn> { conn: &'conn Connection, }

impl<'conn> BacteriaRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    pub fn create(conn: &Connection, bacteria: &Bacteria) -> SqliteResult<i64> {
        bacteria.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        conn.execute(
            "INSERT INTO bacteria (name, bacteria_type, laboratory, product_code,
                optimal_temperature_min, optimal_temperature_max,
                optimal_ph_min, optimal_ph_max, typical_dosage_grams_per_gallon,
                usage_notes, flavor_profile, best_suited_styles, compatible_styles, timing,
                created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
            rusqlite::params![
                bacteria.name, bacteria.bacteria_type, bacteria.laboratory, bacteria.product_code,
                bacteria.optimal_temperature_min.map(|d: Decimal| d.to_string()),
                bacteria.optimal_temperature_max.map(|d: Decimal| d.to_string()),
                bacteria.optimal_ph_min.map(|d: Decimal| d.to_string()),
                bacteria.optimal_ph_max.map(|d: Decimal| d.to_string()),
                bacteria.typical_dosage_grams_per_gallon.map(|d: Decimal| d.to_string()),
                bacteria.usage_notes, bacteria.flavor_profile, bacteria.best_suited_styles,
                bacteria.compatible_styles, bacteria.timing,
                bacteria.created_at, bacteria.updated_at,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<Bacteria> {
        conn.query_row(
            "SELECT id, name, bacteria_type, laboratory, product_code,
                optimal_temperature_min, optimal_temperature_max,
                optimal_ph_min, optimal_ph_max, typical_dosage_grams_per_gallon,
                usage_notes, flavor_profile, best_suited_styles, compatible_styles, timing,
                created_at, updated_at
             FROM bacteria WHERE id = ?1",
            rusqlite::params![id],
            Self::row_to_bacteria
        )
    }

    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<Bacteria>> {
        let query = match limit {
            Some(_) => "SELECT id, name, bacteria_type, laboratory, product_code,
                optimal_temperature_min, optimal_temperature_max,
                optimal_ph_min, optimal_ph_max, typical_dosage_grams_per_gallon,
                usage_notes, flavor_profile, best_suited_styles, compatible_styles, timing,
                created_at, updated_at FROM bacteria ORDER BY name ASC LIMIT ?1",
            None => "SELECT id, name, bacteria_type, laboratory, product_code,
                optimal_temperature_min, optimal_temperature_max,
                optimal_ph_min, optimal_ph_max, typical_dosage_grams_per_gallon,
                usage_notes, flavor_profile, best_suited_styles, compatible_styles, timing,
                created_at, updated_at FROM bacteria ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_bacteria)?
        } else {
            stmt.query_map([], Self::row_to_bacteria)?
        };
        rows.collect()
    }

    pub fn search(conn: &Connection, query: &str, limit: Option<i64>) -> SqliteResult<Vec<Bacteria>> {
        if query.trim().is_empty() { return Ok(Vec::new()); }

        let pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "SELECT id, name, bacteria_type, laboratory, product_code,
                optimal_temperature_min, optimal_temperature_max,
                optimal_ph_min, optimal_ph_max, typical_dosage_grams_per_gallon,
                usage_notes, flavor_profile, best_suited_styles, compatible_styles, timing,
                created_at, updated_at FROM bacteria
                WHERE name LIKE ?1 OR product_code LIKE ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT id, name, bacteria_type, laboratory, product_code,
                optimal_temperature_min, optimal_temperature_max,
                optimal_ph_min, optimal_ph_max, typical_dosage_grams_per_gallon,
                usage_notes, flavor_profile, best_suited_styles, compatible_styles, timing,
                created_at, updated_at FROM bacteria
                WHERE name LIKE ?1 OR product_code LIKE ?1 ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![pattern, lim], Self::row_to_bacteria)?
        } else {
            stmt.query_map(rusqlite::params![pattern], Self::row_to_bacteria)?
        };
        rows.collect()
    }

    pub fn get_by_type(conn: &Connection, bacteria_type: &str, limit: Option<i64>) -> SqliteResult<Vec<Bacteria>> {
        let sql = match limit {
            Some(_) => "SELECT id, name, bacteria_type, laboratory, product_code,
                optimal_temperature_min, optimal_temperature_max,
                optimal_ph_min, optimal_ph_max, typical_dosage_grams_per_gallon,
                usage_notes, flavor_profile, best_suited_styles, compatible_styles, timing,
                created_at, updated_at FROM bacteria
                WHERE bacteria_type = ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT id, name, bacteria_type, laboratory, product_code,
                optimal_temperature_min, optimal_temperature_max,
                optimal_ph_min, optimal_ph_max, typical_dosage_grams_per_gallon,
                usage_notes, flavor_profile, best_suited_styles, compatible_styles, timing,
                created_at, updated_at FROM bacteria
                WHERE bacteria_type = ?1 ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![bacteria_type, lim], Self::row_to_bacteria)?
        } else {
            stmt.query_map(rusqlite::params![bacteria_type], Self::row_to_bacteria)?
        };
        rows.collect()
    }

    pub fn update(conn: &Connection, bacteria: &Bacteria) -> SqliteResult<()> {
        bacteria.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        let rows = conn.execute(
            "UPDATE bacteria SET name = ?2, bacteria_type = ?3, laboratory = ?4, product_code = ?5,
                optimal_temperature_min = ?6, optimal_temperature_max = ?7,
                optimal_ph_min = ?8, optimal_ph_max = ?9, typical_dosage_grams_per_gallon = ?10,
                usage_notes = ?11, flavor_profile = ?12, best_suited_styles = ?13,
                compatible_styles = ?14, timing = ?15, updated_at = ?16 WHERE id = ?1",
            rusqlite::params![
                bacteria.id, bacteria.name, bacteria.bacteria_type, bacteria.laboratory, bacteria.product_code,
                bacteria.optimal_temperature_min.map(|d: Decimal| d.to_string()),
                bacteria.optimal_temperature_max.map(|d: Decimal| d.to_string()),
                bacteria.optimal_ph_min.map(|d: Decimal| d.to_string()),
                bacteria.optimal_ph_max.map(|d: Decimal| d.to_string()),
                bacteria.typical_dosage_grams_per_gallon.map(|d: Decimal| d.to_string()),
                bacteria.usage_notes, bacteria.flavor_profile, bacteria.best_suited_styles,
                bacteria.compatible_styles, bacteria.timing, bacteria.updated_at,
            ],
        )?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let rows = conn.execute("DELETE FROM bacteria WHERE id = ?1", rusqlite::params![id])?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM bacteria", [], |row| row.get(0))
    }

    pub fn count_by_type(conn: &Connection, bacteria_type: &str) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM bacteria WHERE bacteria_type = ?1",
                       rusqlite::params![bacteria_type], |row| row.get(0))
    }

    fn row_to_bacteria(row: &Row) -> SqliteResult<Bacteria> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }

        Ok(Bacteria {
            id: row.get(0)?,
            name: row.get(1)?,
            bacteria_type: row.get(2)?,
            laboratory: row.get(3)?,
            product_code: row.get(4)?,
            optimal_temperature_min: parse_decimal(row.get(5)?),
            optimal_temperature_max: parse_decimal(row.get(6)?),
            optimal_ph_min: parse_decimal(row.get(7)?),
            optimal_ph_max: parse_decimal(row.get(8)?),
            typical_dosage_grams_per_gallon: parse_decimal(row.get(9)?),
            usage_notes: row.get(10)?,
            flavor_profile: row.get(11)?,
            best_suited_styles: row.get(12)?,
            compatible_styles: row.get(13)?,
            timing: row.get(14)?,
            created_at: row.get(15)?,
            updated_at: row.get(16)?,
        })
    }
}