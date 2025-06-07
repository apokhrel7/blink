# fast-find

A fast command-line utility for searching text patterns in files, written in Rust.

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

## Building from Source

1. Clone the repository
2. Run `cargo build --release`
3. The binary will be available at `target/release/fast-find`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 