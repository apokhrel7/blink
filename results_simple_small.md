| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe TODO benchmark_data/synthetic/basic_100_files` | 32.9 ± 10.8 | 12.6 | 51.3 | 1.00 |
| `findstr /S /R TODO benchmark_data/synthetic/basic_100_files\*.* 2>NUL` | 42.4 ± 6.8 | 32.6 | 57.0 | 1.29 ± 0.47 |
| `rg --no-messages TODO benchmark_data/synthetic/basic_100_files` | 65.2 ± 8.8 | 49.9 | 80.2 | 1.98 ± 0.71 |
