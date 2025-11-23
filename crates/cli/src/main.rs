use mazerion_core::get_all_calculators;
use std::env;

fn main() {
    mazerion_calculators::init();

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
            println!("Available Calculators:");
            for calc in get_all_calculators() {
                println!("  {} - {}", calc.id(), calc.name());
                println!("    {}", calc.description());
            }
        }
        _ => {
            println!("Mazerion - Precision Beverage Calculator");
            println!("\nUsage:");
            println!("  mazerion gui   - Launch GUI (recommended)");
            println!("  mazerion tui   - Launch TUI");
            println!("  mazerion list  - List all calculators");
        }
    }
}