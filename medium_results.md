| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\fast-find.exe TODO benchmark_data\medium` | 58.5 ± 22.3 | 37.0 | 145.5 | 1.32 ± 0.54 |
| `findstr /S TODO benchmark_data\medium\*.* 2>NUL` | 44.4 ± 7.0 | 32.6 | 57.8 | 1.00 |
| `rg --no-messages TODO benchmark_data\medium` | 90.5 ± 31.2 | 57.6 | 181.8 | 2.04 ± 0.77 |
