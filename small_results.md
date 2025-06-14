| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\blink.exe TODO benchmark_data\small` | 44.7 ± 16.5 | 22.6 | 71.7 | 1.00 |
| `findstr /S TODO benchmark_data\small\*.* 2>NUL` | 45.6 ± 6.8 | 36.4 | 63.5 | 1.02 ± 0.41 |
| `rg --no-messages TODO benchmark_data\small` | 87.3 ± 9.5 | 70.3 | 111.5 | 1.95 ± 0.75 |
