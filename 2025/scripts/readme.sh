#!/bin/bash

cargo build --release

cat > README.md <<EOF
# Advent of Code 2025

These are my solutions for the 2025 Advent of Code (AoC). They are written in Rust. Goals are
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
- [rayon](https://crates.io/crates/rayon) (used for the first time last year. \`par_iter\` ftw!)
- [num](https://crates.io/crates/num) (if big ints are required)
- [rustworkx-core](https://crates.io/crates/rustworkx-core) (used once in a previous year)

## Benchmarks

EOF

TARGETS=$(basename -a target/release/p??)

DAYS=${TARGETS//$'\n'/,}

hyperfine -u millisecond -w 3 -r 10 -L day ${DAYS} "target/release/{day} < input/{day}.txt" --export-markdown hf.md
cat hf.md >> README.md
rm hf.md
echo >> README.md

echo "## Lines of Code" >> README.md
echo >> README.md
scc --by-file -i rs -f csv --exclude-file template.rs --exclude-file csv2md.rs | cargo run --release --bin csv2md >> README.md
echo >> README.md
