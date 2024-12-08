# Advent of Code 2024

These are my solutions for the 2024 Advent of Code. They are written in Rust. Goals are
readable code, clean algorithms and decent runtimes. Am sub 100ms for all problems so far.
This probably will not continue...

## Libraries

I tend to rely on minimal external libraries though do have a few standard ones I use.

- anyhow
- regex
- itertools
- rayon (using this for the first time this year. `par_iter` ftw!)
- num (if big ints are required)
- rustworkx-core (used once last year)

## Benchmarks

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/p01 < input/p01.txt` | 3.7 ± 0.1 | 3.6 | 3.8 | 7.55 ± 0.92 |
| `target/release/p02 < input/p02.txt` | 3.7 ± 1.0 | 2.4 | 4.8 | 7.62 ± 2.17 |
| `target/release/p03 < input/p03.txt` | 4.9 ± 0.9 | 2.9 | 5.6 | 10.06 ± 2.17 |
| `target/release/p04 < input/p04.txt` | 4.6 ± 1.0 | 2.5 | 5.4 | 9.37 ± 2.32 |
| `target/release/p05 < input/p05.txt` | 30.5 ± 0.2 | 30.4 | 31.0 | 62.53 ± 7.43 |
| `target/release/p06 < input/p06.txt` | 87.2 ± 0.3 | 86.7 | 87.5 | 178.70 ± 21.20 |
| `target/release/p07 < input/p07.txt` | 48.2 ± 1.3 | 46.4 | 50.5 | 98.79 ± 11.99 |
| `target/release/p08 < input/p08.txt` | 0.5 ± 0.1 | 0.4 | 0.6 | 1.00 |

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
| Rust | src/bin/p08.rs | p08.rs | 80 | 62 | 0 | 18 | 19 | 2119 | 0 |

