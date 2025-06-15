| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe TODO benchmark_data/synthetic/basic_10000_files` | 34.3 ± 6.1 | 23.2 | 47.0 | 1.00 |
| `findstr /S /R TODO benchmark_data/synthetic/basic_10000_files\*.* 2>NUL` | 49.9 ± 7.8 | 35.8 | 68.5 | 1.45 ± 0.34 |
| `rg --no-messages TODO benchmark_data/synthetic/basic_10000_files` | 48.1 ± 4.9 | 42.5 | 61.7 | 1.40 ± 0.29 |
