//! Mazerion CLI launcher.

use mazerion_core::Result;
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("gui") => {
            mazerion_gui::run()?;
        }
        Some("tui") => {
            mazerion_tui::run()?;
        }
        Some("list") => {
            println!("Available calculators:");
            for calc_id in mazerion_core::list_calculators() {
                println!("  - {}", calc_id);
            }
        }
        _ => {
            println!("Mazerion - Precision Mead & Beverage Calculator");
            println!("\nUsage:");
            println!("  mazerion gui      Launch GUI");
            println!("  mazerion tui      Launch TUI");
            println!("  mazerion list     List calculators");
        }
    }

    Ok(())
}
