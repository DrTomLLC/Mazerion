use mazerion_core::get_all_calculators;
use std::env;

// Force calculators to link
use mazerion_calculators as _;

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
            println!("Available Calculators ({}):\n", get_all_calculators().len());
            for calc in get_all_calculators() {
                println!("  {} - {}", calc.id(), calc.name());
                println!("    {}", calc.description());
                println!();
            }
        }
        _ => {
            println!("Mazerion - Precision Beverage Calculator");
            println!("\nUsage:");
            println!("  mazerion gui   - Launch GUI");
            println!("  mazerion tui   - Launch TUI");
            println!("  mazerion list  - List all calculators");
        }
    }
}