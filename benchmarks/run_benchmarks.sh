#!/bin/bash

# Ensure hyperfine is installed
if ! command -v hyperfine &> /dev/null; then
    echo "Installing hyperfine..."
    cargo install hyperfine
fi

# Ensure ripgrep is installed for comparison
if ! command -v rg &> /dev/null; then
    echo "Installing ripgrep..."
    cargo install ripgrep
fi

# Build blink in release mode
cargo build --release

# Define search patterns and directories
PATTERN="TODO"
SMALL_DIR="benchmark_data/small"
MEDIUM_DIR="benchmark_data/medium"
LARGE_DIR="benchmark_data/large"

echo "Running benchmarks..."

# Small dataset benchmarks
echo "Small dataset (100 files):"
hyperfine --warmup 3 \
    "./target/release/blink '$PATTERN' $SMALL_DIR" \
    "grep -r '$PATTERN' $SMALL_DIR" \
    "rg '$PATTERN' $SMALL_DIR" \
    --export-markdown small_results.md

# Medium dataset benchmarks
echo "Medium dataset (1000 files):"
hyperfine --warmup 3 \
    "./target/release/blink '$PATTERN' $MEDIUM_DIR" \
    "grep -r '$PATTERN' $MEDIUM_DIR" \
    "rg '$PATTERN' $MEDIUM_DIR" \
    --export-markdown medium_results.md

# Large dataset benchmarks
echo "Large dataset (10000 files):"
hyperfine --warmup 3 \
    "./target/release/blink '$PATTERN' $LARGE_DIR" \
    "grep -r '$PATTERN' $LARGE_DIR" \
    "rg '$PATTERN' $LARGE_DIR" \
    --export-markdown large_results.md

# Multi-threaded comparison
echo "Multi-threaded search (large dataset):"
hyperfine --warmup 3 \
    "./target/release/blink -j 1 '$PATTERN' $LARGE_DIR" \
    "./target/release/blink -j 4 '$PATTERN' $LARGE_DIR" \
    "./target/release/blink -j 8 '$PATTERN' $LARGE_DIR" \
    --export-markdown threading_results.md 