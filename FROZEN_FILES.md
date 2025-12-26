# FROZEN FILES - DO NOT MODIFY

**Version:** 0.25.0  
**Last Updated:** December 25, 2024  
**Status:** ACTIVE FREEZE

---

## ⚠️ CRITICAL NOTICE

The files listed in this document are **FROZEN** and must **NOT** be modified except under the strict conditions outlined in this document. These files represent working, tested, production-ready code that has been thoroughly debugged and verified.

**Modifying frozen files without authorization will break working functionality and waste development time.**

---

## 🔒 FROZEN GUI TABS (v0.25.0)

These tabs are fully functional, thoroughly tested, and production-ready.

### 1. Basic Calculators Tab
**File:** `crates/gui/src/tabs/basic.rs`  
**Status:** 🔒 FROZEN  
**Lines:** ~450  
**Last Verified:** v0.25.0 (2024-12-25)

**Description:**  
Core brewing calculators for everyday use.

**Features:**
- ABV Calculator (OG/FG → ABV%)
- Brix ↔ Specific Gravity converter
- Dilution calculator (gravity adjustment)
- Temperature-corrected gravity readings
- Full Imperial/Metric unit support
- Real-time calculation updates
- Comprehensive error handling
- User-friendly result display

**Dependencies:**
- mazerion_core::CalcInput
- mazerion_core::traits::get_calculator
- AppState, UnitSystem, CustomColors

**DO NOT MODIFY** - Working perfectly since v0.25.0

---

### 2. Advanced Calculators Tab
**File:** `crates/gui/src/tabs/advanced.rs`  
**Status:** 🔒 FROZEN  
**Lines:** ~550  
**Last Verified:** v0.25.0 (2024-12-25)

**Description:**  
Advanced brewing calculations for precision brewers.

**Features:**
- **Refractometer Corrections:**
   - Terrill cubic formula (most accurate)
   - Standard refractometer correction
   - Linear refractometer correction
   - OG/Brix input with FG calculation
- **Priming Sugar Calculator:**
   - Multiple sugar types (corn sugar, table sugar, DME, honey)
   - Volume-based calculations
   - Target CO2 levels
- **Pitch Rate Calculator:**
   - Cell count calculations
   - Viability adjustments
   - Starter recommendations
- **Yeast Viability:**
   - Manufacturing date tracking
   - Cell count decay calculations
   - Storage condition adjustments

**Dependencies:**
- All mazerion_core calculators
- Date/time handling for viability
- Comprehensive unit conversions

**DO NOT MODIFY** - Working perfectly since v0.25.0

---

### 3. Brewing Calculators Tab
**File:** `crates/gui/src/tabs/brewing.rs`  
**Status:** 🔒 FROZEN  
**Lines:** ~600  
**Last Verified:** v0.25.0 (2024-12-25)

**Description:**  
Specialized brewing and fermentation calculators.

**Features:**
- **TOSNA Nutrition Calculators:**
   - TOSNA 2.0 protocol
   - TOSNA 3.0 protocol
   - Staggered nutrient addition schedules
   - YAN calculations
   - Fermaid O/K recommendations
- **Gravity Calculations:**
   - Must gravity calculations
   - Strike water temperature
   - Mash efficiency
- **Must Calculations:**
   - Volume calculations
   - Weight calculations
   - Honey requirements
   - Water additions

**Dependencies:**
- TOSNA protocol calculators
- Gravity conversion utilities
- Volume/weight conversions

**DO NOT MODIFY** - Working perfectly since v0.25.0

---

### 4. Utilities Tab
**File:** `crates/gui/src/tabs/utilities.rs`  
**Status:** 🔒 FROZEN  
**Lines:** ~400  
**Last Verified:** v0.25.0 (2024-12-25)

**Description:**  
Utility calculators and helper functions.

**Features:**
- **Hydrometer Temperature Correction:**
   - Celsius and Fahrenheit support
   - Accurate correction formulas
   - Real-time corrections
- **Bottles Per Batch:**
   - 12 oz, 22 oz, 750 mL bottle calculations
   - Batch volume to bottle count
   - Loss factor adjustments
