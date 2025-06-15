param(
    [switch]$SkipReport
)

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

# Build blink in release mode
cargo build --release

# Generate test data if it doesn't exist
if (!(Test-Path "benchmark_data")) {
    Write-Host "Generating test data..."
    cargo run --bin generate_test_data
}

# Define search patterns and directories
$PATTERNS = @(
    @{name="simple"; pattern="TODO"; desc="Simple word search"},
    @{name="case_insensitive"; pattern="(?i)fixme"; desc="Case-insensitive search"},
    @{name="regex"; pattern="class.*\{"; desc="Class definition search"}
)

$DATASETS = @(
    @{name="small"; dir="benchmark_data/synthetic/basic_100_files"; desc="Small dataset (100 files)"},
    @{name="medium"; dir="benchmark_data/synthetic/basic_1000_files"; desc="Medium dataset (1000 files)"},
    @{name="large"; dir="benchmark_data/synthetic/basic_10000_files"; desc="Large dataset (10000 files)"}
)

Write-Host "Running benchmarks..."

foreach ($pattern in $PATTERNS) {
    Write-Host "`nTesting pattern: $($pattern.desc)"
    
    foreach ($dataset in $DATASETS) {
        Write-Host "`n$($dataset.desc):"
        $output_file = "results_$($pattern.name)_$($dataset.name).md"
        
        hyperfine --warmup 10 --min-runs 20 --ignore-failure --show-output --export-markdown $output_file `
            --prepare "echo 'Running $($dataset.desc) with $($pattern.desc)...'" `
            ".\target\release\blink.exe $($pattern.pattern) $($dataset.dir)" `
            "findstr /S /R $($pattern.pattern) $($dataset.dir)\*.* 2>NUL" `
            "rg --no-messages $($pattern.pattern) $($dataset.dir)"
    }
}

# Multi-threaded comparison
Write-Host "`nMulti-threaded search (large dataset):"
hyperfine --warmup 10 --min-runs 20 --ignore-failure --show-output --export-markdown threading_results.md `
    --prepare "echo 'Running threading benchmark...'" `
    ".\target\release\blink.exe -j 1 'TODO' benchmark_data/synthetic/basic_10000_files" `
    ".\target\release\blink.exe -j 4 'TODO' benchmark_data/synthetic/basic_10000_files" `
    ".\target\release\blink.exe -j 8 'TODO' benchmark_data/synthetic/basic_10000_files"

# Generate combined report if not skipped
if (-not $SkipReport) {
    Write-Host "`nGenerating combined report..."
    & "$PSScriptRoot\combine_results.ps1"
} else {
    Write-Host "`nBenchmark results have been generated in individual files."
    Write-Host "To combine them into a single report, run: .\benches\scripts\combine_results.ps1"
}