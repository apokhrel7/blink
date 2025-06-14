# Blink

A fast command-line utility for searching text patterns in files written in Rust.

## Table of Contents
- [Blink](#blink)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Performance](#performance)
    - [Benchmark Results](#benchmark-results)
    - [Multi-threading Performance](#multi-threading-performance)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Options](#options)
  - [Development](#development)
    - [Prerequisites](#prerequisites)
    - [Building from Source](#building-from-source)
    - [Running Locally](#running-locally)
    - [Installing Globally](#installing-globally)
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
- Context lines around matches
- Binary file detection and skipping
- Cross-platform support

## Performance

Blink is designed for speed, leveraging Rust's performance and parallel processing capabilities. Our benchmarks show significant performance advantages over traditional search tools.

### Benchmark Results

| Dataset Size | Blink | findstr | ripgrep | vs findstr | vs ripgrep |
|-------------|-----------|---------|----------|------------|------------|
| Small (100 files) | 44.7 ms | 45.6 ms | 87.3 ms | 1.02x faster | 1.95x faster |
| Medium (1000 files) | 58.5 ms | 44.4 ms | 90.5 ms | 0.76x | 1.55x faster |
| Large (10000 files) | 25.0 ms | 46.4 ms | 100.2 ms | 1.86x faster | 4.01x faster |

*Note: Lower times are better. Results from Windows 10, AMD Ryzen 7 5800X*

### Multi-threading Performance

Blink supports parallel processing with a simple `-j` flag to control thread count. Our testing shows optimal performance with 4 threads on most systems:

| Thread Count | Time (ms) | vs Single Thread |
|-------------|-----------|------------------|
| 1 thread | 32.0 ± 6.2 | baseline |
| 4 threads | 26.3 ± 5.8 | 21.8% faster |
| 8 threads | 33.3 ± 7.8 | similar to single thread |

**Recommended Usage:**
- For most systems, using `-j 4` provides the best balance of performance and resource usage
- Adjust thread count based on your specific hardware if needed
- Default thread count matches your CPU core count

## Installation

Using Cargo:

```bash
cargo install blink
```

## Usage

Basic usage:

```bash
blink <pattern> [path...]
```

Examples:

```bash
# Search for "TODO" in current directory
blink TODO

# Case-insensitive search for "error" in src directory
blink -i error src/

# Search for "fn" in Rust files only
blink -e rs "fn" src/

# Show 2 lines of context around matches
blink -C 2 "panic!" src/

# Search with 8 threads
blink -j 8 "test"
```

### Options

- `-i, --case-insensitive`: Perform case-insensitive matching
- `-h, --hidden`: Include hidden files and directories
- `-e, --extensions <EXTENSIONS>`: Filter by file extension (e.g., "rs,txt")
- `-C, --context-lines <N>`: Number of context lines to show
- `-j, --threads <N>`: Number of worker threads (defaults to CPU cores)

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

### Running Locally

You can run the program directly using Cargo:

```bash
cargo run -- <pattern> [path...]
```

For example:
```bash
# Search for "TODO" in current directory
cargo run -- TODO

# Case-insensitive search with file extension filter
cargo run -- -i -e rs "fn" src/
```

### Installing Globally

To install the binary globally on your system:

```bash
cargo install --path .
```

This will install the `blink` binary to your Cargo binary directory (usually `~/.cargo/bin/` on Unix-like systems or `%USERPROFILE%\.cargo\bin` on Windows).

### Running Tests

Run the test suite:
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test <test_name>
```

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

