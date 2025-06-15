#!/bin/bash

output_file="benchmark_report.md"

# Function to write a section header
write_section_header() {
    echo -e "\n## $1\n" >> "$output_file"
}

# Function to write table header
write_table_header() {
    cat << EOF >> "$output_file"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
EOF
}

# Function to write table contents, skipping header
write_table_contents() {
    local file="$1"
    # Always skip the header (first 2 lines) and write only the data rows
    tail -n +3 "$file" | sed 's/Â±/±/g' >> "$output_file"
}

# Clear and initialize the report file
cat << EOF > "$output_file"
# Comprehensive Benchmark Results

This report combines results from multiple benchmark runs across different dataset sizes (100, 1000, and 10000 files) and search patterns.
EOF

# Simple Pattern Search
write_section_header "Simple Pattern Search"
write_table_header
write_table_contents "results_simple_small.md"
write_table_contents "results_simple_medium.md"
write_table_contents "results_simple_large.md"

# Case-Insensitive Search
write_section_header "Case-Insensitive Search"
write_table_header
write_table_contents "results_case_insensitive_small.md"
write_table_contents "results_case_insensitive_medium.md"
write_table_contents "results_case_insensitive_large.md"

# Regex Pattern Search
write_section_header "Regex Pattern Search"
write_table_header
write_table_contents "results_regex_small.md"
write_table_contents "results_regex_medium.md"
write_table_contents "results_regex_large.md"

# Multi-threading Performance
write_section_header "Multi-threading Performance"
write_table_header
write_table_contents "threading_results.md"

echo "Results combined successfully into $output_file" 