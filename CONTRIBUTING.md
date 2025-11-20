# Contributing to Mazerion

Thank you for your interest in contributing to Mazerion!

## Code of Conduct

- Be respectful and constructive
- Focus on technical merit
- Help others learn
- Keep discussions on-topic

## Development Setup

```bash
# Clone repository
git clone https://github.com/yourorg/mazerion
cd mazerion

# Build project
cargo build

# Run tests
cargo test --all-features

# Run line guard
cargo run --bin line-guard
```

## Contribution Guidelines

### 1. File Size Limit

**CRITICAL**: All `.rs` files must be ≤150 lines.

```bash
# Verify before committing
cargo run --bin line-guard
```

If a file exceeds 150 lines, split it into modules.

### 2. No Panics

**REQUIRED**: No panic-inducing code allowed.

Forbidden:
- `panic!()`
- `unwrap()`
- `expect()`
- `todo!()`
- `unimplemented!()`
- Index operations `[i]` without bounds check

Use instead:
- `Result<T, Error>`
- `.get(i)` for safe indexing
- Proper error handling with `?`

### 3. Error Handling

All functions should return `Result<T, Error>`:

```rust
pub fn my_function(input: &str) -> Result<Output> {
    let value = input.parse::<i32>()
        .map_err(|e| Error::Parse(format!("Invalid input: {}", e)))?;
    Ok(Output { value })
}
```

### 4. Testing

Add tests for all new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        let result = my_function("42");
        assert!(result.is_ok());
    }
}
```

### 5. Documentation

Document public APIs:

```rust
/// Calculate alcohol by volume from gravity readings.
///
/// # Arguments
/// * `og` - Original gravity
/// * `fg` - Final gravity
///
/// # Returns
/// ABV as percentage
pub fn calculate_abv(og: Decimal, fg: Decimal) -> Result<Decimal> {
    // ...
}
```

## Adding a Calculator

1. Create file in `crates/calculators/src/`:

```rust
//! Brief description.

use mazerion_core::{register_calculator, Calculator, /* ... */};

#[derive(Default)]
pub struct MyCalculator;

impl MyCalculator {
    pub const ID: &'static str = "my_calc";
}

impl Calculator for MyCalculator {
    fn id(&self) -> &'static str { Self::ID }
    fn name(&self) -> &'static str { "My Calculator" }
    fn description(&self) -> &'static str { "What it does" }
    
    fn calculate(&self, input: CalcInput) -> Result<CalcResult> {
        // Implementation
    }
}

register_calculator!(MyCalculator);
```

2. Export in `crates/calculators/src/lib.rs`:

```rust
pub mod my_calc;
pub use my_calc::MyCalculator;
```

3. Add tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculation() {
        let calc = MyCalculator;
        // Test logic
    }
}
```

That's it! The calculator auto-registers.

## Code Style

### Formatting

```bash
cargo fmt --all
```

### Linting

```bash
cargo clippy --all-targets --all-features
```

Fix all warnings before submitting.

### Naming Conventions

- Types: `PascalCase`
- Functions: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`

## Pull Request Process

1. **Fork** the repository
2. **Create branch**: `git checkout -b feature/my-feature`
3. **Make changes**
4. **Test**: `cargo test --all-features`
5. **Verify**: `./verify.sh`
6. **Commit**: Clear, descriptive messages
7. **Push**: `git push origin feature/my-feature`
8. **PR**: Submit with description

### PR Requirements

✅ All tests pass
✅ Line limits enforced (≤150)
✅ No clippy warnings
✅ Formatted with `cargo fmt`
✅ Documentation updated
✅ No panics

## Review Process

1. Automated CI checks
2. Code review by maintainers
3. Feedback and iteration
4. Approval and merge

## Getting Help

- **Issues**: GitHub issue tracker
- **Discussions**: GitHub discussions
- **Documentation**: README.md, ARCHITECTURE.md

## License

By contributing, you agree that your contributions will be licensed under MIT OR Apache-2.0.

## Recognition

Contributors are listed in CONTRIBUTORS.md (please add yourself!).

Thank you for making Mazerion better! 🍯
