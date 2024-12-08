# Advent of Code 2024

These are my solutions for the 2024 Advent of Code. They are written in Rust. Goals are
readable code, clean algorithms and decent runtimes. Am sub 100ms for all problems so far.
This probably will not continue...

## Libraries

I tend to rely on minimal external libraries though do have a few standard ones I use.

- anyhow
- regex
- itertools
- rayon (using this for the first time this year.  ftw!)
- num (if big ints are required)
- rustworkx-core (used once last year)

## Benchmarks

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/p01 < input/p01.txt` | 0.7 ± 0.1 | 0.6 | 1.0 | 1.00 |
| `target/release/p02 < input/p02.txt` | 1.4 ± 0.6 | 0.9 | 2.5 | 2.02 ± 0.96 |
| `target/release/p03 < input/p03.txt` | 4.5 ± 0.7 | 3.3 | 5.2 | 6.58 ± 1.43 |
| `target/release/p04 < input/p04.txt` | 5.5 ± 0.6 | 5.1 | 7.2 | 7.98 ± 1.57 |
| `target/release/p05 < input/p05.txt` | 30.8 ± 0.4 | 30.4 | 31.6 | 44.73 ± 7.16 |
| `target/release/p06 < input/p06.txt` | 88.6 ± 1.6 | 87.1 | 92.3 | 128.62 ± 20.67 |
| `target/release/p07 < input/p07.txt` | 49.3 ± 1.5 | 47.0 | 52.3 | 71.64 ± 11.64 |

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

