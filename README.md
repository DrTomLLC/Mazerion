# Mazerion 🍯

**The Ultimate Precision Brewing Toolkit** — A modular Rust 2024 workspace delivering industrial-grade calculators, encyclopedias, and management tools for beer, mead, wine, sake, distilled spirits, kombucha, yeast, soda, and other ferments. Designed for everyone from novice home brewers and chefs to professional enterprises like AB InBev, Dom Pérignon, Schrams Meadery, and beyond.

> Branch: `first-dev`  
> Workspace version: **0.30.0**  
> Rust toolchain: **1.91.1** (see `Cargo.toml`)  
> Last updated: **December 29, 2025**  
> License: MIT OR Apache-2.0

---

## Vision and Goals
Mazerion aims to be the definitive, offline-first app/program for all brewing and fermentation needs—one place for calculations, recipes, encyclopedias, inventory, batch tracking, and more. It emphasizes:
- **Hands-Free Operation**: Full voice capabilities for sticky/wet brewing scenarios (e.g., "Calculate ABV for OG 1.090 and FG 1.010" or "Log racking to secondary").
- **Offline Security**: All core features run locally; internet only for optional updates/packs.
- **Scalability**: From 1-gallon home batches to 1000+ gallon commercial ops.
- **Extensibility**: Dynamic discovery for drop-in databases, features, and calculators.
- **Monetization**: Buy-once core app; optional paid packs (encyclopedias, recipes) for repeat value.
- **Platforms**: Desktop (CLI/GUI/TUI), Android (Kotlin wrapper for native UI), Wear OS (e.g., Galaxy Watch 7), with FFI/API for integrations.

Built with Rust for flawless performance, zero panics, and energy efficiency—no garbage collection pauses, full error handling, and 100% test coverage.

## What's Working Right Now
- **Calculator Registry + Shared Types** (`crates/core`): Central engine with strict validation.
- **47 Calculators Wired In** (`crates/calculators`): See **`CALCULATORS.md`** for the full catalog, including the fresh vs. frozen fruits comparator.
- **CLI** (`crates/cli`): Launch interfaces, list calculators, execute via params.
- **GUI** (`crates/gui`, egui/eframe): Themed UI with unit toggles, guided screens, and style references. (Note: Some tabs frozen for stability—see `FROZEN_FILES.md`.)
- **TUI** (`crates/tui`, ratatui): Lightweight terminal UI.
- **JSON API Helpers** (`crates/api`): List/execute calculators programmatically.
- **C FFI** (`crates/ffi` + `include/mazerion.h`): Native integrations; foundation for Android Kotlin bridge.
- **Data Catalogs**: JSON/TOML in `/data/` for styles/ingredients (migrating to DBs).

## Planned and In-Progress Features
- **Databases** (`crates/db`): Multiple isolated SQLite files (e.g., calcs_master.db, recipes_master.db, encyc_master.db) for data protection, read-only masters, writable user.db, dynamic discovery.
- **Encyclopedias**: Comprehensive references across categories (beer, mead, wine, etc.), with dynamic loading.
- **Recipe/Logbook/Inventory**: Management, scaling, costing, tracking with Tilt hydrometer integration.
- **Voice Integration**: Offline STT/TTS via Vosk, full hands-free commands.
- **Hardware**: BLE support for Tilt (real-time gravity/temp logging).
- **Packs**: Optional paid downloads (e.g., Exotic Ferments Pack, Historical Recipes Pack).
- **Android App**: Thin Kotlin/Jetpack Compose wrapper over Rust core via UniFFI.
- **Enterprise Tools**: Reporting, compliance, multi-user for B2B.

See **`PROJECT_SUMMARY.md`** for detailed roadmap.

## Quick Start

### 1) Install the required Rust toolchain
```bash
rustup toolchain install 1.91.1
rustup override set 1.91.1

2) Clone and Build
git clone https://github.com/DrTomLLC/Mazerion.git -b first-dev
cd Mazerion
cargo build --release


3) Run interfaces
CLI: cargo run --bin mazerion_cli
GUI: cargo run --bin mazerion_gui
TUI: cargo run --bin mazerion_tui

Contributing
See CONTRIBUTING.md for guidelines. We welcome expansions to calculators, DB schemas, and packs!
License
Dual-licensed under MIT or Apache-2.0—see LICENSE-MIT and LICENSE-APACHE.