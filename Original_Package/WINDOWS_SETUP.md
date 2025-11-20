# Mazerion - Windows Setup Guide

## Quick Start (PowerShell)

### Option 1: Automated Setup

1. Download both files:
   - `mazerion.tar.gz` (workspace archive)
   - `Build-Mazerion.ps1` (setup script)

2. Place them in the same directory

3. Run PowerShell as Administrator (for first-time Rust setup) or regular user:
   ```powershell
   .\Build-Mazerion.ps1
   ```

4. The script will:
   - Check for Rust/Cargo
   - Extract the archive
   - Build the workspace
   - Run tests
   - Verify line limits

### Option 2: Manual Setup

1. Extract the archive:
   ```powershell
   tar -xzf mazerion.tar.gz
   ```

2. Navigate to directory:
   ```powershell
   cd mazerion
   ```

3. Build the project:
   ```powershell
   cargo build --release
   ```

4. Run the GUI:
   ```powershell
   cargo run --bin mazerion -- gui
   ```

## Prerequisites

### Install Rust (if not already installed)

1. Download from https://rustup.rs/
2. Run the installer
3. Follow the prompts (default options work great)
4. Restart your terminal/PowerShell
5. Verify installation:
   ```powershell
   rustc --version
   cargo --version
   ```

### Windows 10/11 Built-in Tools

Windows 10+ includes `tar` command, so no additional extraction tools needed!

## Available Commands

After building, you can run:

```powershell
# Launch GUI (egui)
cargo run --bin mazerion -- gui

# Launch TUI (terminal interface)
cargo run --bin mazerion -- tui

# List all calculators
cargo run --bin mazerion -- list

# Run tests
cargo test --all-features

# Check line limits
cargo run --bin line-guard

# Build with database feature
cargo build --features db
```

## Project Structure

```
mazerion/
├── crates/              # Rust crates
│   ├── core/           # Core types and traits
│   ├── calculators/    # Calculator implementations
│   ├── config/         # Configuration
│   ├── db/             # Optional database
│   ├── gui/            # egui GUI
│   ├── tui/            # ratatui TUI
│   └── cli/            # CLI launcher
├── tools/
│   └── line-guard/     # Line limit enforcer
├── config.toml         # Configuration file
├── ingredients.toml    # Ingredients database
└── README.md           # Documentation
```

## Troubleshooting

### "cargo: command not found"

Rust is not installed or not in PATH. Install from https://rustup.rs/

### Build fails with linking errors

You may need Visual Studio Build Tools:
- Download from: https://visualstudio.microsoft.com/downloads/
- Select "Desktop development with C++"
- Or run: `rustup default stable-msvc`

### PowerShell execution policy

If you get "script cannot be loaded":
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Slow first build

The first build downloads and compiles dependencies. This can take 5-10 minutes.
Subsequent builds are much faster (incremental compilation).

## Features

- ✅ Zero panics - all errors handled
- 🎯 Decimal precision with rust_decimal
- 🔌 Modular calculator system
- 🔥 Hot-reload configuration
- 🎨 Cross-platform GUI and TUI
- 📊 Optional SQLite database
- 📏 Enforced line limits (≤150 per file)

## Support

See README.md in the mazerion/ directory for full documentation.

## License

MIT OR Apache-2.0
