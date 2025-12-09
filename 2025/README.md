# Advent of Code 2025

These are my solutions for the 2025 Advent of Code (AoC). They are written in Rust. Goals are
readable code, clean algorithms and decent runtimes.

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
- [phf](https://crates.io/crates/phf) (perfect hash functions for static data)
- [rayon](https://crates.io/crates/rayon) (used for the first time last year. `par_iter` ftw!)
- [num](https://crates.io/crates/num) (if big ints are required)
- [rustworkx-core](https://crates.io/crates/rustworkx-core) (used once in a previous year)

## Benchmarks

| Command | Mean [ms] | Min [ms] | Max [ms] |
|:---|---:|---:|---:|
| `target/release/p01 < input/p01.txt` | 4.3 ± 1.3 | 2.9 | 6.4 |
| `target/release/p02 < input/p02.txt` | 17.3 ± 2.9 | 14.1 | 23.0 |
| `target/release/p03 < input/p03.txt` | 0.7 ± 0.1 | 0.6 | 0.9 |
| `target/release/p04 < input/p04.txt` | 16.6 ± 0.3 | 16.2 | 17.2 |
| `target/release/p05 < input/p05.txt` | 1.2 ± 0.5 | 0.5 | 1.8 |
| `target/release/p06 < input/p06.txt` | 2.4 ± 0.5 | 1.7 | 3.1 |
| `target/release/p07 < input/p07.txt` | 3.8 ± 0.6 | 2.4 | 4.1 |
| `target/release/p08 < input/p08.txt` | 20.6 ± 0.5 | 20.1 | 21.8 |

## Lines of Code

| Language | Provider | Filename | Lines | Code | Comments | Blanks | Complexity | Bytes | ULOC |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Rust | src/bin/p01.rs | p01.rs | 50 | 38 | 0 | 12 | 8 | 1100 | 0 |
| Rust | src/bin/p02.rs | p02.rs | 79 | 58 | 0 | 21 | 11 | 1758 | 0 |
| Rust | src/bin/p03.rs | p03.rs | 59 | 43 | 1 | 15 | 5 | 1387 | 0 |
| Rust | src/bin/p04.rs | p04.rs | 73 | 54 | 0 | 19 | 10 | 1528 | 0 |
| Rust | src/bin/p05.rs | p05.rs | 62 | 44 | 4 | 14 | 11 | 1504 | 0 |
| Rust | src/bin/p06.rs | p06.rs | 77 | 54 | 5 | 18 | 11 | 1991 | 0 |
| Rust | src/bin/p07.rs | p07.rs | 53 | 37 | 3 | 13 | 8 | 1426 | 0 |
| Rust | src/bin/p08.rs | p08.rs | 107 | 75 | 8 | 24 | 16 | 2901 | 0 |

