fn main() {
    // Safe generation with proper error handling
    if let Err(e) = uniffi_build::generate_scaffolding("./src/mazerion.udl") {
        // Print a clear, helpful message and exit with error code
        eprintln!("Failed to generate UniFFI scaffolding:");
        eprintln!("  Error: {e}");
        eprintln!("  Check your mazerion.udl file for syntax errors.");
        eprintln!("  Common issues: missing braces, incorrect types, or misplaced semicolons.");
        std::process::exit(1);
    }
}