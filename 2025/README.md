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
| `target/release/p01 < input/p01.txt` | 8.1 ± 1.8 | 4.0 | 9.9 |
| `target/release/p02 < input/p02.txt` | 159.8 ± 0.5 | 159.0 | 160.6 |
| `target/release/p03 < input/p03.txt` | 0.2 ± 0.0 | 0.1 | 0.2 |

## Lines of Code

| Language | Provider | Filename | Lines | Code | Comments | Blanks | Complexity | Bytes | ULOC |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Rust | src/bin/p01.rs | p01.rs | 50 | 38 | 0 | 12 | 8 | 1100 | 0 |
| Rust | src/bin/p02.rs | p02.rs | 75 | 55 | 0 | 20 | 12 | 1481 | 0 |
| Rust | src/bin/p03.rs | p03.rs | 59 | 43 | 1 | 15 | 5 | 1354 | 0 |

