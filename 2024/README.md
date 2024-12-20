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
| `target/release/p01 < input/p01.txt` | 0.7 ± 0.0 | 0.6 | 0.7 | 1.03 ± 0.11 |
| `target/release/p02 < input/p02.txt` | 1.2 ± 0.4 | 0.8 | 1.8 | 1.85 ± 0.58 |
| `target/release/p03 < input/p03.txt` | 2.5 ± 1.0 | 1.1 | 4.6 | 4.02 ± 1.65 |
| `target/release/p04 < input/p04.txt` | 3.8 ± 1.3 | 1.4 | 5.0 | 6.01 ± 2.16 |
| `target/release/p05 < input/p05.txt` | 30.2 ± 1.1 | 28.6 | 32.4 | 47.77 ± 4.43 |
| `target/release/p06 < input/p06.txt` | 89.5 ± 4.2 | 85.6 | 96.0 | 141.60 ± 13.79 |
| `target/release/p07 < input/p07.txt` | 48.5 ± 1.2 | 47.2 | 51.4 | 76.82 ± 6.84 |
| `target/release/p08 < input/p08.txt` | 0.6 ± 0.1 | 0.5 | 0.7 | 1.00 |
| `target/release/p09 < input/p09.txt` | 10.4 ± 0.1 | 10.2 | 10.5 | 16.39 ± 1.41 |
| `target/release/p10 < input/p10.txt` | 2.3 ± 1.0 | 1.4 | 4.7 | 3.70 ± 1.64 |
| `target/release/p11 < input/p11.txt` | 12.1 ± 1.8 | 10.4 | 15.8 | 19.13 ± 3.23 |
| `target/release/p12 < input/p12.txt` | 10.0 ± 3.5 | 6.2 | 15.3 | 15.81 ± 5.72 |
| `target/release/p13 < input/p13.txt` | 8.5 ± 1.6 | 5.9 | 10.5 | 13.42 ± 2.73 |
| `target/release/p14 < input/p14.txt` | 24.6 ± 0.8 | 23.3 | 26.0 | 38.88 ± 3.53 |
| `target/release/p15 < input/p15.txt` | 2.4 ± 0.2 | 2.2 | 2.8 | 3.82 ± 0.45 |
| `target/release/p16 < input/p16.txt` | 32.8 ± 1.3 | 31.2 | 35.9 | 51.92 ± 4.89 |
| `target/release/p17 < input/p17.txt` | 1.3 ± 0.1 | 1.2 | 1.5 | 2.07 ± 0.23 |
| `target/release/p18 < input/p18.txt` | 2.5 ± 0.4 | 2.2 | 3.5 | 3.97 ± 0.78 |
| `target/release/p19 < input/p19.txt` | 27.5 ± 0.7 | 27.0 | 29.0 | 43.57 ± 3.89 |
| `target/release/p20 < input/p20.txt` | 124.3 ± 3.6 | 120.0 | 128.7 | 196.67 ± 17.72 |

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
| Rust | src/bin/p20.rs | p20.rs | 109 | 81 | 0 | 28 | 20 | 2259 | 0 |

