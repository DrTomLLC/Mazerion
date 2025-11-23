use crate::{Logbook, Result};
use mazerion_core::Error;
use rusqlite::{Connection, params};

pub struct SqliteLogbook {
    conn: Connection,
}

impl SqliteLogbook {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)
            .map_err(|e| Error::DatabaseError(format!("Failed to open database: {}", e)))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS calculations (
                id INTEGER PRIMARY KEY,
                calc_id TEXT NOT NULL,
                inputs TEXT NOT NULL,
                outputs TEXT NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).map_err(|e| Error::DatabaseError(format!("Failed to create table: {}", e)))?;

        Ok(Self { conn })
    }
}

impl Logbook for SqliteLogbook {
    fn save_calculation(&mut self, calc_id: &str, inputs: &str, outputs: &str) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO calculations (calc_id, inputs, outputs) VALUES (?1, ?2, ?3)",
                params![calc_id, inputs, outputs],
            )
            .map_err(|e| Error::DatabaseError(format!("Failed to save: {}", e)))?;
        Ok(())
    }

    fn list_calculations(&self) -> Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT calc_id, timestamp FROM calculations ORDER BY timestamp DESC LIMIT 100")
            .map_err(|e| Error::DatabaseError(format!("Failed to prepare: {}", e)))?;

        let rows = stmt
            .query_map([], |row| {
                let calc_id: String = row.get(0)?;
                let timestamp: String = row.get(1)?;
                Ok(format!("{} at {}", calc_id, timestamp))
            })
            .map_err(|e| Error::DatabaseError(format!("Failed to query: {}", e)))?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row.map_err(|e| {
                Error::DatabaseError(format!("Failed to read row: {}", e))
            })?);
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_logbook() {
        let mut logbook = SqliteLogbook::new(":memory:").unwrap();
        logbook.save_calculation("abv", "og=1.100,fg=1.000", "abv=13.125").unwrap();
        let calcs = logbook.list_calculations().unwrap();
        assert_eq!(calcs.len(), 1);
    }
}
