| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\fast-find.exe -j 1 TODO benchmark_data\large` | 32.0 ± 6.2 | 21.8 | 47.5 | 1.21 ± 0.36 |
| `.\target\release\fast-find.exe -j 4 TODO benchmark_data\large` | 26.3 ± 5.8 | 17.3 | 36.1 | 1.00 |
| `.\target\release\fast-find.exe -j 8 TODO benchmark_data\large` | 33.3 ± 7.8 | 20.9 | 56.2 | 1.26 ± 0.41 |
