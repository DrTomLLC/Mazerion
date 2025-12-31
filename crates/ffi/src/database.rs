// Database FFI types and operations

use mazerion_db::{
    Batch, BatchReading, BatchStatus, BatchRepository, DatabaseManager,
    InventoryItem, InventoryRepository, Recipe, RecipeRepository,
};
use rust_decimal::Decimal;
use std::sync::Arc;
use std::str::FromStr;

use crate::error::MazerionError;

// ══════════════════════════════════════════════════════════════════════════════
// BATCH FFI TYPES
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, uniffi::Record)]
pub struct BatchInfo {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub batch_size_l: String,
    pub brew_date: String,
    pub target_og: Option<String>,
    pub target_fg: Option<String>,
    pub target_abv: Option<String>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Batch> for BatchInfo {
    fn from(batch: Batch) -> Self {
        Self {
            id: batch.id.unwrap_or(0),
            name: batch.name,
            category: batch.category,
            batch_size_l: batch.batch_size_l.to_string(),
            brew_date: batch.brew_date,
            target_og: batch.target_og.map(|v| v.to_string()),
            target_fg: batch.target_fg.map(|v| v.to_string()),
            target_abv: batch.target_abv.map(|v| v.to_string()),
            status: batch.status.as_str().to_string(),
            notes: batch.notes,
            created_at: batch.created_at,
            updated_at: batch.updated_at,
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct CreateBatchRequest {
    pub name: String,
    pub category: String,
    pub batch_size_l: String,
    pub brew_date: String,
    pub target_og: Option<String>,
    pub target_fg: Option<String>,
    pub target_abv: Option<String>,
    pub notes: Option<String>,
}

impl CreateBatchRequest {
    fn to_batch(&self) -> Result<Batch, MazerionError> {
        if self.name.trim().is_empty() || self.name.len() > 200 {
            return Err(MazerionError::ValidationError {
                msg: "Invalid batch name".into(),
            });
        }

        let batch_size = Decimal::from_str_exact(&self.batch_size_l)
            .map_err(|_| MazerionError::InvalidInput {
                msg: "Invalid batch size".into(),
            })?;

        Ok(Batch {
            id: None,
            name: self.name.clone(),
            recipe_id: None,
            category: self.category.clone(),
            batch_size_l: batch_size,
            brew_date: self.brew_date.clone(),
            target_og: self.target_og.as_ref()
                .map(|s| Decimal::from_str_exact(s))
                .transpose()
                .map_err(|_| MazerionError::InvalidInput { msg: "Invalid OG".into() })?,
            target_fg: self.target_fg.as_ref()
                .map(|s| Decimal::from_str_exact(s))
                .transpose()
                .map_err(|_| MazerionError::InvalidInput { msg: "Invalid FG".into() })?,
            target_abv: self.target_abv.as_ref()
                .map(|s| Decimal::from_str_exact(s))
                .transpose()
                .map_err(|_| MazerionError::InvalidInput { msg: "Invalid ABV".into() })?,
            status: BatchStatus::Planning,
            notes: self.notes.clone(),
            created_at: String::new(),
            updated_at: String::new(),
        })
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct ReadingInfo {
    pub id: i64,
    pub batch_id: i64,
    pub reading_date: String,
    pub gravity: String,
    pub temperature_c: Option<String>,
    pub ph: Option<String>,
    pub notes: Option<String>,
    pub source: String,
}

impl From<BatchReading> for ReadingInfo {
    fn from(reading: BatchReading) -> Self {
        Self {
            id: reading.id.unwrap_or(0),
            batch_id: reading.batch_id,
            reading_date: reading.reading_date,
            gravity: reading.gravity.to_string(),
            temperature_c: reading.temperature_c.map(|v| v.to_string()),
            ph: reading.ph.map(|v| v.to_string()),
            notes: reading.notes,
            source: reading.source,
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct AddReadingRequest {
    pub batch_id: i64,
    pub reading_date: String,
    pub gravity: String,
    pub temperature_c: Option<String>,
    pub ph: Option<String>,
    pub notes: Option<String>,
    pub source: String,
}

impl AddReadingRequest {
    fn to_reading(&self) -> Result<BatchReading, MazerionError> {
        let gravity = Decimal::from_str_exact(&self.gravity)
            .map_err(|_| MazerionError::InvalidInput { msg: "Invalid gravity".into() })?;

        Ok(BatchReading {
            id: None,
            batch_id: self.batch_id,
            reading_date: self.reading_date.clone(),
            gravity,
            temperature_c: self.temperature_c.as_ref()
                .map(|s| Decimal::from_str_exact(s))
                .transpose()
                .map_err(|_| MazerionError::InvalidInput { msg: "Invalid temperature".into() })?,
            ph: self.ph.as_ref()
                .map(|s| Decimal::from_str_exact(s))
                .transpose()
                .map_err(|_| MazerionError::InvalidInput { msg: "Invalid pH".into() })?,
            notes: self.notes.clone(),
            source: self.source.clone(),
        })
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// INVENTORY FFI TYPES
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, uniffi::Record)]
pub struct InventoryItemInfo {
    pub id: i64,
    pub item_type: String,
    pub item_name: String,
    pub quantity: String,
    pub unit: String,
    pub location: Option<String>,
    pub purchase_date: Option<String>,
    pub expiration_date: Option<String>,
    pub cost: Option<String>,
    pub notes: Option<String>,
}

impl From<InventoryItem> for InventoryItemInfo {
    fn from(item: InventoryItem) -> Self {
        Self {
            id: item.id.unwrap_or(0),
            item_type: item.item_type,
            item_name: item.item_name,
            quantity: item.quantity.to_string(),
            unit: item.unit,
            location: item.location,
            purchase_date: item.purchase_date,
            expiration_date: item.expiration_date,
            cost: item.cost.map(|v| v.to_string()),
            notes: item.notes,
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct AddInventoryRequest {
    pub item_type: String,
    pub item_name: String,
    pub quantity: String,
    pub unit: String,
    pub location: Option<String>,
    pub purchase_date: Option<String>,
    pub expiration_date: Option<String>,
    pub cost: Option<String>,
    pub notes: Option<String>,
}

impl AddInventoryRequest {
    fn to_inventory_item(&self) -> Result<InventoryItem, MazerionError> {
        if self.item_name.trim().is_empty() || self.item_name.len() > 200 {
            return Err(MazerionError::ValidationError {
                msg: "Invalid item name".into(),
            });
        }

        let quantity = Decimal::from_str_exact(&self.quantity)
            .map_err(|_| MazerionError::InvalidInput { msg: "Invalid quantity".into() })?;

        Ok(InventoryItem {
            id: None,
            item_type: self.item_type.clone(),
            item_name: self.item_name.clone(),
            quantity,
            unit: self.unit.clone(),
            location: self.location.clone(),
            purchase_date: self.purchase_date.clone(),
            expiration_date: self.expiration_date.clone(),
            cost: self.cost.as_ref()
                .map(|s| Decimal::from_str_exact(s))
                .transpose()
                .map_err(|_| MazerionError::InvalidInput { msg: "Invalid cost".into() })?,
            notes: self.notes.clone(),
            created_at: String::new(),
            updated_at: String::new(),
        })
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// RECIPE FFI TYPES
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, uniffi::Record)]
pub struct RecipeInfo {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub source: Option<String>,
    pub batch_size_l: String,
    pub target_og: Option<String>,
    pub target_fg: Option<String>,
    pub target_abv: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Recipe> for RecipeInfo {
    fn from(recipe: Recipe) -> Self {
        Self {
            id: recipe.id.unwrap_or(0),
            name: recipe.name,
            category: recipe.category,
            subcategory: recipe.subcategory,
            description: recipe.description,
            author: recipe.author,
            source: recipe.source,
            batch_size_l: recipe.batch_size_l.to_string(),
            target_og: recipe.target_og.map(|v| v.to_string()),
            target_fg: recipe.target_fg.map(|v| v.to_string()),
            target_abv: recipe.target_abv.map(|v| v.to_string()),
            created_at: recipe.created_at,
            updated_at: recipe.updated_at,
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct CreateRecipeRequest {
    pub name: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub description: Option<String>,
    pub batch_size_l: String,
    pub target_og: Option<String>,
    pub target_fg: Option<String>,
    pub target_abv: Option<String>,
}

impl CreateRecipeRequest {
    fn to_recipe(&self) -> Result<Recipe, MazerionError> {
        if self.name.trim().is_empty() || self.name.len() > 200 {
            return Err(MazerionError::ValidationError {
                msg: "Invalid recipe name".into(),
            });
        }

        let batch_size = Decimal::from_str_exact(&self.batch_size_l)
            .map_err(|_| MazerionError::InvalidInput {
                msg: "Invalid batch size".into(),
            })?;

        Ok(Recipe {
            id: None,
            name: self.name.clone(),
            category: self.category.clone(),
            subcategory: self.subcategory.clone(),
            description: self.description.clone(),
            author: None,
            source: None,
            difficulty: None,
            batch_size_l: batch_size,
            target_og: self.target_og.as_ref()
                .map(|s| Decimal::from_str_exact(s))
                .transpose()
                .map_err(|_| MazerionError::InvalidInput { msg: "Invalid OG".into() })?,
            target_fg: self.target_fg.as_ref()
                .map(|s| Decimal::from_str_exact(s))
                .transpose()
                .map_err(|_| MazerionError::InvalidInput { msg: "Invalid FG".into() })?,
            target_abv: self.target_abv.as_ref()
                .map(|s| Decimal::from_str_exact(s))
                .transpose()
                .map_err(|_| MazerionError::InvalidInput { msg: "Invalid ABV".into() })?,
            created_at: String::new(),
            updated_at: String::new(),
        })
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// STATS FFI TYPE
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, uniffi::Record)]
pub struct DbStats {
    pub batch_count: u32,
    pub inventory_count: u32,
    pub reading_count: u32,
    pub calculation_count: u32,
}

// ══════════════════════════════════════════════════════════════════════════════
// DATABASE HANDLE
// ══════════════════════════════════════════════════════════════════════════════

pub struct DbHandle {
    manager: Arc<DatabaseManager>,
}

impl DbHandle {
    pub fn new(user_db_path: String, packs_dir: String) -> Result<Self, MazerionError> {
        let manager = DatabaseManager::new(&user_db_path, &packs_dir)
            .map_err(MazerionError::from_core_error)?;

        Ok(Self {
            manager: Arc::new(manager),
        })
    }

    // ══════════════════════════════════════════════════════════════════════
    // BATCH OPERATIONS
    // ══════════════════════════════════════════════════════════════════════

    pub fn create_batch(&self, request: CreateBatchRequest) -> Result<i64, MazerionError> {
        let batch = request.to_batch()?;

        self.manager
            .with_user_db(|conn| {
                let repo = BatchRepository::new(conn);
                repo.create(&batch)
            })
            .map_err(MazerionError::from_core_error)
    }

    pub fn get_batch(&self, id: i64) -> Result<Option<BatchInfo>, MazerionError> {
        if id <= 0 {
            return Err(MazerionError::ValidationError {
                msg: "Invalid batch ID".into(),
            });
        }

        self.manager
            .with_user_db(|conn| {
                let repo = BatchRepository::new(conn);
                repo.get(id)
            })
            .map(|opt| opt.map(BatchInfo::from))
            .map_err(MazerionError::from_core_error)
    }

    pub fn list_batches(&self, status_filter: Option<String>, limit: u32) -> Result<Vec<BatchInfo>, MazerionError> {
        let limit = limit.min(1000) as usize;

        let status = status_filter
            .map(|s| BatchStatus::from_str(&s))
            .transpose()
            .map_err(MazerionError::from_core_error)?;

        self.manager
            .with_user_db(|conn| {
                let repo = BatchRepository::new(conn);
                repo.list(status, limit)
            })
            .map(|batches| batches.into_iter().map(BatchInfo::from).collect())
            .map_err(MazerionError::from_core_error)
    }

    pub fn update_batch_status(&self, id: i64, status: String) -> Result<(), MazerionError> {
        if id <= 0 {
            return Err(MazerionError::ValidationError {
                msg: "Invalid batch ID".into(),
            });
        }

        let batch_status = BatchStatus::from_str(&status)
            .map_err(MazerionError::from_core_error)?;

        self.manager
            .with_user_db(|conn| {
                let repo = BatchRepository::new(conn);
                repo.update_status(id, batch_status)
            })
            .map_err(MazerionError::from_core_error)
    }

    pub fn update_batch_notes(&self, id: i64, notes: Option<String>) -> Result<(), MazerionError> {
        if id <= 0 {
            return Err(MazerionError::ValidationError {
                msg: "Invalid batch ID".into(),
            });
        }

        if let Some(ref n) = notes
            && n.len() > 5000
        {
            return Err(MazerionError::ValidationError {
                msg: "Notes too long (max 5000 chars)".into(),
            });
        }

        self.manager
            .with_user_db(|conn| {
                let repo = BatchRepository::new(conn);
                repo.update_notes(id, notes)
            })
            .map_err(MazerionError::from_core_error)
    }

    pub fn delete_batch(&self, id: i64) -> Result<(), MazerionError> {
        if id <= 0 {
            return Err(MazerionError::ValidationError {
                msg: "Invalid batch ID".into(),
            });
        }

        self.manager
            .with_user_db(|conn| {
                let repo = BatchRepository::new(conn);
                repo.delete(id)
            })
            .map_err(MazerionError::from_core_error)
    }

    pub fn add_reading(&self, request: AddReadingRequest) -> Result<i64, MazerionError> {
        let reading = request.to_reading()?;

        self.manager
            .with_user_db(|conn| {
                let repo = BatchRepository::new(conn);
                repo.add_reading(&reading)
            })
            .map_err(MazerionError::from_core_error)
    }

    pub fn get_readings(&self, batch_id: i64) -> Result<Vec<ReadingInfo>, MazerionError> {
        if batch_id <= 0 {
            return Err(MazerionError::ValidationError {
                msg: "Invalid batch ID".into(),
            });
        }

        self.manager
            .with_user_db(|conn| {
                let repo = BatchRepository::new(conn);
                repo.get_readings(batch_id)
            })
            .map(|readings| readings.into_iter().map(ReadingInfo::from).collect())
            .map_err(MazerionError::from_core_error)
    }

    // ══════════════════════════════════════════════════════════════════════
    // INVENTORY OPERATIONS
    // ══════════════════════════════════════════════════════════════════════

    pub fn add_inventory(&self, request: AddInventoryRequest) -> Result<i64, MazerionError> {
        let item = request.to_inventory_item()?;

        self.manager
            .with_user_db(|conn| {
                let repo = InventoryRepository::new(conn);
                repo.add(&item)
            })
            .map_err(MazerionError::from_core_error)
    }

    pub fn get_inventory(&self, id: i64) -> Result<Option<InventoryItemInfo>, MazerionError> {
        if id <= 0 {
            return Err(MazerionError::ValidationError {
                msg: "Invalid inventory ID".into(),
            });
        }

        self.manager
            .with_user_db(|conn| {
                let repo = InventoryRepository::new(conn);
                repo.get(id)
            })
            .map(|opt| opt.map(InventoryItemInfo::from))
            .map_err(MazerionError::from_core_error)
    }

    pub fn list_inventory(&self, item_type_filter: Option<String>, limit: u32) -> Result<Vec<InventoryItemInfo>, MazerionError> {
        let limit = limit.min(1000) as usize;

        self.manager
            .with_user_db(|conn| {
                let repo = InventoryRepository::new(conn);
                repo.list(item_type_filter.as_deref(), limit)
            })
            .map(|items| items.into_iter().map(InventoryItemInfo::from).collect())
            .map_err(MazerionError::from_core_error)
    }

    pub fn update_inventory_quantity(&self, id: i64, quantity: String) -> Result<(), MazerionError> {
        if id <= 0 {
            return Err(MazerionError::ValidationError {
                msg: "Invalid inventory ID".into(),
            });
        }

        let qty = Decimal::from_str_exact(&quantity)
            .map_err(|_| MazerionError::InvalidInput { msg: "Invalid quantity".into() })?;

        self.manager
            .with_user_db(|conn| {
                let repo = InventoryRepository::new(conn);
                repo.update_quantity(id, qty)
            })
            .map_err(MazerionError::from_core_error)
    }

    pub fn delete_inventory(&self, id: i64) -> Result<(), MazerionError> {
        if id <= 0 {
            return Err(MazerionError::ValidationError {
                msg: "Invalid inventory ID".into(),
            });
        }

        self.manager
            .with_user_db(|conn| {
                let repo = InventoryRepository::new(conn);
                repo.delete(id)
            })
            .map_err(MazerionError::from_core_error)
    }

    pub fn search_inventory(&self, query: String, limit: u32) -> Result<Vec<InventoryItemInfo>, MazerionError> {
        if query.len() > 200 {
            return Err(MazerionError::ValidationError {
                msg: "Search query too long (max 200 chars)".into(),
            });
        }

        let limit = limit.min(1000) as usize;

        self.manager
            .with_user_db(|conn| {
                let repo = InventoryRepository::new(conn);
                repo.search(&query, limit)
            })
            .map(|items| items.into_iter().map(InventoryItemInfo::from).collect())
            .map_err(MazerionError::from_core_error)
    }

    // ══════════════════════════════════════════════════════════════════════
    // RECIPE OPERATIONS
    // ══════════════════════════════════════════════════════════════════════

    pub fn create_recipe(&self, request: CreateRecipeRequest) -> Result<i64, MazerionError> {
        let recipe = request.to_recipe()?;

        self.manager
            .with_user_db(|conn| {
                let repo = RecipeRepository::new(conn);
                repo.create_user_recipe(&recipe)
            })
            .map_err(MazerionError::from_core_error)
    }

    pub fn get_recipe(&self, id: i64) -> Result<Option<RecipeInfo>, MazerionError> {
        if id <= 0 {
            return Err(MazerionError::ValidationError {
                msg: "Invalid recipe ID".into(),
            });
        }

        self.manager
            .with_user_db(|conn| {
                let repo = RecipeRepository::new(conn);
                repo.get_user_recipe(id)
            })
            .map(|opt| opt.map(RecipeInfo::from))
            .map_err(MazerionError::from_core_error)
    }

    pub fn list_recipes(&self, category: Option<String>, limit: u32) -> Result<Vec<RecipeInfo>, MazerionError> {
        let limit = limit.min(1000) as usize;

        self.manager
            .with_user_db(|conn| {
                let repo = RecipeRepository::new(conn);
                repo.list_user_recipes(category.as_deref(), limit)
            })
            .map(|recipes| recipes.into_iter().map(RecipeInfo::from).collect())
            .map_err(MazerionError::from_core_error)
    }

    pub fn update_recipe(&self, id: i64, request: CreateRecipeRequest) -> Result<(), MazerionError> {
        if id <= 0 {
            return Err(MazerionError::ValidationError {
                msg: "Invalid recipe ID".into(),
            });
        }

        let mut recipe = request.to_recipe()?;
        recipe.id = Some(id);

        self.manager
            .with_user_db(|conn| {
                let repo = RecipeRepository::new(conn);
                repo.update_user_recipe(&recipe)
            })
            .map_err(MazerionError::from_core_error)
    }

    pub fn delete_recipe(&self, id: i64) -> Result<(), MazerionError> {
        if id <= 0 {
            return Err(MazerionError::ValidationError {
                msg: "Invalid recipe ID".into(),
            });
        }

        self.manager
            .with_user_db(|conn| {
                let repo = RecipeRepository::new(conn);
                repo.delete_user_recipe(id)
            })
            .map_err(MazerionError::from_core_error)
    }

    pub fn search_recipes(&self, query: String, limit: u32) -> Result<Vec<RecipeInfo>, MazerionError> {
        if query.len() > 200 {
            return Err(MazerionError::ValidationError {
                msg: "Search query too long (max 200 chars)".into(),
            });
        }

        let limit = limit.min(1000) as usize;

        self.manager
            .with_user_db(|conn| {
                let repo = RecipeRepository::new(conn);
                repo.search_user_recipes(&query, limit)
            })
            .map(|recipes| recipes.into_iter().map(RecipeInfo::from).collect())
            .map_err(MazerionError::from_core_error)
    }

    // ══════════════════════════════════════════════════════════════════════
    // UTILITY OPERATIONS
    // ══════════════════════════════════════════════════════════════════════

    pub fn list_packs(&self) -> Result<Vec<String>, MazerionError> {
        self.manager.list_packs().map_err(MazerionError::from_core_error)
    }

    pub fn get_stats(&self) -> Result<DbStats, MazerionError> {
        let stats = self.manager.user_db_stats()
            .map_err(MazerionError::from_core_error)?;

        Ok(DbStats {
            batch_count: stats.batch_count,
            inventory_count: stats.inventory_count,
            reading_count: stats.reading_count,
            calculation_count: stats.calculation_count,
        })
    }
}