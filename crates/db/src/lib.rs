//! Database layer with optional SQLite support.

#[cfg(feature = "db")]
mod sqlite;

#[cfg(feature = "db")]
pub use sqlite::{Logbook, LogEntry};

#[cfg(not(feature = "db"))]
pub struct Logbook;

#[cfg(not(feature = "db"))]
impl Logbook {
    pub fn new(_path: &str) -> mazerion_core::Result<Self> {
        Err(mazerion_core::Error::DatabaseError(
            "Database feature not enabled".into(),
        ))
    }
}
