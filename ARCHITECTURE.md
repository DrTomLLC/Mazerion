# Mazerion Architecture

## Design Principles

### 1. Zero Panics
- All errors use `thiserror` with typed `Error` enum
- No `unwrap()`, `expect()`, `panic!()`, `todo!()`, or `unimplemented!()`
- Strict clippy lints enforce panic-free code
- All functions return `Result<T, Error>`

### 2. Decimal Precision
- `rust_decimal::Decimal` for all measurements
- Fixed-point arithmetic prevents floating-point errors
- Unit-specific display precision:
  - SG: 4 decimals (0.0001)
  - pH: 3 decimals (0.001)
  - Brix/Plato: 2 decimals (0.01)

### 3. Modular Calculator System

#### Registry Pattern
Uses `linkme` for compile-time registration:

```rust
#[distributed_slice]
pub static CALCULATORS: [CalculatorEntry];
```

Each calculator:
1. Implements `Calculator` trait
2. Calls `register_calculator!()` macro
3. Self-registers at compile time
4. No manual registration needed

#### Adding Calculators
Create file → implement trait → call macro → done!

```rust
// In new_calc.rs
pub struct MyCalc;
impl Calculator for MyCalc { /* ... */ }
register_calculator!(MyCalc);
```

No changes to existing code required.

### 4. Hot Reload

#### Implementation
```rust
pub struct FileWatcher {
    path: PathBuf,
    last_modified: Option<SystemTime>,
    last_size: Option<u64>,
}
```

Polls file metadata (mtime + size) for changes:
- Lightweight (no inotify/kqueue)
- Cross-platform
- No heavy dependencies
- Works in GUI and TUI

#### Usage
```rust
let mut watcher = FileWatcher::new("config.toml");
if watcher.check_changed()? {
    let config = load_config("config.toml")?;
    // Apply new config
}
```

### 5. Error Handling

#### Error Types
```rust
pub enum Error {
    Validation(String),
    OutOfRange(String),
    MissingInput(String),
    Calculation(String),
    Parse(String),
    Io(String),
    Config(String),
    Database(String),
}
```

#### Validation Strategy
1. **Hard limits**: Return `Err()` for invalid values
2. **Soft warnings**: Add to `CalcResult.warnings`
3. **Range checks**: Validator trait methods

Example:
```rust
// Hard limit
Validator::sg(value)?;  // Err if outside 0.6-2.0

// Soft warning
if let Some(w) = Validator::brix_warning(value) {
    result = result.with_warning(w);
}
```

### 6. Feature Gates

#### Database Feature
```toml
[features]
default = []
db = ["dep:rusqlite"]
```

Conditional compilation:
```rust
#[cfg(feature = "db")]
mod sqlite;

#[cfg(not(feature = "db"))]
pub struct Logbook; // Stub
```

Benefits:
- Default build has no SQLite dependency
- Optional functionality for advanced users
- Smaller binary without `db` feature

### 7. UI Architecture

#### GUI (egui)
- Immediate mode GUI
- Pure Rust, cross-platform
- Native look and feel
- Hot reload ready

#### TUI (ratatui)
- Terminal user interface
- Crossterm backend
- Cross-platform
- Low resource usage

#### CLI
- Mode selector (gui/tui/list)
- Thin launcher
- Delegates to UI crates

### 8. Line Limit Enforcement

#### Tool: line-guard
```bash
cargo run --bin line-guard
```

Walks directory tree, counts lines, fails on >150.

#### CI Integration
```yaml
- run: cargo run --bin line-guard
```

Enforced in continuous integration.

#### Benefits
- Forces modular design
- Prevents monolithic files
- Improves maintainability
- Self-documenting code size

### 9. Testing Strategy

#### Unit Tests
- Per-module test files
- `#[cfg(test)]` modules
- Test validation logic
- Test calculator outputs

#### Integration Tests
- Full calculator workflows
- Config hot-reload
- Database operations (feature-gated)

#### Property Tests
Consider adding `proptest` for:
- Range validation
- Conversion correctness
- Decimal precision

### 10. Dependency Management

#### Workspace Dependencies
```toml
[workspace.dependencies]
rust_decimal = { version = "1.36", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
```

Benefits:
- Single version across workspace
- Consistent features
- Easier updates

#### cargo-deny
Checks for:
- Security advisories
- License compatibility
- Duplicate dependencies
- Yanked crates

## Data Flow

```
User Input → Calculator → Validation → Computation → Result
                 ↓
           Registry Lookup
                 ↓
           Trait Method
                 ↓
           Error Handling
```

## Extension Points

1. **New Calculators**: Drop in `.rs` file
2. **New Units**: Add to `Unit` enum
3. **New Validators**: Add to `Validator`
4. **New UI**: Implement using core crates
5. **New Features**: Feature-gate in Cargo.toml

## Build Configurations

### Development
```bash
cargo build
```

### Release (optimized)
```bash
cargo build --release
```

### With Database
```bash
cargo build --features db
```

### Android
```bash
cargo ndk build --target aarch64-linux-android
```

## Performance Considerations

- `Decimal` operations are slower than `f64` but precise
- Calculator registry has zero runtime cost
- Hot reload polling is infrequent (100ms typical)
- GUI/TUI are event-driven, not polling

## Future Enhancements

1. **More Calculators**
   - pH correction
   - Acid additions
   - Nutrient calculations
   - Yeast pitch rates

2. **Export Formats**
   - JSON export
   - CSV logs
   - PDF reports

3. **Advanced Features**
   - Recipe builder
   - Batch tracking
   - Historical analysis

4. **Mobile Apps**
   - iOS (cargo-mobile)
   - Android (cargo-ndk)

## References

- [rust_decimal docs](https://docs.rs/rust_decimal)
- [egui book](https://docs.rs/egui)
- [ratatui tutorial](https://ratatui.rs)
- [linkme crate](https://docs.rs/linkme)
