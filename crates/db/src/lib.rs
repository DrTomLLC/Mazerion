// Database layer for Mazerion

#![allow(clippy::module_inception)]

pub use mazerion_core::{Error, Result};

mod manager;
mod models;
mod repositories;
pub mod schemas;
mod sqlite;

pub use manager::{DatabaseManager, DbStats};
pub use models::{
    Batch, BatchReading, BatchStatus, InventoryItem,
    Recipe, RecipeIngredient, RecipeInstruction,
};
pub use repositories::{BatchRepository, InventoryRepository, RecipeRepository};
pub use schemas::{
    create_user_schema,
    create_encyclopedia_master_schema,
    create_recipes_master_schema,
    create_styles_master_schema,
    verify_database_integrity,
    get_schema_version,
};