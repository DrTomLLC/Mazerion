//! Production CLI with automatic MCL integration

use mazerion_gui::run;
use mazerion_core::{CalcInput, get_calculator, get_calculators_by_category, VALID_CATEGORIES};
use std::env;

fn main() {
    // CRITICAL: Initialize calculators to force linking
    mazerion_calculators::init();

    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(String::as_str);

    match command {
        Some("gui") => launch_gui(),
        Some("tui") => launch_tui(),
        Some("list") => list_calculators(),
        Some("categories") => list_categories(),
        Some("calc") => execute_calculator(&args[2..]),
        Some("help") | Some("--help") | Some("-h") => show_help(),
        _ => show_help(),
    }
}

fn launch_gui() {
    println!("🚀 Launching GUI...");
    if let Err(e) = run() {
        eprintln!("❌ GUI error: {}", e);
        std::process::exit(1);
    }
}

fn launch_tui() {
    println!("🚀 Launching TUI...");
    if let Err(e) = mazerion_tui::run() {
        eprintln!("❌ TUI error: {}", e);
        std::process::exit(1);
    }
}

fn list_calculators() {
    let by_category = get_calculators_by_category();

    println!("═══════════════════════════════════════════════════════════");
    println!("MAZERION CALCULATORS");
    println!("═══════════════════════════════════════════════════════════");
    println!();

    let mut total = 0;

    for category in VALID_CATEGORIES {
        if let Some(calcs) = by_category.get(*category) {
            println!("📂 {} ({} calculators)", category, calcs.len());
            println!("───────────────────────────────────────────────────────────");

            for calc in calcs {
                println!("  🔹 {} ({})", calc.name(), calc.id());
                println!("     {}", calc.description());
            }

            println!();
            total += calcs.len();
        }
    }

    println!("═══════════════════════════════════════════════════════════");
    println!("Total: {} calculators", total);
    println!("═══════════════════════════════════════════════════════════");
}

fn list_categories() {
    let by_category = get_calculators_by_category();

    println!("═══════════════════════════════════════════════════════════");
    println!("MAZERION CATEGORIES");
    println!("═══════════════════════════════════════════════════════════");

    for category in VALID_CATEGORIES {
        let count = by_category.get(*category).map(|v| v.len()).unwrap_or(0);
        if count > 0 {
            println!("  📂 {} - {} calculators", category, count);
        }
    }

    println!("═══════════════════════════════════════════════════════════");
}

fn execute_calculator(args: &[String]) {
    if args.is_empty() {
        eprintln!("❌ Error: Calculator ID required");
        eprintln!();
        eprintln!("Usage: mazerion calc <calculator_id> param=value ...");
        eprintln!();
        eprintln!("Example:");
        eprintln!("  mazerion calc abv og=1.090 fg=1.010");
        eprintln!("  mazerion calc dilution current_volume=20 current_abv=14 target_abv=10");
        eprintln!();
        eprintln!("Run 'mazerion list' to see all available calculators");
        std::process::exit(1);
    }

    let calc_id = &args[0];

    let calc = match get_calculator(calc_id) {
        Some(c) => c,
        None => {
            eprintln!("❌ Error: Calculator '{}' not found", calc_id);
            eprintln!();
            eprintln!("Run 'mazerion list' to see all available calculators");
            std::process::exit(1);
        }
    };

    let mut input = CalcInput::new();

    for arg in &args[1..] {
        if let Some((key, value)) = arg.split_once('=') {
            input = input.add_param(key.trim(), value.trim());
        } else {
            eprintln!("⚠️  Warning: Invalid parameter format: '{}'", arg);
            eprintln!("   Expected: key=value");
        }
    }

    println!("═══════════════════════════════════════════════════════════");
    println!("🧮 {}", calc.name());
    println!("   {}", calc.description());
    println!("───────────────────────────────────────────────────────────");

    match calc.calculate(input) {
        Ok(result) => {
            println!("✓ Result: {}", result.output);

            if !result.warnings.is_empty() {
                println!();
                println!("⚠️  Warnings:");
                for warning in &result.warnings {
                    println!("   • {}", warning);
                }
            }

            if !result.metadata.is_empty() {
                println!();
                println!("ℹ️  Additional Information:");
                for (key, value) in &result.metadata {
                    println!("   • {}: {}", key, value);
                }
            }

            println!("═══════════════════════════════════════════════════════════");
        }
        Err(e) => {
            eprintln!();
            eprintln!("❌ Calculation Error: {}", e);
            eprintln!("═══════════════════════════════════════════════════════════");
            std::process::exit(1);
        }
    }
}

fn show_help() {
    println!("═══════════════════════════════════════════════════════════");
    println!("🍯 MAZERION - PRECISION BEVERAGE CALCULATOR");
    println!("═══════════════════════════════════════════════════════════");
    println!();
    println!("USAGE:");
    println!("  mazerion <command> [options]");
    println!();
    println!("COMMANDS:");
    println!("  gui              Launch graphical interface (recommended)");
    println!("  tui              Launch terminal interface");
    println!("  list             List all available calculators");
    println!("  categories       List all categories with counts");
    println!("  calc <id> ...    Execute a calculator from command line");
    println!("  help             Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("  mazerion gui");
    println!("  mazerion list");
    println!("  mazerion calc abv og=1.090 fg=1.010");
    println!("  mazerion calc dilution current_volume=20 current_abv=14 target_abv=10");
    println!("  mazerion calc carbonation volume=19 temperature=20 target_co2=2.5 method=priming");
    println!();
    println!("MCL VERSION: Production v1.0");
    println!("Total Calculators: {}", mazerion_core::traits::calculator_count());
    println!("═══════════════════════════════════════════════════════════");
}