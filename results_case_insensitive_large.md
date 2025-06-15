| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe (?i)fixme benchmark_data/synthetic/basic_10000_files` | 28.8 ± 4.5 | 21.4 | 36.3 | 1.00 |
| `findstr /S /R (?i)fixme benchmark_data/synthetic/basic_10000_files\*.* 2>NUL` | 33.4 ± 9.5 | 24.2 | 58.2 | 1.16 ± 0.38 |
| `rg --no-messages (?i)fixme benchmark_data/synthetic/basic_10000_files` | 50.3 ± 7.1 | 41.6 | 68.9 | 1.74 ± 0.37 |
