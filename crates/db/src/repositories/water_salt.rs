use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::models::water_salt::WaterSalt;

pub struct WaterSaltRepository<'conn> { conn: &'conn Connection, }

impl<'conn> WaterSaltRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    pub fn create(conn: &Connection, salt: &WaterSalt) -> SqliteResult<i64> {
        salt.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        conn.execute(
            "INSERT INTO water_salts (name, salt_type, chemical_formula, manufacturer,
                calcium_contribution, magnesium_contribution, sodium_contribution,
                chloride_contribution, sulfate_contribution, bicarbonate_contribution,
                typical_dosage_grams_per_gallon, solubility,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles,
                created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
            rusqlite::params![
                salt.name, salt.salt_type, salt.chemical_formula, salt.manufacturer,
                salt.calcium_contribution.map(|d: Decimal| d.to_string()),
                salt.magnesium_contribution.map(|d: Decimal| d.to_string()),
                salt.sodium_contribution.map(|d: Decimal| d.to_string()),
                salt.chloride_contribution.map(|d: Decimal| d.to_string()),
                salt.sulfate_contribution.map(|d: Decimal| d.to_string()),
                salt.bicarbonate_contribution.map(|d: Decimal| d.to_string()),
                salt.typical_dosage_grams_per_gallon.map(|d: Decimal| d.to_string()),
                salt.solubility,
                salt.usage_notes, salt.flavor_impact, salt.best_suited_styles,
                salt.compatible_styles, salt.created_at, salt.updated_at,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<WaterSalt> {
        conn.query_row(
            "SELECT id, name, salt_type, chemical_formula, manufacturer,
                calcium_contribution, magnesium_contribution, sodium_contribution,
                chloride_contribution, sulfate_contribution, bicarbonate_contribution,
                typical_dosage_grams_per_gallon, solubility,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles,
                created_at, updated_at
             FROM water_salts WHERE id = ?1",
            rusqlite::params![id],
            Self::row_to_salt
        )
    }

    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<WaterSalt>> {
        let query = match limit {
            Some(_) => "SELECT id, name, salt_type, chemical_formula, manufacturer,
                calcium_contribution, magnesium_contribution, sodium_contribution,
                chloride_contribution, sulfate_contribution, bicarbonate_contribution,
                typical_dosage_grams_per_gallon, solubility,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles,
                created_at, updated_at FROM water_salts ORDER BY name ASC LIMIT ?1",
            None => "SELECT id, name, salt_type, chemical_formula, manufacturer,
                calcium_contribution, magnesium_contribution, sodium_contribution,
                chloride_contribution, sulfate_contribution, bicarbonate_contribution,
                typical_dosage_grams_per_gallon, solubility,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles,
                created_at, updated_at FROM water_salts ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_salt)?
        } else {
            stmt.query_map([], Self::row_to_salt)?
        };
        rows.collect()
    }

    pub fn search(conn: &Connection, query: &str, limit: Option<i64>) -> SqliteResult<Vec<WaterSalt>> {
        if query.trim().is_empty() { return Ok(Vec::new()); }

        let pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "SELECT id, name, salt_type, chemical_formula, manufacturer,
                calcium_contribution, magnesium_contribution, sodium_contribution,
                chloride_contribution, sulfate_contribution, bicarbonate_contribution,
                typical_dosage_grams_per_gallon, solubility,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles,
                created_at, updated_at FROM water_salts
                WHERE name LIKE ?1 OR chemical_formula LIKE ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT id, name, salt_type, chemical_formula, manufacturer,
                calcium_contribution, magnesium_contribution, sodium_contribution,
                chloride_contribution, sulfate_contribution, bicarbonate_contribution,
                typical_dosage_grams_per_gallon, solubility,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles,
                created_at, updated_at FROM water_salts
                WHERE name LIKE ?1 OR chemical_formula LIKE ?1 ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![pattern, lim], Self::row_to_salt)?
        } else {
            stmt.query_map(rusqlite::params![pattern], Self::row_to_salt)?
        };
        rows.collect()
    }

    pub fn get_by_type(conn: &Connection, salt_type: &str, limit: Option<i64>) -> SqliteResult<Vec<WaterSalt>> {
        let sql = match limit {
            Some(_) => "SELECT id, name, salt_type, chemical_formula, manufacturer,
                calcium_contribution, magnesium_contribution, sodium_contribution,
                chloride_contribution, sulfate_contribution, bicarbonate_contribution,
                typical_dosage_grams_per_gallon, solubility,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles,
                created_at, updated_at FROM water_salts
                WHERE salt_type = ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT id, name, salt_type, chemical_formula, manufacturer,
                calcium_contribution, magnesium_contribution, sodium_contribution,
                chloride_contribution, sulfate_contribution, bicarbonate_contribution,
                typical_dosage_grams_per_gallon, solubility,
                usage_notes, flavor_impact, best_suited_styles, compatible_styles,
                created_at, updated_at FROM water_salts
                WHERE salt_type = ?1 ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![salt_type, lim], Self::row_to_salt)?
        } else {
            stmt.query_map(rusqlite::params![salt_type], Self::row_to_salt)?
        };
        rows.collect()
    }

    pub fn update(conn: &Connection, salt: &WaterSalt) -> SqliteResult<()> {
        salt.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        let rows = conn.execute(
            "UPDATE water_salts SET name = ?2, salt_type = ?3, chemical_formula = ?4, manufacturer = ?5,
                calcium_contribution = ?6, magnesium_contribution = ?7, sodium_contribution = ?8,
                chloride_contribution = ?9, sulfate_contribution = ?10, bicarbonate_contribution = ?11,
                typical_dosage_grams_per_gallon = ?12, solubility = ?13,
                usage_notes = ?14, flavor_impact = ?15, best_suited_styles = ?16,
                compatible_styles = ?17, updated_at = ?18 WHERE id = ?1",
            rusqlite::params![
                salt.id, salt.name, salt.salt_type, salt.chemical_formula, salt.manufacturer,
                salt.calcium_contribution.map(|d: Decimal| d.to_string()),
                salt.magnesium_contribution.map(|d: Decimal| d.to_string()),
                salt.sodium_contribution.map(|d: Decimal| d.to_string()),
                salt.chloride_contribution.map(|d: Decimal| d.to_string()),
                salt.sulfate_contribution.map(|d: Decimal| d.to_string()),
                salt.bicarbonate_contribution.map(|d: Decimal| d.to_string()),
                salt.typical_dosage_grams_per_gallon.map(|d: Decimal| d.to_string()),
                salt.solubility,
                salt.usage_notes, salt.flavor_impact, salt.best_suited_styles,
                salt.compatible_styles, salt.updated_at,
            ],
        )?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let rows = conn.execute("DELETE FROM water_salts WHERE id = ?1", rusqlite::params![id])?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM water_salts", [], |row| row.get(0))
    }

    pub fn count_by_type(conn: &Connection, salt_type: &str) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM water_salts WHERE salt_type = ?1",
                       rusqlite::params![salt_type], |row| row.get(0))
    }

    fn row_to_salt(row: &Row) -> SqliteResult<WaterSalt> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }

        Ok(WaterSalt {
            id: row.get(0)?,
            name: row.get(1)?,
            salt_type: row.get(2)?,
            chemical_formula: row.get(3)?,
            manufacturer: row.get(4)?,
            calcium_contribution: parse_decimal(row.get(5)?),
            magnesium_contribution: parse_decimal(row.get(6)?),
            sodium_contribution: parse_decimal(row.get(7)?),
            chloride_contribution: parse_decimal(row.get(8)?),
            sulfate_contribution: parse_decimal(row.get(9)?),
            bicarbonate_contribution: parse_decimal(row.get(10)?),
            typical_dosage_grams_per_gallon: parse_decimal(row.get(11)?),
            solubility: row.get(12)?,
            usage_notes: row.get(13)?,
            flavor_impact: row.get(14)?,
            best_suited_styles: row.get(15)?,
            compatible_styles: row.get(16)?,
            created_at: row.get(17)?,
            updated_at: row.get(18)?,
        })
    }
}