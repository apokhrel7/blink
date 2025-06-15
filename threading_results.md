| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe -j 1 'TODO' benchmark_data/synthetic/basic_10000_files` | 56.8 ± 9.3 | 44.1 | 80.7 | 2.26 ± 0.55 |
| `.\target\release\blink.exe -j 4 'TODO' benchmark_data/synthetic/basic_10000_files` | 25.1 ± 4.5 | 16.9 | 34.3 | 1.00 |
| `.\target\release\blink.exe -j 8 'TODO' benchmark_data/synthetic/basic_10000_files` | 25.4 ± 19.5 | 15.4 | 106.5 | 1.01 ± 0.80 |
