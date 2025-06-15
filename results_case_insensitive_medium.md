| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe (?i)fixme benchmark_data/synthetic/basic_1000_files` | 26.0 ± 4.2 | 20.3 | 36.1 | 1.00 |
| `findstr /S /R (?i)fixme benchmark_data/synthetic/basic_1000_files\*.* 2>NUL` | 30.7 ± 4.9 | 20.3 | 43.3 | 1.18 ± 0.27 |
| `rg --no-messages (?i)fixme benchmark_data/synthetic/basic_1000_files` | 53.7 ± 25.4 | 42.4 | 194.3 | 2.06 ± 1.03 |
