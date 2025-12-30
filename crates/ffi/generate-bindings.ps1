# Mazerion UniFFI Kotlin Binding Generator (Windows)

Write-Host '=== Mazerion UniFFI Kotlin Binding Generator ===' -ForegroundColor Green

# Build the library with bindgen feature
Write-Host 'Building Rust library with bindgen...' -ForegroundColor Cyan
cargo build --release -p mazerion-ffi --features bindgen

if ($LASTEXITCODE -ne 0) {
    Write-Host 'Build failed!' -ForegroundColor Red
    exit 1
}

# Create output directory
Write-Host 'Creating output directory...' -ForegroundColor Cyan
New-Item -ItemType Directory -Force -Path 'bindings/kotlin' | Out-Null

# Generate Kotlin bindings
Write-Host 'Generating Kotlin bindings...' -ForegroundColor Cyan
cargo run --bin uniffi_bindgen --features bindgen -- generate --library target/release/mazerion_ffi.dll --language kotlin --out-dir bindings/kotlin

if ($LASTEXITCODE -ne 0) {
    Write-Host 'Binding generation failed!' -ForegroundColor Red
    exit 1
}

# List generated files
Write-Host ''
Write-Host 'Generated files:' -ForegroundColor Green
Get-ChildItem -Path 'bindings/kotlin' -Filter '*.kt' | ForEach-Object {
    Write-Host "  - $($_.Name)" -ForegroundColor Gray
}

Write-Host ''
Write-Host 'Kotlin bindings generated in bindings/kotlin/' -ForegroundColor Green
Write-Host ''
Write-Host 'Next steps:' -ForegroundColor Yellow
Write-Host '1. Copy bindings/kotlin/*.kt to your Kotlin project'
Write-Host '2. Add the native library to your resources'
Write-Host '3. Import and use: import mazerion.*'