// Database manager with dynamic discovery and repository access

use rusqlite::{Connection, OpenFlags};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use mazerion_core::{Error, Result};

use crate::schemas;
use crate::repositories::{BatchRepository, InventoryRepository, RecipeRepository};

/// Database manager with dynamic pack discovery
pub struct DbStats {
    pub batch_count: u32,
    pub inventory_count: u32,
    pub reading_count: u32,
    pub calculation_count: u32,
}
pub struct DatabaseManager {
    user_db: Arc<Mutex<Connection>>,
    master_dbs: Arc<Mutex<HashMap<String, Connection>>>,
    packs_dir: PathBuf,
}

impl DatabaseManager {
    /// Create new manager, initializing user.db and discovering master packs
    pub fn new<P: AsRef<Path>>(user_db_path: P, packs_dir: P) -> Result<Self> {
        // Open/create writable user database
        let user_db = Connection::open(user_db_path.as_ref())
            .map_err(|e| Error::DatabaseError(format!("Failed to open user.db: {}", e)))?;

        // Initialize user schema
        schemas::create_user_schema(&user_db)?;

        let user_db = Arc::new(Mutex::new(user_db));

        let mut manager = Self {
            user_db,
            master_dbs: Arc::new(Mutex::new(HashMap::new())),
            packs_dir: packs_dir.as_ref().to_path_buf(),
        };

        // Discover and attach master databases
        manager.discover_packs()?;

        Ok(manager)
    }

    /// Discover and attach all master databases in packs directory
    pub fn discover_packs(&mut self) -> Result<()> {
        if !self.packs_dir.exists() {
            std::fs::create_dir_all(&self.packs_dir)
                .map_err(|e| Error::DatabaseError(format!("Failed to create packs dir: {}", e)))?;
            return Ok(());
        }

        let entries = std::fs::read_dir(&self.packs_dir)
            .map_err(|e| Error::DatabaseError(format!("Failed to read packs dir: {}", e)))?;

        let mut master_dbs = self
            .master_dbs
            .lock()
            .map_err(|_| Error::DatabaseError("Failed to acquire write lock".into()))?;

        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("db")
                && let Some(name) = path.file_stem().and_then(|s| s.to_str())
                && name.ends_with("_master")
            {
                self.attach_master_db(&mut master_dbs, &path, name)?;
            }
        }

