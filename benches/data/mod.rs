use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

const PATTERNS: &[&str] = &["TODO", "FIXME", "NOTE", "ERROR", "WARNING"];

pub fn generate_test_data() -> std::io::Result<()> {
    let test_dir = PathBuf::from("benchmark_data");
    generate_basic_datasets(&test_dir)?;
    generate_complex_datasets(&test_dir)?;
    Ok(())
}

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
                if rand::random::<f32>() < 0.1 {
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

fn main() -> std::io::Result<()> {
    println!("Generating test data...");
    generate_test_data()?;
    println!("Test data generation complete!");
    Ok(())
} 