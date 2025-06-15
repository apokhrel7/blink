| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe class.*\{ benchmark_data/synthetic/basic_100_files` | 28.1 ± 8.5 | 19.5 | 52.3 | 1.24 ± 0.39 |
| `findstr /S /R class.*\{ benchmark_data/synthetic/basic_100_files\*.* 2>NUL` | 22.7 ± 2.0 | 18.9 | 28.7 | 1.00 |
| `rg --no-messages class.*\{ benchmark_data/synthetic/basic_100_files` | 61.0 ± 44.2 | 38.8 | 290.9 | 2.69 ± 1.96 |
