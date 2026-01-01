use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::models::acid::Acid;

pub struct AcidRepository<'conn> { conn: &'conn Connection, }

impl<'conn> AcidRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    pub fn create(conn: &Connection, acid: &Acid) -> SqliteResult<i64> {
        acid.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        conn.execute(
            "INSERT INTO acids (name, acid_type, chemical_formula, manufacturer,
                concentration, ph_adjustment_per_ml, typical_dosage_ml_per_gallon,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles, safety_notes,
                created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            rusqlite::params![
                acid.name, acid.acid_type, acid.chemical_formula, acid.manufacturer,
                acid.concentration.map(|d: Decimal| d.to_string()),
                acid.ph_adjustment_per_ml.map(|d: Decimal| d.to_string()),
                acid.typical_dosage_ml_per_gallon.map(|d: Decimal| d.to_string()),
                acid.usage_notes, acid.flavor_impact, acid.best_suited_styles,
                acid.compatible_styles, acid.safety_notes,
                acid.created_at, acid.updated_at,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<Acid> {
        conn.query_row(
            "SELECT id, name, acid_type, chemical_formula, manufacturer,
                concentration, ph_adjustment_per_ml, typical_dosage_ml_per_gallon,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles, safety_notes,
                created_at, updated_at
             FROM acids WHERE id = ?1",
            rusqlite::params![id],
            Self::row_to_acid
        )
    }

    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<Acid>> {
        let query = match limit {
            Some(_) => "SELECT id, name, acid_type, chemical_formula, manufacturer,
                concentration, ph_adjustment_per_ml, typical_dosage_ml_per_gallon,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles, safety_notes,
                created_at, updated_at FROM acids ORDER BY name ASC LIMIT ?1",
            None => "SELECT id, name, acid_type, chemical_formula, manufacturer,
                concentration, ph_adjustment_per_ml, typical_dosage_ml_per_gallon,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles, safety_notes,
                created_at, updated_at FROM acids ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_acid)?
        } else {
            stmt.query_map([], Self::row_to_acid)?
        };
        rows.collect()
    }

    pub fn search(conn: &Connection, query: &str, limit: Option<i64>) -> SqliteResult<Vec<Acid>> {
        if query.trim().is_empty() { return Ok(Vec::new()); }

        let pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "SELECT id, name, acid_type, chemical_formula, manufacturer,
                concentration, ph_adjustment_per_ml, typical_dosage_ml_per_gallon,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles, safety_notes,
                created_at, updated_at FROM acids 
                WHERE name LIKE ?1 OR chemical_formula LIKE ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT id, name, acid_type, chemical_formula, manufacturer,
                concentration, ph_adjustment_per_ml, typical_dosage_ml_per_gallon,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles, safety_notes,
                created_at, updated_at FROM acids 
                WHERE name LIKE ?1 OR chemical_formula LIKE ?1 ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![pattern, lim], Self::row_to_acid)?
        } else {
            stmt.query_map(rusqlite::params![pattern], Self::row_to_acid)?
        };
        rows.collect()
    }

    pub fn get_by_type(conn: &Connection, acid_type: &str, limit: Option<i64>) -> SqliteResult<Vec<Acid>> {
        let sql = match limit {
            Some(_) => "SELECT id, name, acid_type, chemical_formula, manufacturer,
                concentration, ph_adjustment_per_ml, typical_dosage_ml_per_gallon,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles, safety_notes,
                created_at, updated_at FROM acids 
                WHERE acid_type = ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT id, name, acid_type, chemical_formula, manufacturer,
                concentration, ph_adjustment_per_ml, typical_dosage_ml_per_gallon,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles, safety_notes,
                created_at, updated_at FROM acids 
                WHERE acid_type = ?1 ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![acid_type, lim], Self::row_to_acid)?
        } else {
            stmt.query_map(rusqlite::params![acid_type], Self::row_to_acid)?
        };
        rows.collect()
    }

    pub fn update(conn: &Connection, acid: &Acid) -> SqliteResult<()> {
        acid.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        let rows = conn.execute(
            "UPDATE acids SET name = ?2, acid_type = ?3, chemical_formula = ?4, manufacturer = ?5,
                concentration = ?6, ph_adjustment_per_ml = ?7, typical_dosage_ml_per_gallon = ?8,
                usage_notes = ?9, flavor_impact = ?10, best_suited_styles = ?11,
                compatible_styles = ?12, safety_notes = ?13, updated_at = ?14 WHERE id = ?1",
            rusqlite::params![
                acid.id, acid.name, acid.acid_type, acid.chemical_formula, acid.manufacturer,
                acid.concentration.map(|d: Decimal| d.to_string()),
                acid.ph_adjustment_per_ml.map(|d: Decimal| d.to_string()),
                acid.typical_dosage_ml_per_gallon.map(|d: Decimal| d.to_string()),
                acid.usage_notes, acid.flavor_impact, acid.best_suited_styles,
                acid.compatible_styles, acid.safety_notes, acid.updated_at,
            ],
        )?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let rows = conn.execute("DELETE FROM acids WHERE id = ?1", rusqlite::params![id])?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM acids", [], |row| row.get(0))
    }

    pub fn count_by_type(conn: &Connection, acid_type: &str) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM acids WHERE acid_type = ?1",
                       rusqlite::params![acid_type], |row| row.get(0))
    }

    fn row_to_acid(row: &Row) -> SqliteResult<Acid> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }

        Ok(Acid {
            id: row.get(0)?,
            name: row.get(1)?,
            acid_type: row.get(2)?,
            chemical_formula: row.get(3)?,
            manufacturer: row.get(4)?,
            concentration: parse_decimal(row.get(5)?),
            ph_adjustment_per_ml: parse_decimal(row.get(6)?),
            typical_dosage_ml_per_gallon: parse_decimal(row.get(7)?),
            usage_notes: row.get(8)?,
            flavor_impact: row.get(9)?,
            best_suited_styles: row.get(10)?,
            compatible_styles: row.get(11)?,
            safety_notes: row.get(12)?,
            created_at: row.get(13)?,
            updated_at: row.get(14)?,
        })
    }
}