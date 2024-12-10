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
| `target/release/p01 < input/p01.txt` | 0.7 ± 0.1 | 0.6 | 0.9 | 1.21 ± 0.21 |
| `target/release/p02 < input/p02.txt` | 2.9 ± 1.0 | 1.5 | 4.2 | 4.82 ± 1.68 |
| `target/release/p03 < input/p03.txt` | 4.9 ± 0.1 | 4.7 | 5.1 | 8.19 ± 0.58 |
| `target/release/p04 < input/p04.txt` | 4.7 ± 0.1 | 4.6 | 4.9 | 7.99 ± 0.56 |
| `target/release/p05 < input/p05.txt` | 29.1 ± 0.6 | 28.5 | 30.2 | 48.95 ± 3.46 |
| `target/release/p06 < input/p06.txt` | 85.7 ± 0.5 | 85.4 | 87.1 | 144.38 ± 9.84 |
| `target/release/p07 < input/p07.txt` | 48.0 ± 2.0 | 45.2 | 52.5 | 80.85 ± 6.48 |
| `target/release/p08 < input/p08.txt` | 0.6 ± 0.0 | 0.5 | 0.7 | 1.00 |
| `target/release/p09 < input/p09.txt` | 10.6 ± 0.6 | 9.9 | 11.4 | 17.80 ± 1.53 |

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

