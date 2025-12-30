# Mazerion FFI

UniFFI-based Foreign Function Interface for Mazerion calculator library.

## Features

- ✅ Zero panics in production code
- ✅ Comprehensive input validation with size caps
- ✅ Typed error handling
- ✅ Batch operations for battery efficiency
- ✅ Kotlin Multiplatform bindings
- ✅ Android/Wear OS support

## Quick Start

### Generate Kotlin Bindings
```bash
# Windows
.\generate-bindings.ps1

# Linux/Mac
./generate-bindings.sh
```

### Use in Kotlin
```kotlin
import mazerion.*

// Get version
val version = version()

// List calculators
val calculators = listCalculators()

// Calculate ABV
val result = executeCalculator("abv", listOf(
    CalcParam("og", "1.080"),
    CalcParam("fg", "1.010")
))
println("ABV: ${result.value}${result.unit}")

// Batch operations
val results = executeBatch(listOf(
    BatchCalculatorRequest("abv", params1),
    BatchCalculatorRequest("abv", params2)
))
```

## Documentation

- [Android Integration Guide](ANDROID_INTEGRATION.md)
- [API Documentation](../../docs/FFI_API.md)

## Testing
```bash
cargo test -p mazerion-ffi
cargo clippy -p mazerion-ffi
```

## Building
```bash
# Desktop
cargo build --release -p mazerion-ffi

# Android (all ABIs)
./build-android.sh
```

## Security

- Hard size limits on all inputs
- No unwrap/expect/panic in production code
- Validation at FFI boundary
- Rate limiting ready (implemented but not enforced)