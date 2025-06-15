# Comprehensive Benchmark Results

This report combines results from multiple benchmark runs across different dataset sizes (100, 1000, and 10000 files) and search patterns.

## Simple Pattern Search

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe TODO benchmark_data/synthetic/basic_100_files` | 32.9 ± 10.8 | 12.6 | 51.3 | 1.00 |
| `findstr /S /R TODO benchmark_data/synthetic/basic_100_files\*.* 2>NUL` | 42.4 ± 6.8 | 32.6 | 57.0 | 1.29 ± 0.47 |
| `rg --no-messages TODO benchmark_data/synthetic/basic_100_files` | 65.2 ± 8.8 | 49.9 | 80.2 | 1.98 ± 0.71 |
| `.\target\release\blink.exe TODO benchmark_data/synthetic/basic_1000_files` | 30.0 ± 4.2 | 23.7 | 39.8 | 1.00 |
| `findstr /S /R TODO benchmark_data/synthetic/basic_1000_files\*.* 2>NUL` | 35.3 ± 5.7 | 28.3 | 56.5 | 1.17 ± 0.25 |
| `rg --no-messages TODO benchmark_data/synthetic/basic_1000_files` | 61.8 ± 52.9 | 43.2 | 284.7 | 2.06 ± 1.79 |
| `.\target\release\blink.exe TODO benchmark_data/synthetic/basic_10000_files` | 34.3 ± 6.1 | 23.2 | 47.0 | 1.00 |
| `findstr /S /R TODO benchmark_data/synthetic/basic_10000_files\*.* 2>NUL` | 49.9 ± 7.8 | 35.8 | 68.5 | 1.45 ± 0.34 |
| `rg --no-messages TODO benchmark_data/synthetic/basic_10000_files` | 48.1 ± 4.9 | 42.5 | 61.7 | 1.40 ± 0.29 |

## Case-Insensitive Search

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe (?i)fixme benchmark_data/synthetic/basic_100_files` | 48.4 ± 60.7 | 26.5 | 374.3 | 1.72 ± 2.18 |
| `findstr /S /R (?i)fixme benchmark_data/synthetic/basic_100_files\*.* 2>NUL` | 28.1 ± 4.8 | 20.1 | 46.9 | 1.00 |
| `rg --no-messages (?i)fixme benchmark_data/synthetic/basic_100_files` | 43.6 ± 2.5 | 39.3 | 51.0 | 1.55 ± 0.28 |
| `.\target\release\blink.exe (?i)fixme benchmark_data/synthetic/basic_1000_files` | 26.0 ± 4.2 | 20.3 | 36.1 | 1.00 |
| `findstr /S /R (?i)fixme benchmark_data/synthetic/basic_1000_files\*.* 2>NUL` | 30.7 ± 4.9 | 20.3 | 43.3 | 1.18 ± 0.27 |
| `rg --no-messages (?i)fixme benchmark_data/synthetic/basic_1000_files` | 53.7 ± 25.4 | 42.4 | 194.3 | 2.06 ± 1.03 |
| `.\target\release\blink.exe (?i)fixme benchmark_data/synthetic/basic_10000_files` | 28.8 ± 4.5 | 21.4 | 36.3 | 1.00 |
| `findstr /S /R (?i)fixme benchmark_data/synthetic/basic_10000_files\*.* 2>NUL` | 33.4 ± 9.5 | 24.2 | 58.2 | 1.16 ± 0.38 |
| `rg --no-messages (?i)fixme benchmark_data/synthetic/basic_10000_files` | 50.3 ± 7.1 | 41.6 | 68.9 | 1.74 ± 0.37 |

## Regex Pattern Search

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe class.*\{ benchmark_data/synthetic/basic_100_files` | 28.1 ± 8.5 | 19.5 | 52.3 | 1.24 ± 0.39 |
| `findstr /S /R class.*\{ benchmark_data/synthetic/basic_100_files\*.* 2>NUL` | 22.7 ± 2.0 | 18.9 | 28.7 | 1.00 |
| `rg --no-messages class.*\{ benchmark_data/synthetic/basic_100_files` | 61.0 ± 44.2 | 38.8 | 290.9 | 2.69 ± 1.96 |
| `.\target\release\blink.exe class.*\{ benchmark_data/synthetic/basic_1000_files` | 32.4 ± 5.4 | 23.7 | 44.9 | 1.00 |
| `findstr /S /R class.*\{ benchmark_data/synthetic/basic_1000_files\*.* 2>NUL` | 35.8 ± 5.7 | 25.5 | 46.3 | 1.10 ± 0.25 |
| `rg --no-messages class.*\{ benchmark_data/synthetic/basic_1000_files` | 72.5 ± 16.4 | 45.5 | 101.8 | 2.24 ± 0.63 |
| `.\target\release\blink.exe class.*\{ benchmark_data/synthetic/basic_10000_files` | 46.2 ± 11.3 | 29.7 | 69.3 | 1.54 ± 0.40 |
| `findstr /S /R class.*\{ benchmark_data/synthetic/basic_10000_files\*.* 2>NUL` | 30.0 ± 2.3 | 25.1 | 34.4 | 1.00 |
| `rg --no-messages class.*\{ benchmark_data/synthetic/basic_10000_files` | 55.1 ± 7.7 | 42.9 | 69.7 | 1.84 ± 0.29 |

## Multi-threading Performance

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe -j 1 'TODO' benchmark_data/synthetic/basic_10000_files` | 56.8 ± 9.3 | 44.1 | 80.7 | 2.26 ± 0.55 |
| `.\target\release\blink.exe -j 4 'TODO' benchmark_data/synthetic/basic_10000_files` | 25.1 ± 4.5 | 16.9 | 34.3 | 1.00 |
| `.\target\release\blink.exe -j 8 'TODO' benchmark_data/synthetic/basic_10000_files` | 25.4 ± 19.5 | 15.4 | 106.5 | 1.01 ± 0.80 |
