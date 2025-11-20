//! SQLite logbook for calculation history.

use mazerion_core::{Error, Result};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

/// Log entry for calculations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: Option<i64>,
    pub timestamp: String,
    pub calculator_id: String,
    pub input: String,
    pub output: String,
}

/// SQLite-backed logbook.
pub struct Logbook {
    conn: Connection,
}

impl Logbook {
    /// Create or open logbook at path.
    pub fn new(path: &str) -> Result<Self> {
        let conn =
            Connection::open(path).map_err(|e| Error::Database(format!("Open failed: {}", e)))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS log (
                id INTEGER PRIMARY KEY,
                timestamp TEXT NOT NULL,
                calculator_id TEXT NOT NULL,
                input TEXT NOT NULL,
                output TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| Error::Database(format!("Schema creation failed: {}", e)))?;

        Ok(Self { conn })
    }

    /// Add entry to log.
    pub fn add(&mut self, entry: &LogEntry) -> Result<i64> {
        self.conn
            .execute(
                "INSERT INTO log (timestamp, calculator_id, input, output) VALUES (?1, ?2, ?3, ?4)",
                params![
                    &entry.timestamp,
                    &entry.calculator_id,
                    &entry.input,
                    &entry.output
                ],
            )
            .map_err(|e| Error::Database(format!("Insert failed: {}", e)))?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Get recent entries (limit 100).
    pub fn recent(&self, limit: usize) -> Result<Vec<LogEntry>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, timestamp, calculator_id, input, output FROM log ORDER BY id DESC LIMIT ?1")
            .map_err(|e| Error::Database(format!("Query prepare failed: {}", e)))?;

        let entries = stmt
            .query_map([limit], |row| {
                Ok(LogEntry {
                    id: Some(row.get(0)?),
                    timestamp: row.get(1)?,
                    calculator_id: row.get(2)?,
                    input: row.get(3)?,
                    output: row.get(4)?,
                })
            })
            .map_err(|e| Error::Database(format!("Query failed: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::Database(format!("Row parse failed: {}", e)))?;

        Ok(entries)
    }
}
