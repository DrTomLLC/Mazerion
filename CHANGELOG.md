# Changelog

All notable changes to Mazerion will be documented in this file.

## [0.8.0] - 2025-12-02

### Added
- Theme system with 5 color schemes (Honey Gold, Forest Green, Ocean Blue, Sunset Orange, Lavender Purple)
- Unit system toggle (Imperial/US Standard vs Metric)
- Decimal precision controls (SG: 1-6, pH: 1-4, Brix: 0-4)
- Dynamic label updates based on selected unit system
- Volume labels show L or gal
- Temperature labels show °C or °F
- Weight labels show g or oz

### Changed
- Result text increased to size 28 for better visibility
- Result panel now has black text on white background with green border
- All calculators now respect unit system setting
- Settings tab now functional with working controls

### Fixed
- Result text visibility (was invisible with light colors)
- Theme changes now apply immediately
- Unit system changes reflected in all calculator labels
- All GUI compilation errors resolved
- Zero warnings in final build

## [0.7.0] - 2025-12-01

### Added
- GUI implementation with egui
- TUI implementation with ratatui
- 39 calculators across 7 categories
- Calculator auto-registration system
- Hot-reload configuration

### Fixed
- GUI missing run() function
- TUI unused Result warning
- FFI unsafe code warnings
- State.rs import issues

## Previous versions
See PROJECT_SUMMARY.md for complete history.