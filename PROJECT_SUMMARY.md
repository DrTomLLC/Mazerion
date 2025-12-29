# Project Summary — Mazerion (first-dev)

**Version:** 0.30.0  
**Last Updated:** December 29, 2025

**Mazerion** is a Rust 2024 workspace building the ultimate offline-first brewing toolkit, with precision calculators, encyclopedias, and management for all ferments.

## Primary Goal
Consolidate everything needed for brewing—from recipes to tracking—for all users, with hands-free voice and hardware integrations.

## Current Capabilities
- 47 precision calculators across multiple categories
- Modular crate architecture (core, calculators, cli, gui, tui, api, ffi, db)
- Dynamic calculator registry
- Themed GUI with unit system support
- C FFI layer ready for mobile integration
- Foundation for DB migration (isolation, dynamic)

## Design Principles
- Zero panics — all operations use Result/Error propagation
- Offline-first — no cloud dependencies
- Energy-efficient — optimized for mobile and desktop
- Secure — data isolation, read-only masters, encrypted user data
- Extensible — dynamic discovery of databases, packs, and features

## Near-Term Roadmap
- Wire `crates/db` with rusqlite; schemas for domains
- Dynamic registries for packs/encyclopedias
- Voice STT/TTS integration (Vosk)
- Tilt BLE parsing
- Fresh vs. frozen comparator full integration

## Long-Term Roadmap
- Full Android app (Kotlin wrapper, Play Store)
- Wear OS support (Galaxy Watch 7)
- Enterprise features (reporting, compliance, multi-user)
- Optional paid packs (encyclopedias, historical recipes, creator recipes)
- Hardware expansions (more sensors)
- Chef integrations (ferments in culinary workflows)

## Monetization
- Core app: One-time purchase ($9.99 home / $99 pro)
- Optional packs: $4.99–$19.99 each (encyclopedias, recipe collections)
- B2B: Tiered licensing for breweries, wineries, restaurants

Mazerion — Brew Better. Forever.