use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::PathBuf;
use regex::Regex;
use blink::search::search_files;

// Constants for different test patterns
pub const TEST_PATTERNS: &[(&str, &str)] = &[
    ("Simple Word", "TODO"),
    ("Case Insensitive", "(?i)fixme"),
    ("Complex Regex", "class.*\\{"),
];

pub fn run_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_patterns");
    
    // Benchmark different pattern types
    for &(name, pattern) in TEST_PATTERNS {
        group.bench_function(name, |b| {
            let regex = Regex::new(pattern).unwrap();
            let path = PathBuf::from("benchmark_data/synthetic/complex");
            
            b.iter(|| {
                black_box(search_files(
                    &regex,
                    &[path.clone()],
                    false,
                    &[],
                    &[],
                ))
            });
        });
    }
    group.finish();

    // Benchmark different dataset sizes
    let mut group = c.benchmark_group("dataset_sizes");
    let sizes = ["small", "medium", "large"];
    
    for size in sizes {
        group.bench_function(size, |b| {
            let regex = Regex::new("TODO").unwrap();
            let path = PathBuf::from(format!("benchmark_data/synthetic/basic_{}_files", size));
            
            b.iter(|| {
                black_box(search_files(
                    &regex,
                    &[path.clone()],
                    false,
                    &[],
                    &[],
                ))
            });
        });
    }
    group.finish();

    // Benchmark different file types
    let mut group = c.benchmark_group("file_types");
    let types = ["rs", "py", "js", "ts", "go"];
    
    for ext in types {
        group.bench_function(ext, |b| {
            let regex = Regex::new("TODO").unwrap();
            let path = PathBuf::from("benchmark_data/synthetic/complex");
            
            b.iter(|| {
                black_box(search_files(
                    &regex,
                    &[path.clone()],
                    false,
                    &[ext.to_string()],
                    &[],
                ))
            });
        });
    }
    group.finish();
}

criterion_group!(benches, run_benchmarks);
criterion_main!(benches); 