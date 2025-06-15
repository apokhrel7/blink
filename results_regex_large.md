| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe class.*\{ benchmark_data/synthetic/basic_10000_files` | 46.2 ± 11.3 | 29.7 | 69.3 | 1.54 ± 0.40 |
| `findstr /S /R class.*\{ benchmark_data/synthetic/basic_10000_files\*.* 2>NUL` | 30.0 ± 2.3 | 25.1 | 34.4 | 1.00 |
| `rg --no-messages class.*\{ benchmark_data/synthetic/basic_10000_files` | 55.1 ± 7.7 | 42.9 | 69.7 | 1.84 ± 0.29 |
