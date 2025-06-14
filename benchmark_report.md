# Benchmark Results

## Small Dataset (100 files)
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe TODO benchmark_data\small` | 44.7 Â± 16.5 | 22.6 | 71.7 | 1.00 |
| `findstr /S TODO benchmark_data\small\*.* 2>NUL` | 45.6 Â± 6.8 | 36.4 | 63.5 | 1.02 Â± 0.41 |
| `rg --no-messages TODO benchmark_data\small` | 87.3 Â± 9.5 | 70.3 | 111.5 | 1.95 Â± 0.75 |


## Medium Dataset (1000 files)
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe TODO benchmark_data\medium` | 58.5 Â± 22.3 | 37.0 | 145.5 | 1.32 Â± 0.54 |
| `findstr /S TODO benchmark_data\medium\*.* 2>NUL` | 44.4 Â± 7.0 | 32.6 | 57.8 | 1.00 |
| `rg --no-messages TODO benchmark_data\medium` | 90.5 Â± 31.2 | 57.6 | 181.8 | 2.04 Â± 0.77 |


## Large Dataset (10000 files)
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe TODO benchmark_data\large` | 25.0 Â± 7.0 | 18.5 | 52.0 | 1.00 |
| `findstr /S TODO benchmark_data\large\*.* 2>NUL` | 46.4 Â± 17.7 | 29.4 | 97.7 | 1.86 Â± 0.88 |
| `rg --no-messages TODO benchmark_data\large` | 100.2 Â± 35.1 | 71.4 | 184.7 | 4.01 Â± 1.80 |


## Multi-threading Performance
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe -j 1 TODO benchmark_data\large` | 32.0 Â± 6.2 | 21.8 | 47.5 | 1.21 Â± 0.36 |
| `.\target\release\blink.exe -j 4 TODO benchmark_data\large` | 26.3 Â± 5.8 | 17.3 | 36.1 | 1.00 |
| `.\target\release\blink.exe -j 8 TODO benchmark_data\large` | 33.3 Â± 7.8 | 20.9 | 56.2 | 1.26 Â± 0.41 |

