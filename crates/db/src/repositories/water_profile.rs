use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::models::water_profile::WaterProfile;

pub struct WaterProfileRepository<'conn> { conn: &'conn Connection, }

impl<'conn> WaterProfileRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    pub fn create(conn: &Connection, profile: &WaterProfile) -> SqliteResult<i64> {
        profile.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        conn.execute(
            "INSERT INTO water_profiles (name, water_type, source, location,
                calcium, magnesium, sodium, chloride, sulfate, bicarbonate,
                ph_level, total_dissolved_solids, hardness,
                best_suited_styles, usage_notes, flavor_impact, compatible_styles,
                created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)",
            rusqlite::params![
                profile.name, profile.water_type, profile.source, profile.location,
                profile.calcium.map(|d: Decimal| d.to_string()),
                profile.magnesium.map(|d: Decimal| d.to_string()),
                profile.sodium.map(|d: Decimal| d.to_string()),
                profile.chloride.map(|d: Decimal| d.to_string()),
                profile.sulfate.map(|d: Decimal| d.to_string()),
                profile.bicarbonate.map(|d: Decimal| d.to_string()),
                profile.ph_level.map(|d: Decimal| d.to_string()),
                profile.total_dissolved_solids.map(|d: Decimal| d.to_string()),
                profile.hardness.map(|d: Decimal| d.to_string()),
                profile.best_suited_styles, profile.usage_notes, profile.flavor_impact,
                profile.compatible_styles, profile.created_at, profile.updated_at,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<WaterProfile> {
        conn.query_row(
            "SELECT id, name, water_type, source, location,
                calcium, magnesium, sodium, chloride, sulfate, bicarbonate,
                ph_level, total_dissolved_solids, hardness,
                best_suited_styles, usage_notes, flavor_impact, compatible_styles,
                created_at, updated_at
             FROM water_profiles WHERE id = ?1",
            rusqlite::params![id],
            Self::row_to_profile
        )
    }

    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<WaterProfile>> {
        let query = match limit {
            Some(_) => "SELECT id, name, water_type, source, location,
                calcium, magnesium, sodium, chloride, sulfate, bicarbonate,
                ph_level, total_dissolved_solids, hardness,
                best_suited_styles, usage_notes, flavor_impact, compatible_styles,
                created_at, updated_at FROM water_profiles ORDER BY name ASC LIMIT ?1",
            None => "SELECT id, name, water_type, source, location,
                calcium, magnesium, sodium, chloride, sulfate, bicarbonate,
                ph_level, total_dissolved_solids, hardness,
                best_suited_styles, usage_notes, flavor_impact, compatible_styles,
                created_at, updated_at FROM water_profiles ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_profile)?
        } else {
            stmt.query_map([], Self::row_to_profile)?
        };
        rows.collect()
    }

    pub fn search(conn: &Connection, query: &str, limit: Option<i64>) -> SqliteResult<Vec<WaterProfile>> {
        if query.trim().is_empty() { return Ok(Vec::new()); }

        let pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "SELECT id, name, water_type, source, location,
                calcium, magnesium, sodium, chloride, sulfate, bicarbonate,
                ph_level, total_dissolved_solids, hardness,
                best_suited_styles, usage_notes, flavor_impact, compatible_styles,
                created_at, updated_at FROM water_profiles
                WHERE name LIKE ?1 OR location LIKE ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT id, name, water_type, source, location,
                calcium, magnesium, sodium, chloride, sulfate, bicarbonate,
                ph_level, total_dissolved_solids, hardness,
                best_suited_styles, usage_notes, flavor_impact, compatible_styles,
                created_at, updated_at FROM water_profiles
                WHERE name LIKE ?1 OR location LIKE ?1 ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![pattern, lim], Self::row_to_profile)?
        } else {
            stmt.query_map(rusqlite::params![pattern], Self::row_to_profile)?
        };
        rows.collect()
    }

    pub fn get_by_type(conn: &Connection, water_type: &str, limit: Option<i64>) -> SqliteResult<Vec<WaterProfile>> {
        let sql = match limit {
            Some(_) => "SELECT id, name, water_type, source, location,
                calcium, magnesium, sodium, chloride, sulfate, bicarbonate,
                ph_level, total_dissolved_solids, hardness,
                best_suited_styles, usage_notes, flavor_impact, compatible_styles,
                created_at, updated_at FROM water_profiles
                WHERE water_type = ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT id, name, water_type, source, location,
                calcium, magnesium, sodium, chloride, sulfate, bicarbonate,
                ph_level, total_dissolved_solids, hardness,
                best_suited_styles, usage_notes, flavor_impact, compatible_styles,
                created_at, updated_at FROM water_profiles
                WHERE water_type = ?1 ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![water_type, lim], Self::row_to_profile)?
        } else {
            stmt.query_map(rusqlite::params![water_type], Self::row_to_profile)?
        };
        rows.collect()
    }

    pub fn update(conn: &Connection, profile: &WaterProfile) -> SqliteResult<()> {
        profile.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        let rows = conn.execute(
            "UPDATE water_profiles SET name = ?2, water_type = ?3, source = ?4, location = ?5,
                calcium = ?6, magnesium = ?7, sodium = ?8, chloride = ?9, sulfate = ?10, bicarbonate = ?11,
                ph_level = ?12, total_dissolved_solids = ?13, hardness = ?14,
                best_suited_styles = ?15, usage_notes = ?16, flavor_impact = ?17,
                compatible_styles = ?18, updated_at = ?19 WHERE id = ?1",
            rusqlite::params![
                profile.id, profile.name, profile.water_type, profile.source, profile.location,
                profile.calcium.map(|d: Decimal| d.to_string()),
                profile.magnesium.map(|d: Decimal| d.to_string()),
                profile.sodium.map(|d: Decimal| d.to_string()),
                profile.chloride.map(|d: Decimal| d.to_string()),
                profile.sulfate.map(|d: Decimal| d.to_string()),
                profile.bicarbonate.map(|d: Decimal| d.to_string()),
                profile.ph_level.map(|d: Decimal| d.to_string()),
                profile.total_dissolved_solids.map(|d: Decimal| d.to_string()),
                profile.hardness.map(|d: Decimal| d.to_string()),
                profile.best_suited_styles, profile.usage_notes, profile.flavor_impact,
                profile.compatible_styles, profile.updated_at,
            ],
        )?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let rows = conn.execute("DELETE FROM water_profiles WHERE id = ?1", rusqlite::params![id])?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM water_profiles", [], |row| row.get(0))
    }

    pub fn count_by_type(conn: &Connection, water_type: &str) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM water_profiles WHERE water_type = ?1",
                       rusqlite::params![water_type], |row| row.get(0))
    }

    fn row_to_profile(row: &Row) -> SqliteResult<WaterProfile> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }

        Ok(WaterProfile {
            id: row.get(0)?,
            name: row.get(1)?,
            water_type: row.get(2)?,
            source: row.get(3)?,
            location: row.get(4)?,
            calcium: parse_decimal(row.get(5)?),
            magnesium: parse_decimal(row.get(6)?),
            sodium: parse_decimal(row.get(7)?),
            chloride: parse_decimal(row.get(8)?),
            sulfate: parse_decimal(row.get(9)?),
            bicarbonate: parse_decimal(row.get(10)?),
            ph_level: parse_decimal(row.get(11)?),
            total_dissolved_solids: parse_decimal(row.get(12)?),
            hardness: parse_decimal(row.get(13)?),
            best_suited_styles: row.get(14)?,
            usage_notes: row.get(15)?,
            flavor_impact: row.get(16)?,
            compatible_styles: row.get(17)?,
            created_at: row.get(18)?,
            updated_at: row.get(19)?,
        })
    }
}