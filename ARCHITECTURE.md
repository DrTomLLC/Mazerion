# Mazerion Architecture

## Design Principles

1. **Zero Panics** - All functions return Result<T, Error>
2. **Decimal Precision** - rust_decimal for all calculations
3. **Modular Calculators** - Drop-in system with compile-time registry
4. **File Size Limits** - Maximum 150 lines per file
5. **Type Safety** - Strong typing throughout

## Crate Structure

### mazerion-core
Core types, traits, and validation. Provides:
- `Calculator` trait - Interface all calculators implement
- `CalcInput` / `CalcResult` - Standard input/output types
- `Error` - Typed error enum
- Validation functions for all measurement types

### mazerion-calculators
11 calculator implementations, each in its own file.
Automatically registered at compile-time via linkme.

### mazerion-config
Hot-reload configuration system using notify file watcher.
Loads config.toml and ingredients.toml.

### mazerion-db
Optional SQLite integration (feature-gated).
For batch tracking and recipe storage.

### mazerion-gui
egui-based GUI with modular tab system:
- 4 tabs organizing 11 calculators
- Centralized state management
- Helper functions for common patterns

### mazerion-tui
ratatui-based terminal UI for server environments.

### mazerion-cli
Binary launcher that orchestrates GUI/TUI modes.

## Calculator Registry

Uses linkme for compile-time registration:

```rust
#[linkme::distributed_slice]
pub static CALCULATORS: [CalculatorEntry];

register_calculator!(MyCalculator);
```

No runtime overhead, all calculators discovered at compile-time.

## Error Handling Strategy

Custom Error enum with variants for:
- InvalidInput
- ParseError
- OutOfRange
- CalculationError
- ConfigError
- DatabaseError
- IoError
- Other

All public APIs return Result<T, Error>.
