| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe class.*\{ benchmark_data/synthetic/basic_1000_files` | 32.4 ± 5.4 | 23.7 | 44.9 | 1.00 |
| `findstr /S /R class.*\{ benchmark_data/synthetic/basic_1000_files\*.* 2>NUL` | 35.8 ± 5.7 | 25.5 | 46.3 | 1.10 ± 0.25 |
| `rg --no-messages class.*\{ benchmark_data/synthetic/basic_1000_files` | 72.5 ± 16.4 | 45.5 | 101.8 | 2.24 ± 0.63 |
