#!/bin/bash

cargo build --release

echo "# Advent of Code 2024" > README.md
echo >> README.md

echo "## Benchmarks" >> README.md
echo >> README.md

TARGETS=$(basename -a target/release/p??)

DAYS=${TARGETS//$'\n'/,}

hyperfine -w 3 -r 10 -L day ${DAYS} "target/release/{day} < input/{day}.txt" --export-markdown hf.md
cat hf.md >> README.md
rm hf.md
echo >> README.md

echo "## Lines of Code" >> README.md
echo >> README.md
scc --by-file -i rs -f csv --exclude-file template.rs --exclude-file csv2md.rs | cargo run --release --bin csv2md >> README.md
echo >> README.md