        Ok(())
    }

    /// Attach a master database (read-only)
    fn attach_master_db(
        &self,
        master_dbs: &mut HashMap<String, Connection>,
        path: &Path,
        name: &str,
    ) -> Result<()> {
        let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .map_err(|e| Error::DatabaseError(format!("Failed to open master DB {}: {}", name, e)))?;

        master_dbs.insert(name.to_string(), conn);
        Ok(())
    }

    /// Execute operation with user database connection
    pub fn with_user_db<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Connection) -> Result<T>,
    {
        let conn = self
            .user_db
            .lock()
            .map_err(|_| Error::DatabaseError("Failed to acquire lock".into()))?;

        f(&conn)
    }

    /// Execute operation with master database connection
    pub fn with_master_db<F, T>(&self, name: &str, f: F) -> Result<T>
    where
        F: FnOnce(&Connection) -> Result<T>,
    {
        let master_dbs = self
            .master_dbs
            .lock()
            .map_err(|_| Error::DatabaseError("Failed to acquire lock".into()))?;

        let conn = master_dbs
            .get(name)
            .ok_or_else(|| Error::DatabaseError(format!("Master DB {} not found", name)))?;

        f(conn)
    }

    /// Get batch repository for user database
    pub fn batch_repo(&self) -> Result<UserBatchRepo> {
        Ok(UserBatchRepo {
            manager: self.user_db.clone(),
        })
    }

    /// Get inventory repository for user database
    pub fn inventory_repo(&self) -> Result<UserInventoryRepo> {
        Ok(UserInventoryRepo {
            manager: self.user_db.clone(),
        })
    }

    /// Get recipe repository for a master database
    pub fn recipe_repo(&self, pack_name: &str) -> Result<MasterRecipeRepo> {
        let master_dbs = self
            .master_dbs
            .lock()
            .map_err(|_| Error::DatabaseError("Failed to acquire lock".into()))?;

        if !master_dbs.contains_key(pack_name) {
            return Err(Error::DatabaseError(format!(
                "Pack {} not found",
                pack_name
            )));
        }

        Ok(MasterRecipeRepo {
            manager: self.master_dbs.clone(),
            pack_name: pack_name.to_string(),
        })
    }

    /// List all available master databases
    pub fn list_packs(&self) -> Result<Vec<String>> {
        let master_dbs = self
            .master_dbs
            .lock()
            .map_err(|_| Error::DatabaseError("Failed to acquire lock".into()))?;

        Ok(master_dbs.keys().cloned().collect())
    }

    /// Get statistics about user database
    pub fn user_db_stats(&self) -> Result<UserDbStats> {
        self.with_user_db(|conn| {
            let batch_count: i64 = conn
                .query_row("SELECT COUNT(*) FROM batches", [], |row| row.get(0))
                .map_err(|e| Error::DatabaseError(format!("Failed to count batches: {}", e)))?;

            let inventory_count: i64 = conn
                .query_row("SELECT COUNT(*) FROM inventory", [], |row| row.get(0))
                .map_err(|e| Error::DatabaseError(format!("Failed to count inventory: {}", e)))?;

            let reading_count: i64 = conn
                .query_row("SELECT COUNT(*) FROM batch_readings", [], |row| row.get(0))
                .map_err(|e| Error::DatabaseError(format!("Failed to count readings: {}", e)))?;

            let calculation_count: i64 = conn
                .query_row("SELECT COUNT(*) FROM calculation_log", [], |row| row.get(0))
                .map_err(|e| Error::DatabaseError(format!("Failed to count calculations: {}", e)))?;

            Ok(UserDbStats {
                batch_count: batch_count as u32,
                inventory_count: inventory_count as u32,
                reading_count: reading_count as u32,
                calculation_count: calculation_count as u32,
            })
        })
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// REPOSITORY WRAPPERS
// ══════════════════════════════════════════════════════════════════════════════

/// Thread-safe batch repository wrapper
pub struct UserBatchRepo {
    manager: Arc<Mutex<Connection>>,
}

impl UserBatchRepo {
    pub fn with_repo<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(BatchRepository) -> Result<T>,
    {
        let conn = self
            .manager
            .lock()
            .map_err(|_| Error::DatabaseError("Failed to acquire lock".into()))?;

        let repo = BatchRepository::new(&conn);
        f(repo)
    }
}

/// Thread-safe inventory repository wrapper
pub struct UserInventoryRepo {
    manager: Arc<Mutex<Connection>>,
}

impl UserInventoryRepo {
    pub fn with_repo<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(InventoryRepository) -> Result<T>,
    {
        let conn = self
            .manager
            .lock()
            .map_err(|_| Error::DatabaseError("Failed to acquire lock".into()))?;

        let repo = InventoryRepository::new(&conn);
        f(repo)
    }
}

/// Thread-safe recipe repository wrapper for master databases
pub struct MasterRecipeRepo {
    manager: Arc<Mutex<HashMap<String, Connection>>>,
    pack_name: String,
}

impl MasterRecipeRepo {
    pub fn with_repo<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(RecipeRepository) -> Result<T>,
    {
        let master_dbs = self
            .manager
            .lock()
            .map_err(|_| Error::DatabaseError("Failed to acquire lock".into()))?;

        let conn = master_dbs
            .get(&self.pack_name)
            .ok_or_else(|| Error::DatabaseError(format!("Pack {} not found", self.pack_name)))?;

        let repo = RecipeRepository::new(conn);
        f(repo)
    }
}

/// User database statistics
#[derive(Debug, Clone)]
pub struct UserDbStats {
    pub batch_count: u32,
    pub inventory_count: u32,
    pub reading_count: u32,
    pub calculation_count: u32,
}