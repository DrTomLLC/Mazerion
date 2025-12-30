# Complete verification script for Mazerion FFI

Write-Host "=== Mazerion FFI Verification ===" -ForegroundColor Green

$ErrorCount = 0

# Test compilation
Write-Host "`n1. Testing compilation..." -ForegroundColor Cyan
cargo build --release -p mazerion-ffi
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ Compilation failed" -ForegroundColor Red
    $ErrorCount++
} else {
    Write-Host "✓ Compilation successful" -ForegroundColor Green
}

# Run tests
Write-Host "`n2. Running tests..." -ForegroundColor Cyan
cargo test -p mazerion-ffi
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ Tests failed" -ForegroundColor Red
    $ErrorCount++
} else {
    Write-Host "✓ All tests passed" -ForegroundColor Green
}

# Run clippy
Write-Host "`n3. Running clippy..." -ForegroundColor Cyan
cargo clippy -p mazerion-ffi -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ Clippy warnings found" -ForegroundColor Red
    $ErrorCount++
} else {
    Write-Host "✓ No clippy warnings" -ForegroundColor Green
}

# Build with bindgen feature
Write-Host "`n4. Building with bindgen..." -ForegroundColor Cyan
cargo build --release -p mazerion-ffi --features bindgen
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ Bindgen build failed" -ForegroundColor Red
    $ErrorCount++
} else {
    Write-Host "✓ Bindgen build successful" -ForegroundColor Green
}

# Generate bindings
Write-Host "`n5. Generating Kotlin bindings..." -ForegroundColor Cyan
New-Item -ItemType Directory -Force -Path "bindings/kotlin" | Out-Null
cargo run --bin uniffi_bindgen --features bindgen -- generate `
    --library target/release/mazerion_ffi.dll `
    --language kotlin `
    --out-dir bindings/kotlin
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ Binding generation failed" -ForegroundColor Red
    $ErrorCount++
} else {
    Write-Host "✓ Bindings generated successfully" -ForegroundColor Green
}

# Check binding files
Write-Host "`n6. Verifying binding files..." -ForegroundColor Cyan
$KotlinFiles = Get-ChildItem -Path "bindings/kotlin" -Filter "*.kt" -ErrorAction SilentlyContinue
if ($KotlinFiles.Count -eq 0) {
    Write-Host "✗ No Kotlin files generated" -ForegroundColor Red
    $ErrorCount++
} else {
    Write-Host "✓ Found $($KotlinFiles.Count) Kotlin binding file(s)" -ForegroundColor Green
    foreach ($file in $KotlinFiles) {
        Write-Host "  - $($file.Name)" -ForegroundColor Gray
    }
}

# Check library file
Write-Host "`n7. Verifying library file..." -ForegroundColor Cyan
if (Test-Path "target/release/mazerion_ffi.dll") {
    $FileSize = (Get-Item "target/release/mazerion_ffi.dll").Length
    $FileSizeMB = [math]::Round($FileSize / 1MB, 2)
    Write-Host "✓ Library found: $FileSizeMB MB" -ForegroundColor Green
} else {
    Write-Host "✗ Library file not found" -ForegroundColor Red
    $ErrorCount++
}

# Summary
Write-Host "`n=== Verification Summary ===" -ForegroundColor Green
if ($ErrorCount -eq 0) {
    Write-Host "✓ All checks passed! UniFFI integration is complete." -ForegroundColor Green
    Write-Host "`nGenerated files:" -ForegroundColor Yellow
    Write-Host "  - target/release/mazerion_ffi.dll (Rust library)"
    if ($KotlinFiles) {
        foreach ($file in $KotlinFiles) {
            Write-Host "  - bindings/kotlin/$($file.Name) (Kotlin binding)"
        }
    }
    Write-Host "`nNext steps:" -ForegroundColor Yellow
    Write-Host "1. Copy bindings/kotlin/*.kt to your Android project"
    Write-Host "2. Copy target/release/mazerion_ffi.dll to your resources"
    Write-Host "3. For Android: Build with build-android.sh"
    Write-Host "4. Follow ANDROID_INTEGRATION.md for full setup"
} else {
    Write-Host "✗ $ErrorCount check(s) failed" -ForegroundColor Red
    exit 1
}