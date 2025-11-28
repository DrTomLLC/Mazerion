use std::env;
use mazerion_api::ApiEngine;

fn main() {
    // Make sure calculators are registered.
    mazerion_calculators::init();

    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(String::as_str);

    // Create the stable API engine once.
    let api = ApiEngine::new();

    match mode {
        Some("gui") => {
            if let Err(e) = mazerion_gui::run() {
                eprintln!("GUI error: {}", e);
                std::process::exit(1);
            }
        }
        Some("tui") => {
            if let Err(e) = mazerion_tui::run() {
                eprintln!("TUI error: {}", e);
                std::process::exit(1);
            }
        }
        Some("list") => {
            println!("🍯 Mazerion - Available Calculators (39 Total):\n");

            let calcs = api.list_calculator_ids();
            for (i, calc_id) in calcs.iter().enumerate() {
                println!("  {:2}. {}", i + 1, calc_id);
            }

            println!("\nRun with: mazerion gui | tui");
        }
        _ => {
            println!("🍯 Mazerion - Professional Beverage Calculator Suite");
            println!("39 Production-Ready Calculators\n");
            println!("Usage:");
            println!("  mazerion gui   - Launch GUI (recommended)");
            println!("  mazerion tui   - Launch Terminal UI");
            println!("  mazerion list  - List all 39 calculators");
        }
    }
}
