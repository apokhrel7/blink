#[cfg(test)]
mod benchmarks {
    use blink::search::{search_files};
    use regex::Regex;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    use std::time::Instant;

    fn create_test_files(dir: &str, num_files: u32, lines_per_file: u32) -> std::io::Result<()> {
        let patterns = ["TODO", "FIXME", "NOTE", "ERROR", "WARNING"];
        fs::create_dir_all(dir)?;
        
        for i in 0..num_files {
            let file_path = format!("{}/file_{}.txt", dir, i);
            let mut file = File::create(file_path)?;
            
            for _ in 0..lines_per_file {
                if rand::random::<f32>() < 0.1 {
                    let pattern = patterns[rand::random::<usize>() % patterns.len()];
                    writeln!(file, "This line contains a {}", pattern)?;
                } else {
                    writeln!(file, "Just a regular line of text without any special patterns")?;
                }
            }
        }
        Ok(())
    }

    #[test]
    fn benchmark_search_performance() {
        // Create test directories
        let test_dirs = [
            ("small", 100, 10),
            ("medium", 1000, 50),
            ("large", 10000, 100),
        ];

        for (size, num_files, lines_per_file) in test_dirs {
            let dir = format!("benchmark_data_{}", size);
            create_test_files(&dir, num_files, lines_per_file).unwrap();

            // Benchmark search
            let pattern = Regex::new("TODO").unwrap();
            let paths = vec![PathBuf::from(&dir)];
            
            let start = Instant::now();
            let results = search_files(&pattern, &paths, false, &[]).unwrap();
            let duration = start.elapsed();

            println!(
                "\nBenchmark results for {} dataset ({} files):",
                size, num_files
            );
            println!("Time taken: {:.2?}", duration);
            println!("Matches found: {}", results.len());
            println!("Files per second: {:.2}", num_files as f64 / duration.as_secs_f64());

            // Cleanup
            fs::remove_dir_all(&dir).unwrap();
        }
    }
} 