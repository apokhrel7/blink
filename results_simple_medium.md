| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe TODO benchmark_data/synthetic/basic_1000_files` | 30.0 ± 4.2 | 23.7 | 39.8 | 1.00 |
| `findstr /S /R TODO benchmark_data/synthetic/basic_1000_files\*.* 2>NUL` | 35.3 ± 5.7 | 28.3 | 56.5 | 1.17 ± 0.25 |
| `rg --no-messages TODO benchmark_data/synthetic/basic_1000_files` | 61.8 ± 52.9 | 43.2 | 284.7 | 2.06 ± 1.79 |
