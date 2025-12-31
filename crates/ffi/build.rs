// Build script for UniFFI scaffolding generation

fn main() {
    match uniffi::generate_scaffolding("./src/mazerion.udl") {
        Ok(_) => println!("cargo:rerun-if-changed=src/mazerion.udl"),
        Err(e) => {
            eprintln!("Error generating scaffolding: {}", e);
            std::process::exit(1);
        }
    }
}