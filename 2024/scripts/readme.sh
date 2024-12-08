#!/bin/bash

cargo build --release

cat > README.md <<EOF
# Advent of Code 2024

These are my solutions for the 2024 Advent of Code. They are written in Rust. Goals are
readable code, clean algorithms and decent runtimes. Am sub 100ms for all problems so far.
This probably will not continue...

## Libraries

I tend to rely on minimal external libraries though do have a few standard ones I use.

- anyhow
- regex
- itertools
- rayon (using this for the first time this year. \`par_iter\` ftw!)
- num (if big ints are required)
- rustworkx-core (used once last year)

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
