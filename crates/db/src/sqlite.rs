// SQLite utilities and helper functions

use mazerion_core::{Error, Result};
use rusqlite::Connection;

// Logbook structures (not currently used but kept for future)
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub id: Option<i64>,
    pub timestamp: String,
    pub category: String,
    pub message: String,
}

#[derive(Debug)]
pub struct Logbook {
    conn: Connection,
}

impl Logbook {
    /// Create or open logbook at path.
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)
            .map_err(|e| Error::DatabaseError(format!("Failed to open logbook: {}", e)))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                category TEXT NOT NULL,
                message TEXT NOT NULL
            )",
            [],
        )
            .map_err(|e| Error::DatabaseError(format!("Failed to create logs table: {}", e)))?;

        Ok(Self { conn })
    }

    pub fn add(&mut self, entry: &LogEntry) -> Result<i64> {
        self.conn
            .execute(
                "INSERT INTO logs (timestamp, category, message) VALUES (?1, ?2, ?3)",
                rusqlite::params![&entry.timestamp, &entry.category, &entry.message],
            )
            .map_err(|e| Error::DatabaseError(format!("Failed to add log: {}", e)))?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn recent(&self, limit: i64) -> Result<Vec<LogEntry>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, timestamp, category, message FROM logs ORDER BY id DESC LIMIT ?1")
            .map_err(|e| Error::DatabaseError(format!("Failed to prepare query: {}", e)))?;

        let entries = stmt
            .query_map([limit], |row| {
                Ok(LogEntry {
                    id: Some(row.get(0)?),
                    timestamp: row.get(1)?,
                    category: row.get(2)?,
                    message: row.get(3)?,
                })
            })
            .map_err(|e| Error::DatabaseError(format!("Failed to query logs: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::DatabaseError(format!("Failed to parse logs: {}", e)))?;

        Ok(entries)
    }
}