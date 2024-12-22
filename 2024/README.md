# Advent of Code 2024

These are my solutions for the 2024 Advent of Code (AoC). They are written in Rust. Goals are
readable code, clean algorithms and decent runtimes. Am sub 100ms for all problems so far.
This probably will not continue...

## Hardware

It doesn't take anything special hardware wise to solve AoC problems. From the website:

> every problem has a solution that completes in at most 15 seconds on ten-year-old hardware

Hardware I'm using:

- **Processor:** Intel Core i5-12500
- **Total Cores:** 6
- **Total Threads:** 12
- **RAM:** 80G

## Libraries

I tend to rely on minimal external libraries though do have a few standard ones I use.

- [anyhow](https://crates.io/crates/anyhow)
- [regex](https://crates.io/crates/regex)
- [itertools](https://crates.io/crates/itertools)
- [rayon](https://crates.io/crates/rayon) (using this for the first time this year. `par_iter` ftw!)
- [num](https://crates.io/crates/num) (if big ints are required)
- [rustworkx-core](https://crates.io/crates/rustworkx-core) (used once last year)

## Benchmarks

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/p01 < input/p01.txt` | 0.6 ± 0.0 | 0.5 | 0.6 | 1.00 |
| `target/release/p02 < input/p02.txt` | 1.8 ± 0.9 | 0.8 | 3.4 | 3.16 ± 1.58 |
| `target/release/p03 < input/p03.txt` | 4.7 ± 0.6 | 3.7 | 5.5 | 8.03 ± 1.22 |
| `target/release/p04 < input/p04.txt` | 5.3 ± 0.1 | 5.2 | 5.5 | 9.07 ± 0.72 |
| `target/release/p05 < input/p05.txt` | 30.6 ± 0.1 | 30.4 | 30.9 | 52.29 ± 3.95 |
| `target/release/p06 < input/p06.txt` | 89.1 ± 2.3 | 86.7 | 94.0 | 152.03 ± 12.10 |
| `target/release/p07 < input/p07.txt` | 50.1 ± 1.3 | 48.7 | 52.5 | 85.55 ± 6.82 |
| `target/release/p08 < input/p08.txt` | 0.6 ± 0.1 | 0.6 | 0.7 | 1.08 ± 0.12 |
| `target/release/p09 < input/p09.txt` | 10.8 ± 0.6 | 10.3 | 12.1 | 18.39 ± 1.70 |
| `target/release/p10 < input/p10.txt` | 1.5 ± 0.7 | 1.1 | 3.6 | 2.64 ± 1.28 |
| `target/release/p11 < input/p11.txt` | 11.2 ± 1.4 | 10.1 | 14.0 | 19.08 ± 2.85 |
| `target/release/p12 < input/p12.txt` | 6.7 ± 0.4 | 6.5 | 7.9 | 11.45 ± 1.14 |
| `target/release/p13 < input/p13.txt` | 3.1 ± 0.1 | 3.0 | 3.2 | 5.24 ± 0.41 |
| `target/release/p14 < input/p14.txt` | 25.3 ± 0.6 | 24.9 | 26.6 | 43.23 ± 3.42 |
| `target/release/p15 < input/p15.txt` | 2.9 ± 0.6 | 2.2 | 3.7 | 4.93 ± 1.06 |
| `target/release/p16 < input/p16.txt` | 33.7 ± 0.4 | 33.3 | 34.4 | 57.61 ± 4.39 |
| `target/release/p17 < input/p17.txt` | 1.2 ± 0.1 | 1.2 | 1.3 | 2.09 ± 0.18 |
| `target/release/p18 < input/p18.txt` | 2.2 ± 0.0 | 2.1 | 2.2 | 3.72 ± 0.29 |
| `target/release/p19 < input/p19.txt` | 30.1 ± 1.7 | 28.9 | 34.7 | 51.30 ± 4.87 |
| `target/release/p20 < input/p20.txt` | 18.1 ± 0.7 | 17.0 | 19.1 | 30.94 ± 2.66 |
| `target/release/p21 < input/p21.txt` | 2.3 ± 0.1 | 2.1 | 2.4 | 3.84 ± 0.31 |
| `target/release/p22 < input/p22.txt` | 191.7 ± 2.5 | 188.2 | 196.1 | 327.29 ± 25.04 |

## Lines of Code

| Language | Provider | Filename | Lines | Code | Comments | Blanks | Complexity | Bytes | ULOC |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Rust | src/bin/p01.rs | p01.rs | 45 | 32 | 0 | 13 | 2 | 961 | 0 |
| Rust | src/bin/p02.rs | p02.rs | 58 | 40 | 2 | 16 | 5 | 1300 | 0 |
| Rust | src/bin/p03.rs | p03.rs | 34 | 26 | 0 | 8 | 2 | 883 | 0 |
| Rust | src/bin/p04.rs | p04.rs | 92 | 62 | 6 | 24 | 21 | 2121 | 0 |
| Rust | src/bin/p05.rs | p05.rs | 86 | 67 | 0 | 19 | 15 | 1922 | 0 |
| Rust | src/bin/p06.rs | p06.rs | 80 | 57 | 0 | 23 | 13 | 1845 | 0 |
| Rust | src/bin/p07.rs | p07.rs | 100 | 74 | 6 | 20 | 19 | 2108 | 0 |
| Rust | src/bin/p08.rs | p08.rs | 82 | 63 | 0 | 19 | 17 | 2088 | 0 |
| Rust | src/bin/p09.rs | p09.rs | 118 | 85 | 5 | 28 | 22 | 2729 | 0 |
| Rust | src/bin/p10.rs | p10.rs | 92 | 70 | 0 | 22 | 15 | 2031 | 0 |
| Rust | src/bin/p11.rs | p11.rs | 66 | 49 | 1 | 16 | 11 | 1316 | 0 |
| Rust | src/bin/p12.rs | p12.rs | 139 | 102 | 0 | 37 | 28 | 3374 | 0 |
| Rust | src/bin/p13.rs | p13.rs | 82 | 60 | 1 | 21 | 16 | 2012 | 0 |
| Rust | src/bin/p14.rs | p14.rs | 70 | 51 | 2 | 17 | 19 | 1744 | 0 |
| Rust | src/bin/p15.rs | p15.rs | 228 | 168 | 11 | 49 | 32 | 6064 | 0 |
| Rust | src/bin/p16.rs | p16.rs | 157 | 111 | 10 | 36 | 40 | 3999 | 0 |
| Rust | src/bin/p17.rs | p17.rs | 136 | 101 | 11 | 24 | 19 | 3250 | 0 |
| Rust | src/bin/p18.rs | p18.rs | 82 | 57 | 3 | 22 | 14 | 1974 | 0 |
| Rust | src/bin/p19.rs | p19.rs | 76 | 58 | 0 | 18 | 16 | 1634 | 0 |
| Rust | src/bin/p20.rs | p20.rs | 109 | 82 | 0 | 27 | 19 | 2313 | 0 |
| Rust | src/bin/p21.rs | p21.rs | 109 | 80 | 3 | 26 | 13 | 2980 | 0 |
| Rust | src/bin/p22.rs | p22.rs | 71 | 49 | 1 | 21 | 6 | 1540 | 0 |

