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
| `target/release/p01 < input/p01.txt` | 2.0 ± 0.5 | 1.5 | 3.2 | 3.95 ± 1.29 |
| `target/release/p02 < input/p02.txt` | 3.4 ± 1.3 | 1.7 | 4.9 | 6.56 ± 2.81 |
| `target/release/p03 < input/p03.txt` | 4.5 ± 0.6 | 3.6 | 5.2 | 8.74 ± 2.15 |
| `target/release/p04 < input/p04.txt` | 4.3 ± 1.1 | 2.4 | 5.6 | 8.25 ± 2.73 |
| `target/release/p05 < input/p05.txt` | 31.3 ± 0.9 | 30.6 | 33.3 | 60.70 ± 12.68 |
| `target/release/p06 < input/p06.txt` | 93.0 ± 4.1 | 88.1 | 99.0 | 180.24 ± 38.08 |
| `target/release/p07 < input/p07.txt` | 49.7 ± 1.4 | 47.8 | 52.7 | 96.33 ± 20.10 |
| `target/release/p08 < input/p08.txt` | 0.5 ± 0.1 | 0.4 | 0.7 | 1.00 |
| `target/release/p09 < input/p09.txt` | 10.2 ± 0.1 | 10.1 | 10.4 | 19.82 ± 4.10 |
| `target/release/p10 < input/p10.txt` | 1.1 ± 0.1 | 0.9 | 1.3 | 2.04 ± 0.46 |
| `target/release/p11 < input/p11.txt` | 10.5 ± 0.5 | 9.8 | 11.5 | 20.28 ± 4.30 |
| `target/release/p12 < input/p12.txt` | 7.2 ± 1.4 | 6.3 | 10.7 | 13.87 ± 3.92 |
| `target/release/p13 < input/p13.txt` | 3.1 ± 0.2 | 2.9 | 3.4 | 5.98 ± 1.27 |
| `target/release/p14 < input/p14.txt` | 25.3 ± 0.4 | 24.8 | 26.3 | 49.08 ± 10.18 |
| `target/release/p15 < input/p15.txt` | 3.0 ± 0.8 | 2.1 | 5.0 | 5.91 ± 1.98 |
| `target/release/p16 < input/p16.txt` | 33.8 ± 0.8 | 33.2 | 35.9 | 65.56 ± 13.64 |
| `target/release/p17 < input/p17.txt` | 1.2 ± 0.2 | 1.0 | 1.7 | 2.31 ± 0.60 |
| `target/release/p18 < input/p18.txt` | 3.3 ± 1.3 | 2.0 | 5.5 | 6.42 ± 2.83 |
| `target/release/p19 < input/p19.txt` | 28.9 ± 0.2 | 28.7 | 29.3 | 55.93 ± 11.57 |

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