- **Alcohol Tolerance:**
   - Yeast strain tolerance calculations
   - ABV prediction
   - Attenuation estimates
- Full unit system integration
- Clean, user-friendly UI

**Dependencies:**
- Temperature conversion utilities
- Volume calculations
- Yeast database (for tolerance)

**DO NOT MODIFY** - Working perfectly since v0.25.0

---

### 5. Settings Tab
**File:** `crates/gui/src/tabs/settings.rs`  
**Status:** 🔒 FROZEN  
**Lines:** ~250  
**Last Verified:** v0.25.0 (2024-12-25)

**Description:**  
Application settings and user preferences.

**Features:**
- **Unit System Toggle:**
   - Imperial/US Standard (gallons, pounds, °F)
   - Metric (liters, kilograms, °C)
   - Persistent across sessions
   - Instant application-wide updates
- **Theme Selection:**
   - Color scheme customization
   - Custom color definitions
   - Preview support
- **Settings Persistence:**
   - Save/load preferences
   - Default value management
- Clean, intuitive UI

**Dependencies:**
- AppState management
- UnitSystem enum
- CustomColors struct
- Persistence layer

**DO NOT MODIFY** - Working perfectly since v0.25.0

---

### 6. Mead Encyclopedia (PERMANENTLY FROZEN)
**File:** `crates/gui/src/tabs/mead_encyclopedia.rs`  
**Status:** ⛔ **PERMANENTLY FROZEN - NEVER MODIFY** ⛔  
**Lines:** 980+  
**Last Verified:** v0.25.0 (2024-12-25)

**Description:**  
Complete mead styles encyclopedia and reference guide. This is the **crown jewel** of the application - irreplaceable reference content.

**Content Breakdown:**
- **5 Major Categories:**
   1. Traditional Meads
   2. Fruit Meads (Melomels)
   3. Spiced Meads (Metheglins)
   4. Specialty Meads
   5. Advanced Techniques

- **8 Substyle Groups:**
   1. Berry Melomels (raspberry, blueberry, blackberry, strawberry, etc.)
   2. Stone Fruit Melomels (cherry, peach, plum, apricot)
   3. Tropical Melomels (mango, pineapple, passion fruit)
   4. Tea Meads (black, green, oolong, herbal)
   5. Barrel-Aged Meads (oak, bourbon, wine barrels)
   6. Sour Meads (Lactobacillus, Brett, mixed fermentation)
   7. Sparkling Meads (carbonation methods, champagne-style)
   8. Coffee & Chocolate Meads

- **Detailed Style Information:**
   - Historical background
   - Typical characteristics (ABV, sweetness, color)
   - Ingredient profiles
   - Fermentation notes
   - Food pairings
   - Serving recommendations
   - Common variations
   - Troubleshooting tips

- **Quick Reference:**
   - Sweetness level table (Dry, Semi-sweet, Sweet, Dessert)
   - ABV ranges by style
   - Honey variety recommendations
   - Yeast strain suggestions

**⚠️ CRITICAL WARNING:**  
This file contains 980+ lines of curated reference content that represents extensive research and compilation. It has been **intentionally separated** from the calculator UI to prevent accidental modification or deletion.

**ANY modification to this file is STRICTLY FORBIDDEN.**

**Why This is Permanently Frozen:**
1. Irreplaceable reference content
2. Extensive research invested
3. No functional code (pure content)
4. Already complete and comprehensive
5. High risk of accidental damage if modified

**Dependencies:**
- AppState (for rendering only)
- egui UI components
- NO calculator logic (pure reference)

**NEVER MODIFY THIS FILE UNDER ANY CIRCUMSTANCES**

---

## 📦 FROZEN GUI CORE FILES (v0.25.0)

These files form the core structure of the GUI application.

### 7. Application State
**File:** `crates/gui/src/state.rs`  
**Status:** 🔒 FROZEN (limited modifications allowed)  
**Lines:** ~300  
**Last Verified:** v0.25.0 (2024-12-25)

**Description:**  
Central state management for the entire GUI application.

