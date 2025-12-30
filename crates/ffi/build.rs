// crates/ffi/build.rs

fn main() {
    match uniffi_build::generate_scaffolding("./src/mazerion.udl") {
        Ok(_) => {},
        Err(e) => {
            eprintln!("UniFFI scaffolding generation failed:");
            eprintln!("  Error: {e}");
            eprintln!("  Check your mazerion.udl file for syntax errors.");
            std::process::exit(1);
        }
    }
}