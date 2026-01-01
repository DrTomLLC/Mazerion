use rusqlite::{Connection, Result as SqliteResult, Row};
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::models::fruit::Fruit;

pub struct FruitRepository<'conn> { conn: &'conn Connection, }

impl<'conn> FruitRepository<'conn> {
    pub fn new(conn: &'conn Connection) -> Self { Self { conn } }

    pub fn create(conn: &Connection, fruit: &Fruit) -> SqliteResult<i64> {
        fruit.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        conn.execute(
            "INSERT INTO fruits (name, scientific_name, fruit_type, origin,
                typical_sugar_content, ph_level, color_contribution, flavor_profile,
                aroma_profile, best_suited_styles, usage_notes, sensory_notes,
                pounds_per_gallon, preparation_method, compatible_styles, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
            rusqlite::params![
                fruit.name, fruit.scientific_name, fruit.fruit_type, fruit.origin,
                fruit.typical_sugar_content.map(|d: Decimal| d.to_string()),
                fruit.ph_level.map(|d: Decimal| d.to_string()),
                fruit.color_contribution, fruit.flavor_profile, fruit.aroma_profile,
                fruit.best_suited_styles, fruit.usage_notes, fruit.sensory_notes,
                fruit.pounds_per_gallon.map(|d: Decimal| d.to_string()),
                fruit.preparation_method, fruit.compatible_styles, fruit.created_at, fruit.updated_at,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> SqliteResult<Fruit> {
        conn.query_row(
            "SELECT id, name, scientific_name, fruit_type, origin,
                typical_sugar_content, ph_level, color_contribution, flavor_profile,
                aroma_profile, best_suited_styles, usage_notes, sensory_notes,
                pounds_per_gallon, preparation_method, compatible_styles, created_at, updated_at
             FROM fruits WHERE id = ?1",
            rusqlite::params![id],
            Self::row_to_fruit
        )
    }

    pub fn list(conn: &Connection, limit: Option<i64>) -> SqliteResult<Vec<Fruit>> {
        let query = match limit {
            Some(_) => "SELECT id, name, scientific_name, fruit_type, origin,
                typical_sugar_content, ph_level, color_contribution, flavor_profile,
                aroma_profile, best_suited_styles, usage_notes, sensory_notes,
                pounds_per_gallon, preparation_method, compatible_styles, created_at, updated_at
                FROM fruits ORDER BY name ASC LIMIT ?1",
            None => "SELECT id, name, scientific_name, fruit_type, origin,
                typical_sugar_content, ph_level, color_contribution, flavor_profile,
                aroma_profile, best_suited_styles, usage_notes, sensory_notes,
                pounds_per_gallon, preparation_method, compatible_styles, created_at, updated_at
                FROM fruits ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(query)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![lim], Self::row_to_fruit)?
        } else {
            stmt.query_map([], Self::row_to_fruit)?
        };
        rows.collect()
    }

    pub fn search(conn: &Connection, query: &str, limit: Option<i64>) -> SqliteResult<Vec<Fruit>> {
        if query.trim().is_empty() { return Ok(Vec::new()); }

        let pattern = format!("%{}%", query.trim());
        let sql = match limit {
            Some(_) => "SELECT id, name, scientific_name, fruit_type, origin,
                typical_sugar_content, ph_level, color_contribution, flavor_profile,
                aroma_profile, best_suited_styles, usage_notes, sensory_notes,
                pounds_per_gallon, preparation_method, compatible_styles, created_at, updated_at
                FROM fruits WHERE name LIKE ?1 OR scientific_name LIKE ?1
                ORDER BY name ASC LIMIT ?2",
            None => "SELECT id, name, scientific_name, fruit_type, origin,
                typical_sugar_content, ph_level, color_contribution, flavor_profile,
                aroma_profile, best_suited_styles, usage_notes, sensory_notes,
                pounds_per_gallon, preparation_method, compatible_styles, created_at, updated_at
                FROM fruits WHERE name LIKE ?1 OR scientific_name LIKE ?1 ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![pattern, lim], Self::row_to_fruit)?
        } else {
            stmt.query_map(rusqlite::params![pattern], Self::row_to_fruit)?
        };
        rows.collect()
    }

    pub fn get_by_type(conn: &Connection, fruit_type: &str, limit: Option<i64>) -> SqliteResult<Vec<Fruit>> {
        let sql = match limit {
            Some(_) => "SELECT id, name, scientific_name, fruit_type, origin,
                typical_sugar_content, ph_level, color_contribution, flavor_profile,
                aroma_profile, best_suited_styles, usage_notes, sensory_notes,
                pounds_per_gallon, preparation_method, compatible_styles, created_at, updated_at
                FROM fruits WHERE fruit_type = ?1 ORDER BY name ASC LIMIT ?2",
            None => "SELECT id, name, scientific_name, fruit_type, origin,
                typical_sugar_content, ph_level, color_contribution, flavor_profile,
                aroma_profile, best_suited_styles, usage_notes, sensory_notes,
                pounds_per_gallon, preparation_method, compatible_styles, created_at, updated_at
                FROM fruits WHERE fruit_type = ?1 ORDER BY name ASC",
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(lim) = limit {
            stmt.query_map(rusqlite::params![fruit_type, lim], Self::row_to_fruit)?
        } else {
            stmt.query_map(rusqlite::params![fruit_type], Self::row_to_fruit)?
        };
        rows.collect()
    }

    pub fn update(conn: &Connection, fruit: &Fruit) -> SqliteResult<()> {
        fruit.validate().map_err(|e| rusqlite::Error::InvalidParameterName(e))?;

        let rows = conn.execute(
            "UPDATE fruits SET name = ?2, scientific_name = ?3, fruit_type = ?4, origin = ?5,
                typical_sugar_content = ?6, ph_level = ?7, color_contribution = ?8, flavor_profile = ?9,
                aroma_profile = ?10, best_suited_styles = ?11, usage_notes = ?12, sensory_notes = ?13,
                pounds_per_gallon = ?14, preparation_method = ?15, compatible_styles = ?16, updated_at = ?17
             WHERE id = ?1",
            rusqlite::params![
                fruit.id, fruit.name, fruit.scientific_name, fruit.fruit_type, fruit.origin,
                fruit.typical_sugar_content.map(|d: Decimal| d.to_string()),
                fruit.ph_level.map(|d: Decimal| d.to_string()),
                fruit.color_contribution, fruit.flavor_profile, fruit.aroma_profile,
                fruit.best_suited_styles, fruit.usage_notes, fruit.sensory_notes,
                fruit.pounds_per_gallon.map(|d: Decimal| d.to_string()),
                fruit.preparation_method, fruit.compatible_styles, fruit.updated_at,
            ],
        )?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> SqliteResult<()> {
        let rows = conn.execute("DELETE FROM fruits WHERE id = ?1", rusqlite::params![id])?;
        if rows == 0 { return Err(rusqlite::Error::QueryReturnedNoRows); }
        Ok(())
    }

    pub fn count(conn: &Connection) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM fruits", [], |row| row.get(0))
    }

    pub fn count_by_type(conn: &Connection, fruit_type: &str) -> SqliteResult<i64> {
        conn.query_row("SELECT COUNT(*) FROM fruits WHERE fruit_type = ?1",
                       rusqlite::params![fruit_type], |row| row.get(0))
    }

    fn row_to_fruit(row: &Row) -> SqliteResult<Fruit> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }

        Ok(Fruit {
            id: row.get(0)?,
            name: row.get(1)?,
            scientific_name: row.get(2)?,
            fruit_type: row.get(3)?,
            origin: row.get(4)?,
            typical_sugar_content: parse_decimal(row.get(5)?),
            ph_level: parse_decimal(row.get(6)?),
            color_contribution: row.get(7)?,
            flavor_profile: row.get(8)?,
            aroma_profile: row.get(9)?,
            best_suited_styles: row.get(10)?,
            usage_notes: row.get(11)?,
            sensory_notes: row.get(12)?,
            pounds_per_gallon: parse_decimal(row.get(13)?),
            preparation_method: row.get(14)?,
            compatible_styles: row.get(15)?,
            created_at: row.get(16)?,
            updated_at: row.get(17)?,
        })
    }
}