**Contains:**
- **AppState Struct:**
   - All calculator input fields
   - All calculator output fields
   - Unit system preference
   - Theme/color settings
   - Tab state management
   - Conversion state
   - All mead calculator fields
   - All beer calculator fields
   - Warning/metadata storage

- **UnitSystem Enum:**
   - Imperial variant
   - Metric variant
   - Conversion logic

- **CustomColors Struct:**
   - Theme color definitions
   - Honey gold, forest green, sunset orange, etc.
   - Consistent color palette

- **Default Implementations:**
   - Sane default values for all fields
   - Empty strings for inputs
   - Default unit system (Imperial)
   - Default theme

**Allowed Modifications:**
- ✅ Adding new fields for new features (e.g., new calculator inputs)
- ✅ Adding default values for new fields
- ✅ Adding new calculator state fields

**NOT Allowed:**
- ❌ Modifying existing field types
- ❌ Removing existing fields
- ❌ Changing enum variants
- ❌ Refactoring for "improvement"
- ❌ Changing default values of existing fields

**Rationale:**  
This file is the backbone of state management. Existing fields are used by frozen tabs and cannot be changed without breaking those tabs.

**Dependencies:**
- serde (for serialization)
- egui::Color32 (for colors)
- Copy/Clone/Debug traits

**FROZEN** - New fields only, existing fields immutable

---

### 8. Main Application Structure
**File:** `crates/gui/src/lib.rs`  
**Status:** 🔒 FROZEN (new tabs can be added)  
**Lines:** ~500  
**Last Verified:** v0.25.0 (2024-12-25)

**Description:**  
Main application structure, update/render loops, and tab management.

**Contains:**
- **MazerionApp Struct:**
   - AppState instance
   - Result display fields
   - Warning/metadata vectors
   - Tab-specific state

- **TabView Enum:**
   - Basic, Advanced, Brewing, Recipe, Mead, Beer, Wine, Cider
   - Finishing, Packaging, Utilities, Settings
   - Future tab variants

- **Application Lifecycle:**
   - new() - App initialization
   - update() - Main update loop
   - render() - UI rendering
   - Tab switching logic
   - Result display
   - Warning display

- **Helper Functions:**
   - input_field() - Standard input widget
   - calculate_button() - Standard calculate button
   - Result formatting
   - Error display

**Allowed Modifications:**
- ✅ Adding new tabs to TabView enum
- ✅ Adding new tab render calls in update()
- ✅ Adding new helper functions
- ✅ Adding new application-wide utilities

**NOT Allowed:**
- ❌ Modifying core app structure (MazerionApp fields)
- ❌ Changing update/render loop logic
- ❌ Refactoring existing tab rendering
- ❌ Changing existing helper function signatures
- ❌ Removing existing tabs

**Rationale:**  
This is the application foundation. The update/render loop is proven to work correctly with all frozen tabs. Core structure changes would break everything.

**Dependencies:**
- eframe/egui (UI framework)
- All tab modules
- AppState
- mazerion_core

**FROZEN** - New tabs only, core structure immutable

---

## 🚧 FILES UNDER ACTIVE DEVELOPMENT (NOT FROZEN)

These files are **NOT frozen** and can be modified freely:

### GUI Tabs (In Progress)
- `crates/gui/src/tabs/meads.rs` - Mead style calculators
- `crates/gui/src/tabs/beer.rs` - Beer style calculators
- `crates/gui/src/tabs/wine.rs` - Wine calculators
- `crates/gui/src/tabs/cider.rs` - Cider calculators
- `crates/gui/src/tabs/finishing.rs` - Finishing process calculators
- `crates/gui/src/tabs/tabs_conversions.rs` - Unit conversion utilities
- `crates/gui/src/tabs/packaging.rs` - Packaging calculators
- `crates/gui/src/tabs/recipe.rs` - Recipe management (if exists)

### Supporting Files
- Any new tab files being developed
- Test files
- Documentation files
- Build scripts
- Configuration files

---

## 🛡️ FREEZE RULES AND PROTOCOLS

### ✅ ALLOWED Actions

**With Frozen Files:**
- Read/view frozen files for reference
- Copy code patterns from frozen files
- Use frozen files as examples
- Import from frozen files
- Call functions in frozen files
- Link to frozen files in documentation

