# Blink

A fast command-line utility for searching text patterns in files written in Rust.

## Features

- Fast parallel file searching using multiple threads
- Regular expression pattern matching
- Case-sensitive and case-insensitive search modes
- File extension filtering
- Hidden file inclusion/exclusion
- Colored output highlighting
- Context lines around matches (To be completed)
- Binary file detection and skipping
- Cross-platform support

## Installation

Using Cargo:

```bash
cargo install fast-find
```

## Usage

Basic usage:

```bash
fast-find <pattern> [path...]
```

Examples:

```bash
# Search for "TODO" in current directory
fast-find TODO

# Case-insensitive search for "error" in src directory
fast-find -i error src/

# Search for "fn" in Rust files only
fast-find -e rs "fn" src/

# Show 2 lines of context around matches
fast-find -C 2 "panic!" src/

# Search with 8 threads
fast-find -j 8 "test"
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
   cd fast-find
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
   - Windows: `target/release/fast-find.exe`
   - Unix-like: `target/release/fast-find`

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

This will install the `fast-find` binary to your Cargo binary directory (usually `~/.cargo/bin/` on Unix-like systems or `%USERPROFILE%\.cargo\bin` on Windows).

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

