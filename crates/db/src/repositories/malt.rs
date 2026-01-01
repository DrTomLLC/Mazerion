use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::models::malt::Malt;

pub struct MaltRepository<'conn> { conn: &'conn Connection, }

impl<'conn> MaltRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    pub fn create(conn: &Connection, malt: &Malt) -> SqliteResult<i64> {
        malt.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        let query = "
            INSERT INTO malts (
                name, maltster, origin, grain_type, color_lovibond,
                max_percentage, extract_potential, diastatic_power,
                moisture_content, protein_content, flavor_profile,
                aroma_profile, typical_usage, substitutes,
                best_suited_styles, usage_notes, sensory_notes,
                requires_mashing, compatible_styles, created_at, updated_at
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21
            )
        ";

        conn.execute(
            query,
            rusqlite::params![
                malt.name, malt.maltster, malt.origin, malt.grain_type,
                malt.color_lovibond.map(|d: Decimal| d.to_string()),
                malt.max_percentage.map(|d: Decimal| d.to_string()),
                malt.extract_potential.map(|d: Decimal| d.to_string()),
                malt.diastatic_power.map(|d: Decimal| d.to_string()),
                malt.moisture_content.map(|d: Decimal| d.to_string()),
                malt.protein_content.map(|d: Decimal| d.to_string()),
                malt.flavor_profile, malt.aroma_profile,
                malt.typical_usage, malt.substitutes,
                malt.best_suited_styles, malt.usage_notes,
                malt.sensory_notes, malt.requires_mashing,
                malt.compatible_styles, malt.created_at, malt.updated_at,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<Malt> {
        let query = "
            SELECT 
                id, name, maltster, origin, grain_type, color_lovibond,
                max_percentage, extract_potential, diastatic_power,
                moisture_content, protein_content, flavor_profile,
                aroma_profile, typical_usage, substitutes,
                best_suited_styles, usage_notes, sensory_notes,
                requires_mashing, compatible_styles, created_at, updated_at
            FROM malts
            WHERE id = ?1
        ";

        conn.query_row(query, rusqlite::params![id], Self::row_to_malt)
    }

    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<Malt>> {
        let query = match limit {
            Some(_) => "
                SELECT 
                    id, name, maltster, origin, grain_type, color_lovibond,
                    max_percentage, extract_potential, diastatic_power,
                    moisture_content, protein_content, flavor_profile,
                    aroma_profile, typical_usage, substitutes,
                    best_suited_styles, usage_notes, sensory_notes,
                    requires_mashing, compatible_styles, created_at, updated_at
                FROM malts
                ORDER BY name ASC
                LIMIT ?1
            ",
            None => "
                SELECT 
                    id, name, maltster, origin, grain_type, color_lovibond,
                    max_percentage, extract_potential, diastatic_power,
                    moisture_content, protein_content, flavor_profile,
                    aroma_profile, typical_usage, substitutes,
                    best_suited_styles, usage_notes, sensory_notes,
                    requires_mashing, compatible_styles, created_at, updated_at
                FROM malts
                ORDER BY name ASC
            ",
        };

        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_malt)?
        } else {
            stmt.query_map([], Self::row_to_malt)?
        };

        rows.collect()
    }

    pub fn search(conn: &Connection, query: &str, limit: Option<i64>) -> SqliteResult<Vec<Malt>> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        let search_pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "
                SELECT 
                    id, name, maltster, origin, grain_type, color_lovibond,
                    max_percentage, extract_potential, diastatic_power,
                    moisture_content, protein_content, flavor_profile,
                    aroma_profile, typical_usage, substitutes,
                    best_suited_styles, usage_notes, sensory_notes,
                    requires_mashing, compatible_styles, created_at, updated_at
                FROM malts
                WHERE name LIKE ?1 OR maltster LIKE ?1 OR origin LIKE ?1
                ORDER BY name ASC
                LIMIT ?2
            ",
            None => "
                SELECT 
                    id, name, maltster, origin, grain_type, color_lovibond,
                    max_percentage, extract_potential, diastatic_power,
                    moisture_content, protein_content, flavor_profile,
                    aroma_profile, typical_usage, substitutes,
                    best_suited_styles, usage_notes, sensory_notes,
                    requires_mashing, compatible_styles, created_at, updated_at
                FROM malts
                WHERE name LIKE ?1 OR maltster LIKE ?1 OR origin LIKE ?1
                ORDER BY name ASC
            ",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![search_pattern, lim], Self::row_to_malt)?
        } else {
            stmt.query_map(rusqlite::params![search_pattern], Self::row_to_malt)?
        };

        rows.collect()
    }

    pub fn get_by_type(conn: &Connection, grain_type: &str, limit: Option<i64>) -> SqliteResult<Vec<Malt>> {
        let sql = match limit {
            Some(_) => "
                SELECT 
                    id, name, maltster, origin, grain_type, color_lovibond,
                    max_percentage, extract_potential, diastatic_power,
                    moisture_content, protein_content, flavor_profile,
                    aroma_profile, typical_usage, substitutes,
                    best_suited_styles, usage_notes, sensory_notes,
                    requires_mashing, compatible_styles, created_at, updated_at
                FROM malts
                WHERE grain_type = ?1
                ORDER BY name ASC
                LIMIT ?2
            ",
            None => "
                SELECT 
                    id, name, maltster, origin, grain_type, color_lovibond,
                    max_percentage, extract_potential, diastatic_power,
                    moisture_content, protein_content, flavor_profile,
                    aroma_profile, typical_usage, substitutes,
                    best_suited_styles, usage_notes, sensory_notes,
                    requires_mashing, compatible_styles, created_at, updated_at
                FROM malts
                WHERE grain_type = ?1
                ORDER BY name ASC
            ",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![grain_type, lim], Self::row_to_malt)?
        } else {
            stmt.query_map(rusqlite::params![grain_type], Self::row_to_malt)?
        };

        rows.collect()
    }

    pub fn get_by_maltster(conn: &Connection, maltster: &str, limit: Option<i64>) -> SqliteResult<Vec<Malt>> {
        let sql = match limit {
            Some(_) => "
                SELECT 
                    id, name, maltster, origin, grain_type, color_lovibond,
                    max_percentage, extract_potential, diastatic_power,
                    moisture_content, protein_content, flavor_profile,
                    aroma_profile, typical_usage, substitutes,
                    best_suited_styles, usage_notes, sensory_notes,
                    requires_mashing, compatible_styles, created_at, updated_at
                FROM malts
                WHERE maltster = ?1
                ORDER BY name ASC
                LIMIT ?2
            ",
            None => "
                SELECT 
                    id, name, maltster, origin, grain_type, color_lovibond,
                    max_percentage, extract_potential, diastatic_power,
                    moisture_content, protein_content, flavor_profile,
                    aroma_profile, typical_usage, substitutes,
                    best_suited_styles, usage_notes, sensory_notes,
                    requires_mashing, compatible_styles, created_at, updated_at
                FROM malts
                WHERE maltster = ?1
                ORDER BY name ASC
            ",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![maltster, lim], Self::row_to_malt)?
        } else {
            stmt.query_map(rusqlite::params![maltster], Self::row_to_malt)?
        };

        rows.collect()
    }

    pub fn update(conn: &Connection, malt: &Malt) -> SqliteResult<()> {
        malt.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        let query = "
            UPDATE malts SET
                name = ?2, maltster = ?3, origin = ?4, grain_type = ?5, color_lovibond = ?6,
                max_percentage = ?7, extract_potential = ?8, diastatic_power = ?9,
                moisture_content = ?10, protein_content = ?11, flavor_profile = ?12,
                aroma_profile = ?13, typical_usage = ?14, substitutes = ?15,
                best_suited_styles = ?16, usage_notes = ?17, sensory_notes = ?18,
                requires_mashing = ?19, compatible_styles = ?20, updated_at = ?21
            WHERE id = ?1
        ";

        let rows_affected = conn.execute(
            query,
            rusqlite::params![
                malt.id, malt.name, malt.maltster, malt.origin, malt.grain_type,
                malt.color_lovibond.map(|d: Decimal| d.to_string()),
                malt.max_percentage.map(|d: Decimal| d.to_string()),
                malt.extract_potential.map(|d: Decimal| d.to_string()),
                malt.diastatic_power.map(|d: Decimal| d.to_string()),
                malt.moisture_content.map(|d: Decimal| d.to_string()),
                malt.protein_content.map(|d: Decimal| d.to_string()),
                malt.flavor_profile, malt.aroma_profile,
                malt.typical_usage, malt.substitutes,
                malt.best_suited_styles, malt.usage_notes,
                malt.sensory_notes, malt.requires_mashing,
                malt.compatible_styles, malt.updated_at,
            ],
        )?;

        if rows_affected == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let query = "DELETE FROM malts WHERE id = ?1";
        let rows_affected = conn.execute(query, rusqlite::params![id])?;

        if rows_affected == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        Ok(())
    }

    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        let query = "SELECT COUNT(*) FROM malts";
        conn.query_row(query, [], |row| row.get(0))
    }

    pub fn count_by_type(conn: &Connection, grain_type: &str) -> SqliteResult<i64> {
        let query = "SELECT COUNT(*) FROM malts WHERE grain_type = ?1";
        conn.query_row(query, rusqlite::params![grain_type], |row| row.get(0))
    }

    pub fn count_by_maltster(conn: &Connection, maltster: &str) -> SqliteResult<i64> {
        let query = "SELECT COUNT(*) FROM malts WHERE maltster = ?1";
        conn.query_row(query, rusqlite::params![maltster], |row| row.get(0))
    }

    fn row_to_malt(row: &Row) -> SqliteResult<Malt> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }

        Ok(Malt {
            id: row.get(0)?,
            name: row.get(1)?,
            maltster: row.get(2)?,
            origin: row.get(3)?,
            grain_type: row.get(4)?,
            color_lovibond: parse_decimal(row.get(5)?),
            max_percentage: parse_decimal(row.get(6)?),
            extract_potential: parse_decimal(row.get(7)?),
            diastatic_power: parse_decimal(row.get(8)?),
            moisture_content: parse_decimal(row.get(9)?),
            protein_content: parse_decimal(row.get(10)?),
            flavor_profile: row.get(11)?,
            aroma_profile: row.get(12)?,
            typical_usage: row.get(13)?,
            substitutes: row.get(14)?,
            best_suited_styles: row.get(15)?,
            usage_notes: row.get(16)?,
            sensory_notes: row.get(17)?,
            requires_mashing: row.get(18)?,
            compatible_styles: row.get(19)?,
            created_at: row.get(20)?,
            updated_at: row.get(21)?,
        })
    }
}