**With Protected Files (state.rs, lib.rs):**
- Add new fields to AppState for new features
- Add default values for new fields in Default impl
- Add new tabs to TabView enum
- Add new tab render calls
- Add new helper functions (non-breaking)

### ❌ FORBIDDEN Actions

**With ALL Frozen Files:**
- Modifying existing code "for improvement"
- Refactoring frozen code
- Changing function signatures
- Changing struct field types
- Removing fields or functions
- Renaming fields or functions
- "Cleaning up" code
- Optimizing frozen code
- Updating dependencies that break frozen code
- Copy-pasting frozen code into non-frozen files and modifying it

**With Mead Encyclopedia (PERMANENT FREEZE):**
- **ABSOLUTELY NO MODIFICATIONS UNDER ANY CIRCUMSTANCES**
- Not even for typos (unless critical factual error)
- Not for formatting
- Not for "improvements"
- Not for reorganization
- **THIS FILE IS COMPLETELY OFF-LIMITS**

### 🆘 EXCEPTIONS - Critical Bugs Only

Frozen files may **ONLY** be modified for:

1. **Security Vulnerabilities:**
   - Memory safety issues
   - Injection vulnerabilities
   - Authentication/authorization bypasses
   - Data exposure risks

2. **Safety-Critical Bugs:**
   - Panic/crash bugs
   - Undefined behavior
   - Data corruption
   - Loss of user data

3. **Critical Functional Bugs:**
   - Calculator produces wrong results (verified)
   - UI completely non-functional
   - Cannot save/load state
   - Critical accessibility issues

**Minor bugs, UI tweaks, performance optimizations, code style issues, and "improvements" DO NOT qualify as exceptions.**

---

## 📝 MODIFICATION PROTOCOL

If a frozen file **MUST** be modified (exception applies):

### Step 1: Documentation
- Create detailed bug report
- Include reproduction steps
- Document expected vs actual behavior
- Provide evidence this meets exception criteria
- Estimate impact and urgency

### Step 2: Approval
- Get explicit approval from project lead
- Discuss alternative solutions first
- Verify change is absolutely necessary
- Confirm no workarounds exist

### Step 3: Minimal Change
- Make the **smallest possible fix**
- Change only what's necessary
- Do not "improve while you're there"
- Do not refactor surrounding code
- Keep diff as small as possible

### Step 4: Testing
- Test the specific bug fix
- Test all related functionality
- Run full test suite
- Verify no regressions in frozen features
- Test with both unit systems
- Test all affected calculators

### Step 5: Documentation
- Update CHANGELOG.md with change details
- Update this file (FROZEN_FILES.md) if needed
- Document in commit message
- Add code comments explaining the fix
- Update version number if needed

### Step 6: Review
- Code review required
- Extra scrutiny for frozen files
- Verify minimal change principle followed
- Check for unintended consequences

---

## 🎯 PHILOSOPHY

### Why Freeze Files?

1. **Prevent Regression**
   - Working code stays working
   - No accidental breakage
   - Stable foundation for development

2. **Clear Boundaries**
   - Developers know what's safe to touch
   - Reduces decision fatigue
   - Focuses development effort

3. **Faster Development**
   - No re-fixing working code
   - No regression debugging
   - Time spent on new features

4. **Protect Valuable Content**
   - Mead encyclopedia is irreplaceable
   - Hundreds of hours of work protected
   - Reference content stays intact

5. **Version Stability**
   - v0.25.0 features are locked
   - Users can rely on stable behavior
   - Predictable application behavior

### The Frozen Files Represent:

- **Hundreds of hours** of development work
- **Extensive testing** and debugging
- **User feedback** incorporated
- **Production-ready** quality code
- **Zero-panic** safety guarantees
- **Comprehensive** error handling
- **Full unit system** support
- **Proven** calculation accuracy

**They work perfectly. Leave them alone.**

---

## 📊 VERSION HISTORY

