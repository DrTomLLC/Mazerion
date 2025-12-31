// Database management commands

use clap::Subcommand;
use mazerion_db::{BatchStatus, DatabaseManager};
use rust_decimal::Decimal;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Subcommand)]
pub enum DbCommand {
    /// Initialize database
    Init {
        /// Path to user database
        #[arg(short, long, default_value = "data/user.db")]
        db_path: PathBuf,

        /// Path to packs directory
        #[arg(short, long, default_value = "data/packs")]
        packs_dir: PathBuf,
    },

    /// Create a new batch
    CreateBatch {
        /// Batch name
        name: String,

        /// Category (beer, mead, wine, cider)
        #[arg(short, long)]
        category: String,

        /// Batch size in liters
        #[arg(short, long)]
        size: String,

        /// Brew date (YYYY-MM-DD)
        #[arg(short, long)]
        date: String,

        /// Notes
        #[arg(short, long)]
        notes: Option<String>,
    },

    /// List batches
    ListBatches {
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,

        /// Maximum number of results
        #[arg(short, long, default_value = "100")]
        limit: usize,
    },

    /// Add gravity reading
    AddReading {
        /// Batch ID
        batch_id: i64,

        /// Gravity reading
        #[arg(short, long)]
        gravity: String,

        /// Temperature in Celsius
        #[arg(short, long)]
        temp: Option<String>,

        /// pH value
        #[arg(short, long)]
        ph: Option<String>,

        /// Notes
        #[arg(short, long)]
        notes: Option<String>,
    },

    /// Show batch readings
    ShowReadings {
        /// Batch ID
        batch_id: i64,
    },

    /// Add inventory item
    AddInventory {
        /// Item type (honey, hops, yeast, etc)
        #[arg(short = 't', long)]
        item_type: String,

        /// Item name
        name: String,

        /// Quantity
        #[arg(short, long)]
        quantity: String,

        /// Unit (kg, g, oz, etc)
        #[arg(short, long)]
        unit: String,

        /// Location
        #[arg(short, long)]
        location: Option<String>,
    },

    /// List inventory
    ListInventory {
        /// Filter by type
        #[arg(short = 't', long)]
        item_type: Option<String>,

        /// Maximum number of results
        #[arg(short, long, default_value = "100")]
        limit: usize,
    },

    /// Search inventory
    SearchInventory {
        /// Search query
        query: String,
    },

    /// Create a new recipe
    CreateRecipe {
        /// Recipe name
        name: String,

        /// Category (beer, mead, wine, cider)
        #[arg(short, long)]
        category: String,

        /// Batch size in liters
        #[arg(short, long)]
        size: String,

        /// Description
        #[arg(short, long)]
        description: Option<String>,
    },

    /// List recipes
    ListRecipes {
        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,

        /// Maximum number of results
        #[arg(short, long, default_value = "100")]
        limit: usize,
    },

    /// Search recipes
    SearchRecipes {
        /// Search query
        query: String,
    },

    /// Delete recipe
    DeleteRecipe {
        /// Recipe ID
        id: i64,
    },

    /// Show database statistics
    Stats,
}

