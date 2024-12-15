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
| `target/release/p01 < input/p01.txt` | 3.2 ± 0.3 | 2.6 | 3.6 | 7.59 ± 1.12 |
| `target/release/p02 < input/p02.txt` | 3.5 ± 1.0 | 1.8 | 4.5 | 8.18 ± 2.63 |
| `target/release/p03 < input/p03.txt` | 4.9 ± 0.7 | 2.9 | 5.4 | 11.56 ± 2.14 |
| `target/release/p04 < input/p04.txt` | 4.3 ± 1.4 | 1.9 | 5.4 | 10.14 ± 3.47 |
| `target/release/p05 < input/p05.txt` | 30.8 ± 0.4 | 30.3 | 31.6 | 72.71 ± 7.80 |
| `target/release/p06 < input/p06.txt` | 87.9 ± 1.3 | 86.8 | 91.2 | 207.76 ± 22.37 |
| `target/release/p07 < input/p07.txt` | 50.3 ± 2.9 | 47.2 | 57.8 | 118.72 ± 14.38 |
| `target/release/p08 < input/p08.txt` | 0.4 ± 0.0 | 0.4 | 0.5 | 1.00 |
| `target/release/p09 < input/p09.txt` | 11.1 ± 1.5 | 10.1 | 14.1 | 26.20 ± 4.57 |
| `target/release/p10 < input/p10.txt` | 1.2 ± 0.3 | 0.9 | 1.8 | 2.85 ± 0.86 |
| `target/release/p11 < input/p11.txt` | 12.6 ± 1.4 | 10.5 | 14.7 | 29.65 ± 4.53 |
| `target/release/p12 < input/p12.txt` | 9.4 ± 2.1 | 6.3 | 13.2 | 22.20 ± 5.58 |
| `target/release/p13 < input/p13.txt` | 4.2 ± 1.1 | 3.0 | 5.7 | 9.99 ± 2.82 |
| `target/release/p14 < input/p14.txt` | 24.9 ± 0.4 | 24.4 | 25.5 | 58.77 ± 6.34 |
| `target/release/p15 < input/p15.txt` | 2.0 ± 0.0 | 1.9 | 2.1 | 4.71 ± 0.51 |

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
| Rust | src/bin/p15.rs | p15.rs | 228 | 168 | 11 | 49 | 32 | 6062 | 0 |

