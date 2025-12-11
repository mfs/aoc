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
| `target/release/p01 < input/p01.txt` | 5.8 ± 1.7 | 3.1 | 7.9 |
| `target/release/p02 < input/p02.txt` | 16.8 ± 2.7 | 14.0 | 21.6 |
| `target/release/p03 < input/p03.txt` | 0.9 ± 0.2 | 0.7 | 1.4 |
| `target/release/p04 < input/p04.txt` | 18.2 ± 0.2 | 17.9 | 18.6 |
| `target/release/p05 < input/p05.txt` | 0.8 ± 0.1 | 0.7 | 0.9 |
| `target/release/p06 < input/p06.txt` | 2.2 ± 0.9 | 1.2 | 3.7 |
| `target/release/p07 < input/p07.txt` | 4.6 ± 0.1 | 4.4 | 4.7 |
| `target/release/p08 < input/p08.txt` | 20.6 ± 0.4 | 20.0 | 21.5 |
| `target/release/p09 < input/p09.txt` | 24.2 ± 0.4 | 23.5 | 24.7 |
| `target/release/p10 < input/p10.txt` | 277.0 ± 2.4 | 275.5 | 283.3 |
| `target/release/p11 < input/p11.txt` | 1.0 ± 0.1 | 0.9 | 1.1 |

## Lines of Code

| Language | Provider | Filename | Lines | Code | Comments | Blanks | Complexity | Bytes | ULOC |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Rust | target/release/build/highs-sys-988c7919d4c66132/out/c_bindings.rs | c_bindings.rs | 2066 | 2064 | 1 | 1 | 0 | 136494 | 0 |
| Rust | target/debug/build/highs-sys-219d6521038a7a5e/out/c_bindings.rs | c_bindings.rs | 2066 | 2064 | 1 | 1 | 0 | 136494 | 0 |
| Rust | target/debug/build/highs-sys-5deba722494e6630/out/c_bindings.rs | c_bindings.rs | 2066 | 2064 | 1 | 1 | 0 | 136494 | 0 |
| Rust | target/release/build/clang-sys-cf2c84d6524b2037/out/common.rs | common.rs | 355 | 232 | 80 | 43 | 37 | 13746 | 0 |
| Rust | target/debug/build/clang-sys-74128bf577510b94/out/common.rs | common.rs | 355 | 232 | 80 | 43 | 37 | 13746 | 0 |
| Rust | target/debug/build/clang-sys-74128bf577510b94/out/dynamic.rs | dynamic.rs | 276 | 177 | 58 | 41 | 51 | 9945 | 0 |
| Rust | target/release/build/clang-sys-cf2c84d6524b2037/out/dynamic.rs | dynamic.rs | 276 | 177 | 58 | 41 | 51 | 9945 | 0 |
| Rust | target/release/build/clang-sys-cf2c84d6524b2037/out/macros.rs | macros.rs | 49 | 43 | 1 | 5 | 17 | 1429 | 0 |
| Rust | target/debug/build/clang-sys-74128bf577510b94/out/macros.rs | macros.rs | 49 | 43 | 1 | 5 | 17 | 1429 | 0 |
| Rust | src/bin/p01.rs | p01.rs | 50 | 38 | 0 | 12 | 8 | 1100 | 0 |
| Rust | src/bin/p02.rs | p02.rs | 79 | 58 | 0 | 21 | 11 | 1758 | 0 |
| Rust | src/bin/p03.rs | p03.rs | 59 | 43 | 1 | 15 | 5 | 1387 | 0 |
| Rust | src/bin/p04.rs | p04.rs | 73 | 54 | 0 | 19 | 10 | 1528 | 0 |
| Rust | src/bin/p05.rs | p05.rs | 62 | 44 | 4 | 14 | 11 | 1504 | 0 |
| Rust | src/bin/p06.rs | p06.rs | 77 | 54 | 5 | 18 | 11 | 1991 | 0 |
| Rust | src/bin/p07.rs | p07.rs | 53 | 37 | 3 | 13 | 8 | 1426 | 0 |
| Rust | src/bin/p08.rs | p08.rs | 101 | 70 | 7 | 24 | 15 | 2547 | 0 |
| Rust | src/bin/p09.rs | p09.rs | 89 | 64 | 5 | 20 | 22 | 2419 | 0 |
| Rust | src/bin/p10.rs | p10.rs | 151 | 105 | 11 | 35 | 22 | 3753 | 0 |
| Rust | src/bin/p11.rs | p11.rs | 77 | 55 | 0 | 22 | 18 | 1711 | 0 |

