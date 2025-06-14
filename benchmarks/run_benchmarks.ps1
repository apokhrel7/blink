# Check if hyperfine is installed
if (!(Get-Command hyperfine -ErrorAction SilentlyContinue)) {
    Write-Host "Installing hyperfine..."
    cargo install hyperfine
}

# Check if ripgrep is installed
if (!(Get-Command rg -ErrorAction SilentlyContinue)) {
    Write-Host "Installing ripgrep..."
    cargo install ripgrep
}

# Build fast-find in release mode
cargo build --release

# Define search patterns and directories
$PATTERN = "TODO"
$SMALL_DIR = "benchmark_data\small"
$MEDIUM_DIR = "benchmark_data\medium"
$LARGE_DIR = "benchmark_data\large"

Write-Host "Running benchmarks..."

# Small dataset benchmarks
Write-Host "Small dataset (100 files):"
hyperfine --warmup 3 `
    ".\target\release\fast-find.exe '$PATTERN' $SMALL_DIR" `
    "findstr /s /i /n '$PATTERN' $SMALL_DIR\*.*" `
    "rg '$PATTERN' $SMALL_DIR" `
    --export-markdown small_results.md

# Medium dataset benchmarks
Write-Host "Medium dataset (1000 files):"
hyperfine --warmup 3 `
    ".\target\release\fast-find.exe '$PATTERN' $MEDIUM_DIR" `
    "findstr /s /i /n '$PATTERN' $MEDIUM_DIR\*.*" `
    "rg '$PATTERN' $MEDIUM_DIR" `
    --export-markdown medium_results.md

# Large dataset benchmarks
Write-Host "Large dataset (10000 files):"
hyperfine --warmup 3 `
    ".\target\release\fast-find.exe '$PATTERN' $LARGE_DIR" `
    "findstr /s /i /n '$PATTERN' $LARGE_DIR\*.*" `
    "rg '$PATTERN' $LARGE_DIR" `
    --export-markdown large_results.md

# Multi-threaded comparison
Write-Host "Multi-threaded search (large dataset):"
hyperfine --warmup 3 `
    ".\target\release\fast-find.exe -j 1 '$PATTERN' $LARGE_DIR" `
    ".\target\release\fast-find.exe -j 4 '$PATTERN' $LARGE_DIR" `
    ".\target\release\fast-find.exe -j 8 '$PATTERN' $LARGE_DIR" `
    --export-markdown threading_results.md 