| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe TODO benchmark_data\large` | 25.0 ± 7.0 | 18.5 | 52.0 | 1.00 |
| `findstr /S TODO benchmark_data\large\*.* 2>NUL` | 46.4 ± 17.7 | 29.4 | 97.7 | 1.86 ± 0.88 |
| `rg --no-messages TODO benchmark_data\large` | 100.2 ± 35.1 | 71.4 | 184.7 | 4.01 ± 1.80 |
