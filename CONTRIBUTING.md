# Contributing to Mazerion

## Adding a New Calculator

### Step 1: Create Calculator File

Create `crates/calculators/src/my_calc.rs`:

```rust
use mazerion_core::{register_calculator, Calculator, CalcInput, CalcResult, Result};
use rust_decimal::Decimal;

#[derive(Default)]
pub struct MyCalculator;

impl MyCalculator {
    pub const ID: &'static str = "my_calc";
}

impl Calculator for MyCalculator {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        "My Calculator"
    }

    fn description(&self) -> &'static str {
        "What this calculator does"
    }

    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        // Get inputs - NO unwrap/expect/panic!
        let value = input.get_decimal("value")?;
        
        // Validate
        if value <= Decimal::ZERO {
            return Err(Error::OutOfRange("Value must be positive".into()));
        }
        
        // Calculate
        let result = value * Decimal::from(2);
        
        // Return result
        Ok(CalcResult::new(result))
    }
}

register_calculator!(MyCalculator);
```

### Step 2: Export Calculator

Add to `crates/calculators/src/lib.rs`:

```rust
pub mod my_calc;
pub use my_calc::MyCalculator;
```

### Step 3: Add GUI Interface

Add to appropriate tab file in `crates/gui/src/tabs/`:

```rust
egui::CollapsingHeader::new("My Calculator").show(ui, |ui| {
    ui.horizontal(|ui| {
        ui.label("Value:");
        ui.text_edit_singleline(&mut state.my_value);
    });
    
    if ui.button("Calculate").clicked() {
        state.result = helpers::calc("my_calc", vec![
            ("value", &state.my_value)
        ]);
    }
});
```

### Step 4: Update State

Add to `crates/gui/src/state.rs`:

```rust
pub my_value: String,
```

Initialize in `Default`:

```rust
my_value: String::new(),
```

## Code Standards

- **No panics**: Never use unwrap, expect, panic, todo, unimplemented
- **File size**: Maximum 150 lines per file
- **Error handling**: Always return Result
- **Testing**: Add tests for edge cases
- **Documentation**: Add doc comments for public items

## Testing

Run all tests:
```bash
cargo test --all-features
```

Check line limits:
```bash
cargo run --bin line-guard
```

## Validation

Use existing validators from mazerion-core:
- validate_sg
- validate_brix
- validate_ph
- validate_temperature
- validate_volume
- validate_percentage
