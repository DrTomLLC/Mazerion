use std::sync::{Arc, Mutex};
use std::path::Path;
use rusqlite::Connection;
use mazerion_core::{Error, Result};
use crate::schemas;

const MAX_PACKS: usize = 100;

pub struct DatabaseManager {
    user_db: Arc<Mutex<Connection>>,
    encyclopedia_db: Arc<Mutex<Connection>>,
}

impl DatabaseManager {
    pub fn new<P: AsRef<Path>>(user_db_path: P, encyclopedia_db_path: P) -> Result<Self> {
        let user_db = Connection::open(user_db_path)
            .map_err(|e| Error::DatabaseError(format!("Open user DB: {}", e)))?;
        let encyclopedia_db = Connection::open(encyclopedia_db_path)
            .map_err(|e| Error::DatabaseError(format!("Open encyclopedia DB: {}", e)))?;

        user_db.execute_batch("PRAGMA foreign_keys = ON;")
            .map_err(|e| Error::DatabaseError(format!("Enable foreign keys: {}", e)))?;
        encyclopedia_db.execute_batch("PRAGMA foreign_keys = ON;")
            .map_err(|e| Error::DatabaseError(format!("Enable foreign keys: {}", e)))?;

        let manager = Self {
            user_db: Arc::new(Mutex::new(user_db)),
            encyclopedia_db: Arc::new(Mutex::new(encyclopedia_db)),
        };

        manager.initialize_schemas()?;
        Ok(manager)
    }

    fn initialize_schemas(&self) -> Result<()> {
        let encyclopedia_db = self.encyclopedia_db.lock()
            .map_err(|e| Error::DatabaseError(format!("Lock encyclopedia DB: {}", e)))?;
        schemas::init_all_tables(&encyclopedia_db)
            .map_err(|e| Error::DatabaseError(format!("Init encyclopedia tables: {}", e)))?;
        drop(encyclopedia_db);

        let user_db = self.user_db.lock()
            .map_err(|e| Error::DatabaseError(format!("Lock user DB: {}", e)))?;
        schemas::create_user_schema(&user_db)?;
        drop(user_db);

        Ok(())
    }

    pub fn with_user_db<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&mut Connection) -> Result<T>,
    {
        let mut conn = self.user_db.lock()
            .map_err(|e| Error::DatabaseError(format!("Lock user DB: {}", e)))?;
        f(&mut conn)
    }

    pub fn with_encyclopedia_db<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Connection) -> Result<T>,
    {
        let conn = self.encyclopedia_db.lock()
            .map_err(|e| Error::DatabaseError(format!("Lock encyclopedia DB: {}", e)))?;
        f(&conn)
    }

    pub fn batch_repository(&self) -> Result<impl std::ops::Deref<Target = Connection> + '_> {
        self.user_db.lock()
            .map_err(|e| Error::DatabaseError(format!("Lock for batch repo: {}", e)))
    }

    pub fn recipe_repository(&self) -> Result<impl std::ops::Deref<Target = Connection> + '_> {
        self.user_db.lock()
            .map_err(|e| Error::DatabaseError(format!("Lock for recipe repo: {}", e)))
    }

    pub fn inventory_repository(&self) -> Result<impl std::ops::Deref<Target = Connection> + '_> {
        self.user_db.lock()
            .map_err(|e| Error::DatabaseError(format!("Lock for inventory repo: {}", e)))
    }

    pub fn list_packs(&self) -> Result<Vec<String>> {
        Ok(vec![
            "yeast".to_string(),
            "honey".to_string(),
            "hop".to_string(),
            "malt".to_string(),
            "fruit".to_string(),
            "vegetable".to_string(),
            "spice".to_string(),
            "herb".to_string(),
            "extract".to_string(),
            "syrup".to_string(),
            "adjunct".to_string(),
            "water_profile".to_string(),
            "water_salt".to_string(),
            "acid".to_string(),
            "nutrient".to_string(),
            "enzyme".to_string(),
            "bacteria".to_string(),
            "tannin".to_string(),
        ])
    }

    pub fn user_db_stats(&self) -> Result<UserDbStats> {
        let conn = self.user_db.lock()
            .map_err(|e| Error::DatabaseError(format!("Lock user DB: {}", e)))?;

        let batch_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM batches",
            [],
            |row| row.get(0)
        ).map_err(|e| Error::DatabaseError(format!("Count batches: {}", e)))?;

        let recipe_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM user_recipes",
            [],
            |row| row.get(0)
        ).map_err(|e| Error::DatabaseError(format!("Count recipes: {}", e)))?;

        let inventory_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM inventory",
            [],
            |row| row.get(0)
        ).map_err(|e| Error::DatabaseError(format!("Count inventory: {}", e)))?;

        let reading_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM batch_readings",
            [],
            |row| row.get(0)
        ).map_err(|e| Error::DatabaseError(format!("Count readings: {}", e)))?;

        Ok(UserDbStats {
            batch_count: batch_count as u32,
            recipe_count: recipe_count as u32,
            inventory_count: inventory_count as u32,
            reading_count: reading_count as u32,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UserDbStats {
    pub batch_count: u32,
    pub recipe_count: u32,
    pub inventory_count: u32,
    pub reading_count: u32,
}