impl DbCommand {
    pub fn execute(self) -> anyhow::Result<()> {
        match self {
            Self::Init { db_path, packs_dir } => {
                let manager = DatabaseManager::new(&db_path, &packs_dir)?;
                println!("✓ Database initialized at {}", db_path.display());

                let packs = manager.list_packs()?;
                if packs.is_empty() {
                    println!("  No content packs found in {}", packs_dir.display());
                } else {
                    println!("  Found {} content packs:", packs.len());
                    for pack in packs {
                        println!("    • {}", pack);
                    }
                }
                Ok(())
            }

            Self::CreateBatch { name, category, size, date, notes } => {
                let manager = get_manager()?;

                let batch_size = Decimal::from_str_exact(&size)
                    .map_err(|_| anyhow::anyhow!("Invalid batch size"))?;

                let batch = mazerion_db::Batch {
                    id: None,
                    name: name.clone(),
                    recipe_id: None,
                    category,
                    batch_size_l: batch_size,
                    brew_date: date,
                    target_og: None,
                    target_fg: None,
                    target_abv: None,
                    status: BatchStatus::Planning,
                    notes,
                    created_at: String::new(),
                    updated_at: String::new(),
                };

                let id = manager.with_user_db(|conn| {
                    let repo = mazerion_db::BatchRepository::new(conn);
                    repo.create(&batch)
                })?;

                println!("✓ Created batch '{}' with ID {}", name, id);
                Ok(())
            }

            Self::ListBatches { status, limit } => {
                let manager = get_manager()?;

                let status_filter = status
                    .map(|s| BatchStatus::from_str(&s))
                    .transpose()?;

                let batches = manager.with_user_db(|conn| {
                    let repo = mazerion_db::BatchRepository::new(conn);
                    repo.list(status_filter, limit)
                })?;

                if batches.is_empty() {
                    println!("No batches found");
                    return Ok(());
                }

                println!("Found {} batches:\n", batches.len());
                for batch in batches {
                    println!("ID {}: {} ({})",
                             batch.id.unwrap_or(0),
                             batch.name,
                             batch.status.as_str()
                    );
                    println!("  Size: {}L | Date: {}", batch.batch_size_l, batch.brew_date);
                    if let Some(notes) = &batch.notes {
                        println!("  Notes: {}", notes);
                    }
                    println!();
                }
                Ok(())
            }

            Self::AddReading { batch_id, gravity, temp, ph, notes } => {
                let manager = get_manager()?;

                let gravity_val = Decimal::from_str_exact(&gravity)
                    .map_err(|_| anyhow::anyhow!("Invalid gravity"))?;

                let reading = mazerion_db::BatchReading {
                    id: None,
                    batch_id,
                    reading_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
                    gravity: gravity_val,
                    temperature_c: temp
                        .map(|s| Decimal::from_str_exact(&s))
                        .transpose()
                        .map_err(|_| anyhow::anyhow!("Invalid temperature"))?,
                    ph: ph
                        .map(|s| Decimal::from_str_exact(&s))
                        .transpose()
                        .map_err(|_| anyhow::anyhow!("Invalid pH"))?,
                    notes,
                    source: "manual".to_string(),
                };

                let id = manager.with_user_db(|conn| {
                    let repo = mazerion_db::BatchRepository::new(conn);
                    repo.add_reading(&reading)
                })?;

                println!("✓ Added reading {} to batch {}", id, batch_id);
                Ok(())
            }

            Self::ShowReadings { batch_id } => {
                let manager = get_manager()?;

                let readings = manager.with_user_db(|conn| {
                    let repo = mazerion_db::BatchRepository::new(conn);
                    repo.get_readings(batch_id)
                })?;

                if readings.is_empty() {
                    println!("No readings found for batch {}", batch_id);
                    return Ok(());
                }

                println!("Readings for batch {}:\n", batch_id);
                for reading in readings {
                    println!("Date: {} | Gravity: {}", reading.reading_date, reading.gravity);
                    if let Some(temp) = reading.temperature_c {
                        println!("  Temperature: {}°C", temp);
                    }
                    if let Some(ph_val) = reading.ph {
                        println!("  pH: {}", ph_val);
                    }
                    if let Some(notes) = &reading.notes {
                        println!("  Notes: {}", notes);
                    }
                    println!();
                }
                Ok(())
            }

            Self::AddInventory { item_type, name, quantity, unit, location } => {
                let manager = get_manager()?;

                let qty = Decimal::from_str_exact(&quantity)
                    .map_err(|_| anyhow::anyhow!("Invalid quantity"))?;

                let item = mazerion_db::InventoryItem {
                    id: None,
                    item_type,
                    item_name: name.clone(),
                    quantity: qty,
                    unit,
                    location,
                    purchase_date: Some(chrono::Local::now().format("%Y-%m-%d").to_string()),
                    expiration_date: None,
                    cost: None,
                    notes: None,
                    created_at: String::new(),
                    updated_at: String::new(),
                };

                let id = manager.with_user_db(|conn| {
                    let repo = mazerion_db::InventoryRepository::new(conn);
                    repo.add(&item)
                })?;

                println!("✓ Added '{}' to inventory with ID {}", name, id);
                Ok(())
            }

            Self::ListInventory { item_type, limit } => {
                let manager = get_manager()?;

                let items = manager.with_user_db(|conn| {
                    let repo = mazerion_db::InventoryRepository::new(conn);
                    repo.list(item_type.as_deref(), limit)
                })?;

                if items.is_empty() {
                    println!("No inventory items found");
                    return Ok(());
                }

                println!("Found {} items:\n", items.len());
                for item in items {
                    println!("{}: {} {} {}",
                             item.id.unwrap_or(0),
                             item.quantity,
                             item.unit,
                             item.item_name
                    );
                    if let Some(loc) = &item.location {
                        println!("  Location: {}", loc);
                    }
                    println!();
                }
                Ok(())
            }

            Self::SearchInventory { query } => {
                let manager = get_manager()?;

                let items = manager.with_user_db(|conn| {
                    let repo = mazerion_db::InventoryRepository::new(conn);
                    repo.search(&query, 100)
                })?;

                if items.is_empty() {
                    println!("No items found matching '{}'", query);
                    return Ok(());
                }

                println!("Found {} items matching '{}':\n", items.len(), query);
                for item in items {
                    println!("{}: {} {} {}",
                             item.id.unwrap_or(0),
                             item.quantity,
                             item.unit,
                             item.item_name
                    );
                }
                Ok(())
            }

            Self::CreateRecipe { name, category, size, description } => {
                let manager = get_manager()?;

                let batch_size = Decimal::from_str_exact(&size)
                    .map_err(|_| anyhow::anyhow!("Invalid batch size"))?;

                let recipe = mazerion_db::Recipe {
                    id: None,
                    name: name.clone(),
                    category,
                    subcategory: None,
                    description,
                    author: None,
                    source: None,
                    difficulty: None,
                    batch_size_l: batch_size,
                    target_og: None,
                    target_fg: None,
                    target_abv: None,
                    created_at: String::new(),
                    updated_at: String::new(),
                };

                let id = manager.with_user_db(|conn| {
                    let repo = mazerion_db::RecipeRepository::new(conn);
                    repo.create_user_recipe(&recipe)
                })?;

                println!("✓ Created recipe '{}' with ID {}", name, id);
                Ok(())
            }

            Self::ListRecipes { category, limit } => {
                let manager = get_manager()?;

                let recipes = manager.with_user_db(|conn| {
                    let repo = mazerion_db::RecipeRepository::new(conn);
                    repo.list_user_recipes(category.as_deref(), limit)
                })?;

                if recipes.is_empty() {
                    println!("No recipes found");
                    return Ok(());
                }

                println!("Found {} recipes:\n", recipes.len());
                for recipe in recipes {
                    println!("ID {}: {} ({})",
                             recipe.id.unwrap_or(0),
                             recipe.name,
                             recipe.category
                    );
                    println!("  Size: {}L", recipe.batch_size_l);
                    if let Some(desc) = &recipe.description {
                        println!("  Description: {}", desc);
                    }
                    println!();
                }
                Ok(())
            }

            Self::SearchRecipes { query } => {
                let manager = get_manager()?;

                let recipes = manager.with_user_db(|conn| {
                    let repo = mazerion_db::RecipeRepository::new(conn);
                    repo.search_user_recipes(&query, 100)
                })?;

                if recipes.is_empty() {
                    println!("No recipes found matching '{}'", query);
                    return Ok(());
                }

                println!("Found {} recipes matching '{}':\n", recipes.len(), query);
                for recipe in recipes {
                    println!("ID {}: {} ({})",
                             recipe.id.unwrap_or(0),
                             recipe.name,
                             recipe.category
                    );
                }
                Ok(())
            }

            Self::DeleteRecipe { id } => {
                let manager = get_manager()?;

                manager.with_user_db(|conn| {
                    let repo = mazerion_db::RecipeRepository::new(conn);
                    repo.delete_user_recipe(id)
                })?;

                println!("✓ Deleted recipe {}", id);
                Ok(())
            }

            Self::Stats => {
                let manager = get_manager()?;
                let stats = manager.user_db_stats()?;

                println!("Database Statistics:");
                println!("  Batches: {}", stats.batch_count);
                println!("  Readings: {}", stats.reading_count);
                println!("  Inventory Items: {}", stats.inventory_count);
                println!("  Calculations: {}", stats.calculation_count);

                let packs = manager.list_packs()?;
                println!("  Content Packs: {}", packs.len());
                Ok(())
            }
        }
    }
}

fn get_manager() -> anyhow::Result<DatabaseManager> {
    DatabaseManager::new("data/user.db", "data/packs")
        .map_err(|e| anyhow::anyhow!("Failed to open database: {}", e))
}