#!/bin/bash

# Parse command line arguments
skip_report=false
while getopts "s" opt; do
    case $opt in
        s) skip_report=true ;;
    esac
done

# Check if hyperfine is installed
if ! command -v hyperfine &> /dev/null; then
    echo "Installing hyperfine..."
    cargo install hyperfine
fi

# Check if ripgrep is installed
if ! command -v rg &> /dev/null; then
    echo "Installing ripgrep..."
    cargo install ripgrep
fi

# Build blink in release mode
cargo build --release

# Generate test data if it doesn't exist
if [ ! -d "benchmark_data" ]; then
    echo "Generating test data..."
    cargo run --bin generate_test_data
fi

# Define search patterns and directories
declare -A PATTERNS=(
    ["simple"]="TODO:Simple word search"
    ["case_insensitive"]="(?i)fixme:Case-insensitive search"
    ["regex"]="class.*\{:Class definition search"
)

declare -A DATASETS=(
    ["small"]="benchmark_data/synthetic/basic_100_files:Small dataset (100 files)"
    ["medium"]="benchmark_data/synthetic/basic_1000_files:Medium dataset (1000 files)"
    ["large"]="benchmark_data/synthetic/basic_10000_files:Large dataset (10000 files)"
)

echo "Running benchmarks..."

for pattern_key in "${!PATTERNS[@]}"; do
    IFS=':' read -r pattern desc <<< "${PATTERNS[$pattern_key]}"
    echo -e "\nTesting pattern: $desc"
    
    for dataset_key in "${!DATASETS[@]}"; do
        IFS=':' read -r dir desc <<< "${DATASETS[$dataset_key]}"
        echo -e "\n$desc:"
        output_file="results_${pattern_key}_${dataset_key}.md"
        
        hyperfine --warmup 10 --min-runs 20 --ignore-failure --show-output --export-markdown "$output_file" \
            --prepare "echo 'Running $desc with $desc...'" \
            "./target/release/blink '$pattern' '$dir'" \
            "grep -r '$pattern' '$dir'" \
            "rg --no-messages '$pattern' '$dir'"
    done
done

# Multi-threaded comparison
echo -e "\nMulti-threaded search (large dataset):"
hyperfine --warmup 10 --min-runs 20 --ignore-failure --show-output --export-markdown threading_results.md \
    --prepare "echo 'Running threading benchmark...'" \
    "./target/release/blink -j 1 'TODO' benchmark_data/synthetic/basic_10000_files" \
    "./target/release/blink -j 4 'TODO' benchmark_data/synthetic/basic_10000_files" \
    "./target/release/blink -j 8 'TODO' benchmark_data/synthetic/basic_10000_files"

# Generate combined report if not skipped
if [ "$skip_report" = false ]; then
    echo -e "\nGenerating combined report..."
    "$(dirname "$0")/combine_results.sh"
else
    echo -e "\nBenchmark results have been generated in individual files."
    echo "To combine them into a single report, run: ./benches/scripts/combine_results.sh"
fi