# Architecture — Mazerion (first-dev)

**Version:** 0.30.0  
**Last Updated:** December 29, 2025

## High-Level Overview

Mazerion is built as a **calculator-first, data-isolated, offline-first** Rust workspace. The core business logic lives entirely in Rust for maximum performance, safety, and energy efficiency. All interfaces (CLI, GUI, TUI, API, FFI, future Android/Wear OS) are thin wrappers around a shared Rust core. This design ensures consistency across platforms, eliminates runtime panics, and enables seamless extensibility through dynamic discovery.

The architecture prioritizes:
- **Zero Panics** – Every operation returns `Result<MazerionError>` with exhaustive error handling.
- **Precision** – All calculations use `rust_decimal` for financial-grade accuracy.
- **Security & Isolation** – Multiple embedded SQLite databases with read-only masters and writable user data.
- **Offline-First** – No cloud dependencies; internet used only for optional updates and paid packs.
- **Energy Efficiency** – Optimized hot paths, minimal allocations, suitable for mobile and wearable devices.
- **Extensibility** – Runtime discovery of databases, calculators, encyclopedias, and future plugins.

## Core Principles

| Principle                  | Implementation                                                                 |
|----------------------------|--------------------------------------------------------------------------------|
| **No Panics**              | No `unwrap()`, `expect()`, `panic!()`, or unchecked indexing. All paths return `Result`. |
| **Error Handling**         | Central `MazerionError` enum in `crates/core`; propagated with `?`.               |
| **Performance**            | `rayon` for parallelism where beneficial, caching, prepared statements, minimal heap use. |
| **Modularity**             | Loose coupling between crates; interfaces depend only on core traits.            |
| **Dynamic Discovery**      | Runtime scanning (`walkdir`) for drop-in DBs, packs, and future plugins.         |
| **Security**               | Read-only master DBs, optional encryption (sqlcipher) for user.db, input validation. |

## Data Flow
User Interaction
│
▼
[Voice / CLI / GUI / TUI / API / FFI / Kotlin Wrapper]
│
▼
UniFFI / C FFI Bridge (Android ↔ Rust)
│
▼
Core Registry (crates/core)
│
▼
Calculators (crates/calculators) ◄── Expression-based or compiled Rust
│
▼
Isolated SQLite Databases (crates/db)
│       │       │
▼       ▼       ▼
calcs_master.db  recipes_master.db  encyc_master.db  ...  user.db (writable)
(read-only)      (read-only)         (read-only)                (encrypted optional)
│
▼
Results → UI / TTS / Voice Response / Export
text## Key Components

### 1. Core Registry (`crates/core`)
- Central calculator registry with auto-discovery.
- Shared types: `CalcInput`, `CalcOutput`, `Calculator` trait.
- Dynamic loading of encyclopedias and metadata.
- Future: Event system for hardware (e.g., Tilt updates).

### 2. Calculators (`crates/calculators`)
- Plug-in model: Each calculator implements the `Calculator` trait.
- Supports both compiled Rust functions and future expression-based (evalexpr/meval) for simple dynamic calcs.
- All use `rust_decimal` and return `Result`.

### 3. Databases (`crates/db`)
- Embedded SQLite via `rusqlite` + `r2d2` pooling.
- **Multiple isolated files** for fault tolerance:
    - `*_master.db` files: Pre-populated, shipped read-only (`SQLITE_OPEN_READ_ONLY`).
    - `user.db`: Writable, stored in user directory, optionally encrypted.
- Dynamic discovery: On startup, scan `/data/dbs/` or `/packs/` for new master DBs and attach.
- Schemas per domain (encyclopedias, recipes, inventory, logbook, etc.).
- Migrations via `refinery` for future upgrades.

### 4. Interfaces
| Interface       | Crate            | Status / Notes                                      |
|-----------------|------------------|-----------------------------------------------------|
| CLI             | `crates/cli`     | Fully functional                                     |
| GUI (Desktop)   | `crates/gui`     | egui/eframe, themed, unit-aware; some tabs frozen   |
| TUI             | `crates/tui`     | ratatui, lightweight                                |
| JSON API        | `crates/api`     | Programmatic access                                 |
| C FFI           | `crates/ffi`     | Current bridge; migrating to UniFFI                 |
| Android/Wear OS | Kotlin wrapper   | In planning: Thin Jetpack Compose UI over UniFFI    |

### 5. Extensions & Packs
- **Paid Packs**: Pre-populated SQLite files (encyclopedias, historical recipes, creator recipes).
- Dropped into a packs directory → auto-discovered and merged/registered at runtime.
- No recompilation needed for new content.

### 6. Hardware Integration
- **Tilt Hydrometer**: BLE scanning in Kotlin, parsing in Rust, logging to user.db.
- Future: Other sensors via FFI callbacks.

### 7. Voice System
- Offline STT (Vosk) and TTS.
- Natural command parsing in Rust core.
- Full hands-free workflow (e.g., "Scale recipe to 10 gallons", "Log gravity 1.012").

## Future Expansions

- Full UniFFI bridge for idiomatic Kotlin bindings.
- Runtime plugin system (`.so`/`.dylib`) for community calculators.
- Local multi-device sync (phone ↔ Watch via Wearable API or local network).
- Enterprise features: Multi-user support, compliance reporting, audit logs.
- Chef-specific modules: Culinary fermentation costing and pairings.

## Summary

Mazerion’s architecture is deliberately conservative and robust: a battle-tested Rust core, isolated data, dynamic but safe extensibility, and thin platform-specific wrappers. This foundation minimizes maintenance, maximizes reliability, and positions Mazerion as the ultimate brewing companion—from home hobbyists to world-class producers.

Brew better. Forever. 🍯