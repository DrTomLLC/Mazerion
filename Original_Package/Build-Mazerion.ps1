# Mazerion Builder - Complete Workspace Setup and Build Script
# Requires: PowerShell 5.1+ and Rust toolchain (rustup)

param(
    [string]$TargetDir = ".\mazerion",
    [switch]$SkipBuild,
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"

# Colors for output
function Write-Step { 
    param([string]$Message)
    Write-Host "`n🔹 $Message" -ForegroundColor Cyan 
}

function Write-Success { 
    param([string]$Message)
    Write-Host "✅ $Message" -ForegroundColor Green 
}

function Write-Failure { 
    param([string]$Message)
    Write-Host "❌ $Message" -ForegroundColor Red 
}

function Write-Info { 
    param([string]$Message)
    Write-Host "   $Message" -ForegroundColor Gray 
}

# Check prerequisites
function Test-Prerequisites {
    Write-Step "Checking prerequisites..."
    
    # Check Rust
    try {
        $rustVersion = & rustc --version 2>&1
        Write-Success "Rust found: $rustVersion"
    } catch {
        Write-Failure "Rust not found. Please install from https://rustup.rs/"
        exit 1
    }
    
    # Check Cargo
    try {
        $cargoVersion = & cargo --version 2>&1
        Write-Success "Cargo found: $cargoVersion"
    } catch {
        Write-Failure "Cargo not found."
        exit 1
    }
}

# Create directory structure
function New-DirectoryStructure {
    Write-Step "Creating directory structure..."
    
    $dirs = @(
        "$TargetDir",
        "$TargetDir\.github\workflows",
        "$TargetDir\crates\core\src",
        "$TargetDir\crates\calculators\src",
        "$TargetDir\crates\config\src",
        "$TargetDir\crates\db\src",
        "$TargetDir\crates\gui\src",
        "$TargetDir\crates\tui\src",
        "$TargetDir\crates\cli\src",
        "$TargetDir\tools\line-guard\src"
    )
    
    foreach ($dir in $dirs) {
        if (-not (Test-Path $dir)) {
            New-Item -ItemType Directory -Path $dir -Force | Out-Null
            Write-Info "Created: $dir"
        }
    }
    
    Write-Success "Directory structure created"
}

# Helper to create files
function New-ProjectFile {
    param(
        [string]$Path,
        [string]$Content
    )
    
    $fullPath = Join-Path $TargetDir $Path
    Set-Content -Path $fullPath -Value $Content -Encoding UTF8
    
    if ($Verbose) {
        Write-Info "Created: $Path"
    }
}

# Create all Cargo.toml files
function New-CargoManifests {
    Write-Step "Creating Cargo manifests..."
    
    # Root workspace Cargo.toml
    New-ProjectFile "Cargo.toml" @'
[workspace]
resolver = "2"
members = [
    "crates/core",
    "crates/calculators",
    "crates/config",
    "crates/db",
    "crates/gui",
    "crates/tui",
    "crates/cli",
    "tools/line-guard",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.83"
authors = ["Mazerion Team"]
license = "MIT OR Apache-2.0"

[workspace.dependencies]
rust_decimal = { version = "1.36", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
thiserror = "2.0"
anyhow = "1.0"

[workspace.lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"

[workspace.lints.clippy]
panic = "deny"
unwrap_used = "deny"
expect_used = "deny"
todo = "deny"
unimplemented = "deny"
indexing_slicing = "deny"

[profile.release]
lto = true
codegen-units = 1
strip = true
opt-level = 3
'@

    # Core crate
    New-ProjectFile "crates\core\Cargo.toml" @'
[package]
name = "mazerion-core"
version.workspace = true
edition.workspace = true
rust-version.workspace = true

[lints]
workspace = true

[dependencies]
rust_decimal = { workspace = true }
rust_decimal_macros = "1.36"
serde = { workspace = true }
thiserror = { workspace = true }
linkme = "0.3"
'@

    # Calculators crate
    New-ProjectFile "crates\calculators\Cargo.toml" @'
[package]
name = "mazerion-calculators"
version.workspace = true
edition.workspace = true
rust-version.workspace = true

[lints]
workspace = true

[dependencies]
mazerion-core = { path = "../core" }
rust_decimal = { workspace = true }
rust_decimal_macros = "1.36"
linkme = "0.3"
'@

    # Config crate
    New-ProjectFile "crates\config\Cargo.toml" @'
[package]
name = "mazerion-config"
version.workspace = true
edition.workspace = true
rust-version.workspace = true

[lints]
workspace = true

[dependencies]
mazerion-core = { path = "../core" }
serde = { workspace = true }
toml = { workspace = true }
rust_decimal = { workspace = true }
'@

    # DB crate
    New-ProjectFile "crates\db\Cargo.toml" @'
[package]
name = "mazerion-db"
version.workspace = true
edition.workspace = true
rust-version.workspace = true

[lints]
workspace = true

[features]
default = []
db = ["dep:rusqlite"]

[dependencies]
mazerion-core = { path = "../core" }
serde = { workspace = true }
rusqlite = { version = "0.32", features = ["bundled"], optional = true }
'@

    # GUI crate
    New-ProjectFile "crates\gui\Cargo.toml" @'
[package]
name = "mazerion-gui"
version.workspace = true
edition.workspace = true
rust-version.workspace = true

[lints]
workspace = true

[dependencies]
mazerion-core = { path = "../core" }
mazerion-calculators = { path = "../calculators" }
mazerion-config = { path = "../config" }
eframe = "0.29"
egui = "0.29"
rust_decimal = { workspace = true }
rust_decimal_macros = "1.36"
'@

    # TUI crate
    New-ProjectFile "crates\tui\Cargo.toml" @'
[package]
name = "mazerion-tui"
version.workspace = true
edition.workspace = true
rust-version.workspace = true

[lints]
workspace = true

[dependencies]
mazerion-core = { path = "../core" }
mazerion-calculators = { path = "../calculators" }
mazerion-config = { path = "../config" }
ratatui = "0.29"
crossterm = "0.28"
rust_decimal = { workspace = true }
'@

    # CLI crate
    New-ProjectFile "crates\cli\Cargo.toml" @'
[package]
name = "mazerion-cli"
version.workspace = true
edition.workspace = true
rust-version.workspace = true

[[bin]]
name = "mazerion"
path = "src/main.rs"

[lints]
workspace = true

[dependencies]
mazerion-core = { path = "../core" }
mazerion-calculators = { path = "../calculators" }
mazerion-config = { path = "../config" }
mazerion-gui = { path = "../gui" }
mazerion-tui = { path = "../tui" }
anyhow = { workspace = true }
'@

    # Line-guard tool
    New-ProjectFile "tools\line-guard\Cargo.toml" @'
[package]
name = "line-guard"
version.workspace = true
edition.workspace = true
rust-version.workspace = true

[[bin]]
name = "line-guard"
path = "src/main.rs"

[lints]
workspace = true
'@

    Write-Success "Cargo manifests created"
}

# Extract and build from tar archive
function Invoke-ExtractAndBuild {
    param([string]$ArchivePath)
    
    if (-not (Test-Path $ArchivePath)) {
        Write-Failure "Archive not found: $ArchivePath"
        Write-Info "Please download mazerion.tar.gz first"
        return $false
    }
    
    Write-Step "Extracting archive..."
    
    # Try to use tar (Windows 10+ has built-in tar)
    try {
        & tar -xzf $ArchivePath 2>&1 | Out-Null
        if ($LASTEXITCODE -ne 0) {
            Write-Failure "Extraction failed"
            return $false
        }
        Write-Success "Extracted successfully"
    } catch {
        Write-Failure "tar not available. Please extract mazerion.tar.gz manually"
        return $false
    }
    
    if (-not $SkipBuild) {
        return Invoke-Build -Dir ".\mazerion"
    }
    
    return $true
}

# Build the project
function Invoke-Build {
    param([string]$Dir)
    
    if (-not (Test-Path $Dir)) {
        Write-Failure "Directory not found: $Dir"
        return $false
    }
    
    Write-Step "Building project..."
    
    Push-Location $Dir
    
    try {
        Write-Info "Running cargo build..."
        & cargo build 2>&1 | Out-Null
        
        if ($LASTEXITCODE -ne 0) {
            Write-Failure "Build failed! Run 'cargo build' in $Dir for details"
            return $false
        }
        
        Write-Success "Build successful"
        
        Write-Info "Running tests..."
        & cargo test --all-features 2>&1 | Out-Null
        
        if ($LASTEXITCODE -ne 0) {
            Write-Failure "Tests failed! Run 'cargo test' in $Dir for details"
            return $false
        }
        
        Write-Success "Tests passed"
        
        Write-Info "Checking line limits..."
        & cargo run --bin line-guard 2>&1 | Out-Null
        
        if ($LASTEXITCODE -ne 0) {
            Write-Failure "Line limit violations"
            return $false
        }
        
        Write-Success "All files within line limit"
        
        return $true
    }
    finally {
        Pop-Location
    }
}

# Main execution
function Main {
    Write-Host @"

╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║   🍯  Mazerion Builder                                       ║
║   Precision Mead & Beverage Calculator                      ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝

"@ -ForegroundColor Cyan

    Test-Prerequisites
    
    # Check if we should extract from archive
    $archivePath = ".\mazerion.tar.gz"
    
    if (Test-Path $archivePath) {
        Write-Host "✓ Found mazerion.tar.gz" -ForegroundColor Green
        
        $success = Invoke-ExtractAndBuild -ArchivePath $archivePath
        
        if ($success) {
            Write-Host @"

╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║   ✅  SUCCESS!                                               ║
║                                                              ║
║   Mazerion workspace extracted and built successfully!      ║
║                                                              ║
║   Try it out:                                                ║
║   > cd mazerion                                              ║
║   > cargo run --bin mazerion -- gui                          ║
║   > cargo run --bin mazerion -- tui                          ║
║   > cargo run --bin mazerion -- list                         ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝

"@ -ForegroundColor Green
        }
    } else {
        Write-Host @"

╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║   📦  SETUP INSTRUCTIONS                                     ║
║                                                              ║
║   1. Download mazerion.tar.gz                                ║
║   2. Place it in the same directory as this script          ║
║   3. Run this script again: .\Build-Mazerion.ps1            ║
║                                                              ║
║   OR                                                         ║
║                                                              ║
║   Extract manually:                                          ║
║   > tar -xzf mazerion.tar.gz                                 ║
║   > cd mazerion                                              ║
║   > cargo build                                              ║
║   > cargo run --bin mazerion -- gui                          ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝

"@ -ForegroundColor Cyan
    }
}

# Run main
Main
