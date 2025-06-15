//! Benchmark data generation module
//! 
//! This module is responsible for generating synthetic test data used in benchmarking.
//! It creates two types of datasets:
//! 1. Basic datasets: Simple text files with controlled pattern density
//! 2. Complex datasets: Language-specific files using real-world code templates
//!
//! The generated data structure:
//! ```text
//! benchmark_data/
//! ├── synthetic/
//! │   ├── basic_small_files/    # 100 files × 10 lines
//! │   ├── basic_medium_files/   # 1000 files × 50 lines
//! │   ├── basic_large_files/    # 10000 files × 100 lines
//! │   └── complex/              # Language-specific files
//! │       ├── rust/
//! │       ├── python/
//! │       ├── javascript/
//! │       ├── typescript/
//! │       └── go/
//! ```

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Common patterns to insert into test files.
/// These patterns are randomly distributed with 10% density.
const PATTERNS: &[&str] = &["TODO", "FIXME", "NOTE", "ERROR", "WARNING"];

/// Generate all test datasets for benchmarking.
/// 
/// This function creates both basic and complex datasets under the benchmark_data directory.
/// - Basic datasets contain simple text files with controlled pattern density
/// - Complex datasets contain language-specific files based on templates
pub fn generate_test_data() -> std::io::Result<()> {
    let test_dir = PathBuf::from("benchmark_data");
    generate_basic_datasets(&test_dir)?;
    generate_complex_datasets(&test_dir)?;
    Ok(())
}

/// Generate basic test datasets with different sizes.
/// 
/// Creates three datasets:
/// - Small: 100 files × 10 lines each
/// - Medium: 1000 files × 50 lines each
/// - Large: 10000 files × 100 lines each
/// 
/// Each file contains:
/// - 10% of lines with random patterns from PATTERNS
/// - 90% of lines with regular text
fn generate_basic_datasets(base_dir: &Path) -> std::io::Result<()> {
    let sizes = [(100, 10), (1000, 50), (10000, 100)];
    
    for (num_files, lines_per_file) in sizes {
        let size_name = match num_files {
            100 => "small",
            1000 => "medium",
            10000 => "large",
            _ => unreachable!(),
        };
        
        let dir = base_dir.join("synthetic").join(format!("basic_{}_files", size_name));
        fs::create_dir_all(&dir)?;
        
        for i in 0..num_files {
            let file_path = dir.join(format!("file_{}.txt", i));
            let mut file = File::create(file_path)?;
            
            for _ in 0..lines_per_file {
                if rand::random::<f32>() < 0.1 {  // 10% pattern density
                    let pattern = PATTERNS[rand::random::<usize>() % PATTERNS.len()];
                    writeln!(file, "This line contains a {}", pattern)?;
                } else {
                    writeln!(file, "Just a regular line of text without any special patterns")?;
                }
            }
        }
    }
    Ok(())
}

/// Generate complex test datasets with language-specific content.
/// 
/// Creates a directory for each supported language containing 100 files.
/// Each file is based on a language-specific template that includes:
/// - Realistic code structure
/// - Common language patterns
/// - Typical file organization
/// 
/// Current supported languages:
/// - Rust (.rs)
/// - Python (.py)
/// - JavaScript (.js)
/// - TypeScript (.ts)
/// - Go (.go)
fn generate_complex_datasets(base_dir: &Path) -> std::io::Result<()> {
    let complex_dir = base_dir.join("synthetic").join("complex");
    fs::create_dir_all(&complex_dir)?;
    
    // Generate files with different programming languages
    let languages = [
        ("rust", "rs", include_str!("../templates/rust_template.rs")),
        ("python", "py", include_str!("../templates/python_template.py")),
        ("javascript", "js", include_str!("../templates/javascript_template.js")),
        ("typescript", "ts", include_str!("../templates/typescript_template.ts")),
        ("go", "go", include_str!("../templates/go_template.go")),
    ];
    
    for (lang, ext, template) in languages {
        let lang_dir = complex_dir.join(lang);
        fs::create_dir_all(&lang_dir)?;
        
        for i in 0..100 {
            let file_path = lang_dir.join(format!("example_{}.{}", i, ext));
            let mut file = File::create(file_path)?;
            write!(file, "{}", template)?;
        }
    }
    
    Ok(())
}

/// Entry point for the test data generation binary.
/// 
/// Run with:
/// ```bash
/// cargo run --bin generate_test_data
/// ```
fn main() -> std::io::Result<()> {
    println!("Generating test data...");
    generate_test_data()?;
    println!("Test data generation complete!");
    Ok(())
} 