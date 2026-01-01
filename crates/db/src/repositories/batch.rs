use rusqlite::{params, Connection, OptionalExtension, Row};
use rust_decimal::Decimal;
use mazerion_core::{Error, Result};
use crate::models::{Batch, BatchReading, BatchStatus};
use std::str::FromStr;

pub struct BatchRepository<'a> {
    conn: &'a Connection,
}

impl<'a> BatchRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create(&self, batch: &Batch) -> Result<i64> {
        batch.validate()
            .map_err(|e| Error::Validation(e))?;

        self.conn
            .execute(
                "INSERT INTO batches (name, recipe_id, category, batch_size_l, brew_date,
                 target_og, target_fg, target_abv, status, notes)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    &batch.name,
                    &batch.recipe_id,
                    &batch.category,
                    batch.batch_size_l.to_string(),
                    &batch.brew_date,
                    batch.target_og.as_ref().map(|v| v.to_string()),
                    batch.target_fg.as_ref().map(|v| v.to_string()),
                    batch.target_abv.as_ref().map(|v| v.to_string()),
                    batch.status.as_str(),
                    &batch.notes,
                ],
            )
            .map_err(|e| Error::DatabaseError(format!("Insert batch: {}", e)))?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get(&self, id: i64) -> Result<Option<Batch>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, name, recipe_id, category, batch_size_l, brew_date,
                 target_og, target_fg, target_abv, status, notes, created_at, updated_at
                 FROM batches WHERE id = ?1",
            )
            .map_err(|e| Error::DatabaseError(format!("Prepare query: {}", e)))?;

        let result = stmt
            .query_row([id], |row| Self::row_to_batch(row))
            .optional()
            .map_err(|e| Error::DatabaseError(format!("Get batch: {}", e)))?;

        Ok(result)
    }

    pub fn list(&self, status: Option<BatchStatus>, limit: usize) -> Result<Vec<Batch>> {
        let capped_limit = limit.min(1000);
        let (query, params): (String, Vec<Box<dyn rusqlite::ToSql>>) = if let Some(status) = status {
            (
                "SELECT id, name, recipe_id, category, batch_size_l, brew_date,
                 target_og, target_fg, target_abv, status, notes, created_at, updated_at
                 FROM batches WHERE status = ?1 ORDER BY updated_at DESC LIMIT ?2".to_string(),
                vec![Box::new(status.as_str().to_string()), Box::new(capped_limit as i64)],
            )
        } else {
            (
                "SELECT id, name, recipe_id, category, batch_size_l, brew_date,
                 target_og, target_fg, target_abv, status, notes, created_at, updated_at
                 FROM batches ORDER BY updated_at DESC LIMIT ?1".to_string(),
                vec![Box::new(capped_limit as i64)],
            )
        };

        let mut stmt = self.conn.prepare(&query)
            .map_err(|e| Error::DatabaseError(format!("Prepare list: {}", e)))?;

        let params_ref: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        let rows = stmt.query_map(&params_ref[..], |row| Self::row_to_batch(row))
            .map_err(|e| Error::DatabaseError(format!("Execute list: {}", e)))?;

        let mut results = Vec::new();
        for row_result in rows {
            results.push(row_result.map_err(|e| Error::DatabaseError(format!("Row error: {}", e)))?);
        }
        Ok(results)
    }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<Batch>> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        let search_pattern = format!("%{}%", query.trim());
        let capped_limit = limit.min(1000);

        let mut stmt = self.conn.prepare(
            "SELECT id, name, recipe_id, category, batch_size_l, brew_date,
             target_og, target_fg, target_abv, status, notes, created_at, updated_at
             FROM batches
             WHERE name LIKE ?1 OR category LIKE ?1 OR notes LIKE ?1
             ORDER BY updated_at DESC
             LIMIT ?2"
        ).map_err(|e| Error::DatabaseError(format!("Prepare search: {}", e)))?;

        let rows = stmt.query_map(
            params![search_pattern, capped_limit as i64],
            |row| Self::row_to_batch(row)
        ).map_err(|e| Error::DatabaseError(format!("Execute search: {}", e)))?;

        let mut results = Vec::new();
        for row_result in rows {
            results.push(row_result.map_err(|e| Error::DatabaseError(format!("Row error: {}", e)))?);
        }
        Ok(results)
    }

    pub fn update_status(&self, id: i64, status: BatchStatus) -> Result<()> {
        let updated_at = chrono::Utc::now().to_rfc3339();

        let rows_affected = self.conn.execute(
            "UPDATE batches SET status = ?1, updated_at = ?2 WHERE id = ?3",
            params![status.as_str(), updated_at, id]
        ).map_err(|e| Error::DatabaseError(format!("Update status: {}", e)))?;

        if rows_affected == 0 {
            return Err(Error::DatabaseError(format!("Batch {} not found", id)));
        }

        Ok(())
    }

    pub fn update_notes(&self, id: i64, notes: Option<String>) -> Result<()> {
        if let Some(ref n) = notes {
            if n.len() > 5000 {
                return Err(Error::Validation("Notes exceed 5000 characters".to_string()));
            }
        }

        let updated_at = chrono::Utc::now().to_rfc3339();

        let rows_affected = self.conn.execute(
            "UPDATE batches SET notes = ?1, updated_at = ?2 WHERE id = ?3",
            params![notes, updated_at, id]
        ).map_err(|e| Error::DatabaseError(format!("Update notes: {}", e)))?;

        if rows_affected == 0 {
            return Err(Error::DatabaseError(format!("Batch {} not found", id)));
        }

        Ok(())
    }

    pub fn delete(&self, id: i64) -> Result<()> {
        let rows_affected = self.conn.execute(
            "DELETE FROM batches WHERE id = ?1",
            params![id]
        ).map_err(|e| Error::DatabaseError(format!("Delete batch: {}", e)))?;

        if rows_affected == 0 {
            return Err(Error::DatabaseError(format!("Batch {} not found", id)));
        }

        Ok(())
    }

    pub fn count(&self) -> Result<u32> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM batches",
            [],
            |row| row.get(0)
        ).map_err(|e| Error::DatabaseError(format!("Count batches: {}", e)))?;

        Ok(count as u32)
    }

    pub fn add_reading(&self, reading: &BatchReading) -> Result<i64> {
        reading.validate()
            .map_err(|e| Error::Validation(e))?;

        self.conn.execute(
            "INSERT INTO batch_readings (batch_id, reading_date, gravity,
             temperature_c, ph, notes, source)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                reading.batch_id,
                &reading.reading_date,
                reading.gravity.to_string(),
                reading.temperature_c.as_ref().map(|v| v.to_string()),
                reading.ph.as_ref().map(|v| v.to_string()),
                &reading.notes,
                &reading.source,
            ]
        ).map_err(|e| Error::DatabaseError(format!("Insert reading: {}", e)))?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_readings(&self, batch_id: i64) -> Result<Vec<BatchReading>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, batch_id, reading_date, gravity, temperature_c, ph, notes, source
             FROM batch_readings
             WHERE batch_id = ?1
             ORDER BY reading_date DESC"
        ).map_err(|e| Error::DatabaseError(format!("Prepare readings: {}", e)))?;

        let rows = stmt.query_map(
            params![batch_id],
            |row| Self::row_to_reading(row)
        ).map_err(|e| Error::DatabaseError(format!("Execute readings: {}", e)))?;

        let mut results = Vec::new();
        for row_result in rows {
            results.push(row_result.map_err(|e| Error::DatabaseError(format!("Row error: {}", e)))?);
        }
        Ok(results)
    }

    fn row_to_batch(row: &Row) -> rusqlite::Result<Batch> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }

        let status_str: String = row.get(9)?;
        let status = BatchStatus::from_str(&status_str).unwrap_or(BatchStatus::Planning);

        Ok(Batch {
            id: row.get(0)?,
            name: row.get(1)?,
            recipe_id: row.get(2)?,
            category: row.get(3)?,
            batch_size_l: parse_decimal(Some(row.get::<_, String>(4)?)).unwrap(),
            brew_date: row.get(5)?,
            target_og: parse_decimal(row.get(6)?),
            target_fg: parse_decimal(row.get(7)?),
            target_abv: parse_decimal(row.get(8)?),
            status,
            notes: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        })
    }

    fn row_to_reading(row: &Row) -> rusqlite::Result<BatchReading> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }

        Ok(BatchReading {
            id: row.get(0)?,
            batch_id: row.get(1)?,
            reading_date: row.get(2)?,
            gravity: parse_decimal(Some(row.get::<_, String>(3)?)).unwrap(),
            temperature_c: parse_decimal(row.get(4)?),
            ph: parse_decimal(row.get(5)?),
            notes: row.get(6)?,
            source: row.get(7)?,
        })
    }
}