# 🍯 Mazerion

**Precision Mead & Beverage Calculator** — A modular, panic-free Rust workspace with GUI, TUI, and CLI interfaces.

## Features

- ✅ **Zero Panics**: No `unwrap`, `expect`, `panic!`, `todo!` anywhere
- 🎯 **Decimal Precision**: Using `rust_decimal` for accurate calculations
- 🔌 **Modular Calculators**: Drop-in calculator plugins via compile-time registry
- 🔥 **Hot Reload**: Config and ingredients files reload on change
- 🎨 **Pure Rust UIs**: GUI (egui), TUI (ratatui), CLI
- 📊 **Optional SQLite**: Feature-gated database for calculation history
- 📏 **Line Limits**: Max 150 lines per `.rs` file, enforced by tooling
- 🔒 **Strict Linting**: Denies unsafe code, panics, and common pitfalls

## Quick Start

```bash
# Build everything
cargo build --release

# Run GUI
cargo run --bin mazerion -- gui

# Run TUI
cargo run --bin mazerion -- tui

# List calculators
cargo run --bin mazerion -- list

# Check line counts
cargo run --bin line-guard
```

## Workspace Structure

```
mazerion/
├── crates/
│   ├── core/         # Core types, traits, errors
│   ├── calculators/  # Calculator implementations
│   ├── config/       # Hot-reload configuration
│   ├── db/           # Optional SQLite (feature: db)
│   ├── gui/          # egui/eframe GUI
│   ├── tui/          # ratatui TUI
│   └── cli/          # CLI launcher
└── tools/
    └── line-guard/   # Line count enforcer
```

## Adding a Calculator

Create a new file in `crates/calculators/src/`:

```rust
use mazerion_core::{register_calculator, Calculator, CalcInput, CalcResult};

#[derive(Default)]
pub struct MyCalculator;

impl MyCalculator {
    pub const ID: &'static str = "my_calc";
}

impl Calculator for MyCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "My Calculator" }
    fn description(&self) -> &'static str { "Does something cool" }
    
    fn calculate(&self, input: CalcInput) -> mazerion_core::Result<CalcResult> {
        // Your logic here
        todo!() // This would fail clippy - implement it!
    }
}

register_calculator!(MyCalculator);
```

Then add to `crates/calculators/src/lib.rs`:

```rust
pub mod my_calc;
pub use my_calc::MyCalculator;
```

**No other code needs to change!** The calculator auto-registers at compile time.

## Validation Ranges

- **SG**: 0.6000–2.0000 (4 decimals)
- **pH**: 1.50–8.50 (3 decimals)
- **Brix/Plato**: 0–70 (warn >45, 2 decimals)
- **Temperature**: −5°C to 100°C

## Configuration

### config.toml

Hot-reloaded configuration:

```toml
app_name = "Mazerion"
version = "0.1.0"

[precision]
sg_decimals = 4
ph_decimals = 3
brix_decimals = 2
```

### ingredients.toml

Hot-reloaded ingredient database:

```toml
[[items]]
name = "Honey"
category = "sweetener"
sugar_content = 82.0
```

## Database Feature

Enable SQLite logbook:

```bash
cargo build --features db
```

Without the `db` feature, builds have no database dependencies.

## CI & Testing

```bash
# Linting
cargo clippy -- -D warnings

# Tests
cargo test --all-features

# Line limits
cargo run --bin line-guard

# Deny checks
cargo deny check
```

## Android Build

```bash
# Install cargo-ndk
cargo install cargo-ndk

# Build for Android
cargo ndk build --target aarch64-linux-android
```

## License

MIT OR Apache-2.0

## Architecture

### Calculator Registry

Uses `linkme` for compile-time registration. No runtime initialization needed.

### Error Handling

All errors use `thiserror` with typed variants. No string errors or panics.

### Hot Reload

Polls file metadata (mtime + size) without heavy dependencies. Efficient and simple.

### Precision

`rust_decimal::Decimal` for all measurements. Display precision per unit type.

## Development

Requires Rust 1.83+ (Edition 2024).

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check everything
cargo check --all-features --all-targets
```
