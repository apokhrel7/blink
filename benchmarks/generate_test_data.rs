use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let test_dir = Path::new("benchmark_data");
    
    // Create test directories if they don't exist
    fs::create_dir_all(test_dir)?;
    fs::create_dir_all(test_dir.join("small"))?;
    fs::create_dir_all(test_dir.join("medium"))?;
    fs::create_dir_all(test_dir.join("large"))?;

    // Generate small dataset (100 files)
    generate_dataset(test_dir.join("small"), 100, 10)?;
    
    // Generate medium dataset (1000 files)
    generate_dataset(test_dir.join("medium"), 1000, 50)?;
    
    // Generate large dataset (10000 files)
    generate_dataset(test_dir.join("large"), 10000, 100)?;

    Ok(())
}

fn generate_dataset(dir: impl AsRef<Path>, num_files: u32, lines_per_file: u32) -> std::io::Result<()> {
    let patterns = ["TODO", "FIXME", "NOTE", "ERROR", "WARNING"];
    
    for i in 0..num_files {
        let file_path = dir.as_ref().join(format!("file_{}.txt", i));
        let mut file = File::create(file_path)?;
        
        for _ in 0..lines_per_file {
            if rand::random::<f32>() < 0.1 {
                // 10% chance to include a searchable pattern
                let pattern = patterns[rand::random::<usize>() % patterns.len()];
                writeln!(file, "This line contains a {}", pattern)?;
            } else {
                writeln!(file, "Just a regular line of text without any special patterns")?;
            }
        }
    }
    Ok(())
} 