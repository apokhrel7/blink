# Blink <!-- omit in toc -->

A fast command-line utility for searching text patterns in files written in Rust.

## Table of Contents <!-- omit in toc -->

- [Features](#features)
- [When to Use Blink](#when-to-use-blink)
- [Performance](#performance)
  - [Benchmark Results](#benchmark-results)
    - [Simple Text Search (Pattern: "TODO")](#simple-text-search-pattern-todo)
    - [Case-Insensitive Search (Pattern: "(?i)fixme")](#case-insensitive-search-pattern-ifixme)
    - [Regex Pattern Search (Pattern: "class")](#regex-pattern-search-pattern-class)
    - [Multi-threading Impact (Pattern: "TODO")](#multi-threading-impact-pattern-todo)
    - [Simple Text Search](#simple-text-search)
    - [Case-Insensitive Search](#case-insensitive-search)
    - [Regex Pattern Search](#regex-pattern-search)
    - [Parallelization and Multi-threading Benefits](#parallelization-and-multi-threading-benefits)
  - [Customizing Benchmarks](#customizing-benchmarks)
  - [Areas for Improvement](#areas-for-improvement)
  - [Hardware Note](#hardware-note)
- [Production/Release](#productionrelease)
  - [Prerequisites](#prerequisites)
  - [Options](#options)
  - [Examples](#examples)
- [Testing/Development](#testingdevelopment)
  - [Examples](#examples-1)
  - [Running Tests](#running-tests)
  - [Development Commands](#development-commands)
- [Contributing](#contributing)

## Features

- Fast parallel file searching using multiple threads
- Regular expression pattern matching
- Case-sensitive and case-insensitive search modes
- File extension filtering
- Hidden file inclusion/exclusion
- Colored output highlighting
- Binary file detection and skipping
- Cross-platform support

## When to Use Blink
- Best for: Simple text searches across large codebases
- Good for: Case-insensitive searches on medium/large datasets
- Consider alternatives for: Heavy regex usage on very small datasets

## Performance

Blink is designed for speed, leveraging Rust's performance and parallel processing capabilities. 

To run the benchmarks:

**1. Generate Test Data**:
```bash
cargo run --bin generate_test_data
```
Creates three datasets: small (100 files), medium (1000 files), and large (10000 files)

**2. Run Benchmarks**:

There are two types of benchmarks available:

a) Comparative Benchmarks (Recommended for users):
   ```bash
   # On Windows:
   .\benches\scripts\run_benchmarks.ps1
   ```
   ```bash
   # On Unix:
   ./benches/scripts/run_benchmarks.sh
   ```
   These scripts use hyperfine to compare Blink against other search tools (ripgrep, findstr) in real-world scenarios.

b) Internal Benchmarks (For developers):
   ```bash
   cargo bench
   ```
   These use Criterion.rs to measure detailed internal metrics and are mainly useful when developing/optimizing Blink.

**3. View Results**:
The benchmarks generate several Markdown reports:

Search results:
- `results_[type_of_files]_small.md`: 100-file dataset
- `results_[type_of_files]_medium.md`: 1000-file dataset
- `results_[type_of_files]_large.md`: 10000-file dataset

Additional reports:
- `threading_results.md`: Multi-threading performance comparison
- `benchmark_report.md`: Combined report of all results


### Benchmark Results

The comparative benchmarks test:
1. **Search Speed**: How quickly files can be searched
2. **Threading Impact**: Performance with different thread counts
3. **Statistical Variance**: Consistency of performance
4. **Comparative Performance**: Against other search tools

*The following shows how Blink performed against other popular search tools on a large dataset of 10,000 files:*

#### Simple Text Search (Pattern: "TODO")
| Tool | Mean [ms] | Min [ms] | Max [ms] |
|:---|---:|---:|---:|
| Blink | 34.3 ± 6.1 | 23.2 | 47.0 |
| findstr | 49.9 ± 7.8 | 35.8 | 68.5 |
| ripgrep | 48.1 ± 4.9 | 42.5 | 61.7 |

#### Case-Insensitive Search (Pattern: "(?i)fixme")
| Tool | Mean [ms] | Min [ms] | Max [ms] |
|:---|---:|---:|---:|
| Blink | 28.8 ± 4.5 | 21.4 | 36.3 |
| findstr | 33.4 ± 9.5 | 24.2 | 58.2 |
| ripgrep | 50.3 ± 7.1 | 41.6 | 68.9 |

#### Regex Pattern Search (Pattern: "class")
| Tool | Mean [ms] | Min [ms] | Max [ms] |
|:---|---:|---:|---:|
| Blink | 46.2 ± 11.3 | 29.7 | 69.3 |
| findstr | 30.0 ± 2.3 | 25.1 | 34.4 |
| ripgrep | 55.1 ± 7.7 | 42.9 | 69.7 |

#### Multi-threading Impact (Pattern: "TODO")
| Threads | Mean [ms] | Min [ms] | Max [ms] | Relative Speed |
|:---|---:|---:|---:|---:|
| 1 thread | 56.8 ± 9.3 | 44.1 | 80.7 | 1x (baseline) |
| 4 threads | 25.1 ± 4.5 | 16.9 | 34.3 | 2.26x faster |
| 8 threads | 25.4 ± 19.5 | 15.4 | 106.5 | ~2.26x faster |


[View detailed benchmark methodology and complete results](benchmark_report.md)

*Note: Results from Windows 11, Intel Core i5-10210U, 16 GB RAM. Your results may vary based on hardware.*

#### Simple Text Search
   - Blink performs competitively, often matching or beating both findstr and ripgrep
   - Most consistent performance across dataset sizes
   - Average latency around 30-35ms regardless of dataset size

#### Case-Insensitive Search
   - Mixed results: excellent performance on medium/large datasets
   - Some instability on small datasets (high variance)
   - Generally faster than ripgrep but trades leads with findstr

#### Regex Pattern Search
   - Competitive but not leading performance
   - Findstr generally performs better for regex patterns
   - More consistent than ripgrep but with room for optimization

#### Parallelization and Multi-threading Benefits
   - Optimal performance with 4 threads (2.26x speedup)
   - No additional benefit from 8 threads
   - Recommendation: Use `-j 4` for best results



### Customizing Benchmarks

You can customize the benchmarks by modifying the following files in the `benches/` directory:

1. **Test Patterns** (`suite/mod.rs`):
   - Modify `TEST_PATTERNS` array to add/change search patterns
   - Each pattern needs a name and regex pattern string
   ```rust
   pub const TEST_PATTERNS: &[(&str, &str)] = &[
       ("Simple Word", "TODO"),
       ("Case Insensitive", "(?i)fixme"),
       // Add your patterns here
   ];
   ```

2. **Dataset Generation** (`data/mod.rs`):
   - Adjust `PATTERNS` array to change searchable content
   - Modify `sizes` array to change dataset sizes
   - Current defaults: [(100, 10), (1000, 50), (10000, 100)]
   ```rust
   const PATTERNS: &[&str] = &["TODO", "FIXME", "NOTE", "ERROR", "WARNING"];
   ```

3. **Language Templates** (`templates/`):
   - Add/modify language template files (e.g., `rust_template.rs`)
   - Templates are used for language-specific benchmarks
   - Current languages: Rust, Python, JavaScript, TypeScript, Go

4. **Benchmark Parameters** (`suite/mod.rs`):
   - Adjust file type filters in `file_types` array
   - Modify dataset paths in `benchmark_group` functions
   - Change benchmark group configurations

After modifying parameters:
1. Regenerate test data: `cargo run --bin generate_test_data`
2. Run benchmarks: `cargo bench`
3. [Run the benchmark suite](#performance)

### Areas for Improvement
1. High variance in case-insensitive search on small datasets
2. Regex pattern matching performance could be optimized
3. Thread scaling beyond 4 threads needs investigation

### Hardware Note
These benchmarks were run on Windows with specific hardware. Your results may vary based on:
- CPU characteristics
- File system performance
- Dataset characteristics
- Search pattern complexity



## Production/Release
This section is useful when:
- You want maximum performance
- You're using it as a regular command-line tool
- You need to run it from any directory
- You've finished development

The installed version will always be faster as it's pre-compiled in release mode and doesn't need to check for changes.
### Prerequisites

- Rust toolchain (rustc, cargo) - Install via [rustup](https://rustup.rs/)
- For Windows users: Microsoft Visual Studio Build Tools with C++ build tools
- Git for version control


1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd blink
   ```

2. Build in debug mode (for development):
   ```bash
   cargo build
   ```

3. Build for release (optimized):
   ```bash
   cargo build --release
   ```

   The binary will be available at:
   - Windows: `target/release/blink.exe`
   - Unix-like: `target/release/blink`

For regular usage, install and use the `blink` command directly:
```bash
# Install blink globally
cargo install --path .

# Use from any directory
blink <pattern> [path...]
```

*Note: if you don't specify a path, the search will be performed in the current directory by default.*

### Options
- `-i, --case-insensitive`: Perform case-insensitive matching
- `-H, --hidden`: Include hidden files and directories
- `-e, --extensions <EXTENSIONS>`: Filter by file extension (e.g., "rs,txt")
- `-j, --threads <N>`: Number of worker threads (defaults to CPU cores)
- `-x, --exclude <PATTERNS>`: Exclude files/directories matching these patterns (comma-separated)

### Examples

```bash
# Search for "TODO" in current directory
blink "TODO"

# Search for "TODO" but exclude benchmark and test directories
blink -x benchmark,test "TODO"

# Search for "error" excluding specific file types and directories
blink -x node_modules,target,.git -e rs,txt "error"

# Search for a phrase with spaces
blink "hello world"

# Search for regex pattern with special characters in src directory
blink "(TODO|FIXME)" src/

# Case-insensitive search for "error" in src directory
blink -i "error" src/

# Search for "fn" in Rust files only inside src directory
blink -e rs "fn" src/

# # Search for "TODO" including hidden files and directories
blink -h "TODO" 

# Search with 8 threads and regex pattern
blink -j 8 "(test|spec)"

# Search for "error" case-insensitively in only Rust and text files
blink -i -e rs,txt "error"
```


## Testing/Development
This section is useful when:
- You're actively developing or debugging
- You want to test changes immediately
- You need debug symbols and stack traces
- You're working within the project directory

Using `cargo run` is best for development and debugging:
```bash
# Debug build (slower, with debug symbols)
cargo run -- <pattern> [path...]

# Release build (faster, optimized)
cargo run --release -- <pattern> [path...]
```
*Note: The `--` is required to separate cargo's arguments from blink's arguments*

### Examples

```bash
# Search for "TODO" in current directory
cargo run -- "TODO"

# Case-insensitive search with file extension filter
cargo run -- -i -e rs "fn" src/
```
  
### Running Tests
```bash
# Run all tests
cargo test

# Run only case sensitivity tests (-i command)
cargo test case_sensitivity

# Run only hidden files tests (-h command)
cargo test hidden_files

# Run only extension filtering tests (-e command)
cargo test extension_filtering

# Run only folder exclusion tests (-x command)
cargo test file_exclusion

# Run tests with output
cargo test -- --nocapture

# Run only CLI tests
cargo test --test cli

# Run only performance tests
cargo test --test benchmark
```

The CLI tests verify:
- Basic search functionality
- Binary file detection

The performance tests measure search speed across:
- Small dataset (100 files)
- Medium dataset (1000 files)
- Large dataset (10000 files)

(optional) Run the test suite with coverage (requires cargo-tarpaulin):
```bash
cargo install cargo-tarpaulin
cargo tarpaulin
```

### Development Commands

```bash
# Check for compilation errors without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy

# Run documentation tests
cargo test --doc
```

## Contributing

Pull request template and CI/CD workflow will be created soon. Having said that, feel free to submit a PR or fork the project!

