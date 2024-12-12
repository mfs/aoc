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
| `target/release/p01 < input/p01.txt` | 3.2 ± 0.1 | 3.0 | 3.5 | 9.84 ± 1.20 |
| `target/release/p02 < input/p02.txt` | 3.7 ± 1.0 | 2.0 | 4.7 | 11.46 ± 3.30 |
| `target/release/p03 < input/p03.txt` | 4.8 ± 0.9 | 2.8 | 5.8 | 15.00 ± 3.14 |
| `target/release/p04 < input/p04.txt` | 2.9 ± 0.9 | 1.5 | 3.9 | 8.99 ± 2.91 |
| `target/release/p05 < input/p05.txt` | 30.4 ± 0.2 | 30.1 | 30.9 | 94.07 ± 10.64 |
| `target/release/p06 < input/p06.txt` | 88.0 ± 0.9 | 87.2 | 89.8 | 272.49 ± 30.87 |
| `target/release/p07 < input/p07.txt` | 49.2 ± 1.8 | 47.1 | 52.4 | 152.40 ± 18.07 |
| `target/release/p08 < input/p08.txt` | 0.3 ± 0.0 | 0.3 | 0.4 | 1.00 |
| `target/release/p09 < input/p09.txt` | 10.6 ± 1.2 | 10.0 | 13.9 | 32.88 ± 5.30 |
| `target/release/p10 < input/p10.txt` | 0.9 ± 0.0 | 0.8 | 0.9 | 2.71 ± 0.32 |
| `target/release/p11 < input/p11.txt` | 10.0 ± 0.4 | 9.6 | 10.9 | 31.04 ± 3.72 |
| `target/release/p12 < input/p12.txt` | 6.5 ± 0.3 | 6.2 | 7.2 | 20.25 ± 2.50 |

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
| Rust | src/bin/p12.rs | p12.rs | 140 | 103 | 0 | 37 | 28 | 3471 | 0 |

