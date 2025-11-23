#[cfg(feature = "sqlite")]
pub mod sqlite;

#[cfg(feature = "sqlite")]
pub use sqlite::SqliteLogbook;

// Re-export core types
pub use mazerion_core::{Error, Result};

/// Trait for batch tracking and recipe storage
pub trait Logbook {
    fn save_calculation(&mut self, calc_id: &str, inputs: &str, outputs: &str) -> Result<()>;
    fn list_calculations(&self) -> Result<Vec<String>>;
}
