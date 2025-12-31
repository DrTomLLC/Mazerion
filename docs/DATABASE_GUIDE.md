# Mazerion Database Architecture Guide

## Overview

Mazerion uses multiple isolated SQLite databases for safety, extensibility, and data protection:

- **user.db**: Writable user data (batches, inventory, readings, custom recipes)
- **`*_master.db`**: Read-only reference databases (recipes, encyclopedias, styles)

## Architecture Principles

1. **Fault Isolation**: Master database corruption cannot affect user data
2. **Dynamic Discovery**: Drop new `.db` files into packs directory → auto-loaded
3. **Read-Only Masters**: Prevents accidental modification of shipped content
4. **Thread-Safe**: Arc<RwLock<>> for concurrent access
5. **Zero Panics**: All operations return Result<T, Error>

## Database Schemas

### User Database (`user.db`)

#### Batches Table
```sql
CREATE TABLE batches (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    recipe_id INTEGER,
    category TEXT NOT NULL,
    batch_size_l REAL NOT NULL,
    brew_date TEXT NOT NULL,
    target_og REAL,
    target_fg REAL,
    target_abv REAL,
    status TEXT CHECK(status IN ('planning', 'brewing', 'fermenting', ...)),
    notes TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);
```

#### Batch Readings Table
```sql
CREATE TABLE batch_readings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    batch_id INTEGER NOT NULL,
    reading_date TEXT NOT NULL,
    gravity REAL NOT NULL,
    temperature_c REAL,
    ph REAL,
    notes TEXT,
    source TEXT DEFAULT 'manual',
    FOREIGN KEY (batch_id) REFERENCES batches(id) ON DELETE CASCADE
);
```

#### Inventory Table
```sql
CREATE TABLE inventory (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    item_type TEXT NOT NULL,
    item_name TEXT NOT NULL,
    quantity REAL NOT NULL,
    unit TEXT NOT NULL,
    location TEXT,
    purchase_date TEXT,
    expiration_date TEXT,
    cost REAL,
    notes TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);
```

### Master Databases

#### Recipes Master (`recipes_master.db`)
- **recipes**: Recipe metadata
- **recipe_ingredients**: Ingredient lists with amounts/timing
- **recipe_instructions**: Step-by-step instructions

#### Encyclopedia Master (`encyc_master.db`)
- **encyclopedia_entries**: General reference entries
- **yeast_strains**: Yeast characteristics (attenuation, temp, tolerance)
- **hop_varieties**: Hop profiles (alpha/beta acids, aroma)

#### Styles Master (`styles_master.db`)
- **bjcp_styles**: BJCP style guidelines (OG/FG/ABV/IBU/SRM ranges)

## Rust API Usage

### Initialization
```rust
use mazerion_db::DatabaseManager;

let manager = DatabaseManager::new(
    "./data/user.db",
    "./data/packs"
)?;
```

### Batch Operations
```rust
use mazerion_db::{Batch, BatchStatus, BatchRepository};

manager.with_user_db(|conn| {
    let repo = BatchRepository::new(conn);
    
    // Create batch
    let batch = Batch {
        name: "Traditional Mead".to_string(),
        category: "mead".to_string(),
        batch_size_l: Decimal::from(19),
        brew_date: "2025-01-01".to_string(),
        status: BatchStatus::Planning,
        // ... other fields
    };
    
    let id = repo.create(&batch)?;
    
    // List batches
    let batches = repo.list(Some(BatchStatus::Fermenting), 100)?;
    
    // Update status
    repo.update_status(id, BatchStatus::Bottled)?;
    
    Ok(())
})
```

### Inventory Operations
```rust
use mazerion_db::{InventoryItem, InventoryRepository};

manager.with_user_db(|conn| {
    let repo = InventoryRepository::new(conn);
    
    // Add item
    let item = InventoryItem {
        item_type: "honey".to_string(),
        item_name: "Orange Blossom".to_string(),
        quantity: Decimal::from_str_exact("5.4").unwrap(),
        unit: "kg".to_string(),
        // ... other fields
    };
    
    let id = repo.add(&item)?;
    
    // Search
    let results = repo.search("honey", 10)?;
    
    // Update quantity
    repo.update_quantity(id, Decimal::from(3))?;
    
    Ok(())
})
```

### Recipe Access (Master DB)
```rust
use mazerion_db::RecipeRepository;

manager.with_master_db("recipes_master", |conn| {
    let repo = RecipeRepository::new(conn);
    
    // Get recipe with full ingredients/instructions
    let recipe = repo.get(1)?;
    
    // Search recipes
    let results = repo.search("IPA", 20)?;
    
    // List by category
    let meads = repo.list(Some("mead"), None, 100)?;
    
    Ok(())
})
```