### v0.25.0 (2024-12-25) - Initial Comprehensive Freeze
**Frozen:**
- basic.rs - Basic calculators tab
- advanced.rs - Advanced calculators tab
- brewing.rs - Brewing calculators tab
- utilities.rs - Utilities tab
- settings.rs - Settings tab
- mead_encyclopedia.rs - Mead encyclopedia (PERMANENT FREEZE)
- state.rs - Application state (protected, new fields allowed)
- lib.rs - Main app structure (protected, new tabs allowed)

**Status:** All frozen files verified working
**Tests:** All passing
**Warnings:** All resolved
**Coverage:** Comprehensive

---

## 🔐 ENFORCEMENT

This freeze is enforced through:

1. **Documentation** (this file)
   - Clear rules and protocols
   - Explicit lists of frozen files
   - Modification procedures

2. **Code Review**
   - All changes to frozen files require review
   - Extra scrutiny applied
   - Approval process mandatory

3. **Version Control**
   - Git history tracks all changes
   - Blame shows who modified frozen files
   - Diff reviews catch unauthorized changes

4. **Developer Discipline**
   - Respect the freeze
   - Follow the protocols
   - Ask before modifying

5. **Automated Checks** (future)
   - CI/CD pipelines could check frozen files
   - Automated diff analysis
   - Frozen file hash verification

---

## 🚨 BEFORE YOU MODIFY A FROZEN FILE

**STOP and ask yourself:**

1. Is this modification **absolutely necessary**?
   - 99% of the time, the answer is **NO**

2. Does this meet **exception criteria**?
   - Security vulnerability?
   - Safety-critical bug?
   - Critical functional bug?
   - If no to all three → **DO NOT MODIFY**

3. Have I explored **all alternatives**?
   - Can I work around the issue?
   - Can I fix it elsewhere?
   - Can I add new code instead of modifying frozen code?

4. Have I gotten **approval**?
   - Did I document the issue?
   - Did I get explicit permission?
   - Is this properly tracked?

5. Am I making the **minimal change**?
   - Smallest possible fix?
   - No "improvements"?
   - No refactoring?
   - No scope creep?

**If you cannot answer YES to all relevant questions, DO NOT MODIFY THE FROZEN FILE.**

---

## 📚 RESOURCES

### For Developers

**Working with Frozen Code:**
- Read frozen files for examples
- Copy patterns (don't modify originals)
- Import and use frozen functions
- Build on top of frozen foundation

**Adding New Features:**
- Add new tabs (don't modify existing)
- Add new fields to state.rs
- Add new helper functions
- Create new calculators

**Best Practices:**
- Respect the freeze
- Follow established patterns
- Match coding style of frozen files
- Maintain zero-panic discipline
- Use proper error handling

### Documentation

- See frozen files for implementation examples
- Read CHANGELOG.md for version history
- Check inline comments in frozen files
- Review test files for usage examples

---

## 🎁 SUMMARY

**8 Files are FROZEN:**
1. ✅ basic.rs
2. ✅ advanced.rs
3. ✅ brewing.rs
4. ✅ utilities.rs
5. ✅ settings.rs
6. ⛔ mead_encyclopedia.rs (PERMANENT)
7. 🔒 state.rs (protected)
8. 🔒 lib.rs (protected)

**These represent:**
- ~3,500 lines of production code
- 5 complete, working calculator tabs
- 1 comprehensive reference encyclopedia
- Complete unit system support
- Zero-panic safety guarantees
- Hundreds of hours of work

**Your responsibility:**
- Respect the freeze
- Follow the protocols
- Build on the foundation
- Don't break what works

---

## 🔒 FINAL WORD

**FROZEN FILES ARE FROZEN FOR A REASON.**

They work. They're tested. They're safe. They're complete.

**Leave them alone.**

Focus your energy on:
- Completing in-progress tabs
- Adding new features
- Finishing what's not done

**Not on:**
- "Improving" what works
- Refactoring frozen code
- "Cleaning up" stable code

**The frozen files are a gift to future development. They provide a stable foundation. Protect them.**

---

**Version 0.25.0 - Frozen and Protected**  
**DO NOT MODIFY FROZEN FILES**  
**RESPECT THE FREEZE** 🔒