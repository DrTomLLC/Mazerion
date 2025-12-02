# 🔒 FROZEN FILES - DO NOT MODIFY

## ✅ Version 0.8.0 - WORKING AND LOCKED

These files are **PRODUCTION READY** and **MUST NOT BE MODIFIED**.

All future changes MUST be done by creating NEW files, NOT editing these.

### GUI Core Files (FROZEN)
- `crates/gui/src/lib.rs` (134 lines) - Main app structure
- `crates/gui/src/state.rs` (120 lines) - App state with theme & units
- `crates/gui/src/tabs/mod.rs` - Tab exports
- `crates/gui/src/tabs/basic.rs` - Basic calculators
- `crates/gui/src/tabs/advanced.rs` - Advanced calculators
- `crates/gui/src/tabs/brewing.rs` - Brewing calculators
- `crates/gui/src/tabs/finishing.rs` - Finishing calculators
- `crates/gui/src/tabs/settings.rs` - Settings with theme/units

### Core System Files (FROZEN)
- `crates/core/src/lib.rs` - Core types
- `crates/core/src/error.rs` - Error handling
- `crates/core/src/traits.rs` - Calculator trait
- `crates/core/src/units.rs` - Unit definitions
- `crates/core/src/validation.rs` - Validators

### Calculator Files (FROZEN - ADD NEW ONLY)
- All 39 calculator files in `crates/calculators/src/`
- `crates/calculators/src/lib.rs` - Registry

### TUI/CLI Files (FROZEN)
- `crates/tui/src/lib.rs` - TUI implementation
- `crates/cli/src/main.rs` - CLI launcher

### Config Files (FROZEN)
- `crates/config/src/lib.rs` - Hot reload system
- `crates/db/src/lib.rs` - Database layer
- `crates/ffi/src/lib.rs` - FFI bindings

## 🚫 STRICT RULES

### What You CAN Do:
✅ Create NEW calculator files (e.g., `srm.rs`, `og_calculator.rs`)
✅ Create NEW tab files (e.g., `tabs/conversions.rs`, `tabs/utilities.rs`)
✅ Add NEW crates in `crates/` directory
✅ Add NEW tools in `tools/` directory
✅ Update version numbers in designated locations
✅ Add to `lib.rs` exports ONLY for new files

### What You CANNOT Do:
❌ Modify ANY function signatures in frozen files
❌ Change ANY existing calculator logic
❌ Alter ANY UI layout in working tabs
❌ Remove ANY existing features
❌ Refactor ANY working code
❌ "Improve" ANY functioning files

## 📋 HOW TO ADD NEW FEATURES

### Example: Adding New Calculator
1. Create `crates/calculators/src/new_calc.rs`
2. Implement `Calculator` trait
3. Call `register_calculator!(NewCalc)`
4. Add ONE LINE to `lib.rs`: `pub mod new_calc;`
5. DONE - Never touch other calculator files

### Example: Adding New Tab
1. Create `crates/gui/src/tabs/new_tab.rs`
2. Implement `pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui)`
3. Add to `tabs/mod.rs`:
    - `pub mod new_tab;`
    - `pub use new_tab::render as render_new_tab;`
4. Add enum variant to `state.rs` `TabView`
5. Add match arm in `lib.rs` update function
6. DONE - Never touch other tab files

### Example: Adding New Theme
1. Edit ONLY `state.rs` - add Theme enum variant
2. Add color values in `get_theme_colors()` method
3. Update settings.rs ONLY to add dropdown option
4. DONE

## 🎯 VERSION HISTORY

### v0.8.0 (Current - FROZEN)
- Theme system (5 themes)
- Unit system (Imperial/Metric)
- Decimal precision controls
- Dynamic label updates
- High contrast results (size 28, black text)
- 39 working calculators
- GUI/TUI/CLI all functional
- ZERO warnings, ZERO errors

### v0.7.0
- GUI compilation fixed
- Result visibility fixed
- 38 calculators working

## 🔐 ENFORCEMENT

If you need to modify a frozen file, you MUST:
1. Create a NEW version (e.g., `lib_v2.rs`)
2. Test thoroughly
3. Only replace after 100% working
4. Update this document

**NO EXCEPTIONS. NO REFACTORING. NO "IMPROVEMENTS".**

## 📞 EMERGENCY ONLY

If a frozen file MUST be changed (critical bug):
1. Document the bug clearly
2. Create minimal fix
3. Test exhaustively
4. Update version to 0.8.1
5. Document change in CHANGELOG.md

---

**Last Updated:** December 2, 2025  
**Frozen At:** Version 0.8.0  
**Status:** PRODUCTION READY - DO NOT TOUCH