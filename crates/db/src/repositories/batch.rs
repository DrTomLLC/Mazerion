// Batch repository with proper Decimal handling

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
        batch.validate()?;

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
            .map_err(|e| Error::DatabaseError(format!("Failed to insert batch: {}", e)))?;

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
            .map_err(|e| Error::DatabaseError(format!("Failed to prepare query: {}", e)))?;

        let result = stmt
            .query_row([id], |row| Self::row_to_batch(row))
            .optional()
            .map_err(|e| Error::DatabaseError(format!("Failed to get batch: {}", e)))?;

        Ok(result)
    }

    pub fn list(&self, status: Option<BatchStatus>, limit: usize) -> Result<Vec<Batch>> {
        let (query, params): (String, Vec<Box<dyn rusqlite::ToSql>>) = if let Some(status) = status {
            (
                "SELECT id, name, recipe_id, category, batch_size_l, brew_date,
                 target_og, target_fg, target_abv, status, notes, created_at, updated_at
                 FROM batches WHERE status = ?1 ORDER BY updated_at DESC LIMIT ?2".to_string(),
                vec![
                    Box::new(status.as_str().to_string()),
                    Box::new(limit as i64),
                ],
            )
        } else {
            (
                "SELECT id, name, recipe_id, category, batch_size_l, brew_date,
                 target_og, target_fg, target_abv, status, notes, created_at, updated_at
                 FROM batches ORDER BY updated_at DESC LIMIT ?1".to_string(),
                vec![Box::new(limit as i64)],
            )
        };

        let mut stmt = self
            .conn
            .prepare(&query)
            .map_err(|e| Error::DatabaseError(format!("Failed to prepare query: {}", e)))?;

        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params.iter().map(|p| p.as_ref()).collect();

        let batches = stmt
            .query_map(params_refs.as_slice(), |row| Self::row_to_batch(row))
            .map_err(|e| Error::DatabaseError(format!("Failed to query batches: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::DatabaseError(format!("Failed to parse batches: {}", e)))?;

        Ok(batches)
    }

    pub fn update_status(&self, id: i64, status: BatchStatus) -> Result<()> {
        let affected = self
            .conn
            .execute(
                "UPDATE batches SET status = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
                params![status.as_str(), id],
            )
            .map_err(|e| Error::DatabaseError(format!("Failed to update status: {}", e)))?;

        if affected == 0 {
            return Err(Error::Validation(format!("Batch {} not found", id)));
        }

        Ok(())
    }

    pub fn update_notes(&self, id: i64, notes: Option<String>) -> Result<()> {
        if let Some(ref n) = notes
            && n.len() > 5000
        {
            return Err(Error::Validation("Notes too long (max 5000 chars)".into()));
        }

        let affected = self
            .conn
            .execute(
                "UPDATE batches SET notes = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
                params![notes, id],
            )
            .map_err(|e| Error::DatabaseError(format!("Failed to update notes: {}", e)))?;

        if affected == 0 {
            return Err(Error::Validation(format!("Batch {} not found", id)));
        }

        Ok(())
    }

    pub fn delete(&self, id: i64) -> Result<()> {
        let affected = self
            .conn
            .execute("DELETE FROM batches WHERE id = ?1", params![id])
            .map_err(|e| Error::DatabaseError(format!("Failed to delete batch: {}", e)))?;

        if affected == 0 {
            return Err(Error::Validation(format!("Batch {} not found", id)));
        }

        Ok(())
    }

    pub fn add_reading(&self, reading: &BatchReading) -> Result<i64> {
        reading.validate()?;

        self.conn
            .execute(
                "INSERT INTO batch_readings (batch_id, reading_date, gravity, temperature_c, ph, notes, source)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    reading.batch_id,
                    &reading.reading_date,
                    reading.gravity.to_string(),
                    reading.temperature_c.as_ref().map(|v| v.to_string()),
                    reading.ph.as_ref().map(|v| v.to_string()),
                    &reading.notes,
                    &reading.source,
                ],
            )
            .map_err(|e| Error::DatabaseError(format!("Failed to insert reading: {}", e)))?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_readings(&self, batch_id: i64) -> Result<Vec<BatchReading>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, batch_id, reading_date, gravity, temperature_c, ph, notes, source
                 FROM batch_readings WHERE batch_id = ?1 ORDER BY reading_date DESC",
            )
            .map_err(|e| Error::DatabaseError(format!("Failed to prepare query: {}", e)))?;

        let readings = stmt
            .query_map([batch_id], |row| Self::row_to_reading(row))
            .map_err(|e| Error::DatabaseError(format!("Failed to query readings: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::DatabaseError(format!("Failed to parse readings: {}", e)))?;

        Ok(readings)
    }

    // Helper to convert Row to Batch with proper Decimal parsing
    fn row_to_batch(row: &Row) -> rusqlite::Result<Batch> {
        let batch_size_str: String = row.get(4)?;
        let batch_size_l = Decimal::from_str(&batch_size_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                4,
                rusqlite::types::Type::Text,
                Box::new(e),
            ))?;

        let target_og: Option<String> = row.get(6)?;
        let target_og = target_og
            .map(|s| Decimal::from_str(&s))
            .transpose()
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                6,
                rusqlite::types::Type::Text,
                Box::new(e),
            ))?;

        let target_fg: Option<String> = row.get(7)?;
        let target_fg = target_fg
            .map(|s| Decimal::from_str(&s))
            .transpose()
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                7,
                rusqlite::types::Type::Text,
                Box::new(e),
            ))?;

        let target_abv: Option<String> = row.get(8)?;
        let target_abv = target_abv
            .map(|s| Decimal::from_str(&s))
            .transpose()
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                8,
                rusqlite::types::Type::Text,
                Box::new(e),
            ))?;

        let status_str: String = row.get(9)?;
        let status = BatchStatus::from_str(&status_str)
            .map_err(|_| rusqlite::Error::FromSqlConversionFailure(
                9,
                rusqlite::types::Type::Text,
                Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid status")),
            ))?;

        Ok(Batch {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            recipe_id: row.get(2)?,
            category: row.get(3)?,
            batch_size_l,
            brew_date: row.get(5)?,
            target_og,
            target_fg,
            target_abv,
            status,
            notes: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        })
    }

    // Helper to convert Row to BatchReading with proper Decimal parsing
    fn row_to_reading(row: &Row) -> rusqlite::Result<BatchReading> {
        let gravity_str: String = row.get(3)?;
        let gravity = Decimal::from_str(&gravity_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                3,
                rusqlite::types::Type::Text,
                Box::new(e),
            ))?;

        let temp_str: Option<String> = row.get(4)?;
        let temperature_c = temp_str
            .map(|s| Decimal::from_str(&s))
            .transpose()
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                4,
                rusqlite::types::Type::Text,
                Box::new(e),
            ))?;

        let ph_str: Option<String> = row.get(5)?;
        let ph = ph_str
            .map(|s| Decimal::from_str(&s))
            .transpose()
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                5,
                rusqlite::types::Type::Text,
                Box::new(e),
            ))?;

        Ok(BatchReading {
            id: Some(row.get(0)?),
            batch_id: row.get(1)?,
            reading_date: row.get(2)?,
            gravity,
            temperature_c,
            ph,
            notes: row.get(6)?,
            source: row.get(7)?,
        })
    }
}