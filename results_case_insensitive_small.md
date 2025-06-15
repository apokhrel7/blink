| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe (?i)fixme benchmark_data/synthetic/basic_100_files` | 48.4 ± 60.7 | 26.5 | 374.3 | 1.72 ± 2.18 |
| `findstr /S /R (?i)fixme benchmark_data/synthetic/basic_100_files\*.* 2>NUL` | 28.1 ± 4.8 | 20.1 | 46.9 | 1.00 |
| `rg --no-messages (?i)fixme benchmark_data/synthetic/basic_100_files` | 43.6 ± 2.5 | 39.3 | 51.0 | 1.55 ± 0.28 |
