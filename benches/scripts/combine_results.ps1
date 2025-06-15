# Script to combine benchmark results into a single comprehensive report
$outputFile = "benchmark_report.md"

# Function to write a section header
function Write-SectionHeader {
    param([string]$header)
    "`n## $header`n" | Out-File -Append -Encoding utf8 $outputFile
}

# Function to write table header
function Write-TableHeader {
    @"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
"@ | Out-File -Append -Encoding utf8 $outputFile
}

# Function to write table contents, skipping header
function Write-TableContents {
    param(
        [string]$file,
        [bool]$includeHeader = $false
    )
    $content = Get-Content $file -Encoding utf8
    if ($includeHeader) {
        Write-TableHeader
    }
    # Always skip the header (first 2 lines) and write only the data rows
    $content | Select-Object -Skip 2 | ForEach-Object {
        $_ -replace "Â±", "±"
    } | Out-File -Append -Encoding utf8 $outputFile
}

# Clear existing report file and write main header
@"
# Comprehensive Benchmark Results

This report combines results from multiple benchmark runs across different dataset sizes (100, 1000, and 10000 files) and search patterns.
"@ | Out-File -Encoding utf8 $outputFile

# Simple Pattern Search
Write-SectionHeader "Simple Pattern Search"
Write-TableHeader
Write-TableContents "results_simple_small.md"
Write-TableContents "results_simple_medium.md"
Write-TableContents "results_simple_large.md"

# Case-Insensitive Search
Write-SectionHeader "Case-Insensitive Search"
Write-TableHeader
Write-TableContents "results_case_insensitive_small.md"
Write-TableContents "results_case_insensitive_medium.md"
Write-TableContents "results_case_insensitive_large.md"

# Regex Pattern Search
Write-SectionHeader "Regex Pattern Search"
Write-TableHeader
Write-TableContents "results_regex_small.md"
Write-TableContents "results_regex_medium.md"
Write-TableContents "results_regex_large.md"

# Multi-threading Performance
Write-SectionHeader "Multi-threading Performance"
Write-TableHeader
Write-TableContents "threading_results.md"

Write-Host "Results combined successfully into $outputFile" 