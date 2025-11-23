use std::fs;
use std::path::PathBuf;

const MAX_LINES: usize = 150;

fn main() {
    let mut violations = Vec::new();
    let mut total_files = 0;
    let mut total_lines = 0;

    let paths = vec![
        "crates/core/src",
        "crates/calculators/src",
        "crates/config/src",
        "crates/db/src",
        "crates/gui/src",
        "crates/tui/src",
        "crates/cli/src",
    ];

    for path in paths {
        check_directory(path, &mut violations, &mut total_files, &mut total_lines);
    }

    println!("Line Guard Report");
    println!("=================");
    println!("Total files checked: {}", total_files);
    println!("Total lines: {}", total_lines);
    println!("Max lines per file: {}", MAX_LINES);
    println!();

    if violations.is_empty() {
        println!("✅ All files are within the {} line limit!", MAX_LINES);
    } else {
        println!("❌ {} file(s) exceed the line limit:", violations.len());
        for (file, lines) in violations {
            println!("  {} - {} lines (excess: {})", file, lines, lines - MAX_LINES);
        }
        std::process::exit(1);
    }
}

fn check_directory(
    path: &str,
    violations: &mut Vec<(String, usize)>,
    total_files: &mut usize,
    total_lines: &mut usize,
) {
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            check_directory(&path.to_string_lossy(), violations, total_files, total_lines);
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            check_file(&path, violations, total_files, total_lines);
        }
    }
}

fn check_file(
    path: &PathBuf,
    violations: &mut Vec<(String, usize)>,
    total_files: &mut usize,
    total_lines: &mut usize,
) {
    if let Ok(content) = fs::read_to_string(path) {
        let line_count = content.lines().count();
        *total_files += 1;
        *total_lines += line_count;

        if line_count > MAX_LINES {
            violations.push((path.to_string_lossy().to_string(), line_count));
        }
    }
}
