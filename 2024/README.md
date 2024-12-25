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
- [phf](https://crates.io/crates/phf) (perfect hash functions for static data)
- [rayon](https://crates.io/crates/rayon) (using this for the first time this year. `par_iter` ftw!)
- [num](https://crates.io/crates/num) (if big ints are required)
- [rustworkx-core](https://crates.io/crates/rustworkx-core) (used once last year)

## Benchmarks

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/p01 < input/p01.txt` | 2.7 ± 0.1 | 2.5 | 3.0 | 112.19 ± 161.40 |
| `target/release/p02 < input/p02.txt` | 2.7 ± 1.3 | 0.9 | 4.3 | 112.21 ± 169.77 |
| `target/release/p03 < input/p03.txt` | 4.6 ± 0.3 | 4.1 | 5.1 | 188.42 ± 271.26 |
| `target/release/p04 < input/p04.txt` | 4.2 ± 0.1 | 4.1 | 4.4 | 172.67 ± 248.30 |
| `target/release/p05 < input/p05.txt` | 28.5 ± 0.7 | 27.9 | 30.1 | 1170.27 ± 1682.97 |
| `target/release/p06 < input/p06.txt` | 86.4 ± 1.4 | 85.0 | 88.1 | 3547.48 ± 5101.25 |
| `target/release/p07 < input/p07.txt` | 47.7 ± 2.0 | 45.5 | 52.1 | 1960.42 ± 2820.04 |
| `target/release/p08 < input/p08.txt` | 0.0 ± 0.0 | 0.0 | 0.1 | 1.00 |
| `target/release/p09 < input/p09.txt` | 9.5 ± 0.3 | 9.1 | 10.0 | 390.54 ± 561.69 |
| `target/release/p10 < input/p10.txt` | 3.4 ± 0.6 | 2.4 | 4.3 | 139.05 ± 201.36 |
| `target/release/p11 < input/p11.txt` | 9.4 ± 1.2 | 8.5 | 12.3 | 387.52 ± 559.38 |
| `target/release/p12 < input/p12.txt` | 6.4 ± 0.8 | 5.8 | 8.6 | 263.03 ± 379.80 |
| `target/release/p13 < input/p13.txt` | 2.4 ± 0.0 | 2.3 | 2.4 | 97.03 ± 139.54 |
| `target/release/p14 < input/p14.txt` | 24.5 ± 1.0 | 23.2 | 26.6 | 1004.05 ± 1444.28 |
| `target/release/p15 < input/p15.txt` | 1.7 ± 0.1 | 1.6 | 1.8 | 69.31 ± 99.68 |
| `target/release/p16 < input/p16.txt` | 32.4 ± 0.8 | 30.6 | 33.7 | 1331.90 ± 1915.46 |
| `target/release/p17 < input/p17.txt` | 0.7 ± 0.1 | 0.6 | 0.9 | 28.81 ± 41.56 |
| `target/release/p18 < input/p18.txt` | 1.6 ± 0.1 | 1.5 | 1.8 | 65.23 ± 93.85 |
| `target/release/p19 < input/p19.txt` | 26.9 ± 0.7 | 26.0 | 28.5 | 1106.02 ± 1590.64 |
| `target/release/p20 < input/p20.txt` | 16.4 ± 0.5 | 16.0 | 17.5 | 675.01 ± 970.87 |
| `target/release/p21 < input/p21.txt` | 1.6 ± 0.0 | 1.5 | 1.6 | 64.07 ± 92.13 |
| `target/release/p22 < input/p22.txt` | 177.1 ± 2.2 | 174.5 | 181.7 | 7271.66 ± 10456.36 |
| `target/release/p23 < input/p23.txt` | 217.5 ± 7.9 | 213.4 | 239.4 | 8929.86 ± 12844.36 |
| `target/release/p24 < input/p24.txt` | 0.3 ± 0.1 | 0.2 | 0.7 | 13.34 ± 20.04 |
| `target/release/p25 < input/p25.txt` | 2.8 ± 0.7 | 1.4 | 3.6 | 113.15 ± 165.36 |

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
| Rust | src/bin/p20.rs | p20.rs | 109 | 82 | 0 | 27 | 19 | 2313 | 0 |
| Rust | src/bin/p21.rs | p21.rs | 109 | 80 | 3 | 26 | 13 | 2980 | 0 |
| Rust | src/bin/p22.rs | p22.rs | 71 | 49 | 1 | 21 | 6 | 1540 | 0 |
| Rust | src/bin/p23.rs | p23.rs | 93 | 64 | 2 | 27 | 11 | 2368 | 0 |
| Rust | src/bin/p24.rs | p24.rs | 162 | 129 | 6 | 27 | 34 | 4842 | 0 |
| Rust | src/bin/p25.rs | p25.rs | 54 | 39 | 0 | 15 | 10 | 1089 | 0 |

