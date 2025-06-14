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

# Ensure directories exist
$dirs = @($SMALL_DIR, $MEDIUM_DIR, $LARGE_DIR)
foreach ($dir in $dirs) {
    if (!(Test-Path $dir)) {
        Write-Host "Creating directory: $dir"
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
    }
}

Write-Host "Running benchmarks..."

# Small dataset benchmarks
Write-Host "`nSmall dataset (100 files):"
hyperfine --warmup 10 --min-runs 20 --ignore-failure --show-output --export-markdown small_results.md `
    --prepare "echo 'Running small dataset benchmark...'" `
    ".\target\release\fast-find.exe $PATTERN $SMALL_DIR" `
    "findstr /S $PATTERN $SMALL_DIR\*.* 2>NUL" `
    "rg --no-messages $PATTERN $SMALL_DIR"

# Medium dataset benchmarks
Write-Host "`nMedium dataset (1000 files):"
hyperfine --warmup 10 --min-runs 20 --ignore-failure --show-output --export-markdown medium_results.md `
    --prepare "echo 'Running medium dataset benchmark...'" `
    ".\target\release\fast-find.exe $PATTERN $MEDIUM_DIR" `
    "findstr /S $PATTERN $MEDIUM_DIR\*.* 2>NUL" `
    "rg --no-messages $PATTERN $MEDIUM_DIR"

# Large dataset benchmarks
Write-Host "`nLarge dataset (10000 files):"
hyperfine --warmup 10 --min-runs 20 --ignore-failure --show-output --export-markdown large_results.md `
    --prepare "echo 'Running large dataset benchmark...'" `
    ".\target\release\fast-find.exe $PATTERN $LARGE_DIR" `
    "findstr /S $PATTERN $LARGE_DIR\*.* 2>NUL" `
    "rg --no-messages $PATTERN $LARGE_DIR"

# Multi-threaded comparison
Write-Host "`nMulti-threaded search (large dataset):"
hyperfine --warmup 10 --min-runs 20 --ignore-failure --show-output --export-markdown threading_results.md `
    --prepare "echo 'Running threading benchmark...'" `
    ".\target\release\fast-find.exe -j 1 $PATTERN $LARGE_DIR" `
    ".\target\release\fast-find.exe -j 4 $PATTERN $LARGE_DIR" `
    ".\target\release\fast-find.exe -j 8 $PATTERN $LARGE_DIR"

# Combine results into a single report
Write-Host "`nGenerating combined report..."
$report = @"
# Benchmark Results

## Small Dataset (100 files)
$(Get-Content small_results.md -Raw)

## Medium Dataset (1000 files)
$(Get-Content medium_results.md -Raw)

## Large Dataset (10000 files)
$(Get-Content large_results.md -Raw)

## Multi-threading Performance
$(Get-Content threading_results.md -Raw)
"@

$report | Out-File -FilePath "benchmark_report.md" -Encoding UTF8 