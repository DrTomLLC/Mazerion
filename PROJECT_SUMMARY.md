# Project Summary — Mazerion (first-dev)

**Mazerion** is a Rust 2024 workspace focused on building a **precision beverage calculation engine** (MCL: Mazerion Calculator Library) with multiple front-ends (GUI/TUI/CLI) and integration surfaces (JSON API helpers + C FFI).

## Primary Goal
Provide **industrial-grade, reproducible calculations** for mead/beer/cider/wine workflows—using:
- decimal math (`rust_decimal`)
- strict validation rules
- a shared error model
- a modular calculator registry

## Current Capabilities (this branch)
### Calculator Engine
- Central registry in `crates/core`
- **46 calculators wired** in `crates/calculators`
- Category grouping + lookup by ID

### Interfaces
- **CLI**: list/categories/calc execution + GUI/TUI launch
- **GUI (egui/eframe)**: themed UI + guided forms for key calculators + style references
- **TUI (ratatui)**: lightweight terminal UI

### Integration Surfaces
- **JSON API helper crate** (`crates/api`): execute calculators from `calculator_id` + `params`
- **C FFI** (`crates/ffi` + `include/mazerion.h`): call calculators from native apps

### Data Catalogs
- JSON style datasets in `data/` (mead + beer style references), consumed by the UI layer.

## What’s Present but Not Fully Wired Yet
- `crates/db`: planned logbook/history (not currently a workspace member)
- `crates/core-api`: experimental/older direction, not in workspace members
- `crates/voice`: foundation for voice-driven workflows

## Design Principles
- One shared source of truth for calculations (front-ends call the same engine)
- Calculator plug-in model (compile-time registration)
- Strong input validation and consistent error reporting
- Modular crate boundaries to support:
    - CLI tools
    - desktop UI
    - future Android app
    - future HTTP service

## Near-Term Roadmap
- Wire `crates/db` into the workspace with feature gating
- Add logbook entries + calculation history
- Expand recipe builder capabilities (targets → suggestions → step plan)
- Convert GUI screens toward “full catalog coverage” (generic param UI + per-calc guided UI)
- Add HTTP server wrapper over `crates/api`
- Prepare Android bridge (Kotlin + Rust via FFI)

## Long-Term Roadmap
- Ingredient catalog expansion (honey varietals, fruit sugar models, acids/tannins, nutrients)
- Voice-guided brew day workflows
- Offline-first mobile app (Android) calling the same core library