## FFI/Android Usage

### Kotlin Example
```kotlin
import uniffi.mazerion_ffi.*

// Initialize database
val handle = DbHandle(
    userDbPath = "${context.filesDir}/user.db",
    packsDir = "${context.filesDir}/packs"
)

// Create batch
val request = CreateBatchRequest(
    name = "Traditional Mead",
    category = "mead",
    batchSizeL = "19.0",
    brewDate = "2025-01-01",
    targetOg = "1.100",
    targetFg = "1.010",
    targetAbv = "14.0",
    notes = "Orange blossom honey"
)

val batchId = dbCreateBatch(handle, request)

// Add gravity reading
val reading = AddReadingRequest(
    batchId = batchId,
    readingDate = "2025-01-08",
    gravity = "1.050",
    temperatureC = "20.0",
    ph = "3.5",
    notes = "Fermentation active",
    source = "tilt"
)

dbAddReading(handle, reading)

// List batches
val batches = dbListBatches(handle, statusFilter = "fermenting", limit = 100u)

// Get readings
val readings = dbGetReadings(handle, batchId)

// Inventory
val invRequest = AddInventoryRequest(
    itemType = "honey",
    itemName = "Orange Blossom",
    quantity = "5.0",
    unit = "kg",
    location = "pantry",
    purchaseDate = "2025-01-01",
    expirationDate = null,
    cost = "45.00",
    notes = null
)

val itemId = dbAddInventory(handle, invRequest)

// Search inventory
val honeyItems = dbSearchInventory(handle, query = "honey", limit = 10u)

// Get stats
val stats = dbGetStats(handle)
println("Batches: ${stats.batchCount}, Inventory: ${stats.inventoryCount}")
```

## Testing

Run comprehensive tests:
```bash
# Database layer tests
cargo test -p mazerion-db

# FFI integration tests
cargo test -p mazerion-ffi -- database

# All database tests
cargo test database
```

## Migration Strategy (Future)

When schema changes are needed:

1. Add migration files in `crates/db/migrations/`
2. Use `refinery` for version tracking
3. Apply migrations on database open:
```rust
use refinery::embed_migrations;

embed_migrations!("./migrations");

pub fn apply_migrations(conn: &mut Connection) -> Result<()> {
    migrations::runner().run(conn)
        .map_err(|e| Error::DatabaseError(format!("Migration failed: {}", e)))?;
    Ok(())
}
```

## Pack Distribution

### Creating a Master Database Pack
```bash
# Create pack database
sqlite3 honey_varieties_master.db

# Add schema
.read crates/db/schemas/encyclopedia.sql

# Insert data
INSERT INTO encyclopedia_entries (category, name, description, ...)
VALUES ('honey', 'Orange Blossom', '...', ...);

# Make read-only
chmod 444 honey_varieties_master.db

# Distribute
# Users drop into /packs directory → auto-discovered
```

### Pack Naming Convention
- `*_master.db`: Master database files
- Examples: `recipes_master.db`, `bjcp_styles_master.db`, `historic_recipes_master.db`

## Performance Considerations

1. **Indices**: Critical tables have indices on frequently-queried columns
2. **Batch Operations**: Use transactions for multiple inserts
3. **Connection Pooling**: Arc<RwLock<>> provides concurrent read access
4. **Hard Limits**: All queries capped at 1000 results max

## Security

1. **Input Validation**: All inputs validated at FFI boundary and repository level
2. **Size Caps**: Strings limited to prevent DoS (names: 200 chars, notes: 5000 chars)
3. **SQL Injection**: Using parameterized queries exclusively
4. **Optional Encryption**: Future support for sqlcipher on user.db

## Error Handling

All database operations return `Result<T, Error>`:
```rust
pub enum Error {
    DatabaseError(String),
    Validation(String),
    // ...
}
```

FFI layer maps to:
```rust
pub enum MazerionError {
    DatabaseError { msg: String },
    ValidationError { msg: String },
    // ...
}
```

## Backup Strategy

User data backup:
```bash
# Simple file copy (database must be idle)
cp user.db user_backup_$(date +%Y%m%d).db

# Or use SQLite backup API (safer)
sqlite3 user.db ".backup user_backup.db"
```

## Summary

✅ **Complete** - All schemas, repositories, FFI bindings, and tests  
✅ **Production-Ready** - Zero panics, full validation, thread-safe  
✅ **Extensible** - Drop-in pack support for paid content  
✅ **Android-Ready** - Full UniFFI integration with Kotlin examples  
✅ **Tested** - Comprehensive unit and integration tests

Next step: Build Android UI on top of this complete database foundation.