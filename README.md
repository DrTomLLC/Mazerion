# Mazerion 🍯
**Precision Mead & Beverage Calculator (MCL)** — a modular Rust 2024 workspace with a calculator registry, GUI/TUI/CLI front-ends, JSON API helpers, and a C FFI layer.

> Branch: `first-dev`  
> Workspace version: **0.17.0**  
> Rust toolchain: **1.91.1** (see `Cargo.toml`)

---

## What Mazerion is (today)
Mazerion is a **calculator-first** toolkit for mead / beer / cider / wine workflows—built around a strict, reusable core and a growing catalog of calculators (“MCL”: Mazerion Calculator Library).

### What’s working right now
- **Calculator registry + shared types** (`crates/core`)
- **46 calculators currently wired into the registry** (`crates/calculators`)  
  See: **`CALCULATORS.md`** for the full catalog.
- **CLI** (`crates/cli`)
    - launch GUI/TUI
    - list calculators/categories
    - execute a calculator via `param=value` pairs
- **GUI** (`crates/gui`, egui/eframe)
    - themed UI + unit system options
    - guided screens for core calculators + style references
- **TUI** (`crates/tui`, ratatui)
    - lightweight terminal launcher/UI
- **JSON API helpers** (`crates/api`)
    - `list_calculators()` + `execute_calculation()` from a `HashMap<String,String>`
- **C FFI layer** (`crates/ffi` + `include/mazerion.h`)
    - call calculators from C/C++ safely across the boundary
- **Data catalogs** (`data/*.json`)
    - style references for mead + beer styles (used by the UI layer)

### What is *not* fully shipped yet (but present / planned)
- `crates/db` exists, but is **not currently wired into the workspace build** (logbook/history is planned)
- `crates/core-api` exists but is **not part of the workspace members** (older/experimental direction)
- Some CI “strictness” tools exist (line-guard, deny, clippy rules), but **this branch still has files over 150 lines** (tracked work to refactor/modules)

---

## Quick Start

### 1) Install the required Rust toolchain
```bash
rustup toolchain install 1.91.1
rustup override set 1.91.1
