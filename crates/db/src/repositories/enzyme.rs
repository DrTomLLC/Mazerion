use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::models::enzyme::Enzyme;

pub struct EnzymeRepository<'conn> { conn: &'conn Connection, }

impl<'conn> EnzymeRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    pub fn create(conn: &Connection, enzyme: &Enzyme) -> SqliteResult<i64> {
        enzyme.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        conn.execute(
            "INSERT INTO enzymes (name, enzyme_type, manufacturer,
                optimal_temperature_min, optimal_temperature_max,
                optimal_ph_min, optimal_ph_max, typical_dosage_grams_per_gallon,
                usage_notes, best_suited_styles, compatible_styles, timing, target_compounds,
                created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            rusqlite::params![
                enzyme.name, enzyme.enzyme_type, enzyme.manufacturer,
                enzyme.optimal_temperature_min.map(|d: Decimal| d.to_string()),
                enzyme.optimal_temperature_max.map(|d: Decimal| d.to_string()),
                enzyme.optimal_ph_min.map(|d: Decimal| d.to_string()),
                enzyme.optimal_ph_max.map(|d: Decimal| d.to_string()),
                enzyme.typical_dosage_grams_per_gallon.map(|d: Decimal| d.to_string()),
                enzyme.usage_notes, enzyme.best_suited_styles, enzyme.compatible_styles,
                enzyme.timing, enzyme.target_compounds,
                enzyme.created_at, enzyme.updated_at,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<Enzyme> {
        conn.query_row(
            "SELECT id, name, enzyme_type, manufacturer,
                optimal_temperature_min, optimal_temperature_max,
                optimal_ph_min, optimal_ph_max, typical_dosage_grams_per_gallon,
                usage_notes, best_suited_styles, compatible_styles, timing, target_compounds,
                created_at, updated_at
             FROM enzymes WHERE id = ?1",
            rusqlite::params![id],
            Self::row_to_enzyme
        )
    }

    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<Enzyme>> {
        let query = match limit {
            Some(_) => "SELECT id, name, enzyme_type, manufacturer,
                optimal_temperature_min, optimal_temperature_max,
                optimal_ph_min, optimal_ph_max, typical_dosage_grams_per_gallon,
                usage_notes, best_suited_styles, compatible_styles, timing, target_compounds,
                created_at, updated_at FROM enzymes ORDER BY name ASC LIMIT ?1",
            None => "SELECT id, name, enzyme_type, manufacturer,
                optimal_temperature_min, optimal_temperature_max,
                optimal_ph_min, optimal_ph_max, typical_dosage_grams_per_gallon,
                usage_notes, best_suited_styles, compatible_styles, timing, target_compounds,
                created_at, updated_at FROM enzymes ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_enzyme)?
        } else {
            stmt.query_map([], Self::row_to_enzyme)?
        };
        rows.collect()
    }

    pub fn search(conn: &Connection, query: &str, limit: Option<i64>) -> SqliteResult<Vec<Enzyme>> {
        if query.trim().is_empty() { return Ok(Vec::new()); }

        let pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "SELECT id, name, enzyme_type, manufacturer,
                optimal_temperature_min, optimal_temperature_max,
                optimal_ph_min, optimal_ph_max, typical_dosage_grams_per_gallon,
                usage_notes, best_suited_styles, compatible_styles, timing, target_compounds,
                created_at, updated_at FROM enzymes
                WHERE name LIKE ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT id, name, enzyme_type, manufacturer,
                optimal_temperature_min, optimal_temperature_max,
                optimal_ph_min, optimal_ph_max, typical_dosage_grams_per_gallon,
                usage_notes, best_suited_styles, compatible_styles, timing, target_compounds,
                created_at, updated_at FROM enzymes
                WHERE name LIKE ?1 ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![pattern, lim], Self::row_to_enzyme)?
        } else {
            stmt.query_map(rusqlite::params![pattern], Self::row_to_enzyme)?
        };
        rows.collect()
    }

    pub fn get_by_type(conn: &Connection, enzyme_type: &str, limit: Option<i64>) -> SqliteResult<Vec<Enzyme>> {
        let sql = match limit {
            Some(_) => "SELECT id, name, enzyme_type, manufacturer,
                optimal_temperature_min, optimal_temperature_max,
                optimal_ph_min, optimal_ph_max, typical_dosage_grams_per_gallon,
                usage_notes, best_suited_styles, compatible_styles, timing, target_compounds,
                created_at, updated_at FROM enzymes
                WHERE enzyme_type = ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT id, name, enzyme_type, manufacturer,
                optimal_temperature_min, optimal_temperature_max,
                optimal_ph_min, optimal_ph_max, typical_dosage_grams_per_gallon,
                usage_notes, best_suited_styles, compatible_styles, timing, target_compounds,
                created_at, updated_at FROM enzymes
                WHERE enzyme_type = ?1 ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![enzyme_type, lim], Self::row_to_enzyme)?
        } else {
            stmt.query_map(rusqlite::params![enzyme_type], Self::row_to_enzyme)?
        };
        rows.collect()
    }

    pub fn update(conn: &Connection, enzyme: &Enzyme) -> SqliteResult<()> {
        enzyme.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        let rows = conn.execute(
            "UPDATE enzymes SET name = ?2, enzyme_type = ?3, manufacturer = ?4,
                optimal_temperature_min = ?5, optimal_temperature_max = ?6,
                optimal_ph_min = ?7, optimal_ph_max = ?8, typical_dosage_grams_per_gallon = ?9,
                usage_notes = ?10, best_suited_styles = ?11, compatible_styles = ?12,
                timing = ?13, target_compounds = ?14, updated_at = ?15 WHERE id = ?1",
            rusqlite::params![
                enzyme.id, enzyme.name, enzyme.enzyme_type, enzyme.manufacturer,
                enzyme.optimal_temperature_min.map(|d: Decimal| d.to_string()),
                enzyme.optimal_temperature_max.map(|d: Decimal| d.to_string()),
                enzyme.optimal_ph_min.map(|d: Decimal| d.to_string()),
                enzyme.optimal_ph_max.map(|d: Decimal| d.to_string()),
                enzyme.typical_dosage_grams_per_gallon.map(|d: Decimal| d.to_string()),
                enzyme.usage_notes, enzyme.best_suited_styles, enzyme.compatible_styles,
                enzyme.timing, enzyme.target_compounds, enzyme.updated_at,
            ],
        )?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let rows = conn.execute("DELETE FROM enzymes WHERE id = ?1", rusqlite::params![id])?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM enzymes", [], |row| row.get(0))
    }

    pub fn count_by_type(conn: &Connection, enzyme_type: &str) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM enzymes WHERE enzyme_type = ?1",
                       rusqlite::params![enzyme_type], |row| row.get(0))
    }

    fn row_to_enzyme(row: &Row) -> SqliteResult<Enzyme> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }

        Ok(Enzyme {
            id: row.get(0)?,
            name: row.get(1)?,
            enzyme_type: row.get(2)?,
            manufacturer: row.get(3)?,
            optimal_temperature_min: parse_decimal(row.get(4)?),
            optimal_temperature_max: parse_decimal(row.get(5)?),
            optimal_ph_min: parse_decimal(row.get(6)?),
            optimal_ph_max: parse_decimal(row.get(7)?),
            typical_dosage_grams_per_gallon: parse_decimal(row.get(8)?),
            usage_notes: row.get(9)?,
            best_suited_styles: row.get(10)?,
            compatible_styles: row.get(11)?,
            timing: row.get(12)?,
            target_compounds: row.get(13)?,
            created_at: row.get(14)?,
            updated_at: row.get(15)?,
        })
    }
}