# 🍯 Mazerion - Precision Mead & Beverage Calculator

A production-ready Rust application for precise brewing calculations, specializing in mead, beer, wine, cider, and hybrid beverages.

## Features

### 11 Professional Calculators

**Basic Calculations**
- ABV Calculator - Calculate alcohol by volume from gravity readings
- Brix ↔ SG Converter - Convert between Brix and specific gravity
- SG Temperature Correction - Adjust readings for temperature

**Advanced Operations**
- Dilution Calculator - Adjust ABV with water additions
- Blending Calculator - Mix two beverages with precision
- Refractometer Correction - Alcohol-adjusted refractometer readings

**Fermentation Management**
- Yeast Nutrition - TOSNA protocol with Fermaid O calculations
- Carbonation Calculator - Precise priming sugar calculations

**Finishing Processes**
- Acid Addition - pH adjustment with 4 acid types
- Sulfite Calculator - K-meta and SO2 dosing
- Backsweetening - Post-fermentation sugar additions

## Installation

```bash
cargo build --release
```

## Usage

### GUI Mode (Recommended)
```bash
cargo run --bin mazerion -- gui
```

### TUI Mode
```bash
cargo run --bin mazerion -- tui
```

### List Calculators
```bash
cargo run --bin mazerion -- list
```

## Architecture Highlights

- **Zero Panics**: Comprehensive error handling, no unwrap/expect/panic
- **Decimal Precision**: rust_decimal for accurate brewing calculations
- **Modular Design**: Drop-in calculator system with compile-time registry
- **Hot-Reload**: Configuration files auto-reload during runtime
- **Pure Rust UIs**: egui GUI + ratatui TUI
- **Optional SQLite**: Feature-gated database integration
- **Strict Quality**: All files ≤150 lines, comprehensive validation

## Documentation

- See `ARCHITECTURE.md` for design details
- See `CONTRIBUTING.md` for extension guide

## License

MIT OR Apache-2.0
