use mazerion_core::traits::get_all_calculators;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(String::as_str);

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
            println!("🍯 Mazerion - Available Calculators:\n");
            let calculators = get_all_calculators();

            // Group by category
            let mut by_category: std::collections::HashMap<String, Vec<_>> = std::collections::HashMap::new();
            for calc in calculators {
                by_category.entry(calc.category().to_string())
                    .or_default()
                    .push(calc);
            }

            // Sort categories
            let mut categories: Vec<_> = by_category.keys().collect();
            categories.sort();

            for category in categories {
                println!("📂 {}:", category.to_uppercase());
                let calcs = &by_category[category];
                for calc in calcs {
                    println!("  • {} ({})", calc.name(), calc.id());
                    println!("    {}", calc.description());
                }
                println!();
            }

            println!("Total: {} calculators", get_all_calculators().len());
        }
        _ => {
            println!("🍯 Mazerion - Precision Beverage Calculator v0.2.0");
            println!("\nUsage:");
            println!("  mazerion gui   - Launch GUI (recommended)");
            println!("  mazerion tui   - Launch terminal UI");
            println!("  mazerion list  - List all calculators");
            println!("\nFor help and documentation, visit the README.md");
        }
    }
}