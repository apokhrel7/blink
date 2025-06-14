# Blink

A fast command-line utility for searching text patterns in files written in Rust.

## Table of Contents
- [Blink](#blink)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Performance](#performance)
    - [Benchmark Results](#benchmark-results)
    - [Multi-threading Performance](#multi-threading-performance)
    - [Benchmark Details](#benchmark-details)
  - [Usage](#usage)
    - [Options](#options)
    - [Examples:](#examples)
  - [Development](#development)
    - [Prerequisites](#prerequisites)
    - [Building from Source](#building-from-source)
    - [Running and Installation](#running-and-installation)
      - [Development/Testing Mode](#developmenttesting-mode)
      - [Production Mode](#production-mode)
    - [Examples:](#examples-1)
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

## Performance

Blink is designed for speed, leveraging Rust's performance and parallel processing capabilities. These benchmarks show significant performance advantages over traditional search tools.

To run the benchmarks:

**1. Generate Test Data**:
```bash
cargo run --bin generate_test_data
```
Creates three datasets: small (100 files), medium (1000 files), and large (10000 files)

**2. Run Benchmark Suite**:
```bash
# On Windows:
.\benchmarks\run_benchmarks.ps1

# On Unix:
./benchmarks/run_benchmarks.sh
```

The benchmark suite:
- Uses hyperfine for accurate measurements
- Performs 10 warmup runs
- Runs each test at least 20 times
- Measures statistical variance
- Compares against ripgrep and findstr
- Tests different thread counts

**3. View Results**:
The benchmarks generate several Markdown reports:
- `small_results.md`: Results for 100-file dataset
- `medium_results.md`: Results for 1000-file dataset
- `large_results.md`: Results for 10000-file dataset
- `threading_results.md`: Multi-threading performance comparison
- `benchmark_report.md`: Combined report of all results

### Benchmark Results

| Dataset Size | Blink | findstr | ripgrep | vs findstr | vs ripgrep |
|-------------|-----------|---------|----------|------------|------------|
| Small (100 files) | 44.7 ms | 45.6 ms | 87.3 ms | 1.02x faster | 1.95x faster |
| Medium (1000 files) | 58.5 ms | 44.4 ms | 90.5 ms | 0.76x slower | 1.55x faster |
| Large (10000 files) | 25.0 ms | 46.4 ms | 100.2 ms | 1.86x faster | 4.01x faster |

*Note: Results from Windows 11, Intel Core i5-10210U, 16 GB RAM. Your results may vary based on hardware.*

### Multi-threading Performance

Blink supports parallel processing with a simple `-j` flag to control thread count. This testing shows optimal performance with 4 threads on most systems:

| Thread Count | Time (ms) | vs Single Thread |
|-------------|-----------|------------------|
| 1 thread | 32.0 ± 6.2 | baseline |
| 4 threads | 26.3 ± 5.8 | 21.8% faster |
| 8 threads | 33.3 ± 7.8 | similar to single thread |

**Recommended Usage:**
- For most systems, using `-j 4` provides the best balance of performance and resource usage
- Adjust thread count based on your specific hardware if needed
- Performance may vary based on:
  - CPU core count and speed
  - Disk I/O capabilities
  - Dataset size and file distribution
  - Search pattern complexity
- Default thread count matches your CPU core count

### Benchmark Details

The benchmark suite tests:
1. **Search Speed**: How quickly files can be searched
2. **Threading Impact**: Performance with different thread counts
3. **Statistical Variance**: Consistency of performance
4. **Comparative Performance**: Against other search tools

Each test dataset contains:
- A mix of text files with random content
- ~10% of lines containing searchable patterns
- Varying file sizes and line counts
- Consistent pattern distribution

To run your own custom benchmarks or modify parameters, see the scripts in the `benchmarks/` directory.

## Usage

Basic usage:

```bash
blink <pattern> [optional path...]
```
*Note: if you don't specify a path, the search will be performed in the current directory by default.*

### Options
- `-i, --case-insensitive`: Perform case-insensitive matching
- `-h, --hidden`: Include hidden files and directories
- `-e, --extensions <EXTENSIONS>`: Filter by file extension (e.g., "rs,txt")
- `-j, --threads <N>`: Number of worker threads (defaults to CPU cores)

### Examples:

```bash
# Search for "TODO" in current directory
blink "TODO"

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

## Development

### Prerequisites

- Rust toolchain (rustc, cargo) - Install via [rustup](https://rustup.rs/)
- For Windows users: Microsoft Visual Studio Build Tools with C++ build tools
- Git for version control

### Building from Source

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

### Running and Installation

There are two main ways to use Blink:

#### Development/Testing Mode
Using `cargo run` is best for development and debugging:
```bash
# Debug build (slower, with debug symbols)
cargo run -- <pattern> [path...]

# Release build (faster, optimized)
cargo run --release -- <pattern> [path...]
```
*Note: The `--` is required to separate cargo's arguments from blink's arguments*

Use this mode when:
- You're actively developing or debugging
- You want to test changes immediately
- You need debug symbols and stack traces
- You're working within the project directory

#### Production Mode
For regular usage, install and use the `blink` command directly:
```bash
# Install blink globally
cargo install --path .

# Use from any directory
blink <pattern> [path...]
```

Use this mode when:
- You want maximum performance
- You're using it as a regular command-line tool
- You need to run it from any directory
- You've finished development

The installed version will always be faster as it's pre-compiled in release mode and doesn't need to check for changes.

### Examples:

```bash
# Search for "TODO" in current directory
cargo run -- "TODO"

# Case-insensitive search with file extension filter
cargo run -- -i -e rs "fn" src/
```

### Running Tests

Run the test suite:
```bash
# Run all tests
cargo test

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

Run the test suite with coverage (requires cargo-tarpaulin):
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

Contributions are welcome! Please feel free to submit a Pull Request.

