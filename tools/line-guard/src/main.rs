//! Line count enforcer for Mazerion.

use std::env;
use std::fs;
use std::path::Path;
use std::process;

const MAX_LINES: usize = 150;

fn count_lines(path: &Path) -> std::io::Result<usize> {
    let content = fs::read_to_string(path)?;
    Ok(content.lines().count())
}

fn check_file(path: &Path) -> bool {
    match count_lines(path) {
        Ok(count) if count > MAX_LINES => {
            eprintln!("❌ {}: {} lines (max {})", path.display(), count, MAX_LINES);
            false
        }
        Ok(count) => {
            println!("✓ {}: {} lines", path.display(), count);
            true
        }
        Err(e) => {
            eprintln!("⚠ {}: {}", path.display(), e);
            false
        }
    }
}

fn walk_dir(dir: &Path, pattern: &str) -> Vec<bool> {
    let mut results = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();

            if path.is_dir() && path.file_name().is_some_and(|n| n != "target") {
                results.extend(walk_dir(&path, pattern));
            } else if path.extension().and_then(|e| e.to_str()) == Some(pattern) {
                results.push(check_file(&path));
            }
        }
    }

    results
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let root = args.get(1).map(String::as_str).unwrap_or(".");

    println!(
        "🔍 Checking Rust files in {} (max {} lines per file)",
        root, MAX_LINES
    );

    let results = walk_dir(Path::new(root), "rs");

    let passed = results.iter().filter(|&&r| r).count();
    let failed = results.len().saturating_sub(passed);

    println!("\n📊 Summary: {} passed, {} failed", passed, failed);

    if failed > 0 {
        eprintln!("\n❌ Line limit violations detected!");
        process::exit(1);
    }

    println!("\n✅ All files within line limit");
}
