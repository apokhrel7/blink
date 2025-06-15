# Comprehensive Benchmark Results

## Simple Pattern Search
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe TODO benchmark_data/synthetic/basic_10000_files` | 21.5 Â± 3.8 | 13.5 | 31.7 | 1.00 |
| `findstr /S /R TODO benchmark_data/synthetic/basic_10000_files\*.* 2>NUL` | 23.5 Â± 4.7 | 15.2 | 41.8 | 1.09 Â± 0.29 |
| `rg --no-messages TODO benchmark_data/synthetic/basic_10000_files` | 39.6 Â± 8.0 | 34.1 | 89.4 | 1.84 Â± 0.49 |
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe TODO benchmark_data/synthetic/basic_1000_files` | 19.6 Â± 4.0 | 12.9 | 28.1 | 1.00 |
| `findstr /S /R TODO benchmark_data/synthetic/basic_1000_files\*.* 2>NUL` | 23.0 Â± 5.4 | 14.8 | 43.8 | 1.17 Â± 0.36 |
| `rg --no-messages TODO benchmark_data/synthetic/basic_1000_files` | 39.3 Â± 4.6 | 34.5 | 55.0 | 2.00 Â± 0.47 |
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe TODO benchmark_data/synthetic/basic_100_files` | 21.4 Â± 8.5 | 11.4 | 72.8 | 1.03 Â± 0.44 |
| `findstr /S /R TODO benchmark_data/synthetic/basic_100_files\*.* 2>NUL` | 20.6 Â± 3.0 | 14.9 | 25.4 | 1.00 |
| `rg --no-messages TODO benchmark_data/synthetic/basic_100_files` | 36.8 Â± 4.4 | 32.0 | 58.9 | 1.78 Â± 0.34 |

## Case-Insensitive Search
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe (?i)fixme benchmark_data/synthetic/basic_10000_files` | 24.3 Â± 4.8 | 13.7 | 32.5 | 1.00 |
| `findstr /S /R (?i)fixme benchmark_data/synthetic/basic_10000_files\*.* 2>NUL` | 30.8 Â± 8.6 | 17.2 | 70.9 | 1.27 Â± 0.43 |
| `rg --no-messages (?i)fixme benchmark_data/synthetic/basic_10000_files` | 73.6 Â± 7.6 | 57.7 | 88.2 | 3.02 Â± 0.67 |
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe (?i)fixme benchmark_data/synthetic/basic_1000_files` | 28.9 Â± 7.6 | 14.5 | 42.4 | 1.19 Â± 0.41 |
| `findstr /S /R (?i)fixme benchmark_data/synthetic/basic_1000_files\*.* 2>NUL` | 24.3 Â± 5.5 | 12.5 | 37.8 | 1.00 |
| `rg --no-messages (?i)fixme benchmark_data/synthetic/basic_1000_files` | 57.6 Â± 29.1 | 36.0 | 194.3 | 2.37 Â± 1.31 |
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe (?i)fixme benchmark_data/synthetic/basic_100_files` | 21.0 Â± 3.0 | 14.6 | 28.2 | 1.00 |
| `findstr /S /R (?i)fixme benchmark_data/synthetic/basic_100_files\*.* 2>NUL` | 23.7 Â± 4.2 | 15.8 | 35.9 | 1.13 Â± 0.26 |
| `rg --no-messages (?i)fixme benchmark_data/synthetic/basic_100_files` | 40.5 Â± 4.0 | 35.5 | 54.8 | 1.93 Â± 0.33 |

## Regex Pattern Search
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe class.*\{ benchmark_data/synthetic/basic_10000_files` | 35.3 Â± 6.1 | 22.9 | 48.1 | 1.00 |
| `findstr /S /R class.*\{ benchmark_data/synthetic/basic_10000_files\*.* 2>NUL` | 36.7 Â± 6.0 | 26.4 | 46.3 | 1.04 Â± 0.25 |
| `rg --no-messages class.*\{ benchmark_data/synthetic/basic_10000_files` | 47.9 Â± 3.1 | 43.4 | 59.1 | 1.36 Â± 0.25 |
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe class.*\{ benchmark_data/synthetic/basic_1000_files` | 41.7 Â± 5.6 | 33.6 | 54.9 | 1.05 Â± 0.24 |
| `findstr /S /R class.*\{ benchmark_data/synthetic/basic_1000_files\*.* 2>NUL` | 39.7 Â± 7.2 | 25.6 | 50.7 | 1.00 |
| `rg --no-messages class.*\{ benchmark_data/synthetic/basic_1000_files` | 56.8 Â± 20.9 | 43.9 | 169.0 | 1.43 Â± 0.59 |
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe class.*\{ benchmark_data/synthetic/basic_100_files` | 34.3 Â± 4.3 | 26.7 | 43.0 | 1.04 Â± 0.33 |
| `findstr /S /R class.*\{ benchmark_data/synthetic/basic_100_files\*.* 2>NUL` | 32.9 Â± 9.6 | 27.7 | 81.0 | 1.00 |
| `rg --no-messages class.*\{ benchmark_data/synthetic/basic_100_files` | 60.1 Â± 7.1 | 44.0 | 71.0 | 1.83 Â± 0.58 |

## Real-World Repository Tests
No data available for this test

## Multi-threading Performance
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe -j 1 'TODO' benchmark_data/synthetic/basic_10000_files` | 32.4 Â± 3.6 | 26.2 | 38.6 | 1.00 |
| `.\target\release\blink.exe -j 4 'TODO' benchmark_data/synthetic/basic_10000_files` | 38.7 Â± 24.7 | 26.8 | 202.7 | 1.20 Â± 0.78 |
| `.\target\release\blink.exe -j 8 'TODO' benchmark_data/synthetic/basic_10000_files` | 41.9 Â± 9.1 | 27.4 | 58.3 | 1.29 Â± 0.32